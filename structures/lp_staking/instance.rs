use anchor_lang::prelude::*;
use crate::structures::lp_staking::pool::StakingPool;

#[account]
pub struct StakingInstance {
    pub authority: Pubkey,          // 管理员账户
    pub reward_token_mint: Pubkey,  // 奖励代币 Mint 地址
    pub staking_token_mint: Pubkey, // 质押代币 Mint 地址
    pub pools: [StakingPool; 3],    // 固定3个质押池
    pub lp_token_account: Pubkey,   // 合约接受lp的合约地址
} 