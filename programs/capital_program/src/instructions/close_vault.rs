use crate::{errors::*, state::Vault};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{close_account, CloseAccount, Mint, TokenAccount, TokenInterface},
};
#[derive(Accounts)]
pub struct CloseVault<'info> {
    /// CHECK: checked address of the provider
    #[account( address = vault.node_operator)]
    pub node_operator: Signer<'info>,
    /// CHECK: this will be cheked by the MPL-Program
    pub nft: UncheckedAccount<'info>,
    #[account(
      mut,
      close = node_operator,
      seeds = [b"Vault",vault.node_operator.key().as_ref()],
      bump = vault.bump
    )]
    pub vault: Account<'info, Vault>,
    #[account(
      mut,
      associated_token::mint = reward_token_mint,
      associated_token::authority = vault,
      associated_token::token_program = token_program
    )]
    pub vault_reward_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
      mut,
      associated_token::mint = staking_token_mint,
      associated_token::authority = vault,
      associated_token::token_program = token_program
    )]
    pub vault_lock_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(mint::token_program = token_program)]
    pub staking_token_mint: InterfaceAccount<'info, Mint>,
    #[account(mint::token_program = token_program)]
    pub reward_token_mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> CloseVault<'info> {
    pub fn close_vault_accounts(&mut self) -> Result<()> {
        let clock = Clock::get()?;
        let lock_starts_at = self.vault.lock_phase_start_at;
        let lock_ends_at = lock_starts_at + self.vault.lock_phase_duration;
        require!(
            clock.unix_timestamp > lock_ends_at
                || (clock.unix_timestamp > lock_starts_at
                    && self.vault.min_cap > self.vault.total_capital_collected),
            PhaseError::InvalidPhase
        );
        let vault_reward_balance = self.vault_reward_ata.amount;
        let vault_staked_balance = self.vault_lock_ata.amount;
        require!(
            vault_reward_balance == vault_staked_balance && vault_reward_balance == 0,
            VaultError::VaultNotEmpty
        );

        let operator = self.vault.node_operator.key();
        let seeds = &[b"Vault", operator.as_ref(), &[self.vault.bump]];
        let signer = &[&seeds[..]];
        let colse_reward_accounts = CloseAccount {
            account: self.vault_reward_ata.to_account_info(),
            destination: self.node_operator.to_account_info(),
            authority: self.vault.to_account_info(),
        };
        let closing_program = &self.token_program.to_account_info();

        let close_reward_ctx =
            CpiContext::new_with_signer(closing_program.clone(), colse_reward_accounts, signer);
        close_account(close_reward_ctx)?;

        msg!("position's Rewards ATA is closed");

        let close_staking_accounts = CloseAccount {
            account: self.vault_lock_ata.to_account_info(),
            destination: self.node_operator.to_account_info(),
            authority: self.vault.to_account_info(),
        };

        let close_staking_ctx =
            CpiContext::new_with_signer(closing_program.clone(), close_staking_accounts, signer);

        close_account(close_staking_ctx)?;
        msg!("Vault's Lock Token ATA is close");
        Ok(())
    }
}
