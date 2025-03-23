# BioNeo Staking Contract

这是一个基于 Solana 区块链的质押（Staking）智能合约项目，使用 Anchor 框架开发。该合约支持 LP 代币质押和 NFT 质押两种模式，用户可以通过质押获得奖励代币作为回报。

## 功能特性

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