use crate::state::Config;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitNFTProgram<'info> {
    #[account(
    init,
    seeds = [b"nft_config"],
    bump,
    payer = admin,
    space = Config::INIT_SPACE
  )]
    pub config: Account<'info, Config>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitNFTProgram<'info> {
    pub fn initialize(&mut self, bumps: InitNFTProgramBumps,capital_program:Pubkey) -> Result<()> {
        self.config.set_inner(Config {
            capital_program: capital_program,
            authority: *self.authority.key,
            admin: *self.admin.key,
            bump: bumps.config,
        });
        Ok(())
    }
}
