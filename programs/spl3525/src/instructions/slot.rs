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
    let state = &mut ctx.accounts.state;
    require!(
        state.authority == ctx.accounts.authority.key(),
        ErrorCode::InvalidAuthority
    );

    // Validate slot number
    require!(
        slot_number == state.slot_counter,
        ErrorCode::InvalidSlotNumber
    );

    let slot_data = &mut ctx.accounts.slot_data;
    slot_data.collection = state.key();
    slot_data.slot_number = slot_number;
    slot_data.metadata_uri = metadata_uri;
    slot_data.total_tokens = 0;
    slot_data.total_value = 0;

    // Increment slot counter
    state.slot_counter = state.slot_counter.checked_add(1)
        .ok_or(ErrorCode::Overflow)?;

    emit!(SlotCreated {
        collection: state.key(),
        slot_number,
        metadata_uri: slot_data.metadata_uri.clone()
    });

    Ok(())
}

#[event]
pub struct SlotCreated {
    pub collection: Pubkey,
    pub slot_number: u64,
    pub metadata_uri: String,
}