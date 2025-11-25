#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("Count3AcZucFDPSFBAeHkQ6AvttieKUkyJ8HiQGhQwe");

#[program]
pub mod simplecollection {
    use super::*;

    pub fn close(_ctx: Context<CloseSimplecollection>) -> Result<()> {
        Ok(())
    }

    pub fn decrement(ctx: Context<Update>) -> Result<()> {
        ctx.accounts.simplecollection.count = ctx.accounts.simplecollection.count.checked_sub(1).unwrap();
        Ok(())
    }

    pub fn increment(ctx: Context<Update>) -> Result<()> {
        ctx.accounts.simplecollection.count = ctx.accounts.simplecollection.count.checked_add(1).unwrap();
        Ok(())
    }

    pub fn initialize(_ctx: Context<InitializeSimplecollection>) -> Result<()> {
        Ok(())
    }

    pub fn set(ctx: Context<Update>, value: u8) -> Result<()> {
        ctx.accounts.simplecollection.count = value.clone();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeSimplecollection<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
  init,
  space = 8 + Simplecollection::INIT_SPACE,
  payer = payer
    )]
    pub simplecollection: Account<'info, Simplecollection>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct CloseSimplecollection<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
  mut,
  close = payer, // close account and return lamports to payer
    )]
    pub simplecollection: Account<'info, Simplecollection>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub simplecollection: Account<'info, Simplecollection>,
}

#[account]
#[derive(InitSpace)]
pub struct Simplecollection {
    count: u8,
}
