# BioNeo-Healthy

BioNeo，创新的区块链+AI音乐健康平台

## 功能特性

### 代币系统
- 总量控制：固定 2100 万代币
- 智能分配：自动分配给各个业务模块
- 安全铸造：一次性铸造，确保总量准确
- 模块化管理：各业务模块独立管理代币释放

### LP 代币质押
- 多期限质押池：支持 3个月、6个月 和 12个月 三种质押期限
- 灵活的奖励机制：基于时间计算的奖励分配系统
- 推荐奖励：支持用户推荐奖励机制
- 质押管理：支持质押、取消质押和领取奖励等基本操作
- 安全机制：包含完整的权限控制和错误处理

### NFT 质押
- 多期限质押池：支持 3个月、6个月 和 12个月 三种质押期限
- NFT 稀有度奖励：基于 NFT 稀有度的差异化奖励机制
- 批量质押：支持同时质押多个 NFT
- 安全保管：NFT 在质押期间由合约安全保管
- 灵活的奖励计算：考虑 NFT 稀有度的奖励分配系统

### IDO 系统
- 代币分配：10% 代币用于 IDO
- 灵活释放：支持多种释放机制
- 安全控制：完整的权限和数量控制

### 白名单系统
- 智能分配：5% 代币分配给白名单
- 分级释放：支持三个等级的白名单地址
- 时间控制：分12个月逐步释放
- 比例分配：
  - 白名单1：2.5%
  - 白名单2：1.5%
  - 白名单3：1.0%

### 流动性管理
- 预先分配：5% 代币用于流动性
- 即时可用：初始化时即可使用
- 灵活部署：支持多种流动性方案

### AI音乐结合

## 技术架构

### 主要组件

#### LP 质押系统
1. **StakingInstance（质押实例）**
   - 管理质押池的核心账户
   - 包含奖励代币和质押代币的 Mint 地址
   - 维护三个不同期限的质押池

2. **StakingPool（质押池）**
   - 管理特定期限的质押池
   - 追踪奖励分配和总质押份额
   - 维护奖励计算相关的时间戳

3. **User（用户账户）**
   - 存储用户的质押信息
   - 管理用户的奖励债务和累计奖励
   - 支持多个质押记录

#### NFT 质押系统
1. **NftStakingInstance（NFT 质押实例）**
   - 管理 NFT 质押池的核心账户
   - 包含奖励代币和 NFT 集合地址
   - 维护 NFT 保管账户

2. **NftStakingPool（NFT 质押池）**
   - 管理特定期限的 NFT 质押池
   - 追踪 NFT 质押数量和奖励分配
   - 支持稀有度奖励倍数

3. **NftUser（NFT 用户账户）**
   - 存储用户的 NFT 质押信息
   - 管理 NFT 质押记录和奖励
   - 支持更多质押记录（最多20个）

### 核心功能

#### LP 质押指令
- `initialize_staking`: 初始化质押系统
- `initialize_user`: 初始化用户账户
- `enter_staking`: 进入质押
- `cancel_staking`: 取消质押
- `claim_rewards`: 领取奖励

#### NFT 质押指令
- `initialize_nft_staking`: 初始化 NFT 质押系统
- `initialize_nft_user`: 初始化 NFT 用户账户
- `stake_nft`: 质押 NFT
- `unstake_nft`: 取消 NFT 质押
- `claim_nft_rewards`: 领取 NFT 质押奖励

## 开发环境

- Solana 区块链
- Anchor 框架
- Rust 编程语言

## 项目结构

```
.
├── lib.rs           # 主合约逻辑
├── constants.rs     # 常量定义
├── tools.rs         # 工具函数
├── errors/          # 错误处理
│   ├── mod.rs
│   ├── lp_staking.rs
│   └── nft_staking.rs
├── structures/      # 数据结构定义
│   ├── mod.rs
│   ├── lp_staking/  # LP 质押相关结构
│   │   ├── mod.rs
│   │   ├── instance.rs
│   │   ├── pool.rs
│   │   └── user.rs
│   └── nft_staking/ # NFT 质押相关结构
│       ├── mod.rs
│       ├── instance.rs
│       ├── pool.rs
│       ├── user.rs
│       └── record.rs
└── instructions/    # 指令实现
    ├── mod.rs
    ├── lp_staking/  # LP 质押相关指令
    │   ├── mod.rs
    │   ├── initialize.rs
    │   ├── enter.rs
    │   ├── cancel.rs
    │   └── claim.rs
    └── nft_staking/ # NFT 质押相关指令
        ├── mod.rs
        ├── initialize.rs
        ├── stake.rs
        ├── unstake.rs
        └── claim.rs
```

## 使用说明

### LP 代币质押
1. 初始化质押系统
   - 设置不同期限的奖励率
   - 配置奖励代币和质押代币

2. 用户操作
   - 初始化用户账户
   - 选择质押期限并质押代币
   - 等待质押期满后取消质押
   - 随时领取已获得的奖励

### NFT 质押
1. 初始化 NFT 质押系统
   - 设置 NFT 集合地址
   - 配置奖励代币和保管账户
   - 设置不同稀有度的奖励倍数

