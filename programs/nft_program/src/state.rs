use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Vault {
    pub receiver: Pubkey,
    pub bump: u8,
}


#[account]
#[derive(InitSpace)]
pub struct NFTConfig{
  pub capital_program: Pubkey,
  pub authority:Pubkey,
  pub admin:Pubkey,
  pub bump:u8
}