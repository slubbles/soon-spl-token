use anchor_lang::prelude::*;
use crate::instruction::*;
use crate::state::TokenMetadata;
use anchor_spl::token::{self, MintTo as SplMintTo, MintToAccounts, Token, Transfer as SplTransfer, TransferAccounts, Burn as SplBurn, BurnAccounts};
use anchor_lang::solana_program::clock::Clock;
use crate::error::SoonTokenError;

/// Initializes the token metadata account with the provided parameters.
///
/// Security: Only the payer (signer) can initialize. Mint creation and initial supply minting should be handled by the client or a separate instruction.
pub fn initialize_token(
    ctx: Context<InitializeToken>,
    args: InitializeTokenArgs,
) -> Result<()> {
    // Set up the TokenMetadata account with provided args
    let metadata = &mut ctx.accounts.metadata;
    metadata.name = args.name;
    metadata.symbol = args.symbol;
    metadata.decimals = args.decimals;
    metadata.logo_url = args.logo_url;
    metadata.description = args.description;
    metadata.external_url = args.external_url;
    // Note: Mint creation and initial supply minting should be handled by the client or a separate instruction
    // This program stores metadata and can enforce custom logic
    Ok(())
}

/// Mints tokens to a specified account.
///
/// Security: Only the mint authority (signer) can mint tokens.
pub fn mint_to(
    ctx: Context<MintTo>,
    args: MintToArgs,
) -> Result<()> {
    // Only the mint authority can mint
    // (Anchor will enforce signer on authority)
    let cpi_accounts = MintToAccounts {
        mint: ctx.accounts.mint.to_account_info(),
        to: ctx.accounts.to.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
    token::mint_to(cpi_ctx, args.amount)?;
    Ok(())
}

/// Transfers tokens between accounts.
///
/// Security: Only the owner/authority (signer) of the source account can transfer tokens.
pub fn transfer(
    ctx: Context<Transfer>,
    args: TransferArgs,
) -> Result<()> {
    // Only the owner/authority can transfer
    let cpi_accounts = TransferAccounts {
        from: ctx.accounts.from.to_account_info(),
        to: ctx.accounts.to.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
    token::transfer(cpi_ctx, args.amount)?;
    Ok(())
}

/// Burns tokens from a specified account.
///
/// Security: Only the owner/authority (signer) of the source account can burn tokens.
pub fn burn(
    ctx: Context<Burn>,
    args: BurnArgs,
) -> Result<()> {
    // Only the owner/authority can burn
    let cpi_accounts = BurnAccounts {
        mint: ctx.accounts.mint.to_account_info(),
        from: ctx.accounts.from.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
    token::burn(cpi_ctx, args.amount)?;
    Ok(())
}

/// Releases vested tokens to the beneficiary if the release time has passed.
///
/// Security: Only the beneficiary (signer) can claim vested tokens. Checks the current clock against the release time.
pub fn vest(
    ctx: Context<Vest>,
    args: VestArgs,
) -> Result<()> {
    // Check if the current time is after the release_time
    let clock = Clock::get()?;
    if clock.unix_timestamp < args.release_time {
        return Err(SoonTokenError::VestingNotReleased.into());
    }
    // TODO: Implement actual vesting logic (e.g., transfer tokens from vesting_account to beneficiary)
    Ok(())
} 