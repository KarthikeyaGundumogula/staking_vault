use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Vault {
    pub receiver: Pubkey,
    pub current_owner: Pubkey,
    pub bump: u8,
}
