import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TicketMachine } from "../target/types/ticket_machine";
import { LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";

describe("ticket_machine", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.ticketMachine as Program<TicketMachine>;
  const wallet = anchor.AnchorProvider.env().wallet;
  it("Is initialized!", async () => {
    // Add your test here.

    const [ticketMachinePDA] = PublicKey.findProgramAddressSync(
      [Buffer.from("ticket_machine"), wallet.publicKey.toBuffer()],
      program.programId
    )

    if (!program.provider.connection.getAccountInfo(ticketMachinePDA)) {
      const tx = await program.methods.initializeTicketMachine().rpc();
      console.log("Your transaction signature", tx);
    }
  });

  it("set price", async () => {
    const ticketPrice = new anchor.BN(0.05 * LAMPORTS_PER_SOL);
    const tx = await program.methods.setPrice(ticketPrice).rpc();

    console.log(tx)
  })

  it("withdraw earnings", async () => {
    const earnings = new anchor.BN(0.01 * LAMPORTS_PER_SOL);

    const tx = await program.methods.withdrawEarnings(earnings).rpc();

    console.log(tx)
  })

  it("buy ticket", async () => {
    // Create a new buyer wallet
    const buyer = anchor.web3.Keypair.generate();

    // Airdrop SOL to the buyer
    const airdropSig = await program.provider.connection.requestAirdrop(
      buyer.publicKey,
      1 * LAMPORTS_PER_SOL
    );

    // Wait for airdrop to confirm
    await program.provider.connection.confirmTransaction(airdropSig);

    // Get the ticket machine PDA (created by admin/wallet)
    const [ticketMachinePDA] = PublicKey.findProgramAddressSync(
      [Buffer.from("ticket_machine"), wallet.publicKey.toBuffer()],
      program.programId
    );

    // Call buy_ticket with buyer as signer and pass the admin's pubkey
    const tx = await program.methods
      .buyTicket(wallet.publicKey)  // Pass admin's pubkey as argument
      .accounts({
        ticketMachine: ticketMachinePDA,
        signer: buyer.publicKey,
      })
      .signers([buyer])  // Buyer signs the transaction
      .rpc();

    console.log("Buy ticket transaction:", tx);

    // Verify the ticket machine state updated
    const ticketMachineAccount = await program.account.ticketMachine.fetch(ticketMachinePDA);
    console.log("Ticket number:", ticketMachineAccount.ticketNumber.toString());
    console.log("Total earnings:", ticketMachineAccount.totalEarnings.toString());
  })
});
