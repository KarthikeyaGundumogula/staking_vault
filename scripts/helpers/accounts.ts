import type { Client } from "./client";
import {
  Address,
  createKeyPairSignerFromBytes,
  getProgramDerivedAddress,
  getAddressEncoder,
  KeyPairSigner,
} from "@solana/kit";
import {
  findAssociatedTokenPda,
  TOKEN_PROGRAM_ADDRESS,
} from "@solana-program/token";
import provider from "../../provider-wallet.json";
import staker from "../../staker-wallet.json";

export type Token_Accounts = {
  mint_address: Address;
  vault_acc: Address;
  provider_ata: Address;
  vault_ata: Address;
  receiver_ata: Address;
  vault_program_id: Address;
  provider_acc:KeyPairSigner;
  staker_acc:KeyPairSigner;
};

const STAKING_VAULT_ID = "6AD9gckrLi1LxJuS6TJeA4myevWbSGULYKHc3o2mJkzu";

export async function getAccounts(mint) {
  const provider_acc = await createKeyPairSignerFromBytes(
    new Uint8Array(provider.provider)
  );
  const staker_acc = await createKeyPairSignerFromBytes(
    new Uint8Array(staker.wallet)
  );
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
  let [proivider_ata] = await findAssociatedTokenPda({
    mint: mint.address,
    owner: provider_acc.address,
    tokenProgram: TOKEN_PROGRAM_ADDRESS,
  });
  let [staker_ata] = await findAssociatedTokenPda({
    mint: mint.address,
    owner: staker_acc.address,
    tokenProgram: TOKEN_PROGRAM_ADDRESS,
  });
  let [vault_ata] = await findAssociatedTokenPda({
    mint: mint.address,
    owner: vault_state_pda,
    tokenProgram: TOKEN_PROGRAM_ADDRESS,
  });

  tokenAcc = {
    mint_address: mint.address,
    provider_ata: proivider_ata,
    receiver_ata: staker_ata,
    vault_ata: vault_ata,
    vault_acc: vault_state_pda,
    vault_program_id: STAKING_VAULT_ID as Address,
    provider_acc,
    staker_acc
  };

  return tokenAcc;
}
