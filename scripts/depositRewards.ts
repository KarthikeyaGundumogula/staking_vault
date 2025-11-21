import {
  Address,
  createTransaction,
  signTransactionMessageWithSigners,
  getExplorerLink,
  getSignatureFromTransaction,
} from "gill";
import {
  ASSOCIATED_TOKEN_PROGRAM_ADDRESS,
  TOKEN_PROGRAM_ADDRESS,
  SYSTEM_PROGRAM_ADDRESS,
} from "gill/programs";
import {
  getDepositRewardsInstruction,
  STAKING_VAULT_PROGRAM_ADDRESS,
} from "./codama/generated";
import { Client, getClient, getAccounts, airdrop } from "./helpers/setUp";
import { fund_rewardToken } from "./helpers/token-ops";

async function depositRewards() {
  const client: Client = await getClient();
  airdrop(client, client.staker.address);
  airdrop(client, client.provider.address); // copy paste the Asset address used during open vault -- get from logs of running anchor run open
  const REWARD_TOKEN_MINT =
    "AMtERDjDZi1ooaVHv92XyQDXrdt8Nr9pd7oyrrbx3WR7" as Address;
  const reward_token_atas = await getAccounts(
    REWARD_TOKEN_MINT,
    client.provider.address,
    client.staker.address
  );
  await fund_rewardToken(client, REWARD_TOKEN_MINT);
  console.log("Accounts being passed to depositRewards instruction:", {
    provider: client.provider.address,
    stakingVault: reward_token_atas.vault_acc,
    rewardTokenMint: REWARD_TOKEN_MINT,
    vaultRewardTokenAta: reward_token_atas.vault_ata,
    providerRewardTokenAta: reward_token_atas.provider_ata,
  });
  const depositIns = getDepositRewardsInstruction(
    {
      provider: client.provider,
      stakingVault: reward_token_atas.vault_acc,
      rewardTokenMint: REWARD_TOKEN_MINT,
      vaultRewardTokenAta: reward_token_atas.vault_ata,
      providerRewardTokenAta: reward_token_atas.provider_ata,
      tokenProgram: TOKEN_PROGRAM_ADDRESS,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ADDRESS,
      systemProgram: SYSTEM_PROGRAM_ADDRESS,
      amount: BigInt(5000),
    },
    { programAddress: STAKING_VAULT_PROGRAM_ADDRESS }
  );

  console.log("Deposit Rewards Instruction:", depositIns);
  const { value: latestBlockhash } = await client.rpc
    .getLatestBlockhash()
    .send();
  const tx = createTransaction({
    feePayer: client.provider.address,
    latestBlockhash,
    instructions: [depositIns],
  });
  const signedTx = await signTransactionMessageWithSigners(tx);
  try {
    await client.sendAndConfirmTransaction(signedTx);
  } catch (error) {
    console.log(error);
  }
  console.log(
    "Explorer Link:",
    getExplorerLink({
      cluster: "localnet",
      transaction: getSignatureFromTransaction(signedTx),
    })
  );
}

depositRewards();
