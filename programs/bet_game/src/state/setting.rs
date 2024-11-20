use anchor_lang::prelude::*;

#[account]
pub struct Setting {
    pub admin: Pubkey,
}
