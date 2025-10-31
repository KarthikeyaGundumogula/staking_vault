use crate::state::StakingVault;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

#[derive(Accounts)]
pub struct DepositRewards<'info> {
    #[account(mut)]
    pub provider: Signer<'info>,
    #[account(
      seeds = [b"staking_vault",staking_vault.nft_id.to_le_bytes().as_ref(),staking_vault.provider.key().as_ref()],
      bump = staking_vault.bump
    )]
    pub staking_vault: Account<'info, StakingVault>,
    #[account(mint::token_program = token_program)]
    pub reward_token_mint: InterfaceAccount<'info, Mint>,
    #[account(
      mut,
      associated_token::mint = reward_token_mint,
      associated_token::authority = staking_vault,
      associated_token::token_program = token_program
    )]
    pub vault_reward_token_ata: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> DepositRewards<'info> {
    pub fn deposit_rewards(&mut self, amount: u64) -> Result<()> {
        let transfer_accounts = TransferChecked {
            from: self.provider.to_account_info(),
            to: self.vault_reward_token_ata.to_account_info(),
            authority: self.provider.to_account_info(),
            mint: self.reward_token_mint.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, transfer_accounts);
        transfer_checked(cpi_ctx, amount, self.reward_token_mint.decimals)?;

        Ok(())
    }
}
