use anchor_lang::prelude::*;
use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct ApproveValue<'info> {
    #[account(
        init,
        payer = owner,
        space = ValueApproval::LEN,
        seeds = [b"approval", &token_data.token_id.to_le_bytes(), spender.key().as_ref()],
        bump
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

pub fn value_handler(
    ctx: Context<ApproveValue>,
    value: u64,
) -> Result<()> {
    let token_data = &ctx.accounts.token_data;
    let approval = &mut ctx.accounts.approval;

    // Verify owner
    require!(
        token_data.owner == ctx.accounts.owner.key(),
        ErrorCode::InvalidOwner
    );

    // Check value doesn't exceed token value
    require!(
        value <= token_data.value,
        ErrorCode::ExceedsBalance
    );

    // Set approval
    approval.token_id = token_data.token_id;
    approval.owner = ctx.accounts.owner.key();
    approval.spender = ctx.accounts.spender.key();
    approval.value = value;

    emit!(ValueApproved {
        collection: token_data.collection,
        token_id: token_data.token_id,
        owner: approval.owner,
        spender: approval.spender,
        value,
    });

    Ok(())
}

#[event]
pub struct ValueApproved {
    pub collection: Pubkey,
    pub token_id: u64,
    pub owner: Pubkey,
    pub spender: Pubkey,
    pub value: u64,
}