use anchor_lang::prelude::*;

#[account]
pub struct State {
    pub authority: Pubkey,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub token_counter: u64,
    pub slot_counter: u64,
}

impl State {
    pub const LEN: usize = 8 + // discriminator
        32 + // authority
        32 + // name (max length)
        16 + // symbol (max length)
        1 + // decimals
        8 + // token_counter
        8; // slot_counter
}