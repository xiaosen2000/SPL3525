use anchor_lang::prelude::*;

#[account]
pub struct TokenData {
    pub token_id: u64,
    pub owner: Pubkey,
    pub slot: u64,
    pub value: u64,
    pub metadata: Pubkey,
    pub collection: Pubkey,
}

impl TokenData {
    pub const LEN: usize = 8 + // discriminator
        8 + // token_id
        32 + // owner
        8 + // slot
        8 + // value
        32 + // metadata
        32; // collection
}