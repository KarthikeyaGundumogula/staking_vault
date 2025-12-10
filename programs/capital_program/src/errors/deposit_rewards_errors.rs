use anchor_lang::prelude::*;

#[error_code]
pub enum DepositRewardsError {
    #[msg("Unauthorized: caller is not the authorized agent")]
    UnauthorizedAgent,

    #[msg("Amount must be greater than zero")]
    AmountMustBePositive,

    #[msg("Insufficient balance in account")]
    InsufficientBalance,

    #[msg("Vault has no capital collected")]
    NoCapitalInVault,

    #[msg("Lock phase has not started yet")]
    LockPhaseNotStarted,

    #[msg("Reward deposit exceeds maximum allowed limit")]
    RewardDepositExceedsLimit,

    #[msg("Vault is not active")]
    VaultInactive,

    #[msg("Vault is under dispute")]
    VaultUnderDispute,

    #[msg("Configuration is not active")]
    ConfigInactive,

    #[msg("Invalid reward token mint")]
    InvalidRewardMint,

    #[msg("Arithmetic overflow occurred")]
    ArithmeticOverflow,
}
