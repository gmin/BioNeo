use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use anchor_lang::solana_program::native_token::LAMPORTS_PER_SOL;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

/// 初始化代币接收地址（钱包地址）
pub const INITIAL_TOKEN_RECEIVER: Pubkey = pubkey!("7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU");

/// 白名单合约地址
pub const WHITELIST_CONTRACT: Pubkey = pubkey!("GkzPnMSyYFy4UhVHcmwq5Y9TfNxZQ5Q5Z5Z5Z5Z5Z5Z5Z5");

/// LP 挖矿合约地址
pub const LP_STAKING_CONTRACT: Pubkey = pubkey!("HkzPnMSyYFy4UhVHcmwq5Y9TfNxZQ5Q5Z5Z5Z5Z5Z5Z5Z5");

/// NFT 挖矿合约地址 1
pub const NFT_STAKING_CONTRACT_1: Pubkey = pubkey!("IkzPnMSyYFy4UhVHcmwq5Y9TfNxZQ5Q5Z5Z5Z5Z5Z5Z5Z5");

/// NFT 挖矿合约地址 2
pub const NFT_STAKING_CONTRACT_2: Pubkey = pubkey!("JkzPnMSyYFy4UhVHcmwq5Y9TfNxZQ5Q5Z5Z5Z5Z5Z5Z5Z5");

/// IDO 合约地址
pub const IDO_CONTRACT: Pubkey = pubkey!("KkzPnMSyYFy4UhVHcmwq5Y9TfNxZQ5Q5Z5Z5Z5Z5Z5Z5Z5");

/// 代币总供应量
pub const TOTAL_SUPPLY: u64 = 21_000_000;

/// 代币精度
pub const TOKEN_DECIMALS: u8 = 6;

/// 代币状态账户
#[account]
#[derive(Debug)]
pub struct TokenState {
    /// 合约管理员地址
    pub authority: Pubkey,
    
    /// 代币总供应量
    pub total_supply: u64,
    
    /// 代币精度
    pub decimals: u8,
    
    /// LP 挖矿代币数量
    pub lp_staking_amount: u64,
    
    /// NFT 挖矿代币数量 1
    pub nft_staking_amount_1: u64,
    
    /// NFT 挖矿代币数量 2
    pub nft_staking_amount_2: u64,
    
    /// IDO 代币数量
    pub ido_amount: u64,
    
    /// 白名单代币数量
    pub whitelist_amount: u64,
    
    /// 流动性代币数量
    pub liquidity_amount: u64,
}

impl TokenState {
    /// 账户大小计算
    /// - authority: 32 bytes
    /// - total_supply: 8 bytes
    /// - decimals: 1 byte
    /// - lp_staking_amount: 8 bytes
    /// - nft_staking_amount_1: 8 bytes
    /// - nft_staking_amount_2: 8 bytes
    /// - ido_amount: 8 bytes
    /// - whitelist_amount: 8 bytes
    /// - liquidity_amount: 8 bytes
    pub const LEN: usize = 32 + 8 + 1 + 8 + 8 + 8 + 8 + 8 + 8;
}

/// 代币合约错误类型
#[error_code]
pub enum TokenError {
    /// 总供应量必须为 2100 万
    #[msg("总供应量必须为 2100 万")]
    InvalidTotalSupply,
    
    /// 代币精度无效
    #[msg("代币精度必须为 6")]
    InvalidDecimals,
    
    /// 代币分配溢出
    #[msg("代币分配计算溢出")]
    TokenAllocationOverflow,
    
    /// 代币分配总和错误
    #[msg("代币分配总和不等于总供应量")]
    InvalidTokenAllocation,
    
    /// 代币账户所有权错误
    #[msg("代币账户所有权错误")]
    InvalidTokenAccountOwner,
    
    /// 代币铸造账户所有权错误
    #[msg("代币铸造账户所有权错误")]
    InvalidMintOwner,
    
    /// 提取金额超过余额
    #[msg("提取金额超过合约代币余额")]
    InsufficientBalance,
    
    /// 非管理员操作
    #[msg("只有管理员可以执行此操作")]
    NotAuthority,
}

/// BioNeo 代币合约
/// 负责代币的初始化和总量分配
#[program]
pub mod bioneo_token {
    use super::*;

