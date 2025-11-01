import {
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  Transaction,
  sendAndConfirmTransaction,
} from "@solana/web3.js";
import { AnchorError, Program } from "@coral-xyz/anchor";
import { createKeyPairSignerFromBytes } from "@solana/kit";
import wallet from "../provider-wallet.json";
// Set up a connection to the cluster
const connection = new Connection("http://127.0.0.1:8899", "confirmed");

async function main() {
  
let provider_wallet = await createKeyPairSignerFromBytes(new Uint8Array(wallet.wallet));
console.log(provider_wallet.address)
}

main()
