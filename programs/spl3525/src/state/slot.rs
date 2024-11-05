use anchor_lang::prelude::*;

#[account]
pub struct Slot {
    pub id: u64,
    pub bump: u8,
}

impl Slot {
    pub const LEN: usize = 8 + // discriminator
        8 + // id
        1;  // bump
}