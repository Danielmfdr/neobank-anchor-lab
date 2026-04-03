import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  TOKEN_PROGRAM_ID,
  createAssociatedTokenAccount,
  createMint,
  getAccount,
  getAssociatedTokenAddressSync,
  mintTo,
} from "@solana/spl-token";
import { expect } from "chai";

import { NeobankAnchorDemo } from "../target/types/neobank_anchor_demo";

describe("neobank_anchor_demo", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace
    .NeobankAnchorDemo as Program<NeobankAnchorDemo>;
  const owner = provider.wallet.publicKey;
  const payer = (provider.wallet as anchor.Wallet & {
    payer: anchor.web3.Keypair;
  }).payer;

  const [bankAccountPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("bank-account"), owner.toBuffer()],
    program.programId,
  );
  const [solVaultPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("sol-vault"), owner.toBuffer()],
    program.programId,
  );
  const [vaultAuthorityPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("vault-authority"), owner.toBuffer()],
    program.programId,
  );

  it("covers initialize, SOL flow, SPL vault init, and SPL flow", async () => {
    await program.methods
      .initializeAccount()
      .accounts({
        owner,
        bankAccount: bankAccountPda,
        solVault: solVaultPda,
        vaultAuthority: vaultAuthorityPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    let bankAccount = await program.account.bankAccount.fetch(bankAccountPda);
    expect(bankAccount.owner.toBase58()).to.equal(owner.toBase58());
    expect(bankAccount.solVault.toBase58()).to.equal(solVaultPda.toBase58());
    expect(bankAccount.vaultAuthority.toBase58()).to.equal(
      vaultAuthorityPda.toBase58(),
    );
    expect(bankAccount.solBalance.toNumber()).to.equal(0);
    expect(bankAccount.tokenVaultCount).to.equal(0);

    const initialSolVaultLamports = await provider.connection.getBalance(
      solVaultPda,
    );

    const solDepositAmount = new anchor.BN(400_000_000);
    const solWithdrawAmount = new anchor.BN(150_000_000);

    await program.methods
      .depositSol(solDepositAmount)
      .accounts({
        owner,
        bankAccount: bankAccountPda,
        solVault: solVaultPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    bankAccount = await program.account.bankAccount.fetch(bankAccountPda);
    expect(bankAccount.solBalance.toNumber()).to.equal(
      solDepositAmount.toNumber(),
    );

    const solVaultAfterDeposit = await provider.connection.getBalance(
      solVaultPda,
    );
    expect(solVaultAfterDeposit).to.equal(
      initialSolVaultLamports + solDepositAmount.toNumber(),
    );

    await program.methods
      .withdrawSol(solWithdrawAmount)
      .accounts({
        owner,
        bankAccount: bankAccountPda,
        solVault: solVaultPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    bankAccount = await program.account.bankAccount.fetch(bankAccountPda);
    const finalTrackedSol =
      solDepositAmount.toNumber() - solWithdrawAmount.toNumber();
    expect(bankAccount.solBalance.toNumber()).to.equal(finalTrackedSol);

    const solVaultAfterWithdraw = await provider.connection.getBalance(
      solVaultPda,
    );
    expect(solVaultAfterWithdraw).to.equal(
      initialSolVaultLamports + finalTrackedSol,
    );

    const mint = await createMint(
      provider.connection,
      payer,
      owner,
      null,
      6,
    );

    const ownerTokenAccount = await createAssociatedTokenAccount(
      provider.connection,
      payer,
      mint,
      owner,
    );

    const mintedAmount = 1_000_000n;
    await mintTo(
      provider.connection,
      payer,
      mint,
      ownerTokenAccount,
      payer,
      mintedAmount,
    );

    const tokenVault = getAssociatedTokenAddressSync(
      mint,
      vaultAuthorityPda,
      true,
    );

    await program.methods
      .initializeTokenVault()
      .accounts({
        owner,
        bankAccount: bankAccountPda,
        vaultAuthority: vaultAuthorityPda,
        mint,
        tokenVault,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    bankAccount = await program.account.bankAccount.fetch(bankAccountPda);
    expect(bankAccount.tokenVaultCount).to.equal(1);

    let tokenVaultAccount = await getAccount(provider.connection, tokenVault);
    let ownerTokenState = await getAccount(
      provider.connection,
      ownerTokenAccount,
    );
    expect(tokenVaultAccount.amount).to.equal(0n);
    expect(ownerTokenState.amount).to.equal(mintedAmount);

    const splDepositAmount = new anchor.BN(600_000);
    const splWithdrawAmount = new anchor.BN(250_000);

    await program.methods
      .depositSpl(splDepositAmount)
      .accounts({
        owner,
        bankAccount: bankAccountPda,
        vaultAuthority: vaultAuthorityPda,
        mint,
        ownerTokenAccount,
        tokenVault,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();

    tokenVaultAccount = await getAccount(provider.connection, tokenVault);
    ownerTokenState = await getAccount(provider.connection, ownerTokenAccount);

    expect(tokenVaultAccount.amount).to.equal(
      BigInt(splDepositAmount.toString()),
    );
    expect(ownerTokenState.amount).to.equal(
      mintedAmount - BigInt(splDepositAmount.toString()),
    );

    await program.methods
      .withdrawSpl(splWithdrawAmount)
      .accounts({
        owner,
        bankAccount: bankAccountPda,
        vaultAuthority: vaultAuthorityPda,
        mint,
        ownerTokenAccount,
        tokenVault,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();

    tokenVaultAccount = await getAccount(provider.connection, tokenVault);
    ownerTokenState = await getAccount(provider.connection, ownerTokenAccount);

    expect(tokenVaultAccount.amount).to.equal(
      BigInt(splDepositAmount.toString()) -
        BigInt(splWithdrawAmount.toString()),
    );
    expect(ownerTokenState.amount).to.equal(
      mintedAmount -
        BigInt(splDepositAmount.toString()) +
        BigInt(splWithdrawAmount.toString()),
    );
  });
});
