use anchor_lang::prelude::*;

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct NftStakeRecord {
    pub nft_mint: Pubkey,             // NFT 代币地址
    pub stake_type: u64,              // 质押类型
    pub rarity: u64,                  // NFT 稀有度
    pub shares: u64,                  // 质押份额
    pub reward_debt: u64,             // 奖励债务
    pub start_time: u64,              // 开始时间
    pub end_time: u64,                // 结束时间
    pub is_active: bool,              // 是否激活
} 