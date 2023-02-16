import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Storage } from "../target/types/storage";
import { expect } from "chai";
import { StorageManager } from "../target/types/storage_manager";

describe("storage-manager", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const storageProgram = anchor.workspace.Storage as Program<Storage>;
  const storageManagerProgram = anchor.workspace.StorageManager as Program<StorageManager>;
  const data = anchor.web3.Keypair.generate();

  const fetchValue = async () => {
    return (await storageProgram.account.data.fetch(data.publicKey)).value.toNumber();
  }

  it("should initialize storage slot", async () => {
    await storageProgram.methods.initialize(new anchor.BN(123))
    .accounts({
        data: data.publicKey,
    })
    .signers([data])
    .rpc();
    const value = await fetchValue();
    expect(value).eq(123);
  });

  it("should set data via storage manager",async () => {
    const expected = 456;
    await storageManagerProgram.methods.interact(new anchor.BN(expected))
    .accounts({
        data: data.publicKey,
        storageProgram: storageProgram.programId,
    })
    .rpc();
    const value = await fetchValue();
    expect(value).eq(expected);
  });
});
