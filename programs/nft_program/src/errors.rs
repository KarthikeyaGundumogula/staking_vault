use anchor_lang::prelude::*;

#[error_code]
pub enum CreateCollectionError {
    #[msg("Authority must be the PDA we have given at initialization")]
    InvalidAuthority,
}
