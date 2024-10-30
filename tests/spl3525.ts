import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Spl3525 } from "../target/types/spl3525";
import { expect } from "chai";
import { BN } from "bn.js";

describe("spl3525", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Spl3525 as Program<Spl3525>;
  
  const name = "Test SPL3525";
  const symbol = "TST";
  const decimals = 0;
  
  let state: anchor.web3.Keypair;
  let authority: anchor.web3.Keypair;
  
  beforeEach(async () => {
    state = anchor.web3.Keypair.generate();
    authority = anchor.web3.Keypair.generate();
    
    // Airdrop SOL to authority
    const signature = await provider.connection.requestAirdrop(
      authority.publicKey,
      1000000000
    );
    await provider.connection.confirmTransaction(signature);
  });

  it("Initializes the SPL3525 token", async () => {
    try {
      await program.methods
        .initialize(name, symbol, decimals)
        .accounts({
          state: state.publicKey,
          authority: authority.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([state, authority])
        .rpc();

      const stateAccount = await program.account.state.fetch(state.publicKey);
      expect(stateAccount.name).to.equal(name);
      expect(stateAccount.symbol).to.equal(symbol);
      expect(stateAccount.decimals).to.equal(decimals);
      expect(stateAccount.tokenCounter.toNumber()).to.equal(0);
    } catch (error) {
      console.error("Initialization error:", error);
      throw error;
    }
  });

  it("Mints a new token", async () => {
    // First initialize
    try {
      await program.methods
        .initialize(name, symbol, decimals)
        .accounts({
          state: state.publicKey,
          authority: authority.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([state, authority])
        .rpc();

      const tokenData = anchor.web3.Keypair.generate();
      const owner = anchor.web3.Keypair.generate();

      await program.methods
        .mint(
          new BN(1), // slot
          new BN(100) // value
        )
        .accounts({
          state: state.publicKey,
          tokenData: tokenData.publicKey,
          owner: owner.publicKey,
          authority: authority.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([tokenData, authority])
        .rpc();

      const tokenAccount = await program.account.tokenData.fetch(tokenData.publicKey);
      expect(tokenAccount.slot.toNumber()).to.equal(1);
      expect(tokenAccount.value.toNumber()).to.equal(100);
      expect(tokenAccount.owner.toBase58()).to.equal(owner.publicKey.toBase58());
    } catch (error) {
      console.error("Mint error:", error);
      throw error;
    }
  });

  it("Transfers value between tokens", async () => {
    try {
      // Initialize first
      await program.methods
        .initialize(name, symbol, decimals)
        .accounts({
          state: state.publicKey,
          authority: authority.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([state, authority])
        .rpc();

      const tokenData1 = anchor.web3.Keypair.generate();
      const tokenData2 = anchor.web3.Keypair.generate();
      const owner = anchor.web3.Keypair.generate();

      // Airdrop SOL to owner
      const signature = await provider.connection.requestAirdrop(
        owner.publicKey,
        1000000000
      );
      await provider.connection.confirmTransaction(signature);

      // Mint first token
      await program.methods
        .mint(
          new BN(1), // slot
          new BN(100) // value
        )
        .accounts({
          state: state.publicKey,
          tokenData: tokenData1.publicKey,
          owner: owner.publicKey,
          authority: authority.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([tokenData1, authority])
        .rpc();

      // Mint second token
      await program.methods
        .mint(
          new BN(1), // same slot
          new BN(50) // value
        )
        .accounts({
          state: state.publicKey,
          tokenData: tokenData2.publicKey,
          owner: owner.publicKey,
          authority: authority.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([tokenData2, authority])
        .rpc();

      // Transfer value
      await program.methods
        .transfer(new BN(30))
        .accounts({
          fromToken: tokenData1.publicKey,
          toToken: tokenData2.publicKey,
          owner: owner.publicKey,
        })
        .signers([owner])
        .rpc();

      const fromToken = await program.account.tokenData.fetch(tokenData1.publicKey);
      const toToken = await program.account.tokenData.fetch(tokenData2.publicKey);

      expect(fromToken.value.toNumber()).to.equal(70); // 100 - 30
      expect(toToken.value.toNumber()).to.equal(80); // 50 + 30
    } catch (error) {
      console.error("Transfer error:", error);
      throw error;
    }
  });

  it("Approves value for spending", async () => {
    try {
      // Initialize first
      await program.methods
        .initialize(name, symbol, decimals)
        .accounts({
          state: state.publicKey,
          authority: authority.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([state, authority])
        .rpc();

      const tokenData = anchor.web3.Keypair.generate();
      const owner = anchor.web3.Keypair.generate();
      const spender = anchor.web3.Keypair.generate();
      const approval = anchor.web3.Keypair.generate();

      // Airdrop SOL to owner
      const signature = await provider.connection.requestAirdrop(
        owner.publicKey,
        1000000000
      );
      await provider.connection.confirmTransaction(signature);

      // Mint token
      await program.methods
        .mint(
          new BN(1), // slot
          new BN(100) // value
        )
        .accounts({
          state: state.publicKey,
          tokenData: tokenData.publicKey,
          owner: owner.publicKey,
          authority: authority.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([tokenData, authority])
        .rpc();

      // Approve value
      await program.methods
        .approveValue(new BN(50))
        .accounts({
          approval: approval.publicKey,
          tokenData: tokenData.publicKey,
          owner: owner.publicKey,
          spender: spender.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([approval, owner])
        .rpc();

      const approvalAccount = await program.account.valueApproval.fetch(approval.publicKey);
      expect(approvalAccount.value.toNumber()).to.equal(50);
      expect(approvalAccount.owner.toBase58()).to.equal(owner.publicKey.toBase58());
      expect(approvalAccount.spender.toBase58()).to.equal(spender.publicKey.toBase58());
    } catch (error) {
      console.error("Approval error:", error);
      throw error;
    }
  });
});