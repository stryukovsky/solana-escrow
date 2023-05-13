import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SplEscrow } from "../target/types/spl_escrow";
import {getAccount, getAssociatedTokenAddress} from "@solana/spl-token";
import { expect } from "chai";

describe("spl-escrow", () => {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());

    const program = anchor.workspace.SplEscrow as Program<SplEscrow>;

    const mint = anchor.web3.Keypair.generate();
    const adminAccount = anchor.web3.Keypair.generate();
    const userAuthority = anchor.web3.Keypair.generate();
    const userEscrow = anchor.web3.Keypair.generate();
    const userAccount = anchor.web3.Keypair.generate();
    const initialAmount = new anchor.BN(30000000);

    it("should be initialized", async () => {
        const tx = await program.methods.initialize(initialAmount)
            .accounts({
                mint: mint.publicKey,
                tokenAccount: await getAssociatedTokenAddress(mint.publicKey, anchor.getProvider().publicKey)
            })
            .signers([mint])
            .rpc();
        console.log("Your transaction signature", tx);
    });

    it("should give some native to user", async () => {
        const tx = new anchor.web3.Transaction();
        tx.add(anchor.web3.SystemProgram.transfer({
            fromPubkey: anchor.getProvider().publicKey,
            toPubkey: userAuthority.publicKey,
            lamports: 10000000
        }));
        await anchor.getProvider().sendAndConfirm(tx);
    });

    it("should create escrow and token account for user", async () => {
        const tokenAccount = await getAssociatedTokenAddress(mint.publicKey, userAuthority.publicKey);
        await program.methods.register().accounts({
            tokenAccount,
            escrow: userEscrow.publicKey,
            mint: mint.publicKey,
            authority: userAuthority.publicKey,
        }).signers([userAuthority, userEscrow]).rpc();
    });

    const accruedToUser = new anchor.BN(10);
    it("should accrue some tokens to user's escrow", async () => {
        await program.methods.accrue(accruedToUser).accounts({
            escrow: userEscrow.publicKey,
        }).rpc();
    });

    it("should allow to withdraw", async () => {
        const tokenAccount = await getAssociatedTokenAddress(mint.publicKey, userAuthority.publicKey);
        setTimeout(async () => {
            await program.methods.withdraw(accruedToUser).accounts({
                authority: userAuthority.publicKey,
                tokenAccount,
                mint: mint.publicKey,
                mintAuthority: anchor.getProvider().publicKey,
                adminAccount: await getAssociatedTokenAddress(mint.publicKey, anchor.getProvider().publicKey)
            }).signers([userAuthority]).rpc();
            const account = await getAccount(anchor.getProvider().connection, tokenAccount);
            expect(account.amount).eq(accruedToUser);
            const escrow = await program.account.escrow.fetch(userEscrow.publicKey);
            expect(escrow.amountAccrued).eq(new anchor.BN(0));
        }, 5);
    });


});
