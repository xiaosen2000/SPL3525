use anchor_lang::prelude::*;
use crate::state::{Collection, Token, Approval};
use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct TransferValue<'info> {
    pub collection: Account<'info, Collection>,
    
    #[account(
        mut,
        seeds = [
            b"token",
            collection.key().as_ref(),
            &from_token.id.to_le_bytes()
        ],
        bump = from_token.bump,
    )]
    pub from_token: Account<'info, Token>,
    
    #[account(
        mut,
        seeds = [
            b"token",
            collection.key().as_ref(),
            &to_token.id.to_le_bytes()
        ],
        bump = to_token.bump,
        constraint = from_token.slot == to_token.slot @ ErrorCode::SlotMismatch,
    )]
    pub to_token: Account<'info, Token>,

    #[account(
        mut,
        seeds = [
            b"approval",
            collection.key().as_ref(),
            &from_token.id.to_le_bytes(),
            from_token.owner.key().as_ref(),
            owner.key().as_ref()
        ],
        bump = approval.bump
    )]
    pub approval: Account<'info, Approval>,
    
    pub owner: Signer<'info>,
}

pub fn process_transfer_value(
    ctx: Context<TransferValue>,
    amount: u64,
) -> Result<()> {
    let from_token = &mut ctx.accounts.from_token;
    let to_token = &mut ctx.accounts.to_token;
    let approval = &mut ctx.accounts.approval;

    // Check if owner is the token owner (direct transfer) or has sufficient approval
    if from_token.owner != ctx.accounts.owner.key() {
        require!(
            amount <= approval.value,
            ErrorCode::ExceedsApproval
        );
    }

    require!(
        amount <= from_token.balance,
        ErrorCode::InsufficientValue
    );

    from_token.balance = from_token.balance
        .checked_sub(amount)
        .ok_or(ErrorCode::Overflow)?;

    to_token.balance = to_token.balance
        .checked_add(amount)
        .ok_or(ErrorCode::Overflow)?;

    emit!(ValueTransferred {
        collection: ctx.accounts.collection.key(),
        from_token: from_token.id,
        to_token: to_token.id,
        amount,
        from_owner: from_token.owner,
        to_owner: to_token.owner,
    });

    Ok(())
}

#[event]
pub struct ValueTransferred {
    #[index]
    pub collection: Pubkey,
    #[index]
    pub from_token: u64,
    #[index]
    pub to_token: u64,
    pub amount: u64,
    pub from_owner: Pubkey,
    pub to_owner: Pubkey,
}