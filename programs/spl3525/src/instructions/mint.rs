use anchor_lang::prelude::*;
use crate::utils::verify_and_create_metadata;
use crate::state::{State, SlotData, TokenData};
use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct MintToken<'info> {
    #[account(mut)]
    pub state: Account<'info, State>,
    
    #[account(mut)]
    pub slot_data: Account<'info, SlotData>,
    
    #[account(
        init,
        payer = authority,
        space = TokenData::LEN
    )]
    pub token_data: Account<'info, TokenData>,
    
    /// CHECK: Metaplex metadata account, created in handler
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
    
    pub rent: Sysvar<'info, Rent>,
    
    /// CHECK: Token Metadata Program
    pub token_metadata_program: UncheckedAccount<'info>,
}

pub fn handler(
    ctx: Context<MintToken>,
    slot: u64,
    value: u64,
) -> Result<()> {
    let state = &mut ctx.accounts.state;
    let slot_data = &mut ctx.accounts.slot_data;

    // Verify slot
    require!(
        slot_data.slot_number == slot,
        ErrorCode::InvalidSlot
    );
    require!(
        slot_data.collection == state.key(),
        ErrorCode::InvalidCollection
    );

    // Create token data
    let token_data = &mut ctx.accounts.token_data;
    let token_id = state.token_counter;

    token_data.token_id = token_id;
    token_data.owner = ctx.accounts.authority.key();
    token_data.slot = slot;
    token_data.value = value;
    token_data.metadata = ctx.accounts.metadata.key();
    token_data.collection = state.key();

    // Update slot totals
    slot_data.total_tokens = slot_data.total_tokens
        .checked_add(1)
        .ok_or(ErrorCode::Overflow)?;
    slot_data.total_value = slot_data.total_value
        .checked_add(value)
        .ok_or(ErrorCode::Overflow)?;

    // Create Metaplex metadata
    verify_and_create_metadata(
        &ctx,
        state.name.clone(),
        state.symbol.clone(),
        slot_data.metadata_uri.clone(),
    )?;

    // Increment token counter
    state.token_counter = state.token_counter
        .checked_add(1)
        .ok_or(ErrorCode::Overflow)?;

    emit!(TokenMinted {
        collection: state.key(),
        token_id,
        slot,
        owner: token_data.owner,
        value,
        metadata: ctx.accounts.metadata.key(),
    });

    Ok(())
}

#[event]
pub struct TokenMinted {
    pub collection: Pubkey,
    pub token_id: u64,
    pub slot: u64,
    pub owner: Pubkey,
    pub value: u64,
    pub metadata: Pubkey,
}