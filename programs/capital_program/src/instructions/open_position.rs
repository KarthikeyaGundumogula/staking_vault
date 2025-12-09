use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

use crate::errors::PositionError;
use crate::state::*;

use nft_program::cpi::accounts::CreateAsset;
use nft_program::instructions::CreateAssetArgs;
use nft_program::program::NftProgram;
use nft_program::state::NFTConfig;

#[derive(Accounts)]
pub struct OpenPosition<'info> {
    #[account(mut)]
    pub capital_provider: Signer<'info>,
    pub asset: Signer<'info>,
    /// CHECK: this will be checked by the mpl-core program
    pub vault_collection: UncheckedAccount<'info>,
    #[account(
      seeds = [b"Vault",vault.node_operator.key().as_ref()],
      bump = vault.bump,
    )]
    pub vault: Account<'info, Vault>,
    #[account(
        seeds = [b"Config"],
        bump = config.bump
    )]
    pub config: Account<'info, AuthorityConfig>,
    pub nft_config: Account<'info, NFTConfig>,
    #[account(
      init,
      payer = capital_provider,
      space= Position::INIT_SPACE,
      seeds = [b"Position",asset.key().as_ref()],
      bump,
    )]
    pub position: Account<'info, Position>,
    #[account(
      mut,
      associated_token::mint = locked_token_mint,
      associated_token::authority = capital_provider,
      associated_token::token_program = token_program
    )]
    pub capital_provider_token_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
      init_if_needed,
      payer = capital_provider,
      associated_token::mint = locked_token_mint,
      associated_token::authority = vault,
      associated_token::token_program = token_program
    )]
    pub vault_ata: InterfaceAccount<'info, TokenAccount>,
    pub locked_token_mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    /// CHECK: this will be checked at nft program
    pub mpl_core_program: UncheckedAccount<'info>,
    pub nft_program: Program<'info, NftProgram>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> OpenPosition<'info> {
    fn validate(&mut self, amount: u64) -> Result<()> {
        require!(
            amount >= self.vault.min_lock_amount,
            PositionError::AmountTooLow
        );
        require!(
            self.vault.total_capital_collected + amount < self.vault.max_cap,
            PositionError::VaultMaxCapReached
        ); // TODO: allow partial deposits if the partial deposit is greater than the min_amount
        Ok(())
    }
    pub fn init_position(&mut self, amount: u64, bumps: OpenPositionBumps) -> Result<()> {
        self.validate(amount)?;
        self.position.set_inner(Position {
            vault: self.vault.key(),
            total_value_locked: amount,
            total_rewards_claimed: 0,
            is_listed: false,
            bump: bumps.position,
        });
        Ok(())
    }
    pub fn transfer_funds(&mut self, amount: u64) -> Result<()> {
        let transfer_staking_token_accounts = TransferChecked {
            from: self.capital_provider_token_ata.to_account_info(),
            to: self.vault_ata.to_account_info(),
            authority: self.capital_provider.to_account_info(),
            mint: self.locked_token_mint.to_account_info(),
        };
        let token_program = self.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(token_program, transfer_staking_token_accounts);
        self.vault.total_capital_collected = self
            .vault
            .total_capital_collected
            .checked_add(amount)
            .ok_or(PositionError::ArithmeticOverflow)?;
        transfer_checked(cpi_ctx, amount, self.locked_token_mint.decimals)?;
        Ok(())
    }
    pub fn mint_asset(&mut self) -> Result<()> {
        let mint_asset_accounts = CreateAsset {
            asset: self.asset.to_account_info(),
            payer: self.capital_provider.to_account_info(),
            owner: self.capital_provider.to_account_info(),
            system_program: self.system_program.to_account_info(),
            mpl_core_program: self.mpl_core_program.to_account_info(),
            collection: self.vault_collection.to_account_info(),
            config: self.nft_config.to_account_info(),
            collection_update_authority: self.config.to_account_info(),
        };
        let signer_seeds: &[&[&[u8]]] = &[&[b"Config", &[self.config.bump]]];
        let mint_cpi = CpiContext::new_with_signer(
            self.nft_program.to_account_info(),
            mint_asset_accounts,
            signer_seeds,
        );

        let args = CreateAssetArgs {
            name: String::from("Vault NFT"),
            uri: String::from("MINT_URI"),
        };

        nft_program::cpi::create_core_asset(mint_cpi, args)
            .map_err(|_| error!(PositionError::CPIFail))?;

        Ok(())
    }
}
