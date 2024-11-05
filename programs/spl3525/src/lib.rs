use anchor_lang::prelude::*;

pub mod errors;
pub mod instructions;
pub mod state;
pub mod utils;

use instructions::*;

declare_id!("5ApDdwMbhHBSbmSgJEUXSSUgRnoQi2aMPQ4mpACKgG9y");

#[program]
pub mod spl3525 {
    use super::*;

    /// Initialize a new collection for SPL3525 tokens
    /// 
    /// # Arguments
    /// * `ctx` - Account context containing collection and authority
    /// * `name` - Name of the collection
    /// * `symbol` - Symbol of the collection
    /// * `decimals` - Number of decimal places for token values
    /// * `uri`` - URI
    pub fn initialize(
        ctx: Context<Initialize>,
        name: String,
        symbol: String,
        decimals: u8,
        uri: String,
    ) -> Result<()> {
        process_initialize(ctx, name, symbol, decimals, uri)
    }

    /// Create a new slot for tokens within a collection
    /// 
    /// # Arguments
    /// * `ctx` - Account context containing collection, slot and authority
    /// * `slot_id` - Unique identifier for the slot
    pub fn create_slot(
        ctx: Context<CreateSlot>,
        slot_id: u64,
    ) -> Result<()> {
        process_create_slot(ctx, slot_id)
    }

    /// Mint a new token with specified slot and initial balance
    /// 
    /// # Arguments
    /// * `ctx` - Account context containing collection, slot, token and authority
    /// * `token_id` - Unique identifier for the token
    /// * `balance` - Initial balance for the token
    pub fn mint(
        ctx: Context<MintToken>,
        token_id: u64,
        balance: u64,
    ) -> Result<()> {
        process_mint(ctx, token_id, balance)
    }

    /// Transfer value between tokens of the same slot
    /// 
    /// # Arguments
    /// * `ctx` - Account context containing source and destination tokens
    /// * `amount` - Amount to transfer
    pub fn transfer_value(
        ctx: Context<TransferValue>,
        amount: u64,
    ) -> Result<()> {
        process_transfer_value(ctx, amount)
    }

    /// Approve an operator to manage token value
    /// 
    /// # Arguments
    /// * `ctx` - Account context containing token, approval and authority accounts
    /// * `value` - Maximum value that can be managed
    pub fn approve_value(
        ctx: Context<ApproveValue>,
        value: u64,
    ) -> Result<()> {
        process_approve_value(ctx, value)
    }
}