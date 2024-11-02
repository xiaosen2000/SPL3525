use anchor_lang::prelude::*;
use crate::state::State;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = State::LEN)]
    pub state: Account<'info, State>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(
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
    state.slot_counter = 0;

    emit!(CollectionCreated {
        collection: state.key(),
        authority: state.authority,
        name: state.name.clone(),
        symbol: state.symbol.clone()
    });

    Ok(())
}

#[event]
pub struct CollectionCreated {
    pub collection: Pubkey,
    pub authority: Pubkey,
    pub name: String,
    pub symbol: String,
}