use anchor_lang::prelude::*;
use crate::state::TokenMetadata;

/// Instruction to initialize the token and its metadata
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct InitializeTokenArgs {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub logo_url: String,
    pub description: String,
    pub external_url: String,
    pub total_supply: u64,
    pub mint_authority: Pubkey,
    pub freeze_authority: Option<Pubkey>,
}

/// Instruction to mint tokens to an account
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct MintToArgs {
    pub amount: u64,
}

/// Instruction to transfer tokens
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct TransferArgs {
    pub amount: u64,
}

/// Instruction to burn tokens
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct BurnArgs {
    pub amount: u64,
}

/// Instruction to vest tokens (optional advanced feature)
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct VestArgs {
    pub amount: u64,
    pub release_time: i64, // Unix timestamp
}

/// Accounts for initializing the token and its metadata.
///
/// The payer funds the creation of the metadata account and must be a signer.
#[derive(Accounts)]
pub struct InitializeToken<'info> {
    /// The metadata account to store token info
    #[account(init, payer = payer, space = 8 + 256)]
    pub metadata: Account<'info, TokenMetadata>,
    /// The mint account (should be created by the client)
    #[account(mut)]
    pub mint: UncheckedAccount<'info>,
    /// The payer for account creation and rent
    #[account(mut, signer)]
    pub payer: Signer<'info>,
    /// System program
    pub system_program: Program<'info, System>,
    /// Rent sysvar
    pub rent: Sysvar<'info, Rent>,
}

/// Accounts for minting tokens.
///
/// Only the mint authority (signer) can mint new tokens.
#[derive(Accounts)]
pub struct MintTo<'info> {
    /// The mint account
    #[account(mut)]
    pub mint: UncheckedAccount<'info>,
    /// The destination token account
    #[account(mut)]
    pub to: UncheckedAccount<'info>,
    /// The mint authority (must sign)
    #[account(signer)]
    pub authority: Signer<'info>,
    /// SPL Token program
    pub token_program: Program<'info, Token>,
}

/// Accounts for transferring tokens.
///
/// Only the owner/authority (signer) of the source account can transfer tokens.
#[derive(Accounts)]
pub struct Transfer<'info> {
    /// The source token account
    #[account(mut)]
    pub from: UncheckedAccount<'info>,
    /// The destination token account
    #[account(mut)]
    pub to: UncheckedAccount<'info>,
    /// The owner/authority of the source account (must sign)
    #[account(signer)]
    pub authority: Signer<'info>,
    /// SPL Token program
    pub token_program: Program<'info, Token>,
}

/// Accounts for burning tokens.
///
/// Only the owner/authority (signer) of the source account can burn tokens.
#[derive(Accounts)]
pub struct Burn<'info> {
    /// The mint account
    #[account(mut)]
    pub mint: UncheckedAccount<'info>,
    /// The source token account to burn from
    #[account(mut)]
    pub from: UncheckedAccount<'info>,
    /// The owner/authority of the source account (must sign)
    #[account(signer)]
    pub authority: Signer<'info>,
    /// SPL Token program
    pub token_program: Program<'info, Token>,
}

/// Accounts for vesting tokens.
///
/// Only the beneficiary (signer) can claim vested tokens after the release time.
#[derive(Accounts)]
pub struct Vest<'info> {
    /// The vesting account (custom logic, unchecked for now)
    #[account(mut)]
    pub vesting_account: UncheckedAccount<'info>,
    /// The beneficiary who will receive tokens (must sign)
    #[account(signer)]
    pub beneficiary: Signer<'info>,
    /// System program
    pub system_program: Program<'info, System>,
} 