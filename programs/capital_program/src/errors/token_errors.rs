use anchor_lang::prelude::*;

#[error_code]
pub enum TokenError {
    #[msg("Invalid reward token mint")]
    InvalidRewardMint,

    #[msg("Invalid staking token mint")]
    InvalidLockingMint,

    #[msg("Insufficient balance in account")]
    InsufficientBalance,

    #[msg("Insufficient balance in vault")]
    InsufficientVaultBalance,

    #[msg("The vault has no Reward collected")]
    NoRewardsInVault,
}
