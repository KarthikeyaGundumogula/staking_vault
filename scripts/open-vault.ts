import {
  Address,
  createTransaction,
  generateKeyPairSigner,
  signTransactionMessageWithSigners,
  getExplorerLink,
  getSignatureFromTransaction,
} from "gill";
import {
  ASSOCIATED_TOKEN_PROGRAM_ADDRESS,
  TOKEN_PROGRAM_ADDRESS,
  SYSTEM_PROGRAM_ADDRESS,
} from "gill/programs";
import { getOpenInstruction } from "./codama/generated";
import { MPL_CORE_PROGRAM_ID } from "@metaplex-foundation/mpl-core";
import {
  Client,
  getClient,
  getAccounts,
  NFT_MARKETPLACE_ID,
  STAKING_VAULT_ID,
  airdrop,
} from "./helpers/setUp";
import {
  createFungibleToken,
  fund_rewardToken,
  fund_stakingToken,
} from "./helpers/token-ops";

async function open() {
  const asset = await generateKeyPairSigner();
  const client: Client = await getClient();
  airdrop(client, client.staker.address);
  airdrop(client, client.provider.address);
  const staking_token_mint = await createFungibleToken(client);
  await fund_stakingToken(client, staking_token_mint.address);
  const staking_token_atas = await getAccounts(
    staking_token_mint.address,
    client.provider.address,
    client.staker.address
  );
  const reward_token_mint = await createFungibleToken(client);
  await fund_rewardToken(client, reward_token_mint.address);
  const reward_token_atas = await getAccounts(
    reward_token_mint.address,
    client.provider.address,
    client.staker.address
  );
  console.log("Accounts being passed to openInx instruction:", {
    provider: client.provider.address,
    staker: client.staker.address,
    providerRewardTokensAta: reward_token_atas.provider_ata,
    rewardTokenMint: reward_token_mint.address,
    vaultRewardTokenAta: reward_token_atas.vault_ata,
    stakingTokenMint: staking_token_mint.address,
    asset: asset.address,
    mplCoreProgram: MPL_CORE_PROGRAM_ID,
    nftMarketplace: NFT_MARKETPLACE_ID,
    systemProgram: SYSTEM_PROGRAM_ADDRESS,
    tokenProgram: TOKEN_PROGRAM_ADDRESS,
    associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ADDRESS,
    stakingVault: reward_token_atas.vault_acc,
    stakerArg: client.staker.address,
  });
  const openInx = getOpenInstruction(
    {
      provider: client.provider,
      staker: client.staker.address,
      providerRewardTokensAta: reward_token_atas.provider_ata,
      rewardTokenMint: reward_token_mint.address,
      vaultRewardTokenAta: reward_token_atas.vault_ata,
      stakingTokenMint: staking_token_mint.address,
      asset: asset,
      mplCoreProgram: MPL_CORE_PROGRAM_ID as unknown as Address,
      nftMarketplace: NFT_MARKETPLACE_ID as Address,
      systemProgram: SYSTEM_PROGRAM_ADDRESS,
      tokenProgram: TOKEN_PROGRAM_ADDRESS,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ADDRESS,
      duration: BigInt(1),
      minAmount: BigInt(0),
      maxAmount: BigInt(10000000000),
      initialDeposit: BigInt(0),
      stakingVault: reward_token_atas.vault_acc,
      stakerArg: client.staker.address,
    },
    { programAddress: STAKING_VAULT_ID as Address }
  );

  const { value: latestBlockhash } = await client.rpc
    .getLatestBlockhash()
    .send();

  const tx = createTransaction({
    feePayer: client.provider,
    version: "auto",
    instructions: [openInx],
    latestBlockhash,
  });
  const signed_tx = await signTransactionMessageWithSigners(tx);
  try {
    await client.sendAndConfirmTransaction(signed_tx);
  } catch (e) {
    console.error(e);
  }
  console.log(
    "Explorer:",
    getExplorerLink({
      cluster: "localnet",
      transaction: getSignatureFromTransaction(signed_tx),
    })
  );
}

open();
