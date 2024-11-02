use anchor_lang::prelude::*;
use crate::errors::ErrorCode;
use crate::state::{TokenData, ValueApproval};
#[derive(Accounts)]
pub struct TransferValue<'info> {
    #[account(
        mut,
        constraint = from_token.owner == owner.key()
    )]
    pub from_token: Account<'info, TokenData>,
    #[account(mut)]
    pub to_token: Account<'info, TokenData>,
    pub owner: Signer<'info>,
}

pub fn value_handler(
    ctx: Context<TransferValue>,
    value: u64,
) -> Result<()> {
    let from_token = &mut ctx.accounts.from_token;
    let to_token = &mut ctx.accounts.to_token;

    // Verify same slot and collection
    require!(
        from_token.slot == to_token.slot,
        ErrorCode::SlotMismatch
    );
    require!(
        from_token.collection == to_token.collection,
        ErrorCode::CollectionMismatch
    );

    // Check value
    require!(
        value <= from_token.value,
        ErrorCode::InsufficientValue
    );

    // Check approvals if not owner
    if ctx.accounts.owner.key() != from_token.owner {
        let approval_seeds = ValueApproval::seeds(
            from_token.token_id,
            &ctx.accounts.owner.key()
        );
        let approval = ValueApproval::try_from_seeds(&approval_seeds)?;
        require!(
            value <= approval.value,
            ErrorCode::ExceedsApproval
        );
    }

    // Transfer value
    from_token.value = from_token.value
        .checked_sub(value)
        .ok_or(ErrorCode::Overflow)?;
    to_token.value = to_token.value
        .checked_add(value)
        .ok_or(ErrorCode::Overflow)?;

    emit!(ValueTransferred {
        collection: from_token.collection,
        from_token: from_token.token_id,
        to_token: to_token.token_id,
        value,
        from_owner: from_token.owner,
        to_owner: to_token.owner,
    });

    Ok(())
}

#[event]
pub struct ValueTransferred {
    pub collection: Pubkey,
    pub from_token: u64,
    pub to_token: u64,
    pub value: u64,
    pub from_owner: Pubkey,
    pub to_owner: Pubkey,
}