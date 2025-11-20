import {
  generateKeyPairSigner,
  type KeyPairSigner,
  getProgramDerivedAddress,
  type Address,
  getAddressEncoder,
  createSolanaClient,
  Rpc,
  RpcSubscriptions,
  SolanaRpcApi,
  SendAndConfirmTransactionWithSignersFunction,
  SolanaRpcSubscriptionsApi,
  airdropFactory,
  lamports,
} from "gill";
import { loadKeypairSignerFromFile } from "gill/node";
import { TOKEN_PROGRAM_ADDRESS } from "gill/programs";
import { getAssociatedTokenAccountAddress } from "gill/programs";

export type Token_Accounts = {
  mint_address: Address;
  vault_acc: Address;
  provider_ata: Address;
  vault_ata: Address;
  staker_ata: Address;
};

export type Client = {
  rpc: Rpc<SolanaRpcApi>;
  sendAndConfirmTransaction: SendAndConfirmTransactionWithSignersFunction;
  rpcSubscriptions: RpcSubscriptions<SolanaRpcSubscriptionsApi>;
  god: KeyPairSigner;
  staker: KeyPairSigner;
  provider: KeyPairSigner;
};

export async function getClient(): Promise<Client> {
  const { rpc, sendAndConfirmTransaction, rpcSubscriptions } =
    createSolanaClient({ urlOrMoniker: "localnet" });
  const god = await loadKeypairSignerFromFile();
  const staker = await loadKeypairSignerFromFile("provider-wallet.json");
  const provider = await loadKeypairSignerFromFile("staker-wallet.json");
  return {
    rpc,
    sendAndConfirmTransaction,
    god,
    rpcSubscriptions,
    staker,
    provider,
  };
}

export const STAKING_VAULT_ID = "DW9BXusirecGep9k5FXFDALYiY1HPtBpVWwPJ36ZD8KZ";
export const NFT_MARKETPLACE_ID =
  "3kLob38A4tG8m3fP9ZZwSWsjdr417DjQZ4bkqxGFjaUh";

export async function getAccounts(
  mint: Address,
  provider_acc: Address,
  staker_acc: Address
) {
  const SEED = "staking_vault";
  const addressEncoder = getAddressEncoder();

  const [vault_state_pda] = await getProgramDerivedAddress({
    programAddress: STAKING_VAULT_ID as Address,
    seeds: [
      SEED,
      addressEncoder.encode(provider_acc),
    ],
  });

  let tokenAcc: Token_Accounts | undefined;
  let proivider_ata = await getAssociatedTokenAccountAddress(
    mint,
    provider_acc,
    TOKEN_PROGRAM_ADDRESS
  );
  let staker_ata = await getAssociatedTokenAccountAddress(
    mint,
    staker_acc,
    TOKEN_PROGRAM_ADDRESS
  );
  let vault_ata = await getAssociatedTokenAccountAddress(
    mint,
    vault_state_pda,
    TOKEN_PROGRAM_ADDRESS
  );

  tokenAcc = {
    mint_address: mint,
    provider_ata: proivider_ata,
    staker_ata: staker_ata,
    vault_ata: vault_ata,
    vault_acc: vault_state_pda,
  };

  return tokenAcc;
}

export async function airdrop(client: Client, addr: Address) {
  await airdropFactory({
    rpc: client.rpc,
    rpcSubscriptions: client.rpcSubscriptions,
  })({
    lamports: lamports(10000000000000n),
    commitment: "confirmed",
    recipientAddress: addr,
  });
}
