use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
declare_id!("5ApDdwMbhHBSbmSgJEUXSSUgRnoQi2aMPQ4mpACKgG9y");

#[program]
pub mod spl3525 {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        name: String,
        symbol: String,
        decimals: u8,
    ) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.authority = ctx.accounts.authority.key();
        state.name = name;
        state.symbol = symbol;
        state.decimals = decimals;
        state.token_counter = 0;
        Ok(())
    }

    pub fn mint(
        ctx: Context<MintToken>,
        slot: u64,
        value: u64,
    ) -> Result<()> {
        let state = &mut ctx.accounts.state;
        let token_id = state.token_counter;
        
        // Create new token data
        let token_data = &mut ctx.accounts.token_data;
        token_data.owner = ctx.accounts.owner.key();
        token_data.slot = slot;
        token_data.value = value;
        token_data.token_id = token_id;
        
        state.token_counter = state.token_counter.checked_add(1).unwrap();
        
        Ok(())
    }

    pub fn transfer(
        ctx: Context<Transfer>,
        value: u64,
    ) -> Result<()> {
        let from_token = &mut ctx.accounts.from_token;
        let to_token = &mut ctx.accounts.to_token;
        
        require!(
            from_token.slot == to_token.slot,
            ErrorCode::SlotMismatch
        );
        
        require!(
            value <= from_token.value,
            ErrorCode::InsufficientValue
        );
        
        from_token.value = from_token.value.checked_sub(value).unwrap();
        to_token.value = to_token.value.checked_add(value).unwrap();
        
        Ok(())
    }

    pub fn approve_value(
        ctx: Context<ApproveValue>,
        value: u64,
    ) -> Result<()> {
        let approval = &mut ctx.accounts.approval;
        approval.value = value;
        approval.owner = ctx.accounts.owner.key();
        approval.spender = ctx.accounts.spender.key();
        approval.token_id = ctx.accounts.token_data.token_id;
        
        Ok(())
    }
}

#[account]
pub struct State {
    pub authority: Pubkey,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub token_counter: u64,
}

#[account]
pub struct TokenData {
    pub token_id: u64,
    pub owner: Pubkey,
    pub slot: u64,
    pub value: u64,
}

#[account]
pub struct ValueApproval {
    pub token_id: u64,
    pub owner: Pubkey,
    pub spender: Pubkey,
    pub value: u64,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 8 + 32 + 32 + 32 + 1 + 8)]
    pub state: Account<'info, State>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MintToken<'info> {
    #[account(mut)]
    pub state: Account<'info, State>,
    #[account(
        init,
        payer = authority,
        space = 8 + 8 + 32 + 8 + 8
    )]
    pub token_data: Account<'info, TokenData>,
    /// CHECK: This account is not written to or read from
    pub owner: AccountInfo<'info>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Transfer<'info> {
    #[account(
        mut,
        constraint = from_token.owner == owner.key()
    )]
    pub from_token: Account<'info, TokenData>,
    #[account(mut)]
    pub to_token: Account<'info, TokenData>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct ApproveValue<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + 8 + 32 + 32 + 8
    )]
    pub approval: Account<'info, ValueApproval>,
    #[account(
        constraint = token_data.owner == owner.key()
    )]
    pub token_data: Account<'info, TokenData>,
    #[account(mut)]
    pub owner: Signer<'info>,
    /// CHECK: This account is only used as a reference for approval
    pub spender: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Token slots must match for transfer")]
    SlotMismatch,
    #[msg("Insufficient value for transfer")]
    InsufficientValue,
}