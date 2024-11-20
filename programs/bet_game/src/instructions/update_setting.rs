use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::ErrorCode;
use crate::events::UpdateSettingEvent;
#[derive(Accounts)]
pub struct UpdateSetting<'info> {
    #[account(
        mut,
        seeds = [b"setting"],
        bump
    )]
    pub setting: Account<'info, Setting>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> UpdateSetting<'info> {
    pub fn update_setting(
        &mut self,
        _bumps: &UpdateSettingBumps,
        admin: Pubkey
    ) -> Result<()> {
        let setting = &mut self.setting;
        require!(self.signer.key() == setting.admin, ErrorCode::InvalidParams);
        setting.admin = admin;
        emit!(UpdateSettingEvent{
            new_admin: admin
        });
        Ok(())
    }
}
