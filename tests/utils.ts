import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { StakingVault } from "../target/types/staking_vault";
import { NftMarketplace } from "../target/types/nft_marketplace";
import { Commitment, Connection, PublicKey } from "@solana/web3.js";
import {
  createMint,
  mintTo,
  getOrCreateAssociatedTokenAccount,
} from "@solana/spl-token";

// Configure the client to use the local cluster.
anchor.setProvider(anchor.AnchorProvider.env());

export const commitment: Commitment = "confirmed";
export const local_connection = new Connection(
  "http://localhost:8899",
  commitment
);
export const connection = anchor.getProvider().connection;
export const program = anchor.workspace.stakingVault as Program<StakingVault>;
export const nft_program = anchor.workspace.nftMarketplace as Program<NftMarketplace>;

// Type for the return value of setUp
export interface SetupResult {
  reward_mint: anchor.web3.PublicKey;
  staking_mint: anchor.web3.PublicKey;
  vault_state_pda: anchor.web3.PublicKey;
  provider_reward_ata: anchor.web3.PublicKey;
  staker_reward_ata: anchor.web3.PublicKey;
  vault_reward_ata: anchor.web3.PublicKey;
  staker_staking_ata: anchor.web3.PublicKey;
  vault_staking_ata: anchor.web3.PublicKey;
  un_authorized_staker_staking_ata: anchor.web3.PublicKey;
}

export async function setUp(
  provider: anchor.web3.Keypair,
  staker: anchor.web3.Keypair,
  god: anchor.web3.Keypair,
  un_authorized_staker: anchor.web3.Keypair
): Promise<SetupResult> {
  console.log("Provider: ", provider.publicKey.toBase58());
  console.log("Staker: ", staker.publicKey.toBase58());
  console.log("God: ", god.publicKey.toBase58());
  await getAirdrop(connection,provider.publicKey,100000000000)
  await getAirdrop(connection,staker.publicKey,10000000000000)
  const [vault_state_pda] = PublicKey.findProgramAddressSync(
    [Buffer.from("staking_vault"), provider.publicKey.toBuffer()],
    program.programId
  );
  console.log("Vault State PDA: ", vault_state_pda.toBase58());

  const reward_mint = await createMint(
    connection,
    provider,
    god.publicKey,
    null,
    6
  );

  const provider_reward_ata = await getOrCreateAssociatedTokenAccount(
    connection,
    provider,
    reward_mint,
    provider.publicKey
  ).then((ata) => ata.address);

  const staker_reward_ata = await getOrCreateAssociatedTokenAccount(
    connection,
    staker,
    reward_mint,
    staker.publicKey
  ).then((ata) => ata.address);

  const vault_reward_ata = await getOrCreateAssociatedTokenAccount(
    connection,
    provider,
    reward_mint,
    vault_state_pda,
    true
  ).then((ata) => ata.address);

  console.log("reward mint: ", reward_mint.toBase58());
  console.log("provider reward ata: ", provider_reward_ata.toBase58());
  console.log("staker reward ata: ", staker_reward_ata.toBase58());
  console.log("vault reward ata: ", vault_reward_ata.toBase58());

  const staking_mint = await createMint(
    connection,
    provider,
    god.publicKey,
    null,
    6
  );

  const vault_staking_ata = await getOrCreateAssociatedTokenAccount(
    connection,
    staker,
    staking_mint,
    vault_state_pda,
    true
  ).then((ata) => ata.address);

  const staker_staking_ata = await getOrCreateAssociatedTokenAccount(
    connection,
    provider,
    staking_mint, 
    staker.publicKey
  ).then((ata) => ata.address);

  const un_authorized_staker_staking_ata =  await getOrCreateAssociatedTokenAccount(
    connection,
    provider,
    staking_mint, 
    un_authorized_staker.publicKey
  ).then((ata) => ata.address);

  console.log("staking mint: ", staking_mint.toBase58());
  console.log("staker staking ata: ", staker_staking_ata.toBase58());
  console.log("vault staking ata: ", vault_staking_ata.toBase58());

  //mint some reward tokens to provider and some staking tokens to the staker
  try {
    const tx = await mintTo(
      connection,
      god,
      reward_mint,
      provider_reward_ata,
      god,
      500_000_000000
    );
    console.log("Minted reward tokens to provider: ", tx);
    const tx2 = await mintTo(
      connection,
      god,
      staking_mint,
      staker_staking_ata,
      god,
      1_000000_000000
    );
    console.log("Minted staking tokens to staker: ", tx2);
  } catch (error) {
    console.error("Error in minting tokens: ", error);
  }

  return {
    reward_mint,
    staking_mint,
    vault_state_pda,
    provider_reward_ata,
    staker_reward_ata,
    vault_reward_ata,
    staker_staking_ata,
    vault_staking_ata,
    un_authorized_staker_staking_ata,
  };
}

export async function getAirdrop(
  connection: Connection,
  user: anchor.web3.PublicKey,
  amount: number = 2 * anchor.web3.LAMPORTS_PER_SOL
): Promise<void> {
  const tx = await connection.requestAirdrop(user, amount);
  await connection.confirmTransaction(tx);
}
