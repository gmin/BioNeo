use anchor_lang::prelude::*;

#[account]
pub struct User {
    /// 用户地址
    pub address: Pubkey,
    
    /// 质押池地址
    pub pool: Pubkey,
    
    /// 质押的 NFT 数量
    pub amount: u64,
    
    /// 开始时间
    pub start_time: i64,
    
    /// 最后领取时间
    pub last_claim_time: i64,
    
    /// 已领取奖励
    pub rewards_claimed: u64,
}

impl User {
    /// 账户大小
    pub const SIZE: usize = 8 + // discriminator
        32 + // address
        32 + // pool
        8 + // amount
        8 + // start_time
        8 + // last_claim_time
        8; // rewards_claimed
} 