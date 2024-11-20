import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BetGame } from "../target/types/bet_game";
import { PublicKey, ComputeBudgetProgram } from "@solana/web3.js";
import { BN } from "bn.js";

describe("bet_game", async () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const connection = anchor.getProvider().connection;
  const program = anchor.workspace.BetGame as Program<BetGame>;
  const provider = anchor.AnchorProvider.env();
  const signer = provider.wallet.publicKey;
  const treasury = new PublicKey("2NTqkP5teRzUSye5gYLu5JaYX6J3xFgMYCDPgXYjc5zW");

  const findPDA = (label: string, seed: Buffer = Buffer.alloc(0)) =>
    anchor.web3.PublicKey.findProgramAddressSync([Buffer.from(label), seed], program.programId)[0];

  const setting = findPDA("setting");
  const gameStatus = findPDA("game_status");
  const addressList = findPDA("address_list");
  const topBetInfo = findPDA("top_bet_info");
  const vault = findPDA("vault");

  const getUserBettingInfo = (userAddress: PublicKey) => findPDA("user_betting_info", userAddress.toBuffer());
  const getBettingRound = (id: number) => findPDA("betting_round", Buffer.from(new BN(id).toArray("le", 8)));
  const getActiveBettingInfo = (userAddress: PublicKey) => findPDA("active_betting_info", userAddress.toBuffer());

  const sleep = (ms: number) => new Promise(resolve => setTimeout(resolve, ms));
  const duration  = 2;

  it("Setup Game", async () => {
    try {
      const tx = await program.methods.setupGame(signer)
        .rpc();
      console.log("Setup Game Transaction Signature:", tx);

      const settingData = await program.account.setting.fetch(setting);
      console.log("Setting Data:", settingData);
    } catch (error) {
      console.error("Error in Setup Game:", error);
    }
  });

  it("Bet", async () => {
    try {
      let balance = await connection.getBalance(vault);
      console.log("vault Balance Before:", balance);

      const tx = await program.methods.bet(new BN(100000000), true).rpc();
      console.log("Bet Transaction Signature:", tx);

      balance = await connection.getBalance(vault);
      console.log("vault Balance After:", balance);
    } catch (error) {
      console.error("Error in Bet:", error);
    }
  });

  it("Request Claim", async () => {
    try {
      let balance = await connection.getBalance(treasury);
      console.log("Treasury Balance Before:", balance);

      const tx = await program.methods.requestClaim()
        .rpc();
      console.log("Finish Round Transaction Signature:", tx);

      balance = await connection.getBalance(treasury);
      console.log("Treasury Balance After:", balance);
    } catch (error) {
      console.error("Error in Finish Round:", error);
    }
  });

  it("Transfer Reward", async () => {
    try {
      let balance = await connection.getBalance(treasury);
      console.log("Treasury Balance Before:", balance);
      const tx = await program.methods.transferReward(treasury, new BN(100000000))
      .accounts({
        treasury: treasury
      })
      .rpc();
      balance = await connection.getBalance(treasury);
      console.log("Treasury Balance After:", balance);
    console.log("Finish Round Transaction Signature:", tx);
    } catch (error) {
      console.error("Error in Handle Round:", error);
    }
  });
});
