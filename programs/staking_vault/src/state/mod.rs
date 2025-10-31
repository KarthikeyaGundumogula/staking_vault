use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct StakingVault {
  pub provider: Pubkey,
  pub duration: u64,
  pub start_time: u64,
  pub mint_a: Pubkey,
  pub mint_b: Pubkey,
  pub nft_id:u64,
  pub nft_mint: Pubkey,
  pub bump: u8,
  pub minimum_amount: u64,
  pub maximum_amount: u64,
}