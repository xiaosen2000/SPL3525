use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke_signed;
use mpl_token_metadata::instructions::{
    CreateMetadataAccountV3,
    CreateMetadataAccountV3InstructionArgs,
};
use mpl_token_metadata::types::{DataV2, Creator};
use crate::errors::ErrorCode;

pub fn create_collection_metadata<'a>(
    name: String,
    symbol: String,
    uri: String,
    collection_mint: Pubkey,
    metadata: AccountInfo<'a>,
    payer: AccountInfo<'a>,
    authority: AccountInfo<'a>,
    system_program: AccountInfo<'a>,
    rent: AccountInfo<'a>,
    token_metadata_program: AccountInfo<'a>,
) -> Result<()> {
    // Create metadata for collection
    let data_v2 = DataV2 {
        name,
        symbol,
        uri,
        seller_fee_basis_points: 0,
        creators: Some(vec![Creator {
            address: authority.key(),
            verified: true,
            share: 100,
        }]),
        collection: None,
        uses: None,
    };

    // Build create metadata instruction
    let create_metadata_ix = CreateMetadataAccountV3 {
        metadata: metadata.key(),
        mint: collection_mint,
        mint_authority: authority.key(),
        payer: payer.key(),
        update_authority: (authority.key(), true),
        system_program: system_program.key(),
        rent: Some(rent.key()),
    }.instruction(CreateMetadataAccountV3InstructionArgs {
        data: data_v2,
        is_mutable: true,
        collection_details: None,
    });

    // Create metadata account
    invoke_signed(
        &create_metadata_ix,
        &[
            metadata,
            authority,
            payer,
            system_program,
            rent,
            token_metadata_program,
        ],
        &[],
    )?;

    Ok(())
}

pub fn verify_collection_metadata(
    metadata_account: AccountInfo,
    collection_mint: Pubkey, 
    token_metadata_program: Pubkey,
) -> Result<()> {
    let metadata_seeds = &[
        b"metadata",
        token_metadata_program.as_ref(),
        collection_mint.as_ref(),
    ];
    let (metadata_pda, _) = Pubkey::find_program_address(
        metadata_seeds,
        &mpl_token_metadata::ID,
    );

    require!(
        metadata_pda == *metadata_account.key,
        ErrorCode::InvalidMetadata
    );

    Ok(())
}