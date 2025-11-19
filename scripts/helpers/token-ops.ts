import { getCreateAccountInstruction } from "@solana-program/system";
import { Client, getClient, getAccounts } from "./setUp";
import {
  generateKeyPairSigner,
  createTransaction,
  Address,
  getExplorerLink,
  getSignatureFromTransaction,
  signTransactionMessageWithSigners,
  TransactionSigner,
} from "gill";
import {
  TOKEN_PROGRAM_ADDRESS,
  getTokenMetadataAddress,
  getMintSize,
  getInitializeMintInstruction,
  getCreateMetadataAccountV3Instruction,
  buildMintTokensTransaction,
  buildTransferTokensTransaction,
} from "gill/programs";

export async function createFungibleToken(client: Client) {

  const tokenProgram = TOKEN_PROGRAM_ADDRESS;
  const mintSize = getMintSize();
  const [mint, rentExemption] = await Promise.all([
    generateKeyPairSigner(),
    client.rpc.getMinimumBalanceForRentExemption(BigInt(mintSize)).send(),
  ]);
  const metadataAddress = await getTokenMetadataAddress(mint);

  const createAccountIx = getCreateAccountInstruction({
    space: mintSize,
    lamports: rentExemption,
    newAccount: mint,
    payer: client.god,
    programAddress: tokenProgram,
  });
  const initializeMintIx = getInitializeMintInstruction(
    {
      mint: mint.address,
      mintAuthority: client.god.address,
      freezeAuthority: client.god.address,
      decimals: 2,
    },
    {
      programAddress: tokenProgram,
    }
  );
  const metadataIx = getCreateMetadataAccountV3Instruction({
    collectionDetails: null,
    isMutable: true,
    updateAuthority: client.god,
    mint: mint.address,
    metadata: metadataAddress,
    mintAuthority: client.god,
    payer: client.god,
    data: {
      sellerFeeBasisPoints: 0,
      collection: null,
      creators: null,
      uses: null,
      name: "OG-Faisla",
      symbol: "OGF",
      uri: "karthikeya.framer.ai",
    },
  });

  const { value: latestBlockhash } = await client.rpc
    .getLatestBlockhash()
    .send();

  const transaction = createTransaction({
    feePayer: client.god,
    version: "auto",
    instructions: [createAccountIx, initializeMintIx],
    latestBlockhash,
  });

  const signed_tx = await signTransactionMessageWithSigners(transaction);
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
  return mint;
}

export async function mintFT(client: Client, holder: Address, mint: Address) {
  const { value: latestBlockhash } = await client.rpc
    .getLatestBlockhash()
    .send();
  const tx = await buildMintTokensTransaction({
    feePayer: client.god,
    latestBlockhash,
    mint,
    mintAuthority: client.god,
    amount: 100000,
    destination: holder,
    tokenProgram: TOKEN_PROGRAM_ADDRESS,
  });
  const signedTx = await signTransactionMessageWithSigners(tx);
  await client.sendAndConfirmTransaction(signedTx);
}

export async function transferFt(
  client: Client,
  to: Address | TransactionSigner,
  mint: Address,
  amount: number
) {
  const { value: latestBlockhash } = await client.rpc
    .getLatestBlockhash()
    .send();
  const tx = await buildTransferTokensTransaction({
    feePayer: client.god,
    latestBlockhash,
    mint,
    amount,
    destination: to,
    authority: client.god,
  });
  const signedTx = await signTransactionMessageWithSigners(tx);
  await client.sendAndConfirmTransaction(signedTx);
}

export async function fund_rewardToken(client: Client) {
  const mint = await createFungibleToken(client);
  const tokenAccs = await getAccounts(
    mint.address,
    client.provider.address,
    client.staker.address
  );
  await mintFT(client, client.provider.address, mint.address);

  // await mintFT(client, client.god.address, mint.address);
  // await transferFt(client, client.staker.address, mint.address, 4000);
  // const { value: providerBalance } = await client.rpc
  //   .getTokenAccountBalance(tokenAccs.provider_ata)
  //   .send();
  // console.log(providerBalance);
  return mint;
}

export async function fund_stakingToken(client: Client) {
  const mint = await createFungibleToken(client);
  const tokenAccs = await getAccounts(
    mint.address,
    client.provider.address,
    client.staker.address
  );
  await mintFT(client, client.staker.address, mint.address);
  return mint;
}
async function main() {
  const client = await getClient();
  const mitn = await fund_rewardToken(client);
}
// main();
