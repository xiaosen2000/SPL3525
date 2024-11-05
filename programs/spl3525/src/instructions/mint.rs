use anchor_lang::prelude::*;
use crate::state::{Collection, Slot, Token};
use crate::errors::ErrorCode;

#[derive(Accounts)]
#[instruction(token_id: u64)]
pub struct MintToken<'info> {
    #[account(
        has_one = authority @ ErrorCode::InvalidAuthority,
    )]
    pub collection: Account<'info, Collection>,
    
    #[account(
        seeds = [
            b"slot",
            collection.key().as_ref(),
            &slot.id.to_le_bytes()
        ],
        bump = slot.bump
    )]
    pub slot: Account<'info, Slot>,
    
    #[account(
        init,
        payer = authority,
        space = Token::LEN,
        seeds = [
            b"token",
            collection.key().as_ref(),
            &token_id.to_le_bytes()
        ],
        bump
    )]
    pub token: Account<'info, Token>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn process_mint(
    ctx: Context<MintToken>,
    token_id: u64,
    balance: u64,
) -> Result<()> {
    let token = &mut ctx.accounts.token;
    token.id = token_id;
    token.slot = ctx.accounts.slot.id;
    token.balance = balance;
    token.owner = ctx.accounts.authority.key();
    token.bump = ctx.bumps.token;

    emit!(TokenMinted {
        collection: ctx.accounts.collection.key(),
        token_id,
        slot_id: token.slot,
        owner: token.owner,
        balance,
    });

    Ok(())
}

#[event]
pub struct TokenMinted {
    #[index]
    pub collection: Pubkey,
    #[index]
    pub token_id: u64,
    pub slot_id: u64,
    pub owner: Pubkey,
    pub balance: u64,
}