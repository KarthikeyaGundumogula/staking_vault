import {
  airdropFactory,
  lamports,
  MessageSigner,
  TransactionSigner,
  SolanaRpcApi,
  Rpc,
  RpcSubscriptions,
  SolanaRpcSubscriptionsApi,
  createSolanaRpc,
  createSolanaRpcSubscriptions,
  createKeyPairSignerFromBytes,
  appendTransactionMessageInstruction,
  BaseTransactionMessage,
  TransactionMessageWithFeePayer,
  sendAndConfirmTransactionFactory
} from "@solana/kit";
import {
  estimateComputeUnitLimitFactory,
  getSetComputeUnitLimitInstruction,
} from "@solana-program/compute-budget";

import provider from "../../provider-wallet.json";

export type Client = {
  estimateAndSetComputeUnitLimit: ReturnType<
    typeof estimateAndSetComputeUnitLimitFactory
  >;
  rpc: Rpc<SolanaRpcApi>;
  rpcSubscriptions: RpcSubscriptions<SolanaRpcSubscriptionsApi>;
  sendAndConfirmTransaction: ReturnType<typeof sendAndConfirmTransactionFactory>;
  wallet: TransactionSigner & MessageSigner;
};
let client: Client | undefined;
export async function createClient(): Promise<Client> {
  if (!client) {
    // Create RPC objects and airdrop function.
    const rpc = createSolanaRpc("http://127.0.0.1:8899");
    const rpcSubscriptions = createSolanaRpcSubscriptions(
      "ws://127.0.0.1:8900"
    );
    const airdrop = airdropFactory({ rpc, rpcSubscriptions });

    // Create a wallet with lamports.
    const wallet = await createKeyPairSignerFromBytes(
      new Uint8Array(provider.provider)
    );
    await airdrop({
      recipientAddress: wallet.address,
      lamports: lamports(1_000_000_000n),
      commitment: "confirmed",
    });

    const estimateAndSetComputeUnitLimit =
      estimateAndSetComputeUnitLimitFactory({ rpc });
    
    const sendAndConfirmTransaction = sendAndConfirmTransactionFactory({ rpc, rpcSubscriptions });

    // Store the client.
    client = {
      estimateAndSetComputeUnitLimit,
      rpc,
      rpcSubscriptions,
      sendAndConfirmTransaction,
      wallet,
    };
  }
  return client;
}

function estimateAndSetComputeUnitLimitFactory(
  ...params: Parameters<typeof estimateComputeUnitLimitFactory>
) {
  const estimateComputeUnitLimit = estimateComputeUnitLimitFactory(...params);
  return async <
    T extends BaseTransactionMessage & TransactionMessageWithFeePayer
  >(
    transactionMessage: T
  ) => {
    const computeUnitsEstimate = await estimateComputeUnitLimit(
      transactionMessage
    );
    return appendTransactionMessageInstruction(
      getSetComputeUnitLimitInstruction({ units: computeUnitsEstimate }),
      transactionMessage
    );
  };
}