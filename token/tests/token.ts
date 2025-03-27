import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Token } from "../target/types/token";
import { PublicKey, SystemProgram, SYSVAR_RENT_PUBKEY } from "@solana/web3.js";
import { TOKEN_PROGRAM_ID, createMint, createAccount, mintTo, getAccount } from "@solana/spl-token";
import { assert } from "chai";

describe("token", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Token as Program<Token>;
  
  // 测试账户
  let tokenState: PublicKey;
  let tokenMint: PublicKey;
  let treasuryTokenAccount: PublicKey;
  let whitelistTokenAccount: PublicKey;
  let idoTokenAccount: PublicKey;
  let userTokenAccount: PublicKey;
  
  // 测试参数
  const TOTAL_SUPPLY = new anchor.BN(21000000);
  const TREASURY_AMOUNT = TOTAL_SUPPLY.mul(new anchor.BN(85)).div(new anchor.BN(100)); // 85%
  const WHITELIST_AMOUNT = TOTAL_SUPPLY.mul(new anchor.BN(5)).div(new anchor.BN(100)); // 5%
  const IDO_AMOUNT = TOTAL_SUPPLY.mul(new anchor.BN(10)).div(new anchor.BN(100)); // 10%

  before(async () => {
    // 创建代币
    tokenMint = await createMint(
      provider.connection,
      provider.wallet.payer,
      provider.wallet.publicKey,
      null,
      6
    );

    // 创建国库代币账户
    treasuryTokenAccount = await createAccount(
      provider.connection,
      provider.wallet.payer,
      tokenMint,
      provider.wallet.publicKey
    );

    // 创建白名单代币账户
    whitelistTokenAccount = await createAccount(
      provider.connection,
      provider.wallet.payer,
      tokenMint,
      provider.wallet.publicKey
    );

    // 创建 IDO 代币账户
    idoTokenAccount = await createAccount(
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
  });

  it("初始化代币", async () => {
    // 生成 PDA
    [tokenState] = await PublicKey.findProgramAddress(
      [Buffer.from("token_state")],
      program.programId
    );

    // 初始化代币
    await program.methods
      .initializeToken()
      .accounts({
        tokenState,
        authority: provider.wallet.publicKey,
        tokenMint,
        treasuryTokenAccount,
        whitelistTokenAccount,
        idoTokenAccount,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        rent: SYSVAR_RENT_PUBKEY,
      })
      .rpc();

    // 验证状态
    const state = await program.account.tokenState.fetch(tokenState);
    assert.ok(state.authority.equals(provider.wallet.publicKey));
    assert.ok(state.tokenMint.equals(tokenMint));
    assert.ok(state.treasuryTokenAccount.equals(treasuryTokenAccount));
    assert.ok(state.whitelistTokenAccount.equals(whitelistTokenAccount));
    assert.ok(state.idoTokenAccount.equals(idoTokenAccount));
  });

  it("铸造代币", async () => {
    // 铸造代币
    await program.methods
      .mintTokens()
      .accounts({
        tokenState,
        authority: provider.wallet.publicKey,
        tokenMint,
        treasuryTokenAccount,
        whitelistTokenAccount,
        idoTokenAccount,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();

    // 验证代币余额
    const treasuryBalance = await getAccount(provider.connection, treasuryTokenAccount);
    assert.ok(new anchor.BN(treasuryBalance.amount.toString()).eq(TREASURY_AMOUNT));

    const whitelistBalance = await getAccount(provider.connection, whitelistTokenAccount);
    assert.ok(new anchor.BN(whitelistBalance.amount.toString()).eq(WHITELIST_AMOUNT));

    const idoBalance = await getAccount(provider.connection, idoTokenAccount);
    assert.ok(new anchor.BN(idoBalance.amount.toString()).eq(IDO_AMOUNT));
  });

  it("转移代币", async () => {
    const transferAmount = new anchor.BN(1000000);

    // 转移代币
    await program.methods
      .transferTokens(transferAmount)
      .accounts({
        tokenState,
        authority: provider.wallet.publicKey,
        fromTokenAccount: treasuryTokenAccount,
        toTokenAccount: userTokenAccount,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();

    // 验证代币余额
    const userBalance = await getAccount(provider.connection, userTokenAccount);
    assert.ok(new anchor.BN(userBalance.amount.toString()).eq(transferAmount));
  });

  it("销毁代币", async () => {
    const burnAmount = new anchor.BN(100000);

    // 销毁代币
    await program.methods
      .burnTokens(burnAmount)
      .accounts({
        tokenState,
        authority: provider.wallet.publicKey,
        tokenMint,
        tokenAccount: userTokenAccount,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();

    // 验证代币余额
    const userBalance = await getAccount(provider.connection, userTokenAccount);
    assert.ok(new anchor.BN(userBalance.amount.toString()).eq(new anchor.BN(900000)));
  });

  it("非管理员无法铸造代币", async () => {
    const nonAuthority = new PublicKey("11111111111111111111111111111111");
    
    try {
      await program.methods
        .mintTokens()
        .accounts({
          tokenState,
          authority: nonAuthority,
          tokenMint,
          treasuryTokenAccount,
          whitelistTokenAccount,
          idoTokenAccount,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .signers([{ publicKey: nonAuthority, secretKey: Buffer.from([]) }])
        .rpc();
      assert.fail("应该抛出错误");
    } catch (error) {
      assert.ok(error.toString().includes("Unauthorized"));
    }
  });

  it("非管理员无法销毁代币", async () => {
    const nonAuthority = new PublicKey("11111111111111111111111111111111");
    
    try {
      await program.methods
        .burnTokens(new anchor.BN(100000))
        .accounts({
          tokenState,
          authority: nonAuthority,
          tokenMint,
          tokenAccount: userTokenAccount,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .signers([{ publicKey: nonAuthority, secretKey: Buffer.from([]) }])
        .rpc();
      assert.fail("应该抛出错误");
    } catch (error) {
      assert.ok(error.toString().includes("Unauthorized"));
    }
  });

  it("代币账户所有权验证", async () => {
    const wrongTokenAccount = await createAccount(
      provider.connection,
      provider.wallet.payer,
      tokenMint,
      provider.wallet.publicKey
    );
    
    try {
      await program.methods
        .transferTokens(new anchor.BN(1000000))
        .accounts({
          tokenState,
          authority: provider.wallet.publicKey,
          fromTokenAccount: wrongTokenAccount,
          toTokenAccount: userTokenAccount,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .rpc();
      assert.fail("应该抛出错误");
    } catch (error) {
      assert.ok(error.toString().includes("InvalidTokenAccount"));
    }
  });
}); 