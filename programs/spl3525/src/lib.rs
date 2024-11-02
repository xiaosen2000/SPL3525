use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use mpl_token_metadata::{
    instruction as metadata_instruction,
    state::{Creator as MetaplexCreator, Collection, DataV2, Uses},
};
pub use crate::instructions::*;
declare_id!("5ApDdwMbhHBSbmSgJEUXSSUgRnoQi2aMPQ4mpACKgG9y");

pub mod errors;
pub mod instructions;
pub mod state;
pub mod utils;

#[program]
pub mod spl3525 {
    use super::*;

    /// Initialize a new SPL3525 token mint
    /// 
    /// # Parameters
    /// * `ctx` - Context containing token mint account and authority
    /// * `name` - Name of the token collection
    /// * `symbol` - Symbol of the token collection
    /// * `decimals` - Decimal places for token values

    pub fn initialize(
        ctx: Context<Initialize>,
        name: String,
        symbol: String,
        decimals: u8,
    ) -> Result<()> {
        process_initialize(
            ctx,
            name,
            symbol,
            decimals,
        )
    }

    /// Create a new slot for tokens
    /// 
    /// # Parameters
    /// * `ctx` - Context containing slot data account
    /// * `slot` - Unique identifier for the slot
    /// * `metadata_uri` - URI for slot metadata
    pub fn create_slot(
        ctx: Context<CreateSlot>,
        slot: u64,
        metadata_uri: String,
    ) -> Result<()> {
        process_create_slot(ctx, slot, metadata_uri)
    }

    /// Mint a new token with specified slot and value
    /// 
    /// # Parameters
    /// * `ctx` - Context containing token accounts
    /// * `slot` - Slot identifier for the token
    /// * `value` - Initial value of the token
    pub fn mint(
        ctx: Context<MintToken>,
        slot: u64,
        value: u64,
    ) -> Result<()> {
        process_mint(ctx, slot, value)
    }

    /// Transfer value between tokens of the same slot
    /// 
    /// # Parameters
    /// * `ctx` - Context containing source and destination tokens
    /// * `value` - Amount to transfer
    pub fn transfer_value(
        ctx: Context<TransferValue>,
        value: u64,
    ) -> Result<()> {
        process_transfer_value(ctx, value)
    }

    /// Approve an operator to manage token value
    /// 
    /// # Parameters
    /// * `ctx` - Context containing token and approval accounts
    /// * `value` - Maximum value that can be managed
    pub fn approve_value(
        ctx: Context<ApproveValue>,
        value: u64,
    ) -> Result<()> {
        process_approve_value(ctx, value)
    }
}