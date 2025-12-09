// use crate::state::Vault;
// use anchor_lang::prelude::*;
// use anchor_spl::{
//     associated_token::AssociatedToken,
//     token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
// };

// #[derive(Accounts)]
// pub struct IncrementStake<'info> {
//     #[account(mut)]
//     pub staker: Signer<'info>,
//     #[account(
//       seeds = [b"staking_vault",staking_vault.provider.key().as_ref()],
//       bump = staking_vault.bump
//     )]
//     pub staking_vault: Account<'info, Vault>,
//     #[account(mint::token_program = token_program)]
//     pub staking_token_mint: InterfaceAccount<'info, Mint>,
//     #[account(
//       mut,
//       associated_token::mint = staking_token_mint,
//       associated_token::authority = staking_vault,
//       associated_token::token_program = token_program
//     )]
//     pub vault_staking_token_ata: InterfaceAccount<'info, TokenAccount>,
//     pub token_program: Interface<'info, TokenInterface>,
//     pub associated_token_program: Program<'info, AssociatedToken>,
//     pub system_program: Program<'info, System>,
// }

// impl<'info> IncrementStake<'info> {
//     pub fn deposit_rewards(&mut self, amount: u64) -> Result<()> {
//         let transfer_accounts = TransferChecked {
//             from: self.staker.to_account_info(),
//             to: self.vault_staking_token_ata.to_account_info(),
//             authority: self.staker.to_account_info(),
//             mint: self.staking_token_mint.to_account_info(),
//         };
//         let cpi_program = self.token_program.to_account_info();
//         let cpi_ctx = CpiContext::new(cpi_program, transfer_accounts);
//         transfer_checked(cpi_ctx, amount, self.staking_token_mint.decimals)?;
//         Ok(())
//     }
// }
