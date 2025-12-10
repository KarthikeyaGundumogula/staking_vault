use crate::constants::BASE_BPS;
use crate::{errors::ClaimRewardsError, state::*};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

#[derive(Accounts)]
pub struct ClaimBeneficiaryRewards<'info> {
    #[account(mut)]
    pub beneficiary: Signer<'info>,

    #[account(
        mut,
        seeds = [b"Vault", vault.node_operator.key().as_ref()],
        bump = vault.bump,
    )]
    pub vault: Account<'info, Vault>,

    #[account(mint::token_program = token_program)]
    pub reward_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = reward_mint,
        associated_token::authority = vault,
        associated_token::token_program = token_program
    )]
    pub vault_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = beneficiary,
        associated_token::mint = reward_mint,
        associated_token::authority = beneficiary,
        associated_token::token_program = token_program
    )]
    pub beneficiary_ata: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> ClaimBeneficiaryRewards<'info> {
    pub fn calculate_claimable(&self, beneficiary_index: u8) -> Result<u64> {
        require!(
            beneficiary_index < self.vault.beneficiaries.len() as u8,
            ClaimRewardsError::InvalidBeneficiaryIndex
        );

        let beneficiary = &self.vault.beneficiaries[beneficiary_index as usize];

        // Validate caller is the beneficiary
        require_keys_eq!(
            beneficiary.address,
            self.beneficiary.key(),
            ClaimRewardsError::UnauthorizedBeneficiary
        );

        // Calculate total rewards for beneficiaries
        let total_beneficiary_bps: u16 = self.vault.beneficiaries.iter().map(|b| b.share_bps).sum();

        let total_beneficiary_rewards = self
            .vault
            .total_rewards_deposited
            .checked_mul(total_beneficiary_bps as u64)
            .ok_or(ClaimRewardsError::ArithmeticOverflow)?
            .checked_div(BASE_BPS as u64)
            .ok_or(ClaimRewardsError::ArithmeticOverflow)?;

        // Calculate this beneficiary's share
        let beneficiary_total = total_beneficiary_rewards
            .checked_mul(beneficiary.share_bps as u64)
            .ok_or(ClaimRewardsError::ArithmeticOverflow)?
            .checked_div(total_beneficiary_bps as u64)
            .ok_or(ClaimRewardsError::ArithmeticOverflow)?;

        // Calculate claimable (total - already claimed)
        let claimable = beneficiary_total
            .checked_sub(beneficiary.total_claimed)
            .ok_or(ClaimRewardsError::ArithmeticUnderflow)?;

        Ok(claimable)
    }

    pub fn process_claim(&mut self, beneficiary_index: u8, amount: u64) -> Result<()> {
        require_gt!(amount, 0, ClaimRewardsError::ZeroClaimAmount);

        // Update beneficiary's claimed amount
        self.vault.beneficiaries[beneficiary_index as usize].total_claimed =
            self.vault.beneficiaries[beneficiary_index as usize]
                .total_claimed
                .checked_add(amount)
                .ok_or(ClaimRewardsError::ArithmeticOverflow)?;

        Ok(())
    }

    pub fn transfer_rewards(&self, amount: u64) -> Result<()> {
        let node_operator_key = self.vault.node_operator.key();
        let signer_seeds: &[&[&[u8]]] =
            &[&[b"Vault", node_operator_key.as_ref(), &[self.vault.bump]]];

        let transfer_accounts = TransferChecked {
            from: self.vault_ata.to_account_info(),
            to: self.beneficiary_ata.to_account_info(),
            authority: self.vault.to_account_info(),
            mint: self.reward_mint.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            transfer_accounts,
            signer_seeds,
        );

        transfer_checked(cpi_ctx, amount, self.reward_mint.decimals)?;

        Ok(())
    }
}
