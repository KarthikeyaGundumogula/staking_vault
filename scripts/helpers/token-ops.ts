import { getCreateAccountInstruction } from "@solana-program/system";
import { Client, getClient, Token_Accounts, getAccounts } from "./setUp";
import {
  generateKeyPairSigner,
  createTransaction,
  Address,
  getExplorerLink,
  getSignatureFromTransaction,
  signTransactionMessageWithSigners,
} from "gill";
import {
  TOKEN_PROGRAM_ADDRESS,
  getTokenMetadataAddress,
  getMintSize,
  getInitializeMintInstruction,
  getCreateMetadataAccountV3Instruction,
  TOKEN_METADATA_PROGRAM_ADDRESS,
  buildMintTokensTransaction,
  buildTransferTokensTransaction,
} from "gill/programs";

export async function createFungibleToken(client: Client) {
  console.log(client.god.address);

  const tokenProgram = TOKEN_PROGRAM_ADDRESS;
  console.log(tokenProgram);
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

async function transferFt(client, to, mint, amount) {
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

async function main() {
  const client = await getClient();
  const mint = await createFungibleToken(client);
  const tokenAccs = await getAccounts(mint);
  await mintFT(client, tokenAccs.staker_acc.address, mint.address);
  let { value: postMintBalance } = await client.rpc
    .getTokenAccountBalance(tokenAccs.staker_ata)
    .send();
  console.log(postMintBalance);

  await mintFT(client, client.god.address, mint.address);
  await transferFt(client, tokenAccs.provider_acc, mint.address, 4000);
  const { value: providerBalance } = await client.rpc
    .getTokenAccountBalance(tokenAccs.provider_ata)
    .send();
  console.log(providerBalance);
}

main();
