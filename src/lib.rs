use anchor_lang::prelude::*;

pub mod entrypoint;
pub mod instruction;
pub mod processor;
pub mod state;
pub mod error;

// Anchor program declaration
#[program]
pub mod soon_spl_token {
    use super::*;
    use crate::instruction::*;
    use crate::processor::*;

    pub fn initialize_token(
        ctx: Context<InitializeToken>,
        args: InitializeTokenArgs,
    ) -> Result<()> {
        processor::initialize_token(ctx, args)
    }

    pub fn mint_to(
        ctx: Context<MintTo>,
        args: MintToArgs,
    ) -> Result<()> {
        processor::mint_to(ctx, args)
    }

    pub fn transfer(
        ctx: Context<Transfer>,
        args: TransferArgs,
    ) -> Result<()> {
        processor::transfer(ctx, args)
    }

    pub fn burn(
        ctx: Context<Burn>,
        args: BurnArgs,
    ) -> Result<()> {
        processor::burn(ctx, args)
    }

    pub fn vest(
        ctx: Context<Vest>,
        args: VestArgs,
    ) -> Result<()> {
        processor::vest(ctx, args)
    }
} 