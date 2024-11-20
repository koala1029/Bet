use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient amount")]
    InsufficientAmount,
    #[msg("Current round is already completed")]
    RoundAlreadyCompleted,
    #[msg("Current round is not completed")]
    RoundNotCompleted,
    #[msg("Current round is already finished")]
    RoundAlreadyFinished,
    #[msg("Current round is not finished")]
    RoundNotFinished,
    #[msg("Round is expired")]
    RoundExpired,
    #[msg("Round is not expired")]
    RoundNotExpired,
    #[msg("Address already exists")]
    AddressAlreadyExists,
    #[msg("Address not found")]
    AddressNotFound,
    #[msg("Invalid caller")]
    InvalidCaller,
    #[msg("Invalid params")]
    InvalidParams,
    #[msg("Invalid amount")]
    InvalidAmount,
    #[msg("Already Betted")]
    AlreadyBetted,
}
