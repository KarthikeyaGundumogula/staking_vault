use crate::state::StakingVault;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{close_account, CloseAccount, Mint, TokenAccount, TokenInterface},
};

#[derive(Accounts)]
pub struct Close<'info> {
    pub provider: Signer<'info>,
    #[account( address = staking_vault.staker)]
    pub staker: AccountInfo<'info>,
    #[account(
      mut,
      close = provider,
      seeds = [b"staking_vault",staking_vault.provider.key().as_ref()],
      bump = staking_vault.bump
    )]
    pub staking_vault: Account<'info, StakingVault>,
    #[account(
      mut,
      associated_token::mint = reward_token_mint,
      associated_token::authority = staking_vault,
      associated_token::token_program = token_program
    )]
    pub vault_rewards_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
      mut,
      associated_token::mint = staking_token_mint,
      associated_token::authority = staking_vault,
      associated_token::token_program = token_program
    )]
    pub vault_staking_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(mint::token_program = token_program)]
    pub staking_token_mint: InterfaceAccount<'info, Mint>,
    #[account(mint::token_program = token_program)]
    pub reward_token_mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> Close<'info> {
    pub fn close_vault_accounts(&mut self) -> Result<()> {
        let seeds = &[
            b"staking_vault",
            self.provider.key.as_ref(),
            &[self.staking_vault.bump],
        ];
        let signer = &[&seeds[..]];
        let colse_reward_accounts = CloseAccount {
            account: self.vault_rewards_ata.to_account_info(),
            destination: self.provider.to_account_info(),
            authority: self.staking_vault.to_account_info(),
        };
        let closing_program = &self.token_program.to_account_info();
        let close_reward_ctx =
            CpiContext::new_with_signer(closing_program.clone(), colse_reward_accounts, signer);
        close_account(close_reward_ctx)?;
        msg!("Vault's Rewards ATA is closed");

        let close_staking_accounts = CloseAccount {
            account: self.vault_staking_ata.to_account_info(),
            destination: self.staker.to_account_info(),
            authority: self.staking_vault.to_account_info(),
        };
        let close_staking_ctx =
            CpiContext::new_with_signer(closing_program.clone(), close_staking_accounts, signer);

        close_account(close_staking_ctx)
    }
}
