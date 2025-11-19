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
// import provider from "../../provider-wallet.json";
// import staker from "../../staker-wallet.json";

export type Token_Accounts = {
  mint_address: Address;
  vault_acc: Address;
  provider_ata: Address;
  vault_ata: Address;
  staker_ata: Address;
  vault_program_id: Address;
  provider_acc: KeyPairSigner;
  staker_acc: KeyPairSigner;
};

export type Client = {
  rpc: Rpc<SolanaRpcApi>;
  sendAndConfirmTransaction: SendAndConfirmTransactionWithSignersFunction;
  rpcSubscriptions: RpcSubscriptions<SolanaRpcSubscriptionsApi>;
  god: KeyPairSigner;
};

export async function getClient(): Promise<Client> {
  const { rpc, sendAndConfirmTransaction, rpcSubscriptions } =
    createSolanaClient({ urlOrMoniker: "localnet" });
  const god = await loadKeypairSignerFromFile();

  return { rpc, sendAndConfirmTransaction, god, rpcSubscriptions };
}

const STAKING_VAULT_ID = "6AD9gckrLi1LxJuS6TJeA4myevWbSGULYKHc3o2mJkzu";

export async function getAccounts(mint) {
  const provider_acc = await generateKeyPairSigner();
  const staker_acc = await generateKeyPairSigner();
  const SEED = "staking_vault";
  const addressEncoder = getAddressEncoder();

  const [vault_state_pda] = await getProgramDerivedAddress({
    programAddress: STAKING_VAULT_ID as Address,
    seeds: [
      SEED,
      addressEncoder.encode(provider_acc.address),
      addressEncoder.encode(staker_acc.address),
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
    mint_address: mint.address,
    provider_ata: proivider_ata,
    staker_ata: staker_ata,
    vault_ata: vault_ata,
    vault_acc: vault_state_pda,
    vault_program_id: STAKING_VAULT_ID as Address,
    provider_acc,
    staker_acc,
  };

  return tokenAcc;
}

export async function airdrop(client: Client, addr: Address) {
  await airdropFactory({
    rpc: client.rpc,
    rpcSubscriptions: client.rpcSubscriptions,
  })({
    lamports: lamports(1000n),
    commitment: "confirmed",
    recipientAddress: addr,
  });
}
