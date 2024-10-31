use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct TransferValue<'info> {
    #[account(
        mut,
        constraint = from_token.owner == owner.key()
    )]
    pub from_token: Account<'info, TokenData>,
    #[account(mut)]
    pub to_token: Account<'info, TokenData>,
    pub owner: Signer<'info>,
}

pub fn value_handler(
    ctx: Context<TransferValue>,
    value: u64,
) -> Result<()> {
    // Implementation
    Ok(())
}