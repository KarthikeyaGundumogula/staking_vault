use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct StakingVault {
    pub provider: Pubkey,
    pub duration: u64,
    pub start_time: u64,
    pub reward_mint: Pubkey,
    pub staking_mint: Pubkey,
    pub nft_mint: Pubkey,
    pub bump: u8,
    pub minimum_amount: u64,
    pub maximum_amount: u64,
    pub rewards_value: u64,
    pub staked_value: u64,
}


#[account]
#[derive(InitSpace)]
pub struct Config {
    pub nft_program: Pubkey,
    pub admin: Pubkey,
    pub agent: Pubkey,
    pub early_unlock_fee: u64, // in bps to the base 10_000
    pub bump: u8,
}
