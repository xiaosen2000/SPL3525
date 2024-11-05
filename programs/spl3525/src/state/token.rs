use anchor_lang::prelude::*;

#[account]
pub struct Token {
    pub id: u64,
    pub slot: u64,
    pub balance: u64,
    pub owner: Pubkey,
    pub bump: u8,
}

impl Token {
    pub const LEN: usize = 8 + // discriminator
        8 + // id
        8 + // slot
        8 + // balance 
        32 + // owner
        1;  // bump
}