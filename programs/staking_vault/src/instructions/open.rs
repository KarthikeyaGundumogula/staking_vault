use crate::state::StakingVault;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

#[derive(Accounts)]
#[instruction(nft_id:u64)]
pub struct Open<'info> {
    #[account(mut)]
    pub provider: Signer<'info>,
    #[account(
      init,
      payer = provider,
      seeds = [b"staking_vault",nft_id.to_le_bytes().as_ref(),provider.key().as_ref()],
      space = StakingVault::INIT_SPACE,
      bump
    )]
    pub staking_vault: Account<'info, StakingVault>,
    #[account(
      mut,
      associated_token::mint = reward_token_mint,
      associated_token::authority = provider,
      associated_token::token_program = token_program
    )]
    pub provider_reward_tokens_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(mint::token_program = token_program)]
    pub reward_token_mint: InterfaceAccount<'info, Mint>,
    #[account(
      init,
      payer = provider,
      associated_token::mint = reward_token_mint,
      associated_token::authority = staking_vault,
      associated_token::token_program = token_program
    )]
    pub vault_reward_token_ata: InterfaceAccount<'info, TokenAccount>,
    pub staking_token_mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> Open<'info> {
    pub fn init_config(
        &mut self,
        duration: u64,
        min_amount: u64,
        max_amount: u64,
        start_time: u64,
        bumps: &OpenBumps,
    ) -> Result<()> {
      self.staking_vault.set_inner(StakingVault{
        provider: self.provider.key(),
        duration,
        start_time,
        mint_a: self.staking_token_mint.key(),
        mint_b: self.reward_token_mint.key(),
        nft_mint: self.staking_token_mint.key(),
        bump: bumps.staking_vault,
        minimum_amount: min_amount,
        maximum_amount: max_amount,
        nft_id: 0, // will be set properly in future
      });
      Ok(())
    }
    pub fn deposit_rewards(&mut self, deposit_amount: u64) -> Result<()> {
        let transfer_reward_accounts = TransferChecked {
            from: self.provider_reward_tokens_ata.to_account_info(),
            mint: self.reward_token_mint.to_account_info(),
            to: self.vault_reward_token_ata.to_account_info(),
            authority: self.provider.to_account_info(),
        };

        let transfer_rewards_ctx = CpiContext::new(
            self.token_program.to_account_info(),
            transfer_reward_accounts,
        );

        transfer_checked(
            transfer_rewards_ctx,
            deposit_amount,
            self.reward_token_mint.decimals,
        )
    }
}
