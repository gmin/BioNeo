use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Transfer};
use crate::structures::nft_staking::{
    instance::NftStakingInstance,
    pool::NftStakingPool,
    user::NftUser,
    record::NftStakeRecord,
};

#[derive(Accounts)]
pub struct UnstakeNft<'info> {
    #[account(mut)]
    pub staking_instance: Account<'info, NftStakingInstance>,

    #[account(mut)]
    pub user_account: Account<'info, NftUser>,

    #[account(mut)]
    pub user: Signer<'info>,

    /// CHECK: 这是用户的 NFT 代币账户
    #[account(mut)]
    pub user_nft_account: AccountInfo<'info>,

    /// CHECK: 这是合约的 NFT 保管账户
    #[account(mut)]
    pub vault: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<UnstakeNft>, record_index: u64) -> Result<()> {
    let staking_instance = &mut ctx.accounts.staking_instance;
    let user_account = &mut ctx.accounts.user_account;
    
    // 检查记录索引是否有效
    require!(
        record_index < user_account.staking_count,
        ErrorCode::InvalidRecordIndex
    );

    // 获取质押记录
    let record = &mut user_account.staking_records[record_index as usize];
    
    // 检查记录是否激活
    require!(record.is_active, ErrorCode::RecordNotActive);
    
    // 获取当前时间戳
    let current_timestamp = Clock::get()?.unix_timestamp as u64;
    
    // 检查是否已到期
    require!(
        current_timestamp >= record.end_time,
        ErrorCode::StakingNotEnded
    );

    // 更新质押池
    let pool = &mut staking_instance.pools[record.stake_type as usize];
    pool.total_shares -= record.shares;
    pool.total_nfts -= 1;

    // 更新用户账户
    record.is_active = false;
    user_account.total_staked_nfts -= 1;

    // 更新总质押 NFT 数量
    staking_instance.total_staked_nfts -= 1;

    // 转移 NFT 回用户账户
    let transfer_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.vault.to_account_info(),
            to: ctx.accounts.user_nft_account.to_account_info(),
            authority: ctx.accounts.vault.to_account_info(),
        },
        &[&[
            b"nft_staking_instance".as_ref(),
            &[*ctx.bumps.get("staking_instance").unwrap()],
        ]],
    );
    anchor_spl::token::transfer(transfer_ctx, 1)?;

    Ok(())
} 