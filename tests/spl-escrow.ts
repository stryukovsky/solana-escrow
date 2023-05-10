import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SplEscrow } from "../target/types/spl_escrow";
import {getAssociatedTokenAddress} from "@solana/spl-token";

describe("spl-escrow", () => {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());

    const program = anchor.workspace.SplEscrow as Program<SplEscrow>;

    const mint = anchor.web3.Keypair.generate();
    const userAuthority = anchor.web3.Keypair.generate();
    const userEscrow = anchor.web3.Keypair.generate();
    const userAccount = anchor.web3.Keypair.generate();
    const initialAmount = new anchor.BN(30000000);

    it("should be initialized", async () => {
        const tx = await program.methods.initialize(initialAmount)
            .accounts({
                mint: mint.publicKey,
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

    it("should accrue some tokens to user", async () => {

    });

    it("should allow to withdraw when some time passed", async () => {
        
    });


});
