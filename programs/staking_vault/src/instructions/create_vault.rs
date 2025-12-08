use crate::state::StakingVault;
use nft_marketplace::cpi::accounts::CreateAsset;
use nft_marketplace::instructions::CreateAssetArgs;
use nft_marketplace::program::NftMarketplace;

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

#[derive(Accounts)]
pub struct Open<'info> {
    #[account(mut)]
    pub provider: Signer<'info>,
    #[account(
      init,
      payer = provider,
      seeds = [b"staking_vault",provider.key().as_ref()],
      space = StakingVault::INIT_SPACE + 8,
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
      init_if_needed,
      payer = provider,
      associated_token::mint = reward_token_mint,
      associated_token::authority = staking_vault,
      associated_token::token_program = token_program
    )]
    pub vault_reward_token_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub asset: Signer<'info>,
    pub staking_token_mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    /// CHECK: this account will be checked my the mpl_core_program
    pub mpl_core_program: UncheckedAccount<'info>,
    pub nft_marketplace: Program<'info, NftMarketplace>,
    pub system_program: Program<'info, System>,
}

impl<'info> Open<'info> {
    pub fn init_config(&mut self, config: InitConfig, bumps: OpenBumps) -> Result<()> {
        self.staking_vault.set_inner(StakingVault {
            provider: self.provider.key(),
            duration: config.duration,
            start_time: 0,
            rewards_value: config.initial_deposit,
            staked_value: 0,
            staking_mint: self.staking_token_mint.key(),
            reward_mint: self.reward_token_mint.key(),
            nft_mint: self.asset.key(),
            bump: bumps.staking_vault,
            minimum_amount: config.min_amount,
            maximum_amount: config.max_amount,
        });
        Ok(())
    }
    pub fn deposit_rewards(&mut self) -> Result<()> {
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
            self.staking_vault.rewards_value,
            self.reward_token_mint.decimals,
        )
    }

    pub fn mint_asset(&mut self) -> Result<()> {
        // let mint_asset_accounts = CreateAsset {
        //     asset: self.asset.to_account_info(),
        //     payer: self.provider.to_account_info(),
        //     owner: Some(self.staking_vault.to_account_info()),
        //     system_program: self.system_program.to_account_info(),
        //     mpl_core_program: self.mpl_core_program.to_account_info(),
        //     collection:self.collection,
        // };

        // let mint_cpi = CpiContext::new(
        //     self.nft_marketplace.to_account_info(),
        //     mint_asset_accounts
        // );

        // let args = CreateAssetArgs {
        //     name: String::from("Vault NFT"),
        //     uri: String::from("MINT_URI"),
        // };

        // // call the CPI — now the staking_vault PDA will correctly be treated as a 'signer'
        // nft_marketplace::cpi::create_core_asset(mint_cpi, args)
        //     .map_err(|_| error!(StakingError::CPIFail))?;

        Ok(())
    }
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct InitConfig {
    pub duration: u64,
    pub min_amount: u64,
    pub max_amount: u64,
    pub initial_deposit: u64,
    pub staker: Pubkey,
}

#[error_code]
pub enum StakingError {
    #[msg("Minting CPI failed")]
    CPIFail,
}
