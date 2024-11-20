// src/instructions/setup_game.rs
use anchor_lang::prelude::*;
use crate::state::*;
#[derive(Accounts)]
pub struct SetupGame<'info> {
    #[account(
        init,
        payer = signer,
        space = 8 + 32 + 8 + 8,
        seeds = [b"setting"],
        bump
    )]
    pub setting: Account<'info, Setting>,
    #[account(
        init,
        payer = signer,
        space = 8,
        seeds = [b"vault"],
        bump,
    )]
    /// CHECK: 
    pub vault: AccountInfo<'info>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> SetupGame<'info> {
    pub fn setup_game(
        &mut self,
        _bumps: &SetupGameBumps,
        admin: Pubkey
    ) -> Result<()> {
        let setting = &mut self.setting;
        setting.admin = admin;
        Ok(())
    }
}
