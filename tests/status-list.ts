import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { StatusList } from "../target/types/status_list";
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";
import { expect } from "chai";

function sleep(ms: number) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

function getReturnedData(confirmedTransaction: anchor.web3.VersionedTransactionResponse): Buffer {
  if (!(
    "returnData" in confirmedTransaction.meta
    && typeof confirmedTransaction.meta.returnData === "object"
    && confirmedTransaction.meta.returnData
    && "data" in confirmedTransaction.meta.returnData
    && Array.isArray(confirmedTransaction.meta.returnData.data))) {
    return Buffer.from([])
  }
  const [data, encoding] = confirmedTransaction.meta.returnData.data;

  return Buffer.from(data, encoding);
};

describe("status-list", () => {
  const provider = anchor.AnchorProvider.env();
  const { payer } = provider.wallet as NodeWallet;
  anchor.setProvider(provider);

  const program = anchor.workspace.statusList as Program<StatusList>;
  const [state] = anchor.web3.PublicKey.findProgramAddressSync([
    Buffer.from("status_list"),
    provider.publicKey.toBuffer(),
  ], program.programId);

  it("fails to initialize a too large list", async () => {
    try {
      await program.methods.initialize(600, { suspension: {} }).accountsPartial({
        payer: payer.publicKey,
        systemProgram: SYSTEM_PROGRAM_ID,
        state
      }).rpc();
    } catch (err) {
      expect(err).instanceOf(anchor.AnchorError);
      expect((err as anchor.AnchorError).error.errorCode.code).to.equal("SizeTooLarge")
      expect((err as anchor.AnchorError).error.errorMessage).to.equal("Status list size must be below 512")
    }
  });

  it("initializes the list", async () => {
    await program.methods.initialize(8, { suspension: {} }).accountsPartial({
      payer: payer.publicKey,
      systemProgram: SYSTEM_PROGRAM_ID,
      state
    }).rpc();

    expect(await program.account.statusList.fetch(state)).to.deep.equal({
      list: Buffer.from(new Uint8Array(8)),
      size: 8,
      purpose: { suspension: {} }
    })
  });

  it("fails to toggle an out of bound position", async () => {
    try {
      await program.methods.toggle(130).accountsPartial({
        payer: payer.publicKey,
        systemProgram: SYSTEM_PROGRAM_ID,
        state
      }).rpc();
    } catch (err) {
      expect(err).instanceOf(anchor.AnchorError);
      expect((err as anchor.AnchorError).error.errorCode.code).to.equal("OutOfBounds")
      expect((err as anchor.AnchorError).error.errorMessage).to.equal("Entry out of bounds")
    }
  });

  ([
    { location: 0, expected: Buffer.from([0b00000001, 0, 0, 0, 0, 0, 0, 0]) },
    { location: 50, expected: Buffer.from([0b00000001, 0, 0, 0, 0, 0, 0b00000100, 0]) },
    { location: 52, expected: Buffer.from([0b00000001, 0, 0, 0, 0, 0, 0b00010100, 0]) },
    { location: 50, expected: Buffer.from([0b00000001, 0, 0, 0, 0, 0, 0b00010000, 0]) },
  ]).forEach(({ location, expected }) => it(`toggles entry at position ${location}`, async () => {
    await program.methods.toggle(location).accountsPartial({
      payer: payer.publicKey,
      systemProgram: SYSTEM_PROGRAM_ID,
      state
    }).rpc();

    expect(await program.account.statusList.fetch(state)).to.deep.equal({
      list: expected,
      size: 8,
      purpose: { suspension: {} }
    })
  }))

  it("fails to read an out of bound position", async () => {
    try {
      await program.methods.read(130).accountsPartial({
        payer: payer.publicKey,
        systemProgram: SYSTEM_PROGRAM_ID,
        state
      }).rpc();
    } catch (err) {
      expect(err).instanceOf(anchor.AnchorError);
      expect((err as anchor.AnchorError).error.errorCode.code).to.equal("OutOfBounds")
      expect((err as anchor.AnchorError).error.errorMessage).to.equal("Entry out of bounds")
    }
  });

  ([
    { location: 0, expected: 1 },
    { location: 52, expected: 1 },
    { location: 24, expected: 0 },
    { location: 37, expected: 0 },
    { location: 50, expected: 0 },
  ]).forEach(({ location, expected }) => it(`reads entry at position ${location}`, async () => {
    const tx = await program.methods.read(location).accountsPartial({
      payer: payer.publicKey,
      systemProgram: SYSTEM_PROGRAM_ID,
      state
    }).rpc();
    await sleep(500);
    expect(getReturnedData(await provider.connection.getTransaction(tx, {
      commitment: "confirmed",
      maxSupportedTransactionVersion: 0
    }))).to.deep.equal(Buffer.from([expected]))
  }))
});
