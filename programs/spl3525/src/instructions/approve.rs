use anchor_lang::prelude::*;

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
    // Implementation
    Ok(())
}