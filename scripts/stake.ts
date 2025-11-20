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
import { getStakeInstruction, fetchStakingVault } from "./codama/generated";
import {
  Client,
  getClient,
  getAccounts,
  NFT_MARKETPLACE_ID,
  STAKING_VAULT_ID,
  airdrop,
} from "./helpers/setUp";
import { MPL_CORE_PROGRAM_ID } from "@metaplex-foundation/mpl-core";
import { fund_stakingToken } from "./helpers/token-ops";

async function stake() {
  const client: Client = await getClient();
  airdrop(client, client.staker.address);
  airdrop(client, client.provider.address);
  const STAKING_TOKEN_MINT =
    "737MzWSJUXE4fvX7t7RxTccAtDyoN6PxNdpvb7XX6qzR" as Address; // copy paste the mint address used during open vault -- get from logs of running anchor run open
  const asset = "Y1Y9AqUJnS1CCKy6bpT5qQZtiByfqrMjqqJpgonhVmb" as Address; // copy paste the Asset address used during open vault -- get from logs of running anchor run open
  await fund_stakingToken(client, STAKING_TOKEN_MINT);
  const staking_token_atas = await getAccounts(
    STAKING_TOKEN_MINT,
    client.provider.address,
    client.staker.address
  );
  console.log("Accounts being passed to stakeInx instruction:", {
    staker: client.staker.address,
    stakingVault: staking_token_atas.vault_acc,
    stakerTokenAta: staking_token_atas.staker_ata,
    stakingVaultAta: staking_token_atas.vault_ata,
    stakingTokenMint: STAKING_TOKEN_MINT,
    systemProgram: SYSTEM_PROGRAM_ADDRESS,
    tokenProgram: TOKEN_PROGRAM_ADDRESS,
    associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ADDRESS,
    amount: BigInt(100000),
  });

  const vault_data = await fetchStakingVault(
    client.rpc,
    staking_token_atas.vault_acc
  );
  console.log("Fetched Vault Data:", vault_data);
  const stakeInx = getStakeInstruction(
    {
      staker: client.staker,
      stakingVault: "BFkXqFHqLBJGs8yXSLtAc4qLUQu7ibJTKLtFWdiXCLHD" as Address,
      stakerTokenAta: staking_token_atas.staker_ata,
      stakingVaultAta: staking_token_atas.vault_ata,
      stakingTokenMint: STAKING_TOKEN_MINT,
      tokenProgram: TOKEN_PROGRAM_ADDRESS,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ADDRESS,
      systemProgram: SYSTEM_PROGRAM_ADDRESS,
      amount: BigInt(1000),
      asset: asset,
    },
    { programAddress: STAKING_VAULT_ID as Address }
  );
  const { value: latestBlockhash } = await client.rpc
    .getLatestBlockhash()
    .send();
  const tx = createTransaction({
    feePayer: client.staker.address,
    latestBlockhash,
    instructions: [stakeInx],
  });
  const signedTx = await signTransactionMessageWithSigners(tx);
  try {
    await client.sendAndConfirmTransaction(signedTx);
  } catch (error) {
    console.log(error);
    console.error("Error during staking transaction:", error);
  }
  console.log(
    "Explorer Link:",
    getExplorerLink({
      cluster: "localnet",
      transaction: getSignatureFromTransaction(signedTx),
    })
  );
}

stake();