2. 用户操作
   - 初始化 NFT 用户账户
   - 选择质押期限并质押 NFT
   - 等待质押期满后取消质押
   - 随时领取已获得的奖励

## 注意事项

### LP 代币质押
- 质押期满前无法取消质押
- 每个用户最多可以同时进行 10 个质押
- 奖励计算基于时间戳，确保系统时间准确
- 所有金额计算都考虑了溢出保护

### NFT 质押
- 质押期满前无法取消质押
- 每个用户最多可以同时质押 20 个 NFT
- NFT 在质押期间由合约安全保管
- 稀有度影响奖励计算
- 支持批量质押操作

## 错误处理

合约包含完整的错误处理机制，主要错误类型包括：

### LP 质押错误
- 无效的质押类型
- 余额不足
- 时间戳获取失败
- 代币账户不匹配
- 算术溢出/下溢
- 质押状态错误

### NFT 质押错误
- 无效的质押类型
- NFT 集合不匹配
- 无效的 NFT 稀有度
- NFT 所有权验证失败
- NFT 转移失败
- 记录索引错误

## 测试

项目包含完整的测试用例，覆盖以下场景：

### LP 质押测试
- 系统初始化测试
- 用户账户初始化测试
- 质押操作测试
- 取消质押测试
- 奖励领取测试
- 错误处理测试

### NFT 质押测试
- NFT 系统初始化测试
- NFT 用户账户初始化测试
- NFT 质押操作测试
- NFT 取消质押测试
- NFT 奖励领取测试
- NFT 稀有度奖励测试
- NFT 错误处理测试

## 许可证

[待补充]

## 贡献指南

[待补充]

# BioNeo Token

BioNeo 代币合约，负责代币的初始化和总量分配。

## 代币分配方案

总供应量：21,000,000 代币

- LP 挖矿：20% (4,200,000)
- NFT 挖矿：60% (12,600,000)
- IDO：10% (2,100,000)
- 白名单：5% (1,050,000)
- 流动性：5% (1,050,000)

## 功能说明

### 初始化代币

初始化代币时，将代币分配给各个业务模块：

1. 铸造所有代币
2. 将代币分配给各个业务模块的代币账户：
   - LP 挖矿合约：20%
   - NFT 挖矿合约：60%
   - IDO 合约：10%
   - 白名单合约：5%
   - 流动性账户：5%

## 业务模块

### LP 挖矿合约

负责 LP 代币的质押和奖励发放。

### NFT 挖矿合约

负责 NFT 的质押和奖励发放。

### IDO 合约

负责代币的众筹和释放。

IDO 合约负责代币的众筹发行，主要功能包括：

1. 众筹初始化
   - 设置众筹时间（开始时间和结束时间）
   - 设置代币价格和总份额
   - 设置退款时间范围
   - 初始化众筹状态

2. 参与众筹
   - 用户支付 USDC 参与众筹
   - 记录用户参与份额和支付金额
   - 更新众筹状态

3. 代币领取
   - 按月释放代币
   - 计算可领取数量
   - 转移代币给用户
   - 更新领取记录

4. 退款功能
   - 在指定时间范围内允许退款
   - 仅支持未领取过代币的用户退款
   - 退还用户支付的全部 USDC
   - 更新众筹状态和参与记录

安全特性：
- 时间控制：众筹和退款都有明确的时间范围
- 数量限制：确保不超过总份额
- 状态检查：防止重复领取和非法退款
- 权限控制：只有参与用户才能退款
- 溢出保护：所有计算都有溢出检查

### 白名单合约

负责白名单代币的释放：

1. 白名单分配：
   - 白名单1：2.5%
   - 白名单2：1.5%
   - 白名单3：1.0%

2. 释放规则：
   - 分12个月释放
   - 每个白名单地址每月释放固定比例
   - 只能在指定时间后开始释放
   - 最多释放12个月

3. 安全控制：
   - 时间控制：只能在指定时间后释放
   - 权限控制：只有白名单地址可以释放
   - 数量控制：最多释放12个月
   - 溢出保护：使用 checked_add 防止溢出

## 错误处理

```rust
pub enum TokenError {
    InvalidTotalSupply,    // 总供应量必须为 2100 万
    InsufficientTokens,    // 代币余额不足
}
```

## 使用说明

1. 初始化代币：
   ```rust
   initialize_token(
       total_supply: u64,  // 2100 万
   )
   ```

2. 初始化白名单：
   ```rust
   initialize_whitelist(
       whitelist1: Pubkey,  // 白名单1地址
       whitelist2: Pubkey,  // 白名单2地址
       whitelist3: Pubkey,  // 白名单3地址
       start_time: i64,     // 开始释放时间
   )
   ```

3. 释放白名单代币：
   ```rust
   release_tokens()  // 由白名单地址调用
   ```

## 注意事项

1. 代币合约只负责初始化和总量分配
2. 各个业务模块负责自己的代币释放逻辑
3. 白名单代币分12个月释放
4. 每个白名单地址有固定的释放比例
5. 流动性代币在初始化时就预先挖出 