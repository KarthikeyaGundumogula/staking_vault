import type { Client } from "./client";
import {
  appendTransactionMessageInstructions,
  createTransactionMessage,
  generateKeyPairSigner,
  pipe,
  setTransactionMessageFeePayer,
  setTransactionMessageLifetimeUsingBlockhash,
  signTransactionMessageWithSigners,
  assertIsSendableTransaction,
  Address,
} from "@solana/kit";
import { getCreateAccountInstruction } from "@solana-program/system";
import {
  getCreateAssociatedTokenIdempotentInstructionAsync,
  getInitializeMintInstruction,
  getMintSize,
  getMintToInstruction,
  getTransferCheckedInstruction,
  TOKEN_PROGRAM_ADDRESS,
} from "@solana-program/token";

import { getOpenInstruction } from "../../clients/js/src/generated";

export async function createFungibleToken(
  client: Client,
  options: { decimals?: number }
) {
  const mintSize = getMintSize();
  const [mint, rentExemption] = await Promise.all([
    generateKeyPairSigner(),
    client.rpc.getMinimumBalanceForRentExemption(BigInt(mintSize)).send(),
  ]);

  const createAccountIx = getCreateAccountInstruction({
    payer: client.wallet,
    newAccount: mint,
    space: mintSize,
    lamports: rentExemption,
    programAddress: TOKEN_PROGRAM_ADDRESS,
  });
  const initializeMintIx = getInitializeMintInstruction({
    mint: mint.address,
    decimals: options.decimals ?? 0,
    mintAuthority: client.wallet.address,
  });

  const { value: latestBlockhash } = await client.rpc
    .getLatestBlockhash()
    .send();
  const txMsg = await pipe(
    createTransactionMessage({ version: 0 }),
    (tx) => setTransactionMessageFeePayer(client.wallet.address, tx),
    (tx) => setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, tx),
    (tx) =>
      appendTransactionMessageInstructions(
        [createAccountIx, initializeMintIx],
        tx
      ),
    (tx) => client.estimateAndSetComputeUnitLimit(tx)
  );

  const transaction = await signTransactionMessageWithSigners(txMsg);
  assertIsSendableTransaction(transaction);

  await client.sendAndConfirmTransaction(
    {
      ...transaction,
      lifetimeConstraint: {
        lastValidBlockHeight: latestBlockhash.lastValidBlockHeight,
      },
    },
    { commitment: "confirmed" }
  );

  return mint;
}

export async function mintFT(
  client: Client,
  to_ata: Address,
  holder: Address,
  mint: Address,
) {
  const mintInstructions = [
    await getCreateAssociatedTokenIdempotentInstructionAsync({
      mint,
      payer: client.wallet,
      owner: holder,
    }),
    getMintToInstruction({
      mint: mint,
      token: to_ata,
      amount: BigInt(10 ** 10),
      mintAuthority: client.wallet,
    }),
  ];

  const { value: latestBlockhash } = await client.rpc
    .getLatestBlockhash()
    .send();
  const txMsg = await pipe(
    createTransactionMessage({ version: 0 }),
    (tx) => setTransactionMessageFeePayer(client.wallet.address, tx),
    (tx) => setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, tx),
    (tx) => appendTransactionMessageInstructions(mintInstructions, tx),
    (tx) => client.estimateAndSetComputeUnitLimit(tx)
  );

  const transaction = await signTransactionMessageWithSigners(txMsg);
  assertIsSendableTransaction(transaction);

  const tx = await client.sendAndConfirmTransaction(
    {
      ...transaction,
      lifetimeConstraint: {
        lastValidBlockHeight: latestBlockhash.lastValidBlockHeight,
      },
    },
    { commitment: "confirmed" }
  );

  console.log(tx);
}

async function transferFt() {
  
}
