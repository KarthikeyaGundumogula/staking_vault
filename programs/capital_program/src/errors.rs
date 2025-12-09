use anchor_lang::prelude::*;

#[error_code]
pub enum InitError {
    #[msg("CPI to initialize the Nft_program failed")]
    CPIFail,
}

#[error_code]
pub enum StakingError {
    #[msg("Minting CPI failed")]
    CPIFail,
}

#[error_code]
pub enum CreateVaultError {
    #[msg("Invalid parameters are passed")]
    InvalidConfig,
    #[msg("Atleast 7 days in between the creation and Lockdown starts")]
    TooEarlyToLock,
    #[msg("Create collection CPI failed")]
    CPIFail,
}

#[error_code]
pub enum PositionError {
    #[msg("Only holder of the NFT can only stake")]
    OnlyNFTOwner,
    #[msg("Staked amount is below the minimum allowed.")]
    AmountTooLow,
    #[msg("Create Asset CPI failed")]
    CPIFail,
    #[msg("Vault is already at its maximum capacity")]
    VaultMaxCapReached,
    ArithmeticOverflow,
    ArithmeticUnderflow,
}
