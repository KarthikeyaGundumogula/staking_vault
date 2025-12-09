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
