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
