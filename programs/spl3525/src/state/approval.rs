use anchor_lang::prelude::*;

#[account]
pub struct Approval {
    pub token_id: u64,
    pub owner: Pubkey,
    pub spender: Pubkey,
    pub value: u64,
    pub bump: u8,
}

impl Approval {
    pub const LEN: usize = 8 + // discriminator
        8 + // token_id
        32 + // owner
        32 + // spender
        8 + // value
        1;  // bump
}

