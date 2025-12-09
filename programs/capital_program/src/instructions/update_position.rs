use crate::{
    errors::PositionError,
    state::{Position, Vault},
};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

#[derive(Accounts)]
pub struct UpdatePosition<'info> {
    #[account(mut)]
    pub capital_provider: Signer<'info>,
    #[account(
      seeds = [b"Vault",vault.node_operator.key().as_ref()],
      bump = vault.bump
    )]
    pub vault: Account<'info, Vault>,
    #[account(
      seeds = [b"Position"],
      bump = position.bump
    )]
    pub position: Account<'info, Position>,
    #[account(mint::token_program = token_program)]
    pub locking_token_mint: InterfaceAccount<'info, Mint>,
    #[account(
      mut,
      associated_token::mint = locking_token_mint,
      associated_token::authority = vault,
      associated_token::token_program = token_program
    )]
    pub vault_token_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
      mut,
      associated_token::mint = locking_token_mint,
      associated_token::authority = capital_provider,
      associated_token::token_program = token_program
    )]
    pub capital_provider_token_ata: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> UpdatePosition<'info> {
    pub fn validate_and_update(&mut self, update_amount: i64) -> Result<()> {
        if update_amount > 0 {
            let amount = update_amount as u64;
            let new_postion_value = self
                .position
                .total_value_locked
                .checked_add(amount)
                .ok_or(PositionError::ArithmeticOverflow);
            let new_total_capital_locked = self
                .vault
                .total_capital_collected
                .checked_add(amount)
                .ok_or(PositionError::ArithmeticOverflow);
            require!(
                new_total_capital_locked.unwrap() <= self.vault.max_cap,
                PositionError::VaultMaxCapReached
            );
            self.position.total_value_locked = new_postion_value.unwrap();
            self.vault.total_capital_collected = new_total_capital_locked.unwrap();
            self.deposit_rewards(update_amount as u64)?;
        } else {
            let amount = (-update_amount) as u64;
            let new_postion_value = self
                .position
                .total_value_locked
                .checked_sub(amount)
                .ok_or(PositionError::ArithmeticUnderflow);
            let new_total_capital_locked = self
                .vault
                .total_capital_collected
                .checked_sub(amount)
                .ok_or(PositionError::ArithmeticUnderflow);
            require!(
                new_postion_value.unwrap() > self.vault.min_lock_amount,
                PositionError::AmountTooLow
            );
            self.position.total_value_locked = new_postion_value.unwrap();
            self.vault.total_capital_collected = new_total_capital_locked.unwrap();

            self.withdraw_rewards(amount)?;
        }
        Ok(())
    }

    fn deposit_rewards(&mut self, amount: u64) -> Result<()> {
        let transfer_accounts = TransferChecked {
            from: self.capital_provider_token_ata.to_account_info(),
            to: self.vault_token_ata.to_account_info(),
            authority: self.capital_provider.to_account_info(),
            mint: self.locking_token_mint.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, transfer_accounts);
        transfer_checked(cpi_ctx, amount, self.locking_token_mint.decimals)?;
        Ok(())
    }

    fn withdraw_rewards(&mut self, amount: u64) -> Result<()> {
        let transfer_accounts = TransferChecked {
            from: self.vault_token_ata.to_account_info(),
            to: self.capital_provider_token_ata.to_account_info(),
            authority: self.vault.to_account_info(),
            mint: self.locking_token_mint.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        let node_operator_key = self.vault.node_operator.key();
        let signer_seeds: &[&[&[u8]]] =
            &[&[b"Vault", node_operator_key.as_ref(), &[self.vault.bump]]];
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, transfer_accounts, signer_seeds);
        transfer_checked(cpi_ctx, amount, self.locking_token_mint.decimals)?;
        Ok(())
    }
}
