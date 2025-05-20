use anchor_lang::prelude::*;

/// Custom errors for the Soon SPL Token program.
#[error_code]
pub enum SoonTokenError {
    /// The caller is not authorized to perform this action (e.g., not the mint authority or owner)
    Unauthorized,
    /// Insufficient funds for the operation (e.g., trying to transfer or burn more than available)
    InsufficientFunds,
    /// Tokens are still locked in vesting and cannot be released yet
    VestingNotReleased,
    /// Invalid instruction data or parameters
    InvalidInstruction,
    /// Metadata fields are too long for the allocated space
    MetadataTooLong,
} 