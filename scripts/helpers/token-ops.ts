import { getCreateAccountInstruction } from "@solana-program/system";
import { Client, getClient, Token_Accounts } from "./setUp";
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
} from "gill/programs";

export async function createFungibleToken(
) {
  let client = await getClient()

  const tokenProgram = TOKEN_PROGRAM_ADDRESS;
  const mintSize = getMintSize();
  const [mint, rentExemption] = await Promise.all([
    generateKeyPairSigner(),
    client.rpc.getMinimumBalanceForRentExemption(BigInt(mintSize)).send(),
  ]);

  const createAccountIx = getCreateAccountInstruction({
    payer: client.god,
    newAccount: mint,
    space: mintSize,
    lamports: rentExemption,
    programAddress: TOKEN_PROGRAM_ADDRESS,
  });
  const initializeMintIx = getInitializeMintInstruction({
    mint: mint.address,
    decimals: 9,
    mintAuthority: client.god.address,
  });

  const { value: latestBlockhash } = await client.rpc
    .getLatestBlockhash()
    .send();

  const transaction = createTransaction({
    feePayer: client.god,
    version: "legacy",
    instructions: [
      getCreateAccountInstruction({
        space: mintSize,
        lamports: rentExemption,
        newAccount: mint,
        payer: client.god,
        programAddress: tokenProgram,
      }),
      getInitializeMintInstruction(
        {
          mint: mint.address,
          mintAuthority: client.god.address,
          freezeAuthority: client.god.address,
          decimals: 9,
        },
        {
          programAddress: tokenProgram,
        }
      ),
    ],
    latestBlockhash,
  });

  const signed_tx = await signTransactionMessageWithSigners(transaction);
  await client.sendAndConfirmTransaction(signed_tx);
  console.log(
    "Explorer:",
    getExplorerLink({
      cluster: "devnet",
      transaction: getSignatureFromTransaction(signed_tx),
    })
  );
  return mint;
}

export async function mintFT(
  client: Client,
  to_ata: Address,
  holder: Address,
  mint: Address
) {}

async function transferFt() { }

createFungibleToken()
