# BioNeo-Healthy

BioNeo，创新的区块链+AI音乐健康平台

## 需求说明
分成多个模块来实现，代币合约总控，负责代币的分发和释放。

### 代币合约
- 总量控制：固定 2100 万代币
- 智能分配：自动分配给各个业务模块
- 安全铸造：一次性铸造，确保总量准确
- 模块化管理：各业务模块独立管理代币释放

### LP 代币质押合约
- 代币分配：20% 代币用于 IDO
- 多期限质押池：支持 3个月、6个月 和 12个月 三种质押期限，挖矿周期20年。
- 灵活的奖励机制：基于时间计算的奖励分配系统
- 推荐奖励：支持用户推荐奖励机制
- 质押管理：支持质押、取消质押和领取奖励等基本操作
- 安全机制：包含完整的权限控制和错误处理

### NFT 质押合约
- NFT矿池有两个，具体需求待定

### IDO 合约
- 代币分配：10% 代币用于 IDO
- 购买规则：每份500美金，一起400份，每份5250。
- 分期释放：每份分成12份释放，释放周期在购买后的一年。
- 安全控制：完整的权限和数量控制

### 白名单合约
- 智能分配：5% 代币分配给白名单
- 时间控制：分36个月逐步释放
- 分级释放：硬编码三个白名单地址，每个地址分配比例分配：
  - 白名单1：2.5%
  - 白名单2：1.5%
  - 白名单3：1.0%

### 流动性管理
- 预先分配：代币合约5%预挖，其中2.5%手动进入流动性池，2.5%自由支配。初始化的LP token需要锁定一年。



## 项目结构

```
bioneo/
├── token/                 # 代币合约
│   ├── src/              # 源代码
│   │   └── lib.rs        # 代币合约实现
│   ├── tests/            # 测试文件
│   │   └── token.ts      # 代币合约测试
│   └── README.md         # 代币合约文档
│
├── whitelist/            # 白名单合约
│   ├── src/              # 源代码
│   │   └── lib.rs        # 白名单合约实现
│   ├── tests/            # 测试文件
│   │   └── whitelist.ts  # 白名单合约测试
│   └── README.md         # 白名单合约文档
│
├── ido/                  # IDO 合约
│   ├── src/              # 源代码
│   │   └── lib.rs        # IDO 合约实现
│   ├── tests/            # 测试文件
│   │   └── ido.ts        # IDO 合约测试
│   └── README.md         # IDO 合约文档
│
└── README.md             # 项目总文档
```

## 合约说明

### 1. 代币合约 (Token)
- 管理代币的铸造、转移和销毁
- 代币分配：
  - 国库：85%
  - 白名单：5%
  - IDO：10%

### 2. 白名单合约 (Whitelist)
- 管理白名单用户的代币释放
- 代币分配：
  - 白名单 1：2.5%
  - 白名单 2：1.5%
  - 白名单 3：1.0%
- 释放周期：36 个月线性释放

### 3. IDO 合约 (IDO)
- 管理代币的众筹发行
- 代币分配：10%
- 众筹时间：7 天
- 释放周期：36 个月线性释放
- 支付代币：USDT

## 开发环境

- Solana: 1.17.0
- Anchor: 0.28.0
- Rust: 1.70.0

## 部署说明

1. 克隆项目
```bash
git clone https://github.com/yourusername/bioneo.git
cd bioneo
```

2. 安装依赖
```bash
yarn install
```

3. 编译合约
```bash
anchor build
```

4. 运行测试
```bash
anchor test
```

5. 部署合约
```bash
anchor deploy
```

## 重要说明

1. 代币精度：6
2. 总供应量：21,000,000
3. 代币账户所有权严格控制在合约中
4. 白名单和 IDO 代币按月线性释放
5. 众筹失败时，IDO 代币将被永久锁定 