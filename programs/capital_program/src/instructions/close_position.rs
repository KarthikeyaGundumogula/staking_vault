// use crate::state::Vault;
// use anchor_lang::prelude::*;
// use anchor_spl::{
//     associated_token::AssociatedToken,
//     token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
// };
// use mpl_core::accounts::BaseAssetV1;

// #[derive(Accounts)]
// pub struct UnStake<'info> {
//     #[account(mut)]
//     pub staker: Signer<'info>,
//     #[account(
//       seeds = [b"staking_vault",staking_vault.provider.key().as_ref()],
//       bump = staking_vault.bump,
//     )]
//     pub staking_vault: Account<'info, Vault>,
//     #[account(
//       mut,
//       associated_token::mint = staking_token_mint,
//       associated_token::authority = staker,
//       associated_token::token_program = token_program
//     )]
//     pub staker_token_ata: InterfaceAccount<'info, TokenAccount>,
//     #[account(address = staking_vault.nft_mint)]
//     pub asset: Account<'info, BaseAssetV1>,
//     #[account(
//       mut,
//       associated_token::mint = staking_token_mint,
//       associated_token::authority = staking_vault,
//       associated_token::token_program = token_program
//     )]
//     pub staking_vault_ata: InterfaceAccount<'info, TokenAccount>,
//     #[account(
//       mut,
//       associated_token::mint = reward_token_mint,
//       associated_token::authority = staking_vault,
//       associated_token::token_program = token_program
//     )]
//     pub vault_reward_ata: InterfaceAccount<'info, TokenAccount>,
//     #[account(
//       mut,
//       associated_token::mint = reward_token_mint,
//       associated_token::authority = staker,
//       associated_token::token_program = token_program
//     )]
//     pub staker_reward_ata: InterfaceAccount<'info, TokenAccount>,
//     pub reward_token_mint: InterfaceAccount<'info, Mint>,
//     pub staking_token_mint: InterfaceAccount<'info, Mint>,
//     pub token_program: Interface<'info, TokenInterface>,
//     pub associated_token_program: Program<'info, AssociatedToken>,
//     pub system_program: Program<'info, System>,
// }

// impl<'info> UnStake<'info> {
//     pub fn transfer_staked_tokens(&self, amount: u64) -> Result<()> {
//         require!(
//             self.asset.owner == self.staker.key(),
//             UnstakeError::OnlyNFTOwner
//         );
//         let cpi_accounts = TransferChecked {
//             from: self.staking_vault_ata.to_account_info(),
//             to: self.staker_token_ata.to_account_info(),
//             authority: self.staking_vault.to_account_info(),
//             mint: self.staking_token_mint.to_account_info(),
//         };
//         let provider = self.staking_vault.provider.key();
//         let seeds = &[
//             b"staking_vault",
//             provider.as_ref(),
//             &[self.staking_vault.bump],
//         ];
//         let signer = &[&seeds[..]];
//         let cpi_program = self.token_program.to_account_info();
//         let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
//         transfer_checked(cpi_ctx, amount, self.staking_token_mint.decimals)
//     }

//     pub fn transfer_rewards(&self, amount: u64) -> Result<()> {
//         let cpi_accounts = TransferChecked {
//             from: self.vault_reward_ata.to_account_info(),
//             to: self.staker_reward_ata.to_account_info(), // change this NFT holder ATA
//             authority: self.staking_vault.to_account_info(),
//             mint: self.reward_token_mint.to_account_info(),
//         };
//         let provider = self.staking_vault.provider.key();
//         let seeds = &[
//             b"staking_vault",
//             provider.as_ref(),
//             &[self.staking_vault.bump],
//         ];
//         let signer = &[&seeds[..]];
//         let cpi_program = self.token_program.to_account_info();
//         let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
//         transfer_checked(cpi_ctx, amount, self.reward_token_mint.decimals)
//     }
// }

// #[error_code]
// pub enum UnstakeError {
//     #[msg("Only owner of the NFT can Unstake")]
//     OnlyNFTOwner,
// }
