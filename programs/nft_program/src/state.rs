use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Offer {
    pub seller: Pubkey,
    pub price: u64,
    pub token_mint: Pubkey,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct NFTConfig {
    pub capital_program: Pubkey,
    pub authority: Pubkey,
    pub admin: Pubkey,
    pub bump: u8,
}
