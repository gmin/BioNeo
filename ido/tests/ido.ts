import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Ido } from "../target/types/ido";
import { PublicKey, SystemProgram, SYSVAR_RENT_PUBKEY } from "@solana/web3.js";
import { TOKEN_PROGRAM_ID, createMint, createAccount, mintTo, getAccount } from "@solana/spl-token";
import { assert } from "chai";

describe("ido", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Ido as Program<Ido>;
  
  // 测试账户
  let idoState: PublicKey;
  let tokenMint: PublicKey;
  let idoTokenAccount: PublicKey;
  let whitelistTokenAccount: PublicKey;
  let userTokenAccount: PublicKey;
  let userUsdtAccount: PublicKey;
  
  // 测试参数
  const TOTAL_SHARES = new anchor.BN(1000);
  const PRICE_PER_SHARE = new anchor.BN(1000000); // 1 USDT per share
  const TOTAL_TOKENS = new anchor.BN(2100000); // 10% of total supply

  before(async () => {
    // 创建代币
    tokenMint = await createMint(
      provider.connection,
      provider.wallet.payer,
      provider.wallet.publicKey,
      null,
      9
    );

    // 创建 IDO 代币账户
    idoTokenAccount = await createAccount(
      provider.connection,
      provider.wallet.payer,
      tokenMint,
      provider.wallet.publicKey
    );

    // 创建白名单代币账户（接收 USDT）
    whitelistTokenAccount = await createAccount(
      provider.connection,
      provider.wallet.payer,
      tokenMint,
      provider.wallet.publicKey
    );

    // 创建用户代币账户
    userTokenAccount = await createAccount(
      provider.connection,
      provider.wallet.payer,
      tokenMint,
      provider.wallet.publicKey
    );

    // 创建用户 USDT 账户
    userUsdtAccount = await createAccount(
      provider.connection,
      provider.wallet.payer,
      tokenMint,
      provider.wallet.publicKey
    );

    // 铸造代币到 IDO 账户
    await mintTo(
      provider.connection,
      provider.wallet.payer,
      tokenMint,
      idoTokenAccount,
      provider.wallet.publicKey,
      TOTAL_TOKENS.toNumber()
    );

    // 铸造 USDT 到用户账户
    await mintTo(
      provider.connection,
      provider.wallet.payer,
      tokenMint,
      userUsdtAccount,
      provider.wallet.publicKey,
      PRICE_PER_SHARE.mul(TOTAL_SHARES).toNumber()
    );
  });

  it("初始化众筹", async () => {
    // 生成 PDA
    [idoState] = await PublicKey.findProgramAddress(
      [Buffer.from("ido_state")],
      program.programId
    );

    const startTime = new anchor.BN(Math.floor(Date.now() / 1000));
    const endTime = startTime.add(new anchor.BN(7 * 24 * 60 * 60)); // 7 days

    // 初始化众筹
    await program.methods
      .initializeIdo(
        startTime,
        endTime,
        PRICE_PER_SHARE,
        TOTAL_SHARES
      )
      .accounts({
        idoState,
        authority: provider.wallet.publicKey,
        tokenMint,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    // 验证状态
    const state = await program.account.idoState.fetch(idoState);
    assert.ok(state.authority.equals(provider.wallet.publicKey));
    assert.ok(state.tokenMint.equals(tokenMint));
    assert.ok(state.startTime.eq(startTime));
    assert.ok(state.endTime.eq(endTime));
    assert.ok(state.pricePerShare.eq(PRICE_PER_SHARE));
    assert.ok(state.totalShares.eq(TOTAL_SHARES));
    assert.ok(state.soldShares.eq(new anchor.BN(0)));
    assert.ok(state.totalRaised.eq(new anchor.BN(0)));
  });

  it("参与众筹", async () => {
    const shares = new anchor.BN(100);
    const paymentAmount = PRICE_PER_SHARE.mul(shares);

    // 参与众筹
    await program.methods
      .participateIdo(shares)
      .accounts({
        idoState,
        participation: await PublicKey.findProgramAddress(
          [Buffer.from("participation"), provider.wallet.publicKey.toBuffer()],
          program.programId
        ),
        user: provider.wallet.publicKey,
        userUsdtAccount,
        whitelistTokenAccount,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();

    // 验证状态
    const state = await program.account.idoState.fetch(idoState);
    assert.ok(state.soldShares.eq(shares));
    assert.ok(state.totalRaised.eq(paymentAmount));

    // 验证 USDT 转移
    const whitelistBalance = await getAccount(provider.connection, whitelistTokenAccount);
    assert.ok(new anchor.BN(whitelistBalance.amount.toString()).eq(paymentAmount));
  });

  it("领取代币", async () => {
    // 等待一个月
    await new Promise(resolve => setTimeout(resolve, 1000));

    // 领取代币
    await program.methods
      .claimTokens()
      .accounts({
        idoState,
        participation: await PublicKey.findProgramAddress(
          [Buffer.from("participation"), provider.wallet.publicKey.toBuffer()],
          program.programId
        ),
        user: provider.wallet.publicKey,
        idoTokenAccount,
        userTokenAccount,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();

    // 验证代币余额
    const userBalance = await getAccount(provider.connection, userTokenAccount);
    const monthlyAmount = TOTAL_TOKENS.div(new anchor.BN(36));
    assert.ok(new anchor.BN(userBalance.amount.toString()).eq(monthlyAmount));
  });

  it("众筹未开始无法参与", async () => {
    const shares = new anchor.BN(100);
    
    try {
      await program.methods
        .participateIdo(shares)
        .accounts({
          idoState,
          participation: await PublicKey.findProgramAddress(
            [Buffer.from("participation"), provider.wallet.publicKey.toBuffer()],
            program.programId
          ),
          user: provider.wallet.publicKey,
          userUsdtAccount,
          whitelistTokenAccount,
          systemProgram: SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .rpc();
      assert.fail("应该抛出错误");
    } catch (error) {
      assert.ok(error.toString().includes("IdoNotActive"));
    }
  });

  it("份额不足无法参与", async () => {
    const shares = TOTAL_SHARES.add(new anchor.BN(1));
    
    try {
      await program.methods
        .participateIdo(shares)
        .accounts({
          idoState,
          participation: await PublicKey.findProgramAddress(
            [Buffer.from("participation"), provider.wallet.publicKey.toBuffer()],
            program.programId
          ),
          user: provider.wallet.publicKey,
          userUsdtAccount,
          whitelistTokenAccount,
          systemProgram: SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .rpc();
      assert.fail("应该抛出错误");
    } catch (error) {
      assert.ok(error.toString().includes("InsufficientShares"));
    }
  });
}); 