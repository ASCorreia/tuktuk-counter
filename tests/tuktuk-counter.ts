import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TuktukCounter } from "../target/types/tuktuk_counter";

describe("tuktuk-counter", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);

  const program = anchor.workspace.tuktukCounter as Program<TuktukCounter>;

  const counter = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("counter")], program.programId)[0];

  it("Create and increment counter", async () => {
    const tx = await program.methods.increment()
    .accountsPartial({
      user: provider.publicKey,
      counter: counter,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .rpc();
    console.log("Your transaction signature", tx);
  });
});
