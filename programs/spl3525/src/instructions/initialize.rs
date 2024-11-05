use anchor_lang::prelude::*;
use crate::state::Collection;
use crate::utils::metadata::create_collection_metadata;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = Collection::LEN
    )]
    pub collection: Account<'info, Collection>,
    
    /// The mint account for the collection
    /// CHECK: Initialized in CPI
    #[account(mut)]
    pub collection_mint: AccountInfo<'info>,
    
    /// Metadata account for the collection
    /// CHECK: Initialized in CPI
    #[account(mut)]
    pub metadata: AccountInfo<'info>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    
    /// CHECK: MPL Token Metadata program
    pub token_metadata_program: AccountInfo<'info>,
}

pub fn process_initialize(
    ctx: Context<Initialize>,
    name: String,
    symbol: String,
    decimals: u8,
    uri: String,
) -> Result<()> {
    // Create collection metadata
    create_collection_metadata(
        name.clone(),
        symbol.clone(),
        uri,
        ctx.accounts.collection_mint.key(),
        ctx.accounts.metadata.to_account_info(),
        ctx.accounts.authority.to_account_info(),
        ctx.accounts.authority.to_account_info(),
        ctx.accounts.system_program.to_account_info(),
        ctx.accounts.rent.to_account_info(),
        ctx.accounts.token_metadata_program.to_account_info(),
    )?;

    // Initialize collection account
    let collection = &mut ctx.accounts.collection;
    collection.authority = ctx.accounts.authority.key();
    collection.name = name;
    collection.symbol = symbol;
    collection.decimals = decimals;
    collection.mint = ctx.accounts.collection_mint.key();
    collection.metadata = ctx.accounts.metadata.key();

    emit!(CollectionCreated {
        collection: collection.key(),
        authority: collection.authority,
        name: collection.name.clone(),
        symbol: collection.symbol.clone(),
        mint: collection.mint,
        metadata: collection.metadata,
    });

    Ok(())
}

#[event]
pub struct CollectionCreated {
    #[index]
    pub collection: Pubkey,
    pub authority: Pubkey, 
    pub name: String,
    pub symbol: String,
    pub mint: Pubkey,
    pub metadata: Pubkey,
}