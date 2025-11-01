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
  let un_authorized_staker: anchor.web3.Keypair;
  let mintAccounts;
  let stakeAccounts;
  let unStakeAccounts;

  const TOKEN_DECIMALS = 1_000_000; // 6 decimals
  const duration = new anchor.BN(1); // 1 second for testing only
  const min_amount = new anchor.BN(1_000_000); // 1 token
  const max_amount = new anchor.BN(10_000_000_000); // 10,000 tokens
  const initial_rewards_deposit = new anchor.BN(5_000_000_000); // 5,000 tokens
  const stake_amount = new anchor.BN(2_000_000_000); // 2,000 tokens

  before(async () => {
    provider = anchor.web3.Keypair.generate();
    staker = anchor.web3.Keypair.generate();
    god = anchor.web3.Keypair.generate();
    un_authorized_staker = anchor.web3.Keypair.generate();

    await getAirdrop(connection, provider.publicKey);
    await getAirdrop(connection, staker.publicKey);
    await getAirdrop(connection, god.publicKey);
    await getAirdrop(connection, un_authorized_staker.publicKey);

    setupData = await setUp(provider, staker, god, un_authorized_staker);

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

    stakeAccounts = {
      staker: staker.publicKey,
      stakingVault: setupData.vault_state_pda,
      stakingTokenMint: setupData.staking_mint,
      stakerTokenAta: setupData.staker_staking_ata,
      stakingVaultAta: setupData.vault_staking_ata,
      systemProgram: SYSTEM_PROGRAM_ID,
      tokenProgram: TOKEN_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
    };
    unStakeAccounts = {
      ...stakeAccounts,
      vaultRewardAta: setupData.vault_reward_ata,
      stakerRewardAta: setupData.staker_reward_ata,
      rewardTokenMint: setupData.reward_mint,
    };
  });

  it("Open Staking Vault", async () => {
    try {
      const tx = await program.methods
        .open(duration, min_amount, max_amount, initial_rewards_deposit, staker.publicKey)
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
    // run this sigle test for complete workflow testing 
    // try {
    //   const tx = await program.methods
    //     .open(duration, min_amount, max_amount, initial_rewards_deposit,staker.publicKey)
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
        .accountsStrict(stakeAccounts)
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

  it("Unstake Tokens", async () => {
    console.log("--Unstaking Tokens--");
    const staker_balance_pre_unstake = await connection.getTokenAccountBalance(
      setupData.staker_staking_ata
    );
    const staker_reward_balance_pre_unstake = await connection.getTokenAccountBalance(
      setupData.staker_reward_ata
    );
    try {
      const tx = await program.methods
        .unstake()
        .accountsStrict(unStakeAccounts)
        .signers([staker])
        .rpc();
      console.log("Unstake transaction: ", tx);
    } catch (error) {
      console.error("Error in unstaking tokens: ", error);
    }
    // -- Unstaking assertions -- //
    const staker_balance_post_unstake = await connection.getTokenAccountBalance(
      setupData.staker_staking_ata
    );
    assert.equal(
      staker_balance_post_unstake.value.uiAmount -
        staker_balance_pre_unstake.value.uiAmount,
      stake_amount.toNumber() / TOKEN_DECIMALS
    );
    
    const staker_reward_balance_post_unstake = await connection.getTokenAccountBalance(
      setupData.staker_reward_ata
    );
    assert.isTrue(
      staker_reward_balance_post_unstake.value.uiAmount >
        staker_reward_balance_pre_unstake.value.uiAmount
    );
  })

  it("Fail Stake from Unauthorized Staker", async () => {
    console.log("--Staking Tokens from Unauthorized Staker--");
    try {
      const tx = await program.methods
        .stake(stake_amount)
        .accountsStrict({
          ...stakeAccounts,
          staker: un_authorized_staker.publicKey,
          stakerTokenAta: setupData.un_authorized_staker_staking_ata,
        })
        .signers([un_authorized_staker])
        .rpc();
      console.error("Stake transaction should have failed but succeeded: ", tx);
    }
    catch (error) {
      console.error(
        "Error in staking tokens from unauthorized staker: ",
        error
      );

      //--- error you will see is `Error Code: ConstraintHasOne.` ---//
    }
  });
});
