use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Vault {
    pub receiver: Pubkey,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct Marketplace{
    pub staking_program: Pubkey,
    pub bump:u8
}