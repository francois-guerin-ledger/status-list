use anchor_lang::prelude::*;

use crate::{state::StatusList, types::ListPurpose};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        space = 8 + StatusList::INIT_SPACE,
        seeds = [b"status_list", payer.key().as_ref()],
        bump
    )]
    pub state: Account<'info, StatusList>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, size: u16, purpose: ListPurpose) -> Result<()> {
        self.state.set_inner(StatusList::new(size, purpose)?);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Toggle<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        mut,
        seeds = [b"status_list", payer.key().as_ref()],
        bump
    )]
    pub state: Account<'info, StatusList>,
    pub system_program: Program<'info, System>,
}

impl<'info> Toggle<'info> {
    pub fn toggle(&mut self, location: u16) -> Result<()> {
        self.state.toggle(location)
    }
}

#[derive(Accounts)]
pub struct Read<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        seeds = [b"status_list", payer.key().as_ref()],
        bump
    )]
    pub state: Account<'info, StatusList>,
    pub system_program: Program<'info, System>,
}

impl<'info> Read<'info> {
    pub fn read(&mut self, location: u16) -> Result<bool> {
        self.state.get(location)
    }
}
