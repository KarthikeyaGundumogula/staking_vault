import * as anchor from "@coral-xyz/anchor";
import { assert, expect } from "chai";
import {
  setUp,
  getAirdrop,
  program,
  nft_program,
  connection,
  SetupResult,
} from "./utils";
import { Keypair } from "@solana/web3.js";
// import provider_wallet from "../provider-wallet.json";
// import wallet from "../staker-wallet.json";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";
import {
  MPL_CORE_PROGRAM_ID,
  fetchAssetsByOwner,
  fetchAsset,
} from "@metaplex-foundation/mpl-core";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";

describe("staking_vault", () => {
  let setupData: SetupResult;
  let provider: anchor.web3.Keypair;
  let staker: anchor.web3.Keypair;
  let god: anchor.web3.Keypair;
  let un_authorized_staker: anchor.web3.Keypair;
  const umi = createUmi("http://0.0.0.0:8899", "confirmed");
  const TOKEN_DECIMALS = 1_000_000; // 6 decimals
  const duration = new anchor.BN(1); // 1 second for testing only
  const min_amount = new anchor.BN(1_000_000); // 1 token
  const max_amount = new anchor.BN(10_000_000_000); // 10,000 tokens
  const initial_rewards_deposit = new anchor.BN(0); // 5,000 tokens
  const stake_amount = new anchor.BN(2_000_000_000); // 2,000 tokens
  const asset = Keypair.generate();

  before(async () => {
    // provider = anchor.web3.Keypair.fromSecretKey(
    //   new Uint8Array(provider_wallet.provider)
    // );
    // staker = anchor.web3.Keypair.fromSecretKey(new Uint8Array(wallet.wallet));
    provider = anchor.web3.Keypair.generate();
    staker = anchor.web3.Keypair.generate();
    god = provider;
    un_authorized_staker = anchor.web3.Keypair.generate();
    setupData = await setUp(provider, staker, god, un_authorized_staker);
  });

  it("Open Staking Vault", async () => {
    let init_config = {
      duration: duration,
      minAmount: min_amount,
      maxAmount: max_amount,
      initialDeposit: initial_rewards_deposit,
      staker: staker.publicKey,
    };

    try {
      const tx = await program.methods
        .open(init_config)
        .accountsStrict({
          provider: provider.publicKey,
          staker: staker.publicKey,
          stakingVault: setupData.vault_state_pda,
          providerRewardTokensAta: setupData.provider_reward_ata,
          rewardTokenMint: setupData.reward_mint,
          vaultRewardTokenAta: setupData.vault_reward_ata,
          stakingTokenMint: setupData.staking_mint,
          asset: asset.publicKey,
          mplCoreProgram: MPL_CORE_PROGRAM_ID,
          nftMarketplace: nft_program.programId,
          systemProgram: SYSTEM_PROGRAM_ID,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        })
        .signers([provider, asset])
        .rpc();
    } catch (error) {
      console.log(error);
      console.error("Error in opening staking vault: ", await error.getLogs());
    }
    const asset_data = await fetchAssetsByOwner(
      umi,
      staker.publicKey.toString()
    );
    // console.log(asset_data);
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
          asset:asset.publicKey
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

  it("Unstake Tokens", async () => {
    console.log("--Unstaking Tokens--");
    const staker_balance_pre_unstake = await connection.getTokenAccountBalance(
      setupData.staker_staking_ata
    );
    const staker_reward_balance_pre_unstake =
      await connection.getTokenAccountBalance(setupData.staker_reward_ata);
    try {
      const tx = await program.methods
        .unstake()
        .accountsStrict({
          staker: staker.publicKey,
          stakingVault: setupData.vault_state_pda,
          stakerTokenAta: setupData.staker_staking_ata,
          asset:asset.publicKey,
          stakingVaultAta: setupData.vault_staking_ata,
          vaultRewardAta: setupData.vault_reward_ata,
          stakerRewardAta: setupData.staker_reward_ata,
          rewardTokenMint: setupData.reward_mint,
          stakingTokenMint: setupData.staking_mint,
          systemProgram: SYSTEM_PROGRAM_ID,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        })
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

    const staker_reward_balance_post_unstake =
      await connection.getTokenAccountBalance(setupData.staker_reward_ata);
    assert.isTrue(
      staker_reward_balance_post_unstake.value.uiAmount >=
        staker_reward_balance_pre_unstake.value.uiAmount
    );
  });
  it(" deposit Reawards", async () => {
    console.log("--Depositing Rewards--");
    const vault_reward_balance_pre_deposit =
      await connection.getTokenAccountBalance(setupData.vault_reward_ata);
    try {
      const tx = await program.methods
        .depositRewards(new anchor.BN(5000_000_000)) // 5000 tokens
        .accountsStrict({
          provider: provider.publicKey,
          stakingVault: setupData.vault_state_pda,
          rewardTokenMint: setupData.reward_mint,
          vaultRewardTokenAta: setupData.vault_reward_ata,
          providerRewardTokenAta: setupData.provider_reward_ata,
          systemProgram: SYSTEM_PROGRAM_ID,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        })
        .signers([provider])
        .rpc();
      console.log("Deposit Rewards transaction: ", tx);
    } catch (error) {
      console.error("Error in depositing rewards: ", error);
    }
    // -- Deposit Rewards assertions -- //
    const vault_reward_balance_post_deposit =
      await connection.getTokenAccountBalance(setupData.vault_reward_ata);
    assert.equal(
      vault_reward_balance_post_deposit.value.uiAmount -
        vault_reward_balance_pre_deposit.value.uiAmount,
      5000
    );
  });
  it("Fail Stake from Unauthorized Staker", async () => {
    console.log("--Staking Tokens from Unauthorized Staker--");
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
          asset: asset.publicKey,
        })
        .signers([un_authorized_staker])
        .rpc();
      console.error("Stake transaction should have failed but succeeded: ", tx);
      expect.fail();
    } catch (error) {
      expect(error).to.exist;
      //--- error you will see is `Error Code: ConstraintHasOne.` ---//
    }
  });
});
