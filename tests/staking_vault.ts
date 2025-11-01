import * as anchor from "@coral-xyz/anchor";
import { assert } from "chai";
import { setUp, getAirdrop, program, connection, SetupResult } from "./utils";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";

describe("staking_vault", () => {
  let setupData: SetupResult;
  let provider: anchor.web3.Keypair;
  let staker: anchor.web3.Keypair;
  let god: anchor.web3.Keypair;
  let mintAccounts;

  const TOKEN_DECIMALS = 1_000_000; // 6 decimals
  const duration = new anchor.BN(60 * 60); // 1 hour
  const min_amount = new anchor.BN(1_000_000); // 1 token
  const max_amount = new anchor.BN(10_000_000_000); // 10,000 tokens
  const initial_rewards_deposit = new anchor.BN(5_000_000_000); // 5,000 tokens
  const stake_amount = new anchor.BN(2_000_000_000); // 2,000 tokens

  before(async () => {
    provider = anchor.web3.Keypair.generate();
    staker = anchor.web3.Keypair.generate();
    god = anchor.web3.Keypair.generate();

    await getAirdrop(connection, provider.publicKey);
    await getAirdrop(connection, staker.publicKey);
    await getAirdrop(connection, god.publicKey);

    setupData = await setUp(provider, staker, god);

    mintAccounts = {
      provider: provider.publicKey,
      stakingVault: setupData.vault_state_pda,
      providerRewardTokensAta: setupData.provider_reward_ata,
      rewardTokenMint: setupData.reward_mint,
      vaultRewardTokenAta: setupData.vault_reward_ata,
      stakingTokenMint: setupData.staking_mint,
      systemProgram: SYSTEM_PROGRAM_ID,
      tokenProgram: TOKEN_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
    };
  });

  it("Open Staking Vault", async () => {
    try {
      const tx = await program.methods
        .open(duration, min_amount, max_amount, initial_rewards_deposit)
        .accountsStrict(mintAccounts)
        .signers([provider])
        .rpc();
    } catch (error) {
      console.error("Error in opening staking vault: ", await error.getLogs());
    }

    const vaultAccount = await program.account.stakingVault.fetch(
      setupData.vault_state_pda
    );
    // initialize assertions
    assert.ok(vaultAccount.provider.equals(provider.publicKey));
    assert.ok(vaultAccount.duration.eq(duration));
    assert.ok(vaultAccount.stakingMint.equals(setupData.staking_mint));
    assert.ok(vaultAccount.rewardMint.equals(setupData.reward_mint));
    assert.ok(vaultAccount.maximumAmount.eq(max_amount));
    assert.ok(vaultAccount.minimumAmount.eq(min_amount));

    // deposit assertions
    const vaultRewards = await connection.getTokenAccountBalance(
      setupData.vault_reward_ata
    );
    assert.equal(
      vaultRewards.value.uiAmount,
      initial_rewards_deposit.toNumber() / TOKEN_DECIMALS
    );
  });

  it("Stake Tokens", async () => {
    console.log("--Opening Vault--");
    // try {
    //   const tx = await program.methods
    //     .open(duration, min_amount, max_amount, initial_rewards_deposit)
    //     .accountsStrict(mintAccounts)
    //     .signers([provider])
    //     .rpc();
    // } catch (error) {
    //   console.error("Error in opening staking vault: ", await error.getLogs());
    // }
    console.log("--Staking Tokens--");
    const staker_balance_pre_stake = await connection.getTokenAccountBalance(
      setupData.staker_staking_ata
    );
    try {
      const tx = await program.methods
        .stake(stake_amount)
        .accountsStrict({
          staker: staker.publicKey,
          stakingVault: setupData.vault_state_pda,
          stakingTokenMint: setupData.staking_mint,
          stakerTokenAta: setupData.staker_staking_ata,
          stakingVaultAta: setupData.vault_staking_ata,
          systemProgram: SYSTEM_PROGRAM_ID,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        })
        .signers([staker])
        .rpc();
      console.log("Stake transaction: ", tx);
    } catch (error) {
      console.error("Error in staking tokens: ", error);
    }

    // -- Staking assertions -- //
    const vaultStakingBalance = await connection.getTokenAccountBalance(
      setupData.vault_staking_ata
    );
    assert.equal(
      vaultStakingBalance.value.uiAmount,
      stake_amount.toNumber() / TOKEN_DECIMALS
    );

    const staker_balance_post_stake = await connection.getTokenAccountBalance(
      setupData.staker_staking_ata
    );
    assert.equal(
      staker_balance_pre_stake.value.uiAmount -
        staker_balance_post_stake.value.uiAmount,
      stake_amount.toNumber() / TOKEN_DECIMALS
    );
  });
});
