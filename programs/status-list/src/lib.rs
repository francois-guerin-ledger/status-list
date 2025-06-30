use anchor_lang::prelude::*;

declare_id!("Bs58kbsN1J9c19dCs4h9Yxzi8a9VSpiWJyWYjRpUt9TF");

mod errors;
mod instructions;
mod state;
mod types;

use instructions::*;
use types::*;

#[program]
pub mod status_list {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, size: u16, purpose: ListPurpose) -> Result<()> {
        ctx.accounts.initialize(size, purpose)
    }

    pub fn toggle(ctx: Context<Toggle>, location: u16) -> Result<()> {
        ctx.accounts.toggle(location)
    }

    pub fn read(ctx: Context<Read>, location: u16) -> Result<bool> {
        ctx.accounts.read(location)
    }
}
