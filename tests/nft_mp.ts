import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { NftMarketplace } from "../target/types/nft_marketplace";
import { StakingVault } from "../target/types/staking_vault";
import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import {
  MPL_CORE_PROGRAM_ID,
  mplCore,
  fetchAssetsByOwner,
  fetchAsset,
} from "@metaplex-foundation/mpl-core";
import { assert } from "chai";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";

describe("nft_escrow", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const nft_program = anchor.workspace
    .nft_marketplace as Program<NftMarketplace>;
  const staking_program = anchor.workspace
    .staking_vault as Program<StakingVault>;

  const connection = anchor.getProvider().connection;
  const wallet = anchor.Wallet.local();

  const asset = Keypair.generate();
  const receiver = Keypair.generate();
  const umi = createUmi("http://0.0.0.0:8899", "confirmed");
  let [vault] = PublicKey.findProgramAddressSync(
    [Buffer.from("vault")],
    nft_program.programId
  );

  it("Create Asset", async () => {
    let createAssetArgs = {
      name: "Guess Reaction",
      uri: "https://karthikeya.framer.ai/",
    };

    const createAssetTx = await nft_program.methods
      .createCoreAsset(createAssetArgs)
      .accountsStrict({
        asset: asset.publicKey,
        payer: wallet.publicKey,
        owner: wallet.publicKey,
        systemProgram: SystemProgram.programId,
        mplCoreProgram: MPL_CORE_PROGRAM_ID,
      })
      .signers([asset, wallet.payer])
      .rpc();
    console.log(createAssetTx);

    const asset_data = await fetchAssetsByOwner(
      umi,
      wallet.publicKey.toString(),
      {
        skipDerivePlugins: false,
      }
    );
    console.log(asset_data);
  });

  // it("Deposits NFT", async () => {
  //   console.log(vault);
  //   const transferAssetInstruction = await program.methods
  //     .depositAsset(receiver.publicKey)
  //     .accountsStrict({
  //       asset: asset.publicKey,
  //       collection: null,
  //       payer: wallet.publicKey,
  //       authority: null,
  //       newOwner: vault,
  //       systemProgram: SystemProgram.programId,
  //       mplCoreProgram: MPL_CORE_PROGRAM_ID,
  //       vault: vault,
  //     })
  //     .signers([wallet.payer])
  //     .rpc();
  //   console.log(transferAssetInstruction);

  //   const vault_data = await program.account.vault.fetch(vault);
  //   assert.ok(vault_data.receiver.equals(receiver.publicKey));

  //   // log all the details
  //   console.log("payer address",wallet.publicKey);
  //   console.log("receiver",receiver.publicKey);
  //   console.log("vault",vault);
  //   console.log("asset", asset.publicKey);

  //    const assetAccount = await program.provider.connection.getAccountInfo(
  //      asset.publicKey
  //    );

  // });

  // it("Claims NFT", async () => {
  //   const transferAssetInstruction = await program.methods
  //     .claimAsset()
  //     .accountsStrict({
  //       asset: asset.publicKey,
  //       collection: null,
  //       payer: wallet.publicKey,
  //       authority: null,
  //       newOwner: receiver.publicKey,
  //       systemProgram: SystemProgram.programId,
  //       mplCoreProgram: MPL_CORE_PROGRAM_ID,
  //       vault: vault,
  //     })
  //     .signers([wallet.payer])
  //     .rpc();
  //   console.log(transferAssetInstruction);
  // })
});

//solana-test-validator -r --bpf-program CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d core.so
// anchor test --skip-local-validator
