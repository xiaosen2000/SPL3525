use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    program::invoke_signed,
    system_program,
};
use mpl_token_metadata::state::PREFIX as METADATA_PREFIX;

pub mod constants {
    pub const PREFIX: &[u8] = b"spl3525";
    pub const SLOT_PREFIX: &[u8] = b"slot";
    pub const TOKEN_PREFIX: &[u8] = b"token";
    pub const APPROVAL_PREFIX: &[u8] = b"approval";
}

pub fn verify_and_create_metadata(
    ctx: &Context<MintToken>,
    name: String,
    symbol: String,
    uri: String,
) -> Result<()> {
    let state = &ctx.accounts.state;
    let token_id = state.token_counter;
    
    // Calculate metadata account PDA
    let metadata_seeds = &[
        METADATA_PREFIX.as_bytes(),
        mpl_token_metadata::ID.as_ref(),
        ctx.accounts.token_data.key().as_ref(),
    ];
    let (metadata_pda, _) = Pubkey::find_program_address(
        metadata_seeds,
        &mpl_token_metadata::ID,
    );

    // Verify correct metadata account passed
    require!(
        metadata_pda == ctx.accounts.metadata.key(),
        ErrorCode::InvalidMetadata
    );

    // Build metadata args
    let data_v2 = DataV2 {
        name: format!("{} #{}", name, token_id),
        symbol,
        uri,
        seller_fee_basis_points: 0, // No royalties
        creators: None,             // No creators
        collection: Some(Collection {
            verified: false,
            key: state.key(),
        }),
        uses: None,
    };

    // Create metadata account instruction
    let create_metadata_account_v3_ix = metadata_instruction::create_metadata_accounts_v3(
        mpl_token_metadata::ID,
        metadata_pda,
        ctx.accounts.token_data.key(),
        ctx.accounts.authority.key(),
        ctx.accounts.authority.key(),
        ctx.accounts.authority.key(),
        data_v2.name,
        data_v2.symbol,
        data_v2.uri,
        None,                      // No creators
        0,                         // No royalties
        true,                      // update_authority_is_signer
        true,                      // is_mutable
        data_v2.collection,
        data_v2.uses,
        None,                      // collection_details
    );

    // Create metadata account
    invoke_signed(
        &create_metadata_account_v3_ix,
        &[
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.token_data.to_account_info(),
            ctx.accounts.authority.to_account_info(),
            ctx.accounts.authority.to_account_info(),
            ctx.accounts.authority.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ],
        &[],
    )?;

    emit!(MetadataCreated {
        collection: state.key(),
        token_id,
        metadata: metadata_pda,
        uri: data_v2.uri,
    });

    Ok(())
}



// Add new event for metadata creation
#[event]
pub struct MetadataCreated {
    pub collection: Pubkey,
    pub token_id: u64,
    pub metadata: Pubkey,
    pub uri: String,
}