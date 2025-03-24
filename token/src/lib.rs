use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};

declare_id!("6iadRi4ps7itomsTNa34RikS6hkmx2z5Ls1h9EqLPu1y");

/// BioNeo 代币合约
/// 负责代币的初始化和总量分配
#[program]
pub mod bioneo_token {
    use super::*;

    /// 初始化代币
    /// 
    /// 参数：
    /// - total_supply: 代币总供应量，必须为 2100 万
    /// 
    /// 功能：
    /// 1. 验证总供应量
    /// 2. 铸造代币
    /// 3. 分配代币给各个业务模块
    ///    - LP 挖矿：20%
    ///    - NFT 挖矿：60%
    ///    - IDO：10%
    ///    - 白名单：5%
    ///    - 流动性：5%
    pub fn initialize_token(
        ctx: Context<InitializeToken>,
        total_supply: u64,
    ) -> Result<()> {
        // 验证总供应量是否为 2100 万
        require!(
            total_supply == 21_000_000,
            TokenError::InvalidTotalSupply
        );

        let token_state = &mut ctx.accounts.token_state;
        token_state.authority = ctx.accounts.authority.key();
        token_state.total_supply = total_supply;
        
        // 分配方案
        token_state.lp_staking_amount = total_supply * 20 / 100; // 20% for LP staking
        token_state.nft_staking_amount = total_supply * 60 / 100; // 60% for NFT staking
        token_state.ido_amount = total_supply * 10 / 100; // 10% for IDO
        token_state.whitelist_amount = total_supply * 5 / 100; // 5% for whitelist
        token_state.liquidity_amount = total_supply * 5 / 100; // 5% for liquidity

        // 铸造代币
        let mint_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::MintTo {
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.token_account.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            },
        );

        token::mint_to(mint_ctx, total_supply)?;

        // 转移代币到各个业务模块的代币账户
        // 使用 PDA 签名进行安全的代币转移
        let transfer_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.token_account.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
            },
        );

        // 转移 LP 挖矿代币
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.token_account.to_account_info(),
                    to: ctx.accounts.lp_staking_token_account.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                    token_program: ctx.accounts.token_program.to_account_info(),
                },
                &[&[b"token_state".as_ref(), &[*ctx.bumps.get("token_state").unwrap()]]],
            ),
            token_state.lp_staking_amount,
        )?;

        // 转移 NFT 挖矿代币
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.token_account.to_account_info(),
                    to: ctx.accounts.nft_staking_token_account.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                    token_program: ctx.accounts.token_program.to_account_info(),
                },
                &[&[b"token_state".as_ref(), &[*ctx.bumps.get("token_state").unwrap()]]],
            ),
            token_state.nft_staking_amount,
        )?;

        // 转移 IDO 代币
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.token_account.to_account_info(),
                    to: ctx.accounts.ido_token_account.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                    token_program: ctx.accounts.token_program.to_account_info(),
                },
                &[&[b"token_state".as_ref(), &[*ctx.bumps.get("token_state").unwrap()]]],
            ),
            token_state.ido_amount,
        )?;

        // 转移白名单代币
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.token_account.to_account_info(),
                    to: ctx.accounts.whitelist_token_account.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                    token_program: ctx.accounts.token_program.to_account_info(),
                },
                &[&[b"token_state".as_ref(), &[*ctx.bumps.get("token_state").unwrap()]]],
            ),
            token_state.whitelist_amount,
        )?;

        // 转移流动性代币
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.token_account.to_account_info(),
                    to: ctx.accounts.liquidity_account.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                    token_program: ctx.accounts.token_program.to_account_info(),
                },
                &[&[b"token_state".as_ref(), &[*ctx.bumps.get("token_state").unwrap()]]],
            ),
            token_state.liquidity_amount,
        )?;

        Ok(())
    }
}

/// 初始化代币所需的账户
#[derive(Accounts)]
pub struct InitializeToken<'info> {
    /// 代币状态账户，存储代币分配信息
    #[account(
        init,
        payer = authority,
        space = 8 + TokenState::LEN,
        seeds = [b"token_state"],
        bump
    )]
    pub token_state: Account<'info, TokenState>,
    
    /// 合约管理员
    #[account(mut)]
    pub authority: Signer<'info>,
    
    /// 代币铸造账户
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    
    /// 代币账户，用于接收铸造的代币
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
    
    /// LP 挖矿合约的代币账户
    #[account(mut)]
    pub lp_staking_token_account: Account<'info, TokenAccount>,
    
    /// NFT 挖矿合约的代币账户
    #[account(mut)]
    pub nft_staking_token_account: Account<'info, TokenAccount>,
    
    /// IDO 合约的代币账户
    #[account(mut)]
    pub ido_token_account: Account<'info, TokenAccount>,
    
    /// 白名单合约的代币账户
    #[account(mut)]
    pub whitelist_token_account: Account<'info, TokenAccount>,
    
    /// 流动性账户
    #[account(mut)]
    pub liquidity_account: Account<'info, TokenAccount>,
    
    /// 系统程序
    pub system_program: Program<'info, System>,
    
    /// 代币程序
    pub token_program: Program<'info, Token>,
    
    /// 租金系统
    pub rent: Sysvar<'info, Rent>,
}

/// 代币状态账户
#[account]
pub struct TokenState {
    /// 合约管理员地址
    pub authority: Pubkey,
    
    /// 代币总供应量
    pub total_supply: u64,
    
    /// LP 挖矿代币数量
    pub lp_staking_amount: u64,
    
    /// NFT 挖矿代币数量
    pub nft_staking_amount: u64,
    
    /// IDO 代币数量
    pub ido_amount: u64,
    
    /// 白名单代币数量
    pub whitelist_amount: u64,
    
    /// 流动性代币数量
    pub liquidity_amount: u64,
}

impl TokenState {
    /// 账户大小
    pub const LEN: usize = 32 + 8 + 8 + 8 + 8 + 8 + 8;
}

/// 代币合约错误类型
#[error_code]
pub enum TokenError {
    /// 总供应量必须为 2100 万
    #[msg("总供应量必须为 2100 万")]
    InvalidTotalSupply,
    
    /// 代币余额不足
    #[msg("代币余额不足")]
    InsufficientTokens,
} 