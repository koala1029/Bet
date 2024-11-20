// src/instructions/mod.rs
pub mod setup_game;
pub mod bet;
pub mod request_claim;
pub mod transfer_reward;
pub mod update_setting;



pub use setup_game::*;
pub use bet::*;
pub use request_claim::*;
pub use transfer_reward::*;
pub use update_setting::*;


