use anchor_lang::prelude::*;

#[event]
pub struct BetEvent {
    pub user: Pubkey,
    pub amount: u64,
    pub chosen_option: bool,
}

#[event]
pub struct RequestClaimEvent {
    pub user: Pubkey,
}

#[event]
pub struct TransferRewardEvent {
    pub user: Pubkey,
    pub amount: u64,
}

#[event]
pub struct UpdateSettingEvent {
    pub new_admin: Pubkey,
}