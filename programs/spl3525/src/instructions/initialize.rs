use anchor_lang::prelude::*;
use crate::state::Collection;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = Collection::LEN
    )]
    pub collection: Account<'info, Collection>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn process_initialize(
    ctx: Context<Initialize>,
    name: String,
    symbol: String,
    decimals: u8,
) -> Result<()> {
    let collection = &mut ctx.accounts.collection;
    collection.authority = ctx.accounts.authority.key();
    collection.name = name;
    collection.symbol = symbol;
    collection.decimals = decimals;

    emit!(CollectionCreated {
        collection: collection.key(),
        authority: collection.authority,
        name: collection.name.clone(),
        symbol: collection.symbol.clone()
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
}