use anchor_lang::prelude::*;
use crate::state::*;
use crate::events::TransferRewardEvent;
use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct TransferReward<'info> {
    #[account(
        mut,
        seeds = [b"setting"],
        bump,
    )]
    pub setting: Account<'info, Setting>,
    #[account(
        mut,
        seeds = [b"vault"],
        bump,
    )]
    /// CHECK: 
    pub vault: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: 
    pub treasury: AccountInfo<'info>,  
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> TransferReward<'info> {
    pub fn transfer_reward( &mut self,  _bumps: &TransferRewardBumps, user_address: Pubkey, amount: u64) -> Result<()> {
        require!(self.signer.key() == self.setting.admin, ErrorCode::InvalidCaller);
        require!(self.treasury.key() == user_address, ErrorCode::InvalidParams);

        **self.vault
            .to_account_info()
            .try_borrow_mut_lamports()? -= amount;
            **self.treasury
                .to_account_info()
                .try_borrow_mut_lamports()? += amount;
        emit!(
            TransferRewardEvent{
                user: user_address,
                amount
            }
        );
        Ok(())
    }
}