use anchor_lang::prelude::*;

/// Custom metadata for the SPL token.
///
/// This account stores off-chain-relevant information about the token,
/// such as its name, symbol, decimals, logo, description, and external URL.
#[account]
pub struct TokenMetadata {
    /// Token name (e.g., "SoonToken")
    pub name: String,
    /// Token symbol (e.g., "SOON")
    pub symbol: String,
    /// Number of decimals (e.g., 9)
    pub decimals: u8,
    /// URL to the token logo (PNG, SVG, etc.)
    pub logo_url: String,
    /// Description of the token
    pub description: String,
    /// External URL (project website, etc.)
    pub external_url: String,
}

// Custom state (e.g., metadata) will go here. 