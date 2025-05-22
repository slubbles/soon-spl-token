use anchor_lang::prelude::*;

/// Custom errors for the Soon SPL Token program.
#[error_code]
pub enum TokenError {
    #[msg("Invalid token authority")]
    InvalidAuthority,
    
    #[msg("Insufficient token balance")]
    InsufficientBalance,
    
    #[msg("Invalid token mint")]
    InvalidMint,
    
    #[msg("Invalid token account")]
    InvalidTokenAccount,
    
    #[msg("Vesting schedule already exists")]
    VestingScheduleExists,
    
    #[msg("Invalid vesting schedule")]
    InvalidVestingSchedule,
    
    #[msg("Vesting period not ended")]
    VestingPeriodNotEnded,
    
    #[msg("Invalid token decimals")]
    InvalidDecimals,
    
    #[msg("Token supply exceeded")]
    SupplyExceeded,
    
    #[msg("Invalid token metadata")]
    InvalidMetadata,
    
    #[msg("Operation not authorized")]
    NotAuthorized,
    
    #[msg("Invalid token amount")]
    InvalidAmount,
    
    #[msg("Token account frozen")]
    AccountFrozen,
    
    #[msg("Token account closed")]
    AccountClosed,
    
    #[msg("Invalid token program")]
    InvalidTokenProgram,
} 