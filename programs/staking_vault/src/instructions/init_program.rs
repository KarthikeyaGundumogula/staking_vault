use anchor_lang::prelude::*;
use nft_marketplace::program::NftMarketplace;

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
    #[account(mut)]
    pub admin: Signer<'info>,
    pub nft_program: Program<'info, NftMarketplace>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct Config {
    pub nft_program: Pubkey,
    pub admin: Pubkey,
    pub agent: Pubkey,
    pub early_unlock_fee: u64, // in bps to the base 10_000
    pub bump: u8,
}

impl<'info> InitProgram<'info> {
    pub fn init(
        &mut self,
        agent: Pubkey,
        early_unlock_fee: u64,
        bumps: InitProgramBumps,
    ) -> Result<()> {
        self.config.set_inner(Config {
            nft_program: *self.nft_program.key,
            admin: *self.admin.key,
            agent,
            early_unlock_fee,
            bump: bumps.config,
        });
        Ok(())
    }

    pub fn update_authorities() -> Result<()> {
      Ok(())
    }
}
