use anchor_lang::prelude::*;
use crate::events::RequestClaimEvent;
#[derive(Accounts)]
pub struct RequestClaim<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> RequestClaim<'info> {
    pub fn request_claim( &mut self,  _bumps: &RequestClaimBumps) -> Result<()> {
        emit!(RequestClaimEvent{
            user: self.signer.key(),
        });
        Ok(())
    }
}