use anchor_lang::prelude::*;
use crate::structures::nft_staking::pool::NftStakingPool;

#[account]
pub struct NftStakingInstance {
    pub authority: Pubkey,           // 管理员账户
    pub reward_token_mint: Pubkey,   // 奖励代币 Mint 地址
    pub nft_collection: Pubkey,      // NFT 集合地址
    pub pools: [NftStakingPool; 3],  // 固定3个质押池
    pub vault: Pubkey,               // NFT 保管账户
    pub total_staked_nfts: u64,      // 总质押 NFT 数量
} 