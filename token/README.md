# BioNeo Token Contract

BioNeo 代币合约，负责代币的初始化和总量分配。

## 功能特点

### 1. 代币参数
- 总供应量：2100 万
- 精度：6 位
- 代币分配比例：
  - 流动性：5%
  - 白名单：5%
  - LP 挖矿：20%
  - NFT 挖矿 1：30%
  - NFT 挖矿 2：30%
  - IDO：10%

### 2. 初始化功能
- 一次性铸造所有代币
- 按比例分配给各个模块
- 支持未准备好合约的情况（通过 `Pubkey::default()`）
- 记录代币分配计划

### 3. 提取功能
- 管理员可以提取合约中的代币
- 可以提取到任意账户
- 包含完整的安全检查

### 4. 安全性
- 溢出保护
- 权限控制
- 余额检查
- 所有权验证

## 合约地址

> ⚠️ **重要提示**：以下地址为示例地址，在实际部署时必须修改为正确的地址！
> 
> 部署步骤：
> 1. 修改 `lib.rs` 中的以下地址常量
> 2. 确保所有地址都是有效的 Solana 地址
> 3. 确保地址对应的账户已经创建
> 4. 确保地址对应的账户有足够的 SOL 支付租金

```rust
/// 初始化代币接收地址（钱包地址）
pub const INITIAL_TOKEN_RECEIVER: Pubkey = pubkey!("7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU");

/// 白名单合约地址
pub const WHITELIST_CONTRACT: Pubkey = pubkey!("GkzPnMSyYFy4UhVHcmwq5Y9TfNxZQ5Q5Z5Z5Z5Z5Z5Z5Z5");

/// LP 挖矿合约地址
pub const LP_STAKING_CONTRACT: Pubkey = pubkey!("HkzPnMSyYFy4UhVHcmwq5Y9TfNxZQ5Q5Z5Z5Z5Z5Z5Z5Z5");

/// NFT 挖矿合约地址 1
pub const NFT_STAKING_CONTRACT_1: Pubkey = pubkey!("IkzPnMSyYFy4UhVHcmwq5Y9TfNxZQ5Q5Z5Z5Z5Z5Z5Z5Z5");

/// NFT 挖矿合约地址 2
pub const NFT_STAKING_CONTRACT_2: Pubkey = pubkey!("JkzPnMSyYFy4UhVHcmwq5Y9TfNxZQ5Q5Z5Z5Z5Z5Z5Z5Z5");

/// IDO 合约地址
pub const IDO_CONTRACT: Pubkey = pubkey!("KkzPnMSyYFy4UhVHcmwq5Y9TfNxZQ5Q5Z5Z5Z5Z5Z5Z5Z5");
```

## 使用方法

### 1. 初始化代币
```rust
pub fn initialize_token(
    ctx: Context<InitializeToken>,
    total_supply: u64,  // 必须为 2100 万
    decimals: u8,       // 必须为 6
) -> Result<()>
```

### 2. 提取代币
```rust
pub fn withdraw_tokens(
    ctx: Context<WithdrawTokens>,
    amount: u64,        // 提取的代币数量
) -> Result<()>
```

## 注意事项

1. 初始化时，如果某个合约地址为 `Pubkey::default()`，则对应的代币会保留在合约中
2. 保留的代币可以通过 `withdraw_tokens` 函数提取
3. 只有管理员可以调用 `withdraw_tokens` 函数
4. 提取金额不能超过合约中的代币余额
5. ⚠️ 部署前必须修改所有合约地址为正确的地址

## 错误类型

```rust
pub enum TokenError {
    InvalidTotalSupply,    // 总供应量必须为 2100 万
    InvalidDecimals,       // 代币精度必须为 6
    TokenAllocationOverflow, // 代币分配计算溢出
    InvalidTokenAllocation,  // 代币分配总和不等于总供应量
    InvalidTokenAccountOwner, // 代币账户所有权错误
    InvalidMintOwner,        // 代币铸造账户所有权错误
    InsufficientBalance,     // 提取金额超过合约代币余额
    NotAuthority,           // 非管理员操作
}
```

## 账户结构

### TokenState
```rust
pub struct TokenState {
    pub authority: Pubkey,           // 合约管理员地址
    pub total_supply: u64,           // 代币总供应量
    pub decimals: u8,                // 代币精度
    pub lp_staking_amount: u64,      // LP 挖矿代币数量
    pub nft_staking_amount_1: u64,   // NFT 挖矿代币数量 1
    pub nft_staking_amount_2: u64,   // NFT 挖矿代币数量 2
    pub ido_amount: u64,             // IDO 代币数量
    pub whitelist_amount: u64,       // 白名单代币数量
    pub liquidity_amount: u64,       // 流动性代币数量
}
```

## 开发环境

- Solana 版本：1.17.0
- Anchor 版本：0.28.0
- Rust 版本：1.70.0 