    /// 初始化代币
    /// 
    /// 参数：
    /// - total_supply: 代币总供应量，必须为 2100 万
    /// - decimals: 代币精度，必须为 6
    /// 
    /// 功能：
    /// 1. 验证总供应量
    /// 2. 一次性铸造所有代币到初始化接收地址
    /// 3. 按比例分配给各个接收地址
    ///    - 流动性：5%（钱包地址）
    ///    - 白名单：5%（合约地址）
    ///    - LP 挖矿：20%（合约地址）
    ///    - NFT 挖矿 1：30%（合约地址）
    ///    - NFT 挖矿 2：30%（合约地址）
    ///    - IDO：10%（合约地址）
    /// 
    /// 注意：
    /// - 如果某个合约地址为 Pubkey::default()，则对应的代币会保留在合约中
    /// - 后续可以通过 withdraw_tokens 函数提取这些代币
    pub fn initialize_token(
        ctx: Context<InitializeToken>,
        total_supply: u64,
        decimals: u8,
    ) -> Result<()> {
        // 验证总供应量是否为 2100 万
        require!(
            total_supply == TOTAL_SUPPLY,
            TokenError::InvalidTotalSupply
        );

        // 验证精度是否为 6
        require!(
            decimals == TOKEN_DECIMALS,
            TokenError::InvalidDecimals
        );

        // 验证代币铸造账户所有权
        require!(
            ctx.accounts.mint.mint_authority.unwrap() == ctx.accounts.authority.key(),
            TokenError::InvalidMintOwner
        );

        let token_state = &mut ctx.accounts.token_state;
        token_state.authority = ctx.accounts.authority.key();
        token_state.total_supply = total_supply;
        token_state.decimals = decimals;
        
        // 计算各模块代币数量（使用 checked_mul 和 checked_div 防止溢出）
        let liquidity_amount = total_supply
            .checked_mul(5)
            .and_then(|v| v.checked_div(100))
            .ok_or(TokenError::TokenAllocationOverflow)?;
            
        let whitelist_amount = total_supply
            .checked_mul(5)
            .and_then(|v| v.checked_div(100))
            .ok_or(TokenError::TokenAllocationOverflow)?;
            
        let lp_staking_amount = total_supply
            .checked_mul(20)
            .and_then(|v| v.checked_div(100))
            .ok_or(TokenError::TokenAllocationOverflow)?;
            
        let nft_staking_amount_1 = total_supply
            .checked_mul(30)
            .and_then(|v| v.checked_div(100))
            .ok_or(TokenError::TokenAllocationOverflow)?;
            
        let nft_staking_amount_2 = total_supply
            .checked_mul(30)
            .and_then(|v| v.checked_div(100))
            .ok_or(TokenError::TokenAllocationOverflow)?;
            
        let ido_amount = total_supply
            .checked_mul(10)
            .and_then(|v| v.checked_div(100))
            .ok_or(TokenError::TokenAllocationOverflow)?;
        
        // 验证分配总和是否等于总供应量
        let total_allocation = liquidity_amount
            .checked_add(whitelist_amount)
            .and_then(|v| v.checked_add(lp_staking_amount))
            .and_then(|v| v.checked_add(nft_staking_amount_1))
            .and_then(|v| v.checked_add(nft_staking_amount_2))
            .and_then(|v| v.checked_add(ido_amount))
            .ok_or(TokenError::TokenAllocationOverflow)?;
            
        require!(
            total_allocation == total_supply,
            TokenError::InvalidTokenAllocation
        );
        
        // 记录代币分配计划
        token_state.liquidity_amount = liquidity_amount;
        token_state.whitelist_amount = whitelist_amount;
        token_state.lp_staking_amount = lp_staking_amount;
        token_state.nft_staking_amount_1 = nft_staking_amount_1;
        token_state.nft_staking_amount_2 = nft_staking_amount_2;
        token_state.ido_amount = ido_amount;
        
        // 一次性铸造所有代币到初始化接收地址
        let mint_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::MintTo {
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.initial_token_receiver.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            },
        );
        token::mint_to(mint_ctx, total_supply)?;
        
