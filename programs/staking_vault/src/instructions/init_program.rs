use anchor_lang::prelude::*;

use nft_marketplace::cpi::accounts::InitNFTProgram;
use nft_marketplace::program::NftMarketplace;
use nft_marketplace::state::Config as NftCfg;

use crate::errors::InitError;
use crate::state::Config;

#[derive(Accounts)]
pub struct InitProgram<'info> {
    #[account(
    init,
    payer = admin,
    seeds = [b"Config"],
    space = Config::INIT_SPACE,
    bump
  )]
    pub config: Account<'info, Config>,
    pub nft_config: Account<'info, NftCfg>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub nft_program: Program<'info, NftMarketplace>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitProgram<'info> {
    pub fn init(
        &mut self,
        params:InitParams,
        bumps: InitProgramBumps,
    ) -> Result<()> {
        self.config.set_inner(Config {
            nft_program: *self.nft_program.key,
            admin: *self.admin.key,
            agent:params.agent,
            bump: bumps.config,
            early_unlock_fee:params.early_unlock_fee,
            dispute_window:params.dispute_window,
            min_lock_duration:params.min_lock_duration,
            max_lock_duration:params.max_lock_duration
        });
        Ok(())
    }

    pub fn init_nft_program(&mut self, program_id: Pubkey) -> Result<()> {
        let init_nft_accounts = InitNFTProgram {
            admin: self.admin.to_account_info(),
            authority: self.config.to_account_info(),
            config: self.nft_config.to_account_info(),
            system_program: self.system_program.to_account_info(),
        };
        let signer_seeds: &[&[&[u8]]] = &[&[b"Config", &[self.config.bump]]];
        let nft_init_ctx = CpiContext::new_with_signer(
            self.nft_program.to_account_info(),
            init_nft_accounts,
            signer_seeds,
        );
        nft_marketplace::cpi::initialize_program(nft_init_ctx, program_id)
            .map_err(|_| error!(InitError::CPIFail))?;
        Ok(())
    }
}

#[derive(AnchorDeserialize,AnchorSerialize)]
pub struct InitParams{
    agent: Pubkey,
    early_unlock_fee:u64,
    dispute_window:i64,
    max_lock_duration:i64,
    min_lock_duration:i64
}
