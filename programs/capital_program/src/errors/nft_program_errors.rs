use anchor_lang::prelude::*;

#[error_code]
pub enum NFTProgramError{

    #[msg("NFT collection already exists")]
    CollectionAlreadyExists,

    #[msg("CPI call failed")]
    CPIFailed,

    #[msg("Invalid NFT program address")]
    InvalidNFTProgram,

    #[msg("Invalid Config for NFT Program")]
    InvalidNFTConfigOwner
}