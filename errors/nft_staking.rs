use anchor_lang::prelude::*;

#[error_code]
pub enum NftStakingError {
    #[msg("无效的质押类型")]
    InvalidStakeType,

    #[msg("无效的记录索引")]
    InvalidRecordIndex,

    #[msg("记录未激活")]
    RecordNotActive,

    #[msg("质押未到期")]
    StakingNotEnded,

    #[msg("已达到最大质押记录数")]
    MaxStakingRecordsReached,

    #[msg("没有可领取的奖励")]
    NoRewardsToClaim,

    #[msg("NFT 集合不匹配")]
    NftCollectionMismatch,

    #[msg("无效的 NFT 稀有度")]
    InvalidNftRarity,

    #[msg("NFT 所有权验证失败")]
    NftOwnershipVerificationFailed,

    #[msg("NFT 转移失败")]
    NftTransferFailed,
} 