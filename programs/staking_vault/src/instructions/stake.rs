use crate::state::StakingVault;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};
use mpl_core::accounts::BaseAssetV1;

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub staker: Signer<'info>,
    #[account(
      seeds = [b"staking_vault",staking_vault.provider.key().as_ref()],
      bump = staking_vault.bump,
    )]
    pub staking_vault: Account<'info, StakingVault>,
    #[account(
      mut,
      associated_token::mint = staking_token_mint,
      associated_token::authority = staker,
      associated_token::token_program = token_program
    )]
    pub staker_token_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
      init_if_needed,
      payer = staker,
      associated_token::mint = staking_token_mint,
      associated_token::authority = staking_vault,
      associated_token::token_program = token_program
    )]
    pub staking_vault_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        address = staking_vault.nft_mint
    )]
    pub asset: Account<'info, BaseAssetV1>,
    pub staking_token_mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> Stake<'info> {
    pub fn stake_tokens(&mut self, amount: u64) -> Result<()> {
        require!(
            amount >= self.staking_vault.minimum_amount,
            StakingVaultError::AmountTooLow
        );
        require!(
            amount <= self.staking_vault.maximum_amount,
            StakingVaultError::AmountTooHigh
        );
        require!(
            self.asset.owner == self.staker.key(),
            StakingVaultError::OnlyNFTOwner
        );
        let transfer_staking_token_accounts = TransferChecked {
            from: self.staker_token_ata.to_account_info(),
            to: self.staking_vault_ata.to_account_info(),
            authority: self.staker.to_account_info(),
            mint: self.staking_token_mint.to_account_info(),
        };
        let token_program = self.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(token_program, transfer_staking_token_accounts);
        transfer_checked(cpi_ctx, amount, self.staking_token_mint.decimals)?;
        self.staking_vault.start_time = Clock::get()?.unix_timestamp as u64;
        Ok(())
    }
}
#[error_code]
pub enum StakingVaultError {
    #[msg("Only holder of the NFT can only stake")]
    OnlyNFTOwner,
    #[msg("Staked amount is below the minimum allowed.")]
    AmountTooLow,
    #[msg("Staked amount exceeds the maximum allowed.")]
    AmountTooHigh,
}
