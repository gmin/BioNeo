use anchor_lang::solana_program::hash::hash;
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use bioneo::{
    self,
    instructions::nft_staking::*,
    structures::nft_staking::*,
};

#[tokio::test]
async fn test_initialize_nft_staking() {
    let program = anchor_lang::solana_program::program_id!("6iadRi4ps7itomsTNa34RikS6hkmx2z5Ls1h9EqLPu1y");
    let payer = anchor_lang::solana_program::system_program::id();
    let reward_token_mint = Pubkey::new_unique();
    let staking_instance = Pubkey::new_unique();

    let accounts = InitializeNftStaking {
        staking_instance,
        authority: payer,
        reward_token_mint,
        system_program: anchor_lang::solana_program::system_program::id(),
        token_program: anchor_spl::token::id(),
        rent: anchor_lang::solana_program::sysvar::rent::id(),
    };

    let reward_token_per_sec = 1000u64;

    let result = program
        .instruction(
            bioneo::instruction::InitializeNftStaking { reward_token_per_sec },
            &accounts,
        )
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_initialize_nft_user() {
    let program = anchor_lang::solana_program::program_id!("6iadRi4ps7itomsTNa34RikS6hkmx2z5Ls1h9EqLPu1y");
    let user = Pubkey::new_unique();
    let user_account = Pubkey::new_unique();

    let accounts = InitializeNftUser {
        user_account,
        user,
        system_program: anchor_lang::solana_program::system_program::id(),
        rent: anchor_lang::solana_program::sysvar::rent::id(),
    };

    let result = program
        .instruction(bioneo::instruction::InitializeNftUser, &accounts)
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stake_nft() {
    let program = anchor_lang::solana_program::program_id!("6iadRi4ps7itomsTNa34RikS6hkmx2z5Ls1h9EqLPu1y");
    let staking_instance = Pubkey::new_unique();
    let user_account = Pubkey::new_unique();
    let user = Pubkey::new_unique();
    let nft_mint = Pubkey::new_unique();
    let nft_token_account = Pubkey::new_unique();
    let nft_metadata = Pubkey::new_unique();
    let token_program = anchor_spl::token::id();
    let metadata_program = anchor_lang::solana_program::program_id!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");

    let accounts = StakeNft {
        staking_instance,
        user_account,
        user,
        nft_mint,
        nft_token_account,
        nft_metadata,
        token_program,
        metadata_program,
    };

    let stake_type = 0u64; // 3个月
    let referrer = None;

    let result = program
        .instruction(
            bioneo::instruction::StakeNft {
                stake_type,
                referrer,
            },
            &accounts,
        )
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_unstake_nft() {
    let program = anchor_lang::solana_program::program_id!("6iadRi4ps7itomsTNa34RikS6hkmx2z5Ls1h9EqLPu1y");
    let staking_instance = Pubkey::new_unique();
    let user_account = Pubkey::new_unique();
    let user = Pubkey::new_unique();
    let nft_mint = Pubkey::new_unique();
    let nft_token_account = Pubkey::new_unique();
    let nft_metadata = Pubkey::new_unique();
    let token_program = anchor_spl::token::id();
    let metadata_program = anchor_lang::solana_program::program_id!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");

    let accounts = UnstakeNft {
        staking_instance,
        user_account,
        user,
        nft_mint,
        nft_token_account,
        nft_metadata,
        token_program,
        metadata_program,
    };

    let record_index = 0u64;

    let result = program
        .instruction(
            bioneo::instruction::UnstakeNft { record_index },
            &accounts,
        )
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_claim_nft_rewards() {
    let program = anchor_lang::solana_program::program_id!("6iadRi4ps7itomsTNa34RikS6hkmx2z5Ls1h9EqLPu1y");
    let staking_instance = Pubkey::new_unique();
    let user_account = Pubkey::new_unique();
    let user = Pubkey::new_unique();
    let user_reward_account = Pubkey::new_unique();
    let staking_reward_account = Pubkey::new_unique();
    let token_program = anchor_spl::token::id();

    let accounts = ClaimNftRewards {
        staking_instance,
        user_account,
        user,
        user_reward_account,
        staking_reward_account,
        token_program,
    };

    let result = program
        .instruction(bioneo::instruction::ClaimNftRewards, &accounts)
        .await;

    assert!(result.is_ok());
} 