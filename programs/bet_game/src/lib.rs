use anchor_lang::prelude::*;
declare_id!("24tX3mF3VBKJTDQEb2EpkPLM11da8NgLQSz6sTUBw3Qj");

pub mod errors;
pub mod events;
pub mod instructions;
pub mod state;

pub use instructions::*;

#[program]
mod bet_game {
    use super::*;

    pub fn setup_game(
        ctx: Context<SetupGame>,
        admin: Pubkey,
    ) -> Result<()> {
        ctx.accounts
            .setup_game(&ctx.bumps, admin)
    }

    pub fn update_setting(
        ctx: Context<UpdateSetting>,
        admin: Pubkey,
    ) -> Result<()> {
        ctx.accounts
            .update_setting(&ctx.bumps, admin)
    }

    pub fn bet(
        ctx: Context<Bet>,
        betting_amount: u64,
        chosen_option: bool
    ) -> Result<()> {
        ctx.accounts
            .bet(&ctx.bumps, betting_amount, chosen_option)
    }

    pub fn request_claim(
        ctx: Context<RequestClaim>,
    ) -> Result<()> {
        ctx.accounts
            .request_claim(&ctx.bumps)
    }

    pub fn transfer_reward(
        ctx: Context<TransferReward>,
        user_address: Pubkey,
        amount: u64
    ) -> Result<()> {
        ctx.accounts
            .transfer_reward(&ctx.bumps, user_address, amount)
    }
}
