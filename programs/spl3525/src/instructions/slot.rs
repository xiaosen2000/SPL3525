use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CreateSlot<'info> {
    #[account(mut)]
    pub state: Account<'info, State>,
    #[account(
        init,
        payer = authority,
        space = SlotData::LEN,
        seeds = [b"slot", state.key().as_ref(), &slot_number.to_le_bytes()],
        bump
    )]
    pub slot_data: Account<'info, SlotData>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn create_handler(
    ctx: Context<CreateSlot>,
    slot_number: u64,
    metadata_uri: String,
) -> Result<()> {
    // Implementation
    Ok(())
}