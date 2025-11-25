import {
  Address,
  createTransaction,
  signTransactionMessageWithSigners,
  getExplorerLink,
  getSignatureFromTransaction,
  Account,
} from "gill";
import {
  ASSOCIATED_TOKEN_PROGRAM_ADDRESS,
  TOKEN_PROGRAM_ADDRESS,
  SYSTEM_PROGRAM_ADDRESS,
} from "gill/programs";
import {
  getStakeInstruction,
  fetchStakingVault,
  StakingVault,
} from "./codama/generated";
import {
  Client,
  getClient,
  getAccounts,
  STAKING_VAULT_ID,
  airdrop,
} from "./helpers/setUp";
import { fund_stakingToken } from "./helpers/token-ops";

async function stake() {
  const client: Client = await getClient();
  airdrop(client, client.staker.address);
  airdrop(client, client.provider.address);
  const vault_data: Account<StakingVault> = await fetchStakingVault(
    client.rpc,
    client.vault_state_pda
  );
  const STAKING_TOKEN_MINT = vault_data.data.stakingMint;
  const asset = vault_data.data.nftMint;
  await fund_stakingToken(client, STAKING_TOKEN_MINT);
  const staking_token_atas = await getAccounts(
    STAKING_TOKEN_MINT,
    client.provider.address,
    client.staker.address
  );
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
