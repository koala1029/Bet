use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction;
use crate::events::BetEvent;
#[derive(Accounts)]
pub struct Bet<'info> {
    #[account(
        mut,
        seeds = [b"vault"],
        bump,
    )]
    /// CHECK: 
    pub vault: AccountInfo<'info>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> Bet<'info> {
    pub fn bet( &mut self,  _bumps: &BetBumps, betting_amount: u64, chosen_option: bool) -> Result<()> {
        let ix =
        system_instruction::transfer(&self.signer.key(), &self.vault.key(), betting_amount);

    anchor_lang::solana_program::program::invoke(
        &ix,
        &[self.signer.to_account_info(), self.vault.to_account_info()],
    )?;

    emit!(BetEvent{
        user: self.signer.key(),
        amount: betting_amount,
        chosen_option,
    });
        Ok(())
    }
}