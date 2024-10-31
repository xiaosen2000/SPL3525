use anchor_lang::prelude::*;

#[account]
pub struct SlotData {
    pub collection: Pubkey,
    pub slot_number: u64,
    pub metadata_uri: String,
    pub total_tokens: u64,
    pub total_value: u64,
}

impl SlotData {
    pub const LEN: usize = 8 + // discriminator
        32 + // collection
        8 + // slot_number
        64 + // metadata_uri
        8 + // total_tokens
        8; // total_value

    pub fn seeds(collection: &Pubkey, slot_number: u64) -> [u8; 32] {
        let seeds = [
            b"slot",
            collection.as_ref(),
            &slot_number.to_le_bytes(),
        ];
        let (pda, _bump) = Pubkey::find_program_address(&seeds[..], &crate::ID);
        pda.to_bytes()
    }
}