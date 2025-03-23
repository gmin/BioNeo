use anchor_lang::prelude::*;

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct NftStakingPool {
    pub stake_type: u64,              // 0 代表3个月，1 代表6个月，2代表12个月
    pub reward_token_per_sec: u64,    // 每秒奖励代币数量
    pub accumulated_reward_per_share: u64, // 累计奖励分摊
    pub last_reward_timestamp: u64,   // 上次更新奖励的时间戳
    pub total_shares: u64,            // 该池中质押的总份额
    pub total_nfts: u64,              // 该池中质押的 NFT 总数
    pub rarity_multiplier: u64,       // 稀有度奖励倍数
} 