        // 转移代币到各个接收地址
        // 1. 转移白名单代币
        if ctx.accounts.whitelist_token_account.owner != Pubkey::default() {
            let transfer_whitelist_ctx = CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.initial_token_receiver.to_account_info(),
                    to: ctx.accounts.whitelist_token_account.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                },
            );
            token::transfer(transfer_whitelist_ctx, whitelist_amount)?;
        }
        
        // 2. 转移 LP 挖矿代币
        if ctx.accounts.lp_staking_token_account.owner != Pubkey::default() {
            let transfer_lp_ctx = CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.initial_token_receiver.to_account_info(),
                    to: ctx.accounts.lp_staking_token_account.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                },
            );
            token::transfer(transfer_lp_ctx, lp_staking_amount)?;
        }
        
        // 3. 转移 NFT 挖矿代币
        if ctx.accounts.nft_staking_token_account_1.owner != Pubkey::default() {
            let transfer_nft_ctx_1 = CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.initial_token_receiver.to_account_info(),
                    to: ctx.accounts.nft_staking_token_account_1.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                },
            );
            token::transfer(transfer_nft_ctx_1, nft_staking_amount_1)?;
        }
        
        if ctx.accounts.nft_staking_token_account_2.owner != Pubkey::default() {
            let transfer_nft_ctx_2 = CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.initial_token_receiver.to_account_info(),
                    to: ctx.accounts.nft_staking_token_account_2.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                },
            );
            token::transfer(transfer_nft_ctx_2, nft_staking_amount_2)?;
        }
        
        // 4. 转移 IDO 代币
        if ctx.accounts.ido_token_account.owner != Pubkey::default() {
            let transfer_ido_ctx = CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.initial_token_receiver.to_account_info(),
                    to: ctx.accounts.ido_token_account.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                },
            );
            token::transfer(transfer_ido_ctx, ido_amount)?;
        }
        
        // 5. 流动性代币保留在 initial_token_receiver 账户中

        Ok(())
    }

    /// 提取合约中的代币
    /// 
    /// 参数：
    /// - amount: 提取的代币数量
    /// 
    /// 功能：
    /// 1. 验证调用者是否为管理员
    /// 2. 验证提取金额是否超过合约余额
    /// 3. 将代币转移到目标账户
    pub fn withdraw_tokens(
        ctx: Context<WithdrawTokens>,
        amount: u64,
    ) -> Result<()> {
        // 验证调用者是否为管理员
        require!(
            ctx.accounts.authority.key() == ctx.accounts.token_state.authority,
            TokenError::NotAuthority
        );

        // 验证提取金额是否超过合约余额
        require!(
            amount <= ctx.accounts.contract_token_account.amount,
            TokenError::InsufficientBalance
        );

        // 转移代币到目标账户
        let transfer_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.contract_token_account.to_account_info(),
                to: ctx.accounts.target_token_account.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            },
        );
        token::transfer(transfer_ctx, amount)?;

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
    
    /// 初始化代币接收地址
    #[account(
        mut,
        constraint = initial_token_receiver.owner == INITIAL_TOKEN_RECEIVER @ TokenError::InvalidTokenAccountOwner
    )]
    pub initial_token_receiver: Account<'info, TokenAccount>,
    
    /// 白名单合约代币账户
    #[account(mut)]
    pub whitelist_token_account: Account<'info, TokenAccount>,
    
    /// LP 挖矿合约代币账户
    #[account(mut)]
    pub lp_staking_token_account: Account<'info, TokenAccount>,
    
    /// NFT 挖矿合约代币账户 1
    #[account(mut)]
    pub nft_staking_token_account_1: Account<'info, TokenAccount>,
    
    /// NFT 挖矿合约代币账户 2
    #[account(mut)]
    pub nft_staking_token_account_2: Account<'info, TokenAccount>,
    
    /// IDO 合约代币账户
    #[account(mut)]
    pub ido_token_account: Account<'info, TokenAccount>,
    
    /// 系统程序
    pub system_program: Program<'info, System>,
    
    /// 代币程序
    pub token_program: Program<'info, Token>,
    
    /// 租金系统
    pub rent: Sysvar<'info, Rent>,
}

/// 提取代币所需的账户
#[derive(Accounts)]
pub struct WithdrawTokens<'info> {
    /// 代币状态账户
    #[account(mut)]
    pub token_state: Account<'info, TokenState>,
    
    /// 合约管理员
    #[account(mut)]
    pub authority: Signer<'info>,
    
    /// 合约代币账户
    #[account(mut)]
    pub contract_token_account: Account<'info, TokenAccount>,
    
    /// 目标代币账户
    #[account(mut)]
    pub target_token_account: Account<'info, TokenAccount>,
    
    /// 代币程序
    pub token_program: Program<'info, Token>,
} 