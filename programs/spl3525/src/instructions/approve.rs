use anchor_lang::prelude::*;
use crate::state::{Collection, Token, Approval};
use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct ApproveValue<'info> {
    pub collection: Account<'info, Collection>,
    
    #[account(
        seeds = [
            b"token",
            collection.key().as_ref(),
            &token.id.to_le_bytes()
        ],
        bump = token.bump,
        constraint = token.owner == owner.key() @ ErrorCode::InvalidOwner,
    )]
    pub token: Account<'info, Token>,
    
    #[account(
        init,
        payer = owner,
        space = Approval::LEN,
        seeds = [
            b"approval",
            collection.key().as_ref(),
            &token.id.to_le_bytes(),
            owner.key().as_ref(),
            spender.key().as_ref()
        ],
        bump
    )]
    pub approval: Account<'info, Approval>,
    
    #[account(mut)]
    pub owner: Signer<'info>,
    
    /// CHECK: Spender pubkey is only stored
    pub spender: AccountInfo<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn process_approve_value(
    ctx: Context<ApproveValue>,
    value: u64,
) -> Result<()> {
    let token = &ctx.accounts.token;
    let approval = &mut ctx.accounts.approval;
    let owner = ctx.accounts.owner.key();
    let spender = ctx.accounts.spender.key();

    require!(
        spender != owner,
        ErrorCode::SelfApproval
    );

    require!(
        value <= token.balance,
        ErrorCode::ExceedsBalance
    );

    approval.token_id = token.id;
    approval.owner = ctx.accounts.owner.key();
    approval.spender = ctx.accounts.spender.key();
    approval.value = value;
    approval.bump = ctx.bumps.approval;

    emit!(ValueApproved {
        collection: ctx.accounts.collection.key(),
        token_id: token.id,
        owner: approval.owner,
        spender: approval.spender,
        value,
    });

    Ok(())
}

#[event]
pub struct ValueApproved {
    #[index]
    pub collection: Pubkey,
    #[index]
    pub token_id: u64,
    pub owner: Pubkey,
    pub spender: Pubkey,
    pub value: u64,
}