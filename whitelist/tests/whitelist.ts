import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Whitelist } from "../target/types/whitelist";
import { PublicKey, SystemProgram, SYSVAR_RENT_PUBKEY } from "@solana/web3.js";
import { TOKEN_PROGRAM_ID, createMint, createAccount, mintTo, getAccount } from "@solana/spl-token";
import { assert } from "chai";

describe("whitelist", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Whitelist as Program<Whitelist>;
  
  // 测试账户
  let whitelistState: PublicKey;
  let tokenMint: PublicKey;
  let whitelistTokenAccount: PublicKey;
  let user1TokenAccount: PublicKey;
  let user2TokenAccount: PublicKey;
  let user3TokenAccount: PublicKey;
  
  // 白名单地址
  const WHITELIST_ADDRESSES = [
    new PublicKey("11111111111111111111111111111111"),
    new PublicKey("22222222222222222222222222222222"),
    new PublicKey("33333333333333333333333333333333"),
  ];

  // 代币分配比例
  const TOTAL_SUPPLY = new anchor.BN(21000000);
  const WHITELIST_AMOUNTS = [
    TOTAL_SUPPLY.mul(new anchor.BN(25)).div(new anchor.BN(1000)), // 2.5%
    TOTAL_SUPPLY.mul(new anchor.BN(15)).div(new anchor.BN(1000)), // 1.5%
    TOTAL_SUPPLY.mul(new anchor.BN(10)).div(new anchor.BN(1000)), // 1.0%
  ];

  before(async () => {
    // 创建代币
    tokenMint = await createMint(
      provider.connection,
      provider.wallet.payer,
      provider.wallet.publicKey,
      null,
      9
    );

    // 创建白名单代币账户
    whitelistTokenAccount = await createAccount(
      provider.connection,
      provider.wallet.payer,
      tokenMint,
      provider.wallet.publicKey
    );

    // 创建用户代币账户
    user1TokenAccount = await createAccount(
      provider.connection,
      provider.wallet.payer,
      tokenMint,
      WHITELIST_ADDRESSES[0]
    );

    user2TokenAccount = await createAccount(
      provider.connection,
      provider.wallet.payer,
      tokenMint,
      WHITELIST_ADDRESSES[1]
    );

    user3TokenAccount = await createAccount(
      provider.connection,
      provider.wallet.payer,
      tokenMint,
      WHITELIST_ADDRESSES[2]
    );

    // 铸造代币到白名单账户
    await mintTo(
      provider.connection,
      provider.wallet.payer,
      tokenMint,
      whitelistTokenAccount,
      provider.wallet.publicKey,
      TOTAL_SUPPLY.toNumber()
    );
  });

  it("初始化白名单", async () => {
    // 生成 PDA
    [whitelistState] = await PublicKey.findProgramAddress(
      [Buffer.from("whitelist_state")],
      program.programId
    );

    // 初始化白名单
    await program.methods
      .initializeWhitelist()
      .accounts({
        whitelistState,
        tokenMint,
        whitelistTokenAccount,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        rent: SYSVAR_RENT_PUBKEY,
      })
      .rpc();

    // 验证状态
    const state = await program.account.whitelistState.fetch(whitelistState);
    assert.ok(state.authority.equals(provider.wallet.publicKey));
    assert.ok(state.tokenMint.equals(tokenMint));
    assert.ok(state.whitelistTokenAccount.equals(whitelistTokenAccount));
    assert.ok(state.startTime.gt(new anchor.BN(0)));
  });

  it("查询可领取数量", async () => {
    // 等待一个月
    await new Promise(resolve => setTimeout(resolve, 1000));

    // 查询用户1的可领取数量
    const claimable1 = await program.methods
      .getClaimableAmount(WHITELIST_ADDRESSES[0])
      .accounts({
        whitelistState,
      })
      .view();

    // 验证可领取数量
    const monthlyAmount1 = WHITELIST_AMOUNTS[0].div(new anchor.BN(36));
    assert.ok(claimable1.eq(monthlyAmount1));
  });

  it("领取代币", async () => {
    // 用户1领取代币
    await program.methods
      .releaseTokens()
      .accounts({
        whitelistState,
        whitelistTokenAccount,
        userTokenAccount: user1TokenAccount,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([{ publicKey: WHITELIST_ADDRESSES[0], secretKey: Buffer.from([]) }])
      .rpc();

    // 验证代币余额
    const user1Balance = await getAccount(provider.connection, user1TokenAccount);
    const monthlyAmount1 = WHITELIST_AMOUNTS[0].div(new anchor.BN(36));
    assert.ok(new anchor.BN(user1Balance.amount.toString()).eq(monthlyAmount1));
  });

  it("非白名单地址无法查询", async () => {
    const nonWhitelistedAddress = new PublicKey("44444444444444444444444444444444");
    
    try {
      await program.methods
        .getClaimableAmount(nonWhitelistedAddress)
        .accounts({
          whitelistState,
        })
        .view();
      assert.fail("应该抛出错误");
    } catch (error) {
      assert.ok(error.toString().includes("AddressNotWhitelisted"));
    }
  });

  it("非白名单地址无法领取", async () => {
    const nonWhitelistedAddress = new PublicKey("44444444444444444444444444444444");
    const nonWhitelistedTokenAccount = await createAccount(
      provider.connection,
      provider.wallet.payer,
      tokenMint,
      nonWhitelistedAddress
    );

    try {
      await program.methods
        .releaseTokens()
        .accounts({
          whitelistState,
          whitelistTokenAccount,
          userTokenAccount: nonWhitelistedTokenAccount,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .signers([{ publicKey: nonWhitelistedAddress, secretKey: Buffer.from([]) }])
        .rpc();
      assert.fail("应该抛出错误");
    } catch (error) {
      assert.ok(error.toString().includes("AddressNotWhitelisted"));
    }
  });
}); 