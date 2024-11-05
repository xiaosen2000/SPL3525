use anchor_lang::prelude::*;

#[account]
pub struct Collection {
    pub authority: Pubkey,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub mint: Pubkey,           // Added: Collection mint address
    pub metadata: Pubkey,       // Added: Collection metadata address
}

impl Collection {
    pub const LEN: usize = 8 + // discriminator
        32 + // authority
        32 + // name (max length)
        16 + // symbol (max length)
        1 +  // decimals
        32 + // mint
        32;  // metadata
}