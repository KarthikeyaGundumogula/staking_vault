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
    [Buffer.from("vault"),receiver.publicKey.toBuffer()],
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

  it("Deposits NFT", async () => {
    console.log(vault.toBase58());
    console.log(wallet.publicKey);
    try {
      const transferAssetInstruction = await nft_program.methods
        .depositAsset()
        .accountsStrict({
          asset: asset.publicKey,
          receiver:receiver.publicKey,
          payer: wallet.publicKey,
          newOwner: vault,
          systemProgram: SystemProgram.programId,
          mplCoreProgram: MPL_CORE_PROGRAM_ID,
          vault: vault,
        })
        .signers([wallet.payer])
        .rpc();
      console.log(transferAssetInstruction);
    } catch (error) {
      console.error(error)
      console.error(error.getLogs());
    }


    const vault_data = await nft_program.account.vault.fetch(vault);
    console.log(vault_data)
    // assert.ok(vault_data.receiver.equals(receiver.publicKey));

    console.log("vault address", vault.toBase58);
    let asset_data = await fetchAsset(umi, asset.publicKey.toString());
    console.log(asset_data);
    assert.equal(vault.toBase58(), asset_data.owner as unknown as string);

  });

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
