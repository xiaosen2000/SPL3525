use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid authority for operation")]
    InvalidAuthority,
    
    #[msg("Invalid slot number")]
    InvalidSlotNumber,
    
    #[msg("Slot mismatch")]
    SlotMismatch,
    
    #[msg("Collection mismatch")]
    CollectionMismatch,
    
    #[msg("Insufficient value for transfer")]
    InsufficientValue,
    
    #[msg("Transfer amount exceeds approval")]
    ExceedsApproval,
    
    #[msg("Invalid owner")]
    InvalidOwner,
    
    #[msg("Amount exceeds token balance")]
    ExceedsBalance,
    
    #[msg("Arithmetic overflow")]
    Overflow,
    
    #[msg("Invalid collection")]
    InvalidCollection,
    
    #[msg("Invalid metadata account")]
    InvalidMetadata,
}