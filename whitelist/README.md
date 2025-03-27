# BioNeo 白名单合约

## 需求说明
### 白名单分配
白名单合约控制代币总量的5%，按以下比例分配给三个地址：
- 白名单1：2.5%（总量的50%）
- 白名单2：1.5%（总量的30%）
- 白名单3：1.0%（总量的20%）

###其他规则
- 36个月线性释放代币，白名单合约部署后，一个月就开始释放代币。
- 仅白名单地址可查询可领取数量
- 支持一次性领取所有已释放代币
- 白名单合约先期部署，用来接受代币合约的代币分配。


## 使用方法

### 1. 初始化白名单

```typescript
// 初始化白名单合约
await program.methods
  .initializeWhitelist()
  .accounts({
    whitelistState: whitelistStatePda,
    authority: admin.publicKey,
    tokenAccount: whitelistTokenAccount,
    systemProgram: SystemProgram.programId,
  })
  .signers([admin])
  .rpc();
```

### 2. 释放代币

```typescript
// 释放代币
await program.methods
  .releaseTokens()
  .accounts({
    whitelistState: whitelistStatePda,
    tokenAccount: whitelistTokenAccount,
    userTokenAccount: userTokenAccount,
    user: user.publicKey,
    tokenProgram: TOKEN_PROGRAM_ID,
  })
  .signers([user])
  .rpc();
```

### 3. 查询可领取数量

```typescript
// 仅白名单地址可查询可领取数量
const amount = await program.methods
  .getClaimableAmount()
  .accounts({
    whitelistState: whitelistStatePda,
    user: user.publicKey,  // 必须是白名单地址
  })
  .view();
```

## 注意事项

1. 初始化只能执行一次
2. 只有白名单地址可以释放代币
3. 只有白名单地址可以查询可领取数量
4. 代币账户所有者必须是白名单合约
5. 释放时间从初始化时开始计算
6. 每月释放一次，可以随时领取

## 错误类型

- `ReleaseTimeNotReached`: 释放时间未到
- `NoMoreReleases`: 没有更多释放计划
- `ExceededReleasePeriod`: 超出释放周期
- `UnauthorizedRelease`: 未授权的释放
- `ArithmeticOverflow`: 算术溢出
- `InvalidTokenAccount`: 无效的代币账户
- `AlreadyInitialized`: 合约已经初始化

## 账户结构

### WhitelistState

```rust
pub struct WhitelistState {
    pub authority: Pubkey,          // 合约管理员地址
    pub whitelist1: Pubkey,         // 白名单1地址
    pub whitelist2: Pubkey,         // 白名单2地址
    pub whitelist3: Pubkey,         // 白名单3地址
    pub total_amount: u64,          // 总代币数量
    pub whitelist1_amount: u64,     // 白名单1的释放金额
    pub whitelist2_amount: u64,     // 白名单2的释放金额
    pub whitelist3_amount: u64,     // 白名单3的释放金额
    pub start_time: i64,            // 开始释放时间
    pub last_release_time: i64,     // 最后释放时间
    pub total_released: u64,        // 总释放数量
    pub whitelist1_claimed: u64,    // 白名单1已领取数量
    pub whitelist2_claimed: u64,    // 白名单2已领取数量
    pub whitelist3_claimed: u64,    // 白名单3已领取数量
}
```

## 开发环境

- Solana: 1.17.0
- Anchor: 0.28.0
- Rust: 1.70.0 