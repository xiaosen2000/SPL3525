use anchor_lang::prelude::*;
use crate::state::{Collection, Slot};
use crate::errors::ErrorCode;

#[derive(Accounts)]
#[instruction(slot_id: u64)]
pub struct CreateSlot<'info> {
    #[account(
        has_one = authority @ ErrorCode::InvalidAuthority,
    )]
    pub collection: Account<'info, Collection>,
    
    #[account(
        init,
        payer = authority,
        space = Slot::LEN,
        seeds = [
            b"slot",
            collection.key().as_ref(),
            &slot_id.to_le_bytes()
        ],
        bump
    )]
    pub slot: Account<'info, Slot>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn process_create_slot(
    ctx: Context<CreateSlot>,
    slot_id: u64,
) -> Result<()> {
    let slot = &mut ctx.accounts.slot;
    slot.id = slot_id;
    slot.bump = ctx.bumps.slot;

    emit!(SlotCreated {
        collection: ctx.accounts.collection.key(),
        slot_id,
    });

    Ok(())
}

#[event]
pub struct SlotCreated {
    #[index]
    pub collection: Pubkey,
    #[index]
    pub slot_id: u64,
}