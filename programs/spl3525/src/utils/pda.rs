// utils/pda.rs

use anchor_lang::prelude::*;
use crate::state::{Collection, Token, Slot, Approval};

#[derive(Debug, Clone)]
pub struct TokenPda {
    pub token_id: u64,
    pub collection: Pubkey,
    pub bump: u8,
    id_bytes: [u8; 8],
    bump_bytes: [u8; 1],
}

impl TokenPda {
    pub fn new(collection: &Pubkey, token_id: u64) -> Self {
        Self {
            token_id,
            collection: *collection,
            bump: 0,
            id_bytes: token_id.to_le_bytes(),
            bump_bytes: [0],
        }
    }

    pub fn seed_prefix() -> &'static [u8] {
        b"token"
    }

    pub fn seeds(&self) -> [&[u8]; 3] {
        [
            Self::seed_prefix(),
            self.collection.as_ref(),
            &self.id_bytes,
        ]
    }

    pub fn find(collection: &Pubkey, token_id: u64, program_id: &Pubkey) -> (Pubkey, Self) {
        let mut pda = Self::new(collection, token_id);
        let (addr, bump) = Pubkey::find_program_address(&pda.seeds(), program_id);
        pda.bump = bump;
        pda.bump_bytes = [bump];
        (addr, pda)
    }

    pub fn signer_seeds(&self) -> [&[u8]; 4] {
        [
            Self::seed_prefix(),
            self.collection.as_ref(),
            &self.id_bytes,
            &self.bump_bytes,
        ]
    }
}

#[derive(Debug, Clone)]
pub struct SlotPda {
    pub slot_id: u64,
    pub collection: Pubkey,
    pub bump: u8,
    id_bytes: [u8; 8],
    bump_bytes: [u8; 1],
}

impl SlotPda {
    pub fn new(collection: &Pubkey, slot_id: u64) -> Self {
        Self {
            slot_id,
            collection: *collection,
            bump: 0,
            id_bytes: slot_id.to_le_bytes(),
            bump_bytes: [0],
        }
    }

    pub fn seed_prefix() -> &'static [u8] {
        b"slot"
    }

    pub fn seeds(&self) -> [&[u8]; 3] {
        [
            Self::seed_prefix(),
            self.collection.as_ref(),
            &self.id_bytes,
        ]
    }

    pub fn find(collection: &Pubkey, slot_id: u64, program_id: &Pubkey) -> (Pubkey, Self) {
        let mut pda = Self::new(collection, slot_id);
        let (addr, bump) = Pubkey::find_program_address(&pda.seeds(), program_id);
        pda.bump = bump;
        pda.bump_bytes = [bump];
        (addr, pda)
    }

    pub fn signer_seeds(&self) -> [&[u8]; 4] {
        [
            Self::seed_prefix(),
            self.collection.as_ref(),
            &self.id_bytes,
            &self.bump_bytes,
        ]
    }
}

#[derive(Debug, Clone)]
pub struct ApprovalPda {
    pub token_id: u64,
    pub collection: Pubkey,
    pub owner: Pubkey,
    pub spender: Pubkey,
    pub bump: u8,
    id_bytes: [u8; 8],
    bump_bytes: [u8; 1],
}

impl ApprovalPda {
    pub fn new(collection: &Pubkey, token_id: u64, owner: &Pubkey, spender: &Pubkey) -> Self {
        Self {
            token_id,
            collection: *collection,
            owner: *owner,
            spender: *spender,
            bump: 0,
            id_bytes: token_id.to_le_bytes(),
            bump_bytes: [0],
        }
    }

    pub fn seed_prefix() -> &'static [u8] {
        b"approval"
    }

    pub fn seeds(&self) -> [&[u8]; 5] {
        [
            Self::seed_prefix(),
            self.collection.as_ref(),
            &self.id_bytes,
            self.owner.as_ref(),
            self.spender.as_ref(),
        ]
    }

    pub fn find(
        collection: &Pubkey,
        token_id: u64,
        owner: &Pubkey,
        spender: &Pubkey,
        program_id: &Pubkey
    ) -> (Pubkey, Self) {
        let mut pda = Self::new(collection, token_id, owner, spender);
        let (addr, bump) = Pubkey::find_program_address(&pda.seeds(), program_id);
        pda.bump = bump;
        pda.bump_bytes = [bump];
        (addr, pda)
    }

    pub fn signer_seeds(&self) -> [&[u8]; 6] {
        [
            Self::seed_prefix(),
            self.collection.as_ref(),
            &self.id_bytes,
            self.owner.as_ref(),
            self.spender.as_ref(),
            &self.bump_bytes,
        ]
    }
}