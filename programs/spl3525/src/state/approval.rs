use anchor_lang::prelude::*;

#[account]
pub struct ValueApproval {
    pub token_id: u64,
    pub owner: Pubkey,
    pub spender: Pubkey,
    pub value: u64,
}

impl ValueApproval {
    pub const LEN: usize = 8 + // discriminator
        8 + // token_id
        32 + // owner
        32 + // spender
        8; // value

    pub fn seeds(token_id: u64, spender: &Pubkey) -> [u8; 32] {
        let seeds = [
            b"approval",
            &token_id.to_le_bytes(),
            spender.as_ref(),
        ];
        let (pda, _bump) = Pubkey::find_program_address(&seeds[..], &crate::ID);
        pda.to_bytes()
    }
}
