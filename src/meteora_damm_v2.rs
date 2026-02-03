//! CPI module for Meteora DAMM v2
//!
//! Program: cp_amm
//! Program ID: cpamdpZCGKUy5JxQXB4dcpGPiikHawvSWAd6mEn1sGG
//! Instructions: 35

#![allow(clippy::too_many_arguments)]
#![allow(unused)]

use pinocchio::{
    AccountView,
    Address,
    ProgramResult,
    cpi::{invoke_signed, Signer, CpiAccount},
    instruction::{InstructionView, InstructionAccount},
};

/// Program ID
pub const ID: Address = Address::new_from_array(five8_const::decode_32_const("cpamdpZCGKUy5JxQXB4dcpGPiikHawvSWAd6mEn1sGG"));

// ============================================
// Instruction Discriminators
// ============================================
pub const ADD_LIQUIDITY: [u8; 8] = [181, 157, 89, 67, 143, 182, 52, 72];
pub const CLAIM_PARTNER_FEE: [u8; 8] = [97, 206, 39, 105, 94, 94, 126, 148];
pub const CLAIM_POSITION_FEE: [u8; 8] = [180, 38, 154, 17, 133, 33, 162, 211];
pub const CLAIM_PROTOCOL_FEE: [u8; 8] = [165, 228, 133, 48, 99, 249, 255, 33];
pub const CLAIM_REWARD: [u8; 8] = [149, 95, 181, 242, 94, 90, 158, 162];
pub const CLOSE_CONFIG: [u8; 8] = [145, 9, 72, 157, 95, 125, 61, 85];
pub const CLOSE_OPERATOR_ACCOUNT: [u8; 8] = [171, 9, 213, 74, 120, 23, 3, 29];
pub const CLOSE_POSITION: [u8; 8] = [123, 134, 81, 0, 49, 68, 98, 98];
pub const CLOSE_TOKEN_BADGE: [u8; 8] = [108, 146, 86, 110, 179, 254, 10, 104];
pub const CREATE_CONFIG: [u8; 8] = [201, 207, 243, 114, 75, 111, 47, 189];
pub const CREATE_DYNAMIC_CONFIG: [u8; 8] = [81, 251, 122, 78, 66, 57, 208, 82];
pub const CREATE_OPERATOR_ACCOUNT: [u8; 8] = [221, 64, 246, 149, 240, 153, 229, 163];
pub const CREATE_POSITION: [u8; 8] = [48, 215, 197, 153, 96, 203, 180, 133];
pub const CREATE_TOKEN_BADGE: [u8; 8] = [88, 206, 0, 91, 60, 175, 151, 118];
pub const DUMMY_IX: [u8; 8] = [234, 95, 176, 185, 7, 42, 35, 159];
pub const FUND_REWARD: [u8; 8] = [188, 50, 249, 165, 93, 151, 38, 63];
pub const INITIALIZE_CUSTOMIZABLE_POOL: [u8; 8] = [20, 161, 241, 24, 189, 221, 180, 2];
pub const INITIALIZE_POOL: [u8; 8] = [95, 180, 10, 172, 84, 174, 232, 40];
pub const INITIALIZE_POOL_WITH_DYNAMIC_CONFIG: [u8; 8] = [149, 82, 72, 197, 253, 252, 68, 15];
pub const INITIALIZE_REWARD: [u8; 8] = [95, 135, 192, 196, 242, 129, 230, 68];
pub const LOCK_POSITION: [u8; 8] = [227, 62, 2, 252, 247, 10, 171, 185];
pub const PERMANENT_LOCK_POSITION: [u8; 8] = [165, 176, 125, 6, 231, 171, 186, 213];
pub const REFRESH_VESTING: [u8; 8] = [9, 94, 216, 14, 116, 204, 247, 0];
pub const REMOVE_ALL_LIQUIDITY: [u8; 8] = [10, 51, 61, 35, 112, 105, 24, 85];
pub const REMOVE_LIQUIDITY: [u8; 8] = [80, 85, 209, 72, 24, 206, 177, 108];
pub const SET_POOL_STATUS: [u8; 8] = [112, 87, 135, 223, 83, 204, 132, 53];
pub const SPLIT_POSITION: [u8; 8] = [172, 241, 221, 138, 161, 29, 253, 42];
pub const SPLIT_POSITION2: [u8; 8] = [221, 147, 228, 207, 140, 212, 17, 119];
pub const SWAP: [u8; 8] = [248, 198, 158, 145, 225, 117, 135, 200];
pub const SWAP2: [u8; 8] = [65, 75, 63, 76, 235, 91, 91, 136];
pub const UPDATE_POOL_FEES: [u8; 8] = [118, 217, 203, 179, 60, 8, 70, 89];
pub const UPDATE_REWARD_DURATION: [u8; 8] = [138, 174, 196, 169, 213, 235, 254, 107];
pub const UPDATE_REWARD_FUNDER: [u8; 8] = [211, 28, 48, 32, 215, 160, 35, 23];
pub const WITHDRAW_INELIGIBLE_REWARD: [u8; 8] = [148, 206, 42, 195, 247, 49, 103, 8];
pub const ZAP_PROTOCOL_FEE: [u8; 8] = [213, 155, 187, 34, 56, 182, 91, 240];

// ============================================
// Instruction Arguments
// ============================================
/// Arguments for `add_liquidity`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct AddLiquidityArgs {
    pub params: [u8; 32],
}

/// Arguments for `claim_partner_fee`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct ClaimPartnerFeeArgs {
    pub max_amount_a: u64,
    pub max_amount_b: u64,
}

/// Arguments for `claim_protocol_fee`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct ClaimProtocolFeeArgs {
    pub max_amount_a: u64,
    pub max_amount_b: u64,
}

/// Arguments for `claim_reward`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct ClaimRewardArgs {
    pub reward_index: u8,
    pub skip_reward: u8,
}

/// Arguments for `create_config`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct CreateConfigArgs {
    pub index: u64,
    pub config_parameters: [u8; 32],
}

/// Arguments for `create_dynamic_config`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct CreateDynamicConfigArgs {
    pub index: u64,
    pub config_parameters: [u8; 32],
}

/// Arguments for `create_operator_account`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct CreateOperatorAccountArgs {
    pub permission: u128,
}

/// Arguments for `dummy_ix`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct DummyIxArgs {
    pub ixs: [u8; 32],
}

/// Arguments for `fund_reward`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct FundRewardArgs {
    pub reward_index: u8,
    pub amount: u64,
    pub carry_forward: bool,
}

/// Arguments for `initialize_customizable_pool`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct InitializeCustomizablePoolArgs {
    pub params: [u8; 32],
}

/// Arguments for `initialize_pool`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct InitializePoolArgs {
    pub params: [u8; 32],
}

/// Arguments for `initialize_pool_with_dynamic_config`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct InitializePoolWithDynamicConfigArgs {
    pub params: [u8; 32],
}

/// Arguments for `initialize_reward`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct InitializeRewardArgs {
    pub reward_index: u8,
    pub reward_duration: u64,
    pub funder: [u8; 32],
}

/// Arguments for `lock_position`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct LockPositionArgs {
    pub params: [u8; 32],
}

/// Arguments for `permanent_lock_position`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct PermanentLockPositionArgs {
    pub permanent_lock_liquidity: u128,
}

/// Arguments for `remove_all_liquidity`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct RemoveAllLiquidityArgs {
    pub token_a_amount_threshold: u64,
    pub token_b_amount_threshold: u64,
}

/// Arguments for `remove_liquidity`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct RemoveLiquidityArgs {
    pub params: [u8; 32],
}

/// Arguments for `set_pool_status`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SetPoolStatusArgs {
    pub status: u8,
}

/// Arguments for `split_position`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SplitPositionArgs {
    pub params: [u8; 32],
}

/// Arguments for `split_position2`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SplitPosition2Args {
    pub numerator: u32,
}

/// Arguments for `swap`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SwapArgs {
    pub params: [u8; 32],
}

/// Arguments for `swap2`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct Swap2Args {
    pub params: [u8; 32],
}

/// Arguments for `update_pool_fees`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct UpdatePoolFeesArgs {
    pub params: [u8; 32],
}

/// Arguments for `update_reward_duration`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct UpdateRewardDurationArgs {
    pub reward_index: u8,
    pub new_duration: u64,
}

/// Arguments for `update_reward_funder`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct UpdateRewardFunderArgs {
    pub reward_index: u8,
    pub new_funder: [u8; 32],
}

/// Arguments for `withdraw_ineligible_reward`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct WithdrawIneligibleRewardArgs {
    pub reward_index: u8,
}

/// Arguments for `zap_protocol_fee`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct ZapProtocolFeeArgs {
    pub max_amount: u64,
}


// ============================================
// Instruction Accounts
// ============================================
/// Accounts for `add_liquidity`
pub struct AddLiquidityAccounts<'a> {
    /// pool
    pub pool: &'a AccountView,
    /// position
    pub position: &'a AccountView,
    /// The user token a account
    pub token_a_account: &'a AccountView,
    /// The user token b account
    pub token_b_account: &'a AccountView,
    /// The vault token account for input token
    pub token_a_vault: &'a AccountView,
    /// The vault token account for output token
    pub token_b_vault: &'a AccountView,
    /// The mint of token a
    pub token_a_mint: &'a AccountView,
    /// The mint of token b
    pub token_b_mint: &'a AccountView,
    /// The token account for nft
    pub position_nft_account: &'a AccountView,
    /// owner of position
    pub owner: &'a AccountView,
    /// Token a program
    pub token_a_program: &'a AccountView,
    /// Token b program
    pub token_b_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> AddLiquidityAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 14] {
        [
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.position.address()),
            InstructionAccount::writable(self.token_a_account.address()),
            InstructionAccount::writable(self.token_b_account.address()),
            InstructionAccount::writable(self.token_a_vault.address()),
            InstructionAccount::writable(self.token_b_vault.address()),
            InstructionAccount::readonly(self.token_a_mint.address()),
            InstructionAccount::readonly(self.token_b_mint.address()),
            InstructionAccount::readonly(self.position_nft_account.address()),
            InstructionAccount::readonly_signer(self.owner.address()),
            InstructionAccount::readonly(self.token_a_program.address()),
            InstructionAccount::readonly(self.token_b_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 14] {
        [
            self.pool, self.position, self.token_a_account, self.token_b_account, self.token_a_vault, self.token_b_vault, self.token_a_mint, self.token_b_mint, self.position_nft_account, self.owner, self.token_a_program, self.token_b_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `claim_partner_fee`
pub struct ClaimPartnerFeeAccounts<'a> {
    /// pool_authority
    pub pool_authority: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// The treasury token a account
    pub token_a_account: &'a AccountView,
    /// The treasury token b account
    pub token_b_account: &'a AccountView,
    /// The vault token account for input token
    pub token_a_vault: &'a AccountView,
    /// The vault token account for output token
    pub token_b_vault: &'a AccountView,
    /// The mint of token a
    pub token_a_mint: &'a AccountView,
    /// The mint of token b
    pub token_b_mint: &'a AccountView,
    /// partner
    pub partner: &'a AccountView,
    /// Token a program
    pub token_a_program: &'a AccountView,
    /// Token b program
    pub token_b_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> ClaimPartnerFeeAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 13] {
        [
            InstructionAccount::readonly(self.pool_authority.address()),
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.token_a_account.address()),
            InstructionAccount::writable(self.token_b_account.address()),
            InstructionAccount::writable(self.token_a_vault.address()),
            InstructionAccount::writable(self.token_b_vault.address()),
            InstructionAccount::readonly(self.token_a_mint.address()),
            InstructionAccount::readonly(self.token_b_mint.address()),
            InstructionAccount::readonly_signer(self.partner.address()),
            InstructionAccount::readonly(self.token_a_program.address()),
            InstructionAccount::readonly(self.token_b_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 13] {
        [
            self.pool_authority, self.pool, self.token_a_account, self.token_b_account, self.token_a_vault, self.token_b_vault, self.token_a_mint, self.token_b_mint, self.partner, self.token_a_program, self.token_b_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `claim_position_fee`
pub struct ClaimPositionFeeAccounts<'a> {
    /// pool_authority
    pub pool_authority: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// position
    pub position: &'a AccountView,
    /// The user token a account
    pub token_a_account: &'a AccountView,
    /// The user token b account
    pub token_b_account: &'a AccountView,
    /// The vault token account for input token
    pub token_a_vault: &'a AccountView,
    /// The vault token account for output token
    pub token_b_vault: &'a AccountView,
    /// The mint of token a
    pub token_a_mint: &'a AccountView,
    /// The mint of token b
    pub token_b_mint: &'a AccountView,
    /// The token account for nft
    pub position_nft_account: &'a AccountView,
    /// owner of position
    pub owner: &'a AccountView,
    /// Token a program
    pub token_a_program: &'a AccountView,
    /// Token b program
    pub token_b_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> ClaimPositionFeeAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 15] {
        [
            InstructionAccount::readonly(self.pool_authority.address()),
            InstructionAccount::readonly(self.pool.address()),
            InstructionAccount::writable(self.position.address()),
            InstructionAccount::writable(self.token_a_account.address()),
            InstructionAccount::writable(self.token_b_account.address()),
            InstructionAccount::writable(self.token_a_vault.address()),
            InstructionAccount::writable(self.token_b_vault.address()),
            InstructionAccount::readonly(self.token_a_mint.address()),
            InstructionAccount::readonly(self.token_b_mint.address()),
            InstructionAccount::readonly(self.position_nft_account.address()),
            InstructionAccount::readonly_signer(self.owner.address()),
            InstructionAccount::readonly(self.token_a_program.address()),
            InstructionAccount::readonly(self.token_b_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 15] {
        [
            self.pool_authority, self.pool, self.position, self.token_a_account, self.token_b_account, self.token_a_vault, self.token_b_vault, self.token_a_mint, self.token_b_mint, self.position_nft_account, self.owner, self.token_a_program, self.token_b_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `claim_protocol_fee`
pub struct ClaimProtocolFeeAccounts<'a> {
    /// pool_authority
    pub pool_authority: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// The vault token account for input token
    pub token_a_vault: &'a AccountView,
    /// The vault token account for output token
    pub token_b_vault: &'a AccountView,
    /// The mint of token a
    pub token_a_mint: &'a AccountView,
    /// The mint of token b
    pub token_b_mint: &'a AccountView,
    /// token_a_account
    pub token_a_account: &'a AccountView,
    /// token_b_account
    pub token_b_account: &'a AccountView,
    /// Claim fee operator
    pub operator: &'a AccountView,
    /// Operator
    pub signer: &'a AccountView,
    /// Token a program
    pub token_a_program: &'a AccountView,
    /// Token b program
    pub token_b_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> ClaimProtocolFeeAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 14] {
        [
            InstructionAccount::readonly(self.pool_authority.address()),
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.token_a_vault.address()),
            InstructionAccount::writable(self.token_b_vault.address()),
            InstructionAccount::readonly(self.token_a_mint.address()),
            InstructionAccount::readonly(self.token_b_mint.address()),
            InstructionAccount::writable(self.token_a_account.address()),
            InstructionAccount::writable(self.token_b_account.address()),
            InstructionAccount::readonly(self.operator.address()),
            InstructionAccount::readonly_signer(self.signer.address()),
            InstructionAccount::readonly(self.token_a_program.address()),
            InstructionAccount::readonly(self.token_b_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 14] {
        [
            self.pool_authority, self.pool, self.token_a_vault, self.token_b_vault, self.token_a_mint, self.token_b_mint, self.token_a_account, self.token_b_account, self.operator, self.signer, self.token_a_program, self.token_b_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `claim_reward`
pub struct ClaimRewardAccounts<'a> {
    /// pool_authority
    pub pool_authority: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// position
    pub position: &'a AccountView,
    /// The vault token account for reward token
    pub reward_vault: &'a AccountView,
    /// reward_mint
    pub reward_mint: &'a AccountView,
    /// user_token_account
    pub user_token_account: &'a AccountView,
    /// The token account for nft
    pub position_nft_account: &'a AccountView,
    /// owner of position
    pub owner: &'a AccountView,
    /// token_program
    pub token_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> ClaimRewardAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 11] {
        [
            InstructionAccount::readonly(self.pool_authority.address()),
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.position.address()),
            InstructionAccount::writable(self.reward_vault.address()),
            InstructionAccount::readonly(self.reward_mint.address()),
            InstructionAccount::writable(self.user_token_account.address()),
            InstructionAccount::readonly(self.position_nft_account.address()),
            InstructionAccount::readonly_signer(self.owner.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 11] {
        [
            self.pool_authority, self.pool, self.position, self.reward_vault, self.reward_mint, self.user_token_account, self.position_nft_account, self.owner, self.token_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `close_config`
pub struct CloseConfigAccounts<'a> {
    /// config
    pub config: &'a AccountView,
    /// operator
    pub operator: &'a AccountView,
    /// signer
    pub signer: &'a AccountView,
    /// rent_receiver
    pub rent_receiver: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> CloseConfigAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 6] {
        [
            InstructionAccount::writable(self.config.address()),
            InstructionAccount::readonly(self.operator.address()),
            InstructionAccount::readonly_signer(self.signer.address()),
            InstructionAccount::writable(self.rent_receiver.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 6] {
        [
            self.config, self.operator, self.signer, self.rent_receiver, self.event_authority, self.program
        ]
    }
}

/// Accounts for `close_operator_account`
pub struct CloseOperatorAccountAccounts<'a> {
    /// operator
    pub operator: &'a AccountView,
    /// signer
    pub signer: &'a AccountView,
    /// rent_receiver
    pub rent_receiver: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> CloseOperatorAccountAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 5] {
        [
            InstructionAccount::writable(self.operator.address()),
            InstructionAccount::readonly_signer(self.signer.address()),
            InstructionAccount::writable(self.rent_receiver.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 5] {
        [
            self.operator, self.signer, self.rent_receiver, self.event_authority, self.program
        ]
    }
}

/// Accounts for `close_position`
pub struct ClosePositionAccounts<'a> {
    /// position_nft_mint
    pub position_nft_mint: &'a AccountView,
    /// The token account for nft
    pub position_nft_account: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// position
    pub position: &'a AccountView,
    /// pool_authority
    pub pool_authority: &'a AccountView,
    /// rent_receiver
    pub rent_receiver: &'a AccountView,
    /// Owner of position
    pub owner: &'a AccountView,
    /// Program to create NFT mint/token account and transfer for token22 account
    pub token_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> ClosePositionAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 10] {
        [
            InstructionAccount::writable(self.position_nft_mint.address()),
            InstructionAccount::writable(self.position_nft_account.address()),
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.position.address()),
            InstructionAccount::readonly(self.pool_authority.address()),
            InstructionAccount::writable(self.rent_receiver.address()),
            InstructionAccount::readonly_signer(self.owner.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 10] {
        [
            self.position_nft_mint, self.position_nft_account, self.pool, self.position, self.pool_authority, self.rent_receiver, self.owner, self.token_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `close_token_badge`
pub struct CloseTokenBadgeAccounts<'a> {
    /// token_badge
    pub token_badge: &'a AccountView,
    /// operator
    pub operator: &'a AccountView,
    /// signer
    pub signer: &'a AccountView,
    /// rent_receiver
    pub rent_receiver: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> CloseTokenBadgeAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 6] {
        [
            InstructionAccount::writable(self.token_badge.address()),
            InstructionAccount::readonly(self.operator.address()),
            InstructionAccount::readonly_signer(self.signer.address()),
            InstructionAccount::writable(self.rent_receiver.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 6] {
        [
            self.token_badge, self.operator, self.signer, self.rent_receiver, self.event_authority, self.program
        ]
    }
}

/// Accounts for `create_config`
pub struct CreateConfigAccounts<'a> {
    /// config
    pub config: &'a AccountView,
    /// operator
    pub operator: &'a AccountView,
    /// signer
    pub signer: &'a AccountView,
    /// payer
    pub payer: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> CreateConfigAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 7] {
        [
            InstructionAccount::writable(self.config.address()),
            InstructionAccount::readonly(self.operator.address()),
            InstructionAccount::readonly_signer(self.signer.address()),
            InstructionAccount::writable_signer(self.payer.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 7] {
        [
            self.config, self.operator, self.signer, self.payer, self.system_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `create_dynamic_config`
pub struct CreateDynamicConfigAccounts<'a> {
    /// config
    pub config: &'a AccountView,
    /// operator
    pub operator: &'a AccountView,
    /// signer
    pub signer: &'a AccountView,
    /// payer
    pub payer: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> CreateDynamicConfigAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 7] {
        [
            InstructionAccount::writable(self.config.address()),
            InstructionAccount::readonly(self.operator.address()),
            InstructionAccount::readonly_signer(self.signer.address()),
            InstructionAccount::writable_signer(self.payer.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 7] {
        [
            self.config, self.operator, self.signer, self.payer, self.system_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `create_operator_account`
pub struct CreateOperatorAccountAccounts<'a> {
    /// operator
    pub operator: &'a AccountView,
    /// whitelisted_address
    pub whitelisted_address: &'a AccountView,
    /// signer
    pub signer: &'a AccountView,
    /// payer
    pub payer: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> CreateOperatorAccountAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 7] {
        [
            InstructionAccount::writable(self.operator.address()),
            InstructionAccount::readonly(self.whitelisted_address.address()),
            InstructionAccount::readonly_signer(self.signer.address()),
            InstructionAccount::writable_signer(self.payer.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 7] {
        [
            self.operator, self.whitelisted_address, self.signer, self.payer, self.system_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `create_position`
pub struct CreatePositionAccounts<'a> {
    /// owner
    pub owner: &'a AccountView,
    /// position_nft_mint
    pub position_nft_mint: &'a AccountView,
    /// position nft account
    pub position_nft_account: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// position
    pub position: &'a AccountView,
    /// pool_authority
    pub pool_authority: &'a AccountView,
    /// Address paying to create the position. Can be anyone
    pub payer: &'a AccountView,
    /// Program to create NFT mint/token account and transfer for token22 account
    pub token_program: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> CreatePositionAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 11] {
        [
            InstructionAccount::readonly(self.owner.address()),
            InstructionAccount::writable_signer(self.position_nft_mint.address()),
            InstructionAccount::writable(self.position_nft_account.address()),
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.position.address()),
            InstructionAccount::readonly(self.pool_authority.address()),
            InstructionAccount::writable_signer(self.payer.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 11] {
        [
            self.owner, self.position_nft_mint, self.position_nft_account, self.pool, self.position, self.pool_authority, self.payer, self.token_program, self.system_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `create_token_badge`
pub struct CreateTokenBadgeAccounts<'a> {
    /// token_badge
    pub token_badge: &'a AccountView,
    /// token_mint
    pub token_mint: &'a AccountView,
    /// operator
    pub operator: &'a AccountView,
    /// signer
    pub signer: &'a AccountView,
    /// payer
    pub payer: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> CreateTokenBadgeAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 8] {
        [
            InstructionAccount::writable(self.token_badge.address()),
            InstructionAccount::readonly(self.token_mint.address()),
            InstructionAccount::readonly(self.operator.address()),
            InstructionAccount::readonly_signer(self.signer.address()),
            InstructionAccount::writable_signer(self.payer.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 8] {
        [
            self.token_badge, self.token_mint, self.operator, self.signer, self.payer, self.system_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `dummy_ix`
pub struct DummyIxAccounts<'a> {
    /// pod_aligned_fee_time_scheduler
    pub pod_aligned_fee_time_scheduler: &'a AccountView,
    /// pod_aligned_fee_rate_limiter
    pub pod_aligned_fee_rate_limiter: &'a AccountView,
    /// pod_aligned_fee_market_cap_scheduler
    pub pod_aligned_fee_market_cap_scheduler: &'a AccountView,
}

impl<'a> DummyIxAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 3] {
        [
            InstructionAccount::readonly(self.pod_aligned_fee_time_scheduler.address()),
            InstructionAccount::readonly(self.pod_aligned_fee_rate_limiter.address()),
            InstructionAccount::readonly(self.pod_aligned_fee_market_cap_scheduler.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 3] {
        [
            self.pod_aligned_fee_time_scheduler, self.pod_aligned_fee_rate_limiter, self.pod_aligned_fee_market_cap_scheduler
        ]
    }
}

/// Accounts for `fund_reward`
pub struct FundRewardAccounts<'a> {
    /// pool
    pub pool: &'a AccountView,
    /// reward_vault
    pub reward_vault: &'a AccountView,
    /// reward_mint
    pub reward_mint: &'a AccountView,
    /// funder_token_account
    pub funder_token_account: &'a AccountView,
    /// funder
    pub funder: &'a AccountView,
    /// token_program
    pub token_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> FundRewardAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 8] {
        [
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.reward_vault.address()),
            InstructionAccount::readonly(self.reward_mint.address()),
            InstructionAccount::writable(self.funder_token_account.address()),
            InstructionAccount::readonly_signer(self.funder.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 8] {
        [
            self.pool, self.reward_vault, self.reward_mint, self.funder_token_account, self.funder, self.token_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `initialize_customizable_pool`
pub struct InitializeCustomizablePoolAccounts<'a> {
    /// creator
    pub creator: &'a AccountView,
    /// position_nft_mint
    pub position_nft_mint: &'a AccountView,
    /// position nft account
    pub position_nft_account: &'a AccountView,
    /// Address paying to create the pool. Can be anyone
    pub payer: &'a AccountView,
    /// pool_authority
    pub pool_authority: &'a AccountView,
    /// Initialize an account to store the pool state
    pub pool: &'a AccountView,
    /// position
    pub position: &'a AccountView,
    /// Token a mint
    pub token_a_mint: &'a AccountView,
    /// Token b mint
    pub token_b_mint: &'a AccountView,
    /// Token a vault for the pool
    pub token_a_vault: &'a AccountView,
    /// Token b vault for the pool
    pub token_b_vault: &'a AccountView,
    /// payer token a account
    pub payer_token_a: &'a AccountView,
    /// creator token b account
    pub payer_token_b: &'a AccountView,
    /// Program to create mint account and mint tokens
    pub token_a_program: &'a AccountView,
    /// Program to create mint account and mint tokens
    pub token_b_program: &'a AccountView,
    /// Program to create NFT mint/token account and transfer for token22 account
    pub token_2022_program: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> InitializeCustomizablePoolAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 19] {
        [
            InstructionAccount::readonly(self.creator.address()),
            InstructionAccount::writable_signer(self.position_nft_mint.address()),
            InstructionAccount::writable(self.position_nft_account.address()),
            InstructionAccount::writable_signer(self.payer.address()),
            InstructionAccount::readonly(self.pool_authority.address()),
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.position.address()),
            InstructionAccount::readonly(self.token_a_mint.address()),
            InstructionAccount::readonly(self.token_b_mint.address()),
            InstructionAccount::writable(self.token_a_vault.address()),
            InstructionAccount::writable(self.token_b_vault.address()),
            InstructionAccount::writable(self.payer_token_a.address()),
            InstructionAccount::writable(self.payer_token_b.address()),
            InstructionAccount::readonly(self.token_a_program.address()),
            InstructionAccount::readonly(self.token_b_program.address()),
            InstructionAccount::readonly(self.token_2022_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 19] {
        [
            self.creator, self.position_nft_mint, self.position_nft_account, self.payer, self.pool_authority, self.pool, self.position, self.token_a_mint, self.token_b_mint, self.token_a_vault, self.token_b_vault, self.payer_token_a, self.payer_token_b, self.token_a_program, self.token_b_program, self.token_2022_program, self.system_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `initialize_pool`
pub struct InitializePoolAccounts<'a> {
    /// creator
    pub creator: &'a AccountView,
    /// position_nft_mint
    pub position_nft_mint: &'a AccountView,
    /// position nft account
    pub position_nft_account: &'a AccountView,
    /// Address paying to create the pool. Can be anyone
    pub payer: &'a AccountView,
    /// Which config the pool belongs to.
    pub config: &'a AccountView,
    /// pool_authority
    pub pool_authority: &'a AccountView,
    /// Initialize an account to store the pool state
    pub pool: &'a AccountView,
    /// position
    pub position: &'a AccountView,
    /// Token a mint
    pub token_a_mint: &'a AccountView,
    /// Token b mint
    pub token_b_mint: &'a AccountView,
    /// Token a vault for the pool
    pub token_a_vault: &'a AccountView,
    /// Token b vault for the pool
    pub token_b_vault: &'a AccountView,
    /// payer token a account
    pub payer_token_a: &'a AccountView,
    /// creator token b account
    pub payer_token_b: &'a AccountView,
    /// Program to create mint account and mint tokens
    pub token_a_program: &'a AccountView,
    /// Program to create mint account and mint tokens
    pub token_b_program: &'a AccountView,
    /// Program to create NFT mint/token account and transfer for token22 account
    pub token_2022_program: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> InitializePoolAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 20] {
        [
            InstructionAccount::readonly(self.creator.address()),
            InstructionAccount::writable_signer(self.position_nft_mint.address()),
            InstructionAccount::writable(self.position_nft_account.address()),
            InstructionAccount::writable_signer(self.payer.address()),
            InstructionAccount::readonly(self.config.address()),
            InstructionAccount::readonly(self.pool_authority.address()),
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.position.address()),
            InstructionAccount::readonly(self.token_a_mint.address()),
            InstructionAccount::readonly(self.token_b_mint.address()),
            InstructionAccount::writable(self.token_a_vault.address()),
            InstructionAccount::writable(self.token_b_vault.address()),
            InstructionAccount::writable(self.payer_token_a.address()),
            InstructionAccount::writable(self.payer_token_b.address()),
            InstructionAccount::readonly(self.token_a_program.address()),
            InstructionAccount::readonly(self.token_b_program.address()),
            InstructionAccount::readonly(self.token_2022_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 20] {
        [
            self.creator, self.position_nft_mint, self.position_nft_account, self.payer, self.config, self.pool_authority, self.pool, self.position, self.token_a_mint, self.token_b_mint, self.token_a_vault, self.token_b_vault, self.payer_token_a, self.payer_token_b, self.token_a_program, self.token_b_program, self.token_2022_program, self.system_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `initialize_pool_with_dynamic_config`
pub struct InitializePoolWithDynamicConfigAccounts<'a> {
    /// creator
    pub creator: &'a AccountView,
    /// position_nft_mint
    pub position_nft_mint: &'a AccountView,
    /// position nft account
    pub position_nft_account: &'a AccountView,
    /// Address paying to create the pool. Can be anyone
    pub payer: &'a AccountView,
    /// pool_creator_authority
    pub pool_creator_authority: &'a AccountView,
    /// Which config the pool belongs to.
    pub config: &'a AccountView,
    /// pool_authority
    pub pool_authority: &'a AccountView,
    /// Initialize an account to store the pool state
    pub pool: &'a AccountView,
    /// position
    pub position: &'a AccountView,
    /// Token a mint
    pub token_a_mint: &'a AccountView,
    /// Token b mint
    pub token_b_mint: &'a AccountView,
    /// Token a vault for the pool
    pub token_a_vault: &'a AccountView,
    /// Token b vault for the pool
    pub token_b_vault: &'a AccountView,
    /// payer token a account
    pub payer_token_a: &'a AccountView,
    /// creator token b account
    pub payer_token_b: &'a AccountView,
    /// Program to create mint account and mint tokens
    pub token_a_program: &'a AccountView,
    /// Program to create mint account and mint tokens
    pub token_b_program: &'a AccountView,
    /// Program to create NFT mint/token account and transfer for token22 account
    pub token_2022_program: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> InitializePoolWithDynamicConfigAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 21] {
        [
            InstructionAccount::readonly(self.creator.address()),
            InstructionAccount::writable_signer(self.position_nft_mint.address()),
            InstructionAccount::writable(self.position_nft_account.address()),
            InstructionAccount::writable_signer(self.payer.address()),
            InstructionAccount::readonly_signer(self.pool_creator_authority.address()),
            InstructionAccount::readonly(self.config.address()),
            InstructionAccount::readonly(self.pool_authority.address()),
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.position.address()),
            InstructionAccount::readonly(self.token_a_mint.address()),
            InstructionAccount::readonly(self.token_b_mint.address()),
            InstructionAccount::writable(self.token_a_vault.address()),
            InstructionAccount::writable(self.token_b_vault.address()),
            InstructionAccount::writable(self.payer_token_a.address()),
            InstructionAccount::writable(self.payer_token_b.address()),
            InstructionAccount::readonly(self.token_a_program.address()),
            InstructionAccount::readonly(self.token_b_program.address()),
            InstructionAccount::readonly(self.token_2022_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 21] {
        [
            self.creator, self.position_nft_mint, self.position_nft_account, self.payer, self.pool_creator_authority, self.config, self.pool_authority, self.pool, self.position, self.token_a_mint, self.token_b_mint, self.token_a_vault, self.token_b_vault, self.payer_token_a, self.payer_token_b, self.token_a_program, self.token_b_program, self.token_2022_program, self.system_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `initialize_reward`
pub struct InitializeRewardAccounts<'a> {
    /// pool_authority
    pub pool_authority: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// reward_vault
    pub reward_vault: &'a AccountView,
    /// reward_mint
    pub reward_mint: &'a AccountView,
    /// signer
    pub signer: &'a AccountView,
    /// payer
    pub payer: &'a AccountView,
    /// token_program
    pub token_program: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> InitializeRewardAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 10] {
        [
            InstructionAccount::readonly(self.pool_authority.address()),
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.reward_vault.address()),
            InstructionAccount::readonly(self.reward_mint.address()),
            InstructionAccount::readonly_signer(self.signer.address()),
            InstructionAccount::writable_signer(self.payer.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 10] {
        [
            self.pool_authority, self.pool, self.reward_vault, self.reward_mint, self.signer, self.payer, self.token_program, self.system_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `lock_position`
pub struct LockPositionAccounts<'a> {
    /// pool
    pub pool: &'a AccountView,
    /// position
    pub position: &'a AccountView,
    /// vesting
    pub vesting: &'a AccountView,
    /// The token account for nft
    pub position_nft_account: &'a AccountView,
    /// owner of position
    pub owner: &'a AccountView,
    /// payer
    pub payer: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> LockPositionAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 9] {
        [
            InstructionAccount::readonly(self.pool.address()),
            InstructionAccount::writable(self.position.address()),
            InstructionAccount::writable_signer(self.vesting.address()),
            InstructionAccount::readonly(self.position_nft_account.address()),
            InstructionAccount::readonly_signer(self.owner.address()),
            InstructionAccount::writable_signer(self.payer.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 9] {
        [
            self.pool, self.position, self.vesting, self.position_nft_account, self.owner, self.payer, self.system_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `permanent_lock_position`
pub struct PermanentLockPositionAccounts<'a> {
    /// pool
    pub pool: &'a AccountView,
    /// position
    pub position: &'a AccountView,
    /// The token account for nft
    pub position_nft_account: &'a AccountView,
    /// owner of position
    pub owner: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> PermanentLockPositionAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 6] {
        [
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.position.address()),
            InstructionAccount::readonly(self.position_nft_account.address()),
            InstructionAccount::readonly_signer(self.owner.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 6] {
        [
            self.pool, self.position, self.position_nft_account, self.owner, self.event_authority, self.program
        ]
    }
}

/// Accounts for `refresh_vesting`
pub struct RefreshVestingAccounts<'a> {
    /// pool
    pub pool: &'a AccountView,
    /// position
    pub position: &'a AccountView,
    /// The token account for nft
    pub position_nft_account: &'a AccountView,
    /// owner
    pub owner: &'a AccountView,
}

impl<'a> RefreshVestingAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 4] {
        [
            InstructionAccount::readonly(self.pool.address()),
            InstructionAccount::writable(self.position.address()),
            InstructionAccount::readonly(self.position_nft_account.address()),
            InstructionAccount::readonly(self.owner.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 4] {
        [
            self.pool, self.position, self.position_nft_account, self.owner
        ]
    }
}

/// Accounts for `remove_all_liquidity`
pub struct RemoveAllLiquidityAccounts<'a> {
    /// pool_authority
    pub pool_authority: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// position
    pub position: &'a AccountView,
    /// The user token a account
    pub token_a_account: &'a AccountView,
    /// The user token b account
    pub token_b_account: &'a AccountView,
    /// The vault token account for input token
    pub token_a_vault: &'a AccountView,
    /// The vault token account for output token
    pub token_b_vault: &'a AccountView,
    /// The mint of token a
    pub token_a_mint: &'a AccountView,
    /// The mint of token b
    pub token_b_mint: &'a AccountView,
    /// The token account for nft
    pub position_nft_account: &'a AccountView,
    /// owner of position
    pub owner: &'a AccountView,
    /// Token a program
    pub token_a_program: &'a AccountView,
    /// Token b program
    pub token_b_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> RemoveAllLiquidityAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 15] {
        [
            InstructionAccount::readonly(self.pool_authority.address()),
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.position.address()),
            InstructionAccount::writable(self.token_a_account.address()),
            InstructionAccount::writable(self.token_b_account.address()),
            InstructionAccount::writable(self.token_a_vault.address()),
            InstructionAccount::writable(self.token_b_vault.address()),
            InstructionAccount::readonly(self.token_a_mint.address()),
            InstructionAccount::readonly(self.token_b_mint.address()),
            InstructionAccount::readonly(self.position_nft_account.address()),
            InstructionAccount::readonly_signer(self.owner.address()),
            InstructionAccount::readonly(self.token_a_program.address()),
            InstructionAccount::readonly(self.token_b_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 15] {
        [
            self.pool_authority, self.pool, self.position, self.token_a_account, self.token_b_account, self.token_a_vault, self.token_b_vault, self.token_a_mint, self.token_b_mint, self.position_nft_account, self.owner, self.token_a_program, self.token_b_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `remove_liquidity`
pub struct RemoveLiquidityAccounts<'a> {
    /// pool_authority
    pub pool_authority: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// position
    pub position: &'a AccountView,
    /// The user token a account
    pub token_a_account: &'a AccountView,
    /// The user token b account
    pub token_b_account: &'a AccountView,
    /// The vault token account for input token
    pub token_a_vault: &'a AccountView,
    /// The vault token account for output token
    pub token_b_vault: &'a AccountView,
    /// The mint of token a
    pub token_a_mint: &'a AccountView,
    /// The mint of token b
    pub token_b_mint: &'a AccountView,
    /// The token account for nft
    pub position_nft_account: &'a AccountView,
    /// owner of position
    pub owner: &'a AccountView,
    /// Token a program
    pub token_a_program: &'a AccountView,
    /// Token b program
    pub token_b_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> RemoveLiquidityAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 15] {
        [
            InstructionAccount::readonly(self.pool_authority.address()),
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.position.address()),
            InstructionAccount::writable(self.token_a_account.address()),
            InstructionAccount::writable(self.token_b_account.address()),
            InstructionAccount::writable(self.token_a_vault.address()),
            InstructionAccount::writable(self.token_b_vault.address()),
            InstructionAccount::readonly(self.token_a_mint.address()),
            InstructionAccount::readonly(self.token_b_mint.address()),
            InstructionAccount::readonly(self.position_nft_account.address()),
            InstructionAccount::readonly_signer(self.owner.address()),
            InstructionAccount::readonly(self.token_a_program.address()),
            InstructionAccount::readonly(self.token_b_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 15] {
        [
            self.pool_authority, self.pool, self.position, self.token_a_account, self.token_b_account, self.token_a_vault, self.token_b_vault, self.token_a_mint, self.token_b_mint, self.position_nft_account, self.owner, self.token_a_program, self.token_b_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `set_pool_status`
pub struct SetPoolStatusAccounts<'a> {
    /// pool
    pub pool: &'a AccountView,
    /// operator
    pub operator: &'a AccountView,
    /// signer
    pub signer: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> SetPoolStatusAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 5] {
        [
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::readonly(self.operator.address()),
            InstructionAccount::readonly_signer(self.signer.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 5] {
        [
            self.pool, self.operator, self.signer, self.event_authority, self.program
        ]
    }
}

/// Accounts for `split_position`
pub struct SplitPositionAccounts<'a> {
    /// pool
    pub pool: &'a AccountView,
    /// The first position
    pub first_position: &'a AccountView,
    /// The token account for position nft
    pub first_position_nft_account: &'a AccountView,
    /// The second position
    pub second_position: &'a AccountView,
    /// The token account for position nft
    pub second_position_nft_account: &'a AccountView,
    /// Owner of first position
    pub first_owner: &'a AccountView,
    /// Owner of second position
    pub second_owner: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> SplitPositionAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 9] {
        [
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.first_position.address()),
            InstructionAccount::readonly(self.first_position_nft_account.address()),
            InstructionAccount::writable(self.second_position.address()),
            InstructionAccount::readonly(self.second_position_nft_account.address()),
            InstructionAccount::readonly_signer(self.first_owner.address()),
            InstructionAccount::readonly_signer(self.second_owner.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 9] {
        [
            self.pool, self.first_position, self.first_position_nft_account, self.second_position, self.second_position_nft_account, self.first_owner, self.second_owner, self.event_authority, self.program
        ]
    }
}

/// Accounts for `split_position2`
pub struct SplitPosition2Accounts<'a> {
    /// pool
    pub pool: &'a AccountView,
    /// The first position
    pub first_position: &'a AccountView,
    /// The token account for position nft
    pub first_position_nft_account: &'a AccountView,
    /// The second position
    pub second_position: &'a AccountView,
    /// The token account for position nft
    pub second_position_nft_account: &'a AccountView,
    /// Owner of first position
    pub first_owner: &'a AccountView,
    /// Owner of second position
    pub second_owner: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> SplitPosition2Accounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 9] {
        [
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.first_position.address()),
            InstructionAccount::readonly(self.first_position_nft_account.address()),
            InstructionAccount::writable(self.second_position.address()),
            InstructionAccount::readonly(self.second_position_nft_account.address()),
            InstructionAccount::readonly_signer(self.first_owner.address()),
            InstructionAccount::readonly_signer(self.second_owner.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 9] {
        [
            self.pool, self.first_position, self.first_position_nft_account, self.second_position, self.second_position_nft_account, self.first_owner, self.second_owner, self.event_authority, self.program
        ]
    }
}

/// Accounts for `swap`
pub struct SwapAccounts<'a> {
    /// pool_authority
    pub pool_authority: &'a AccountView,
    /// Pool account
    pub pool: &'a AccountView,
    /// The user token account for input token
    pub input_token_account: &'a AccountView,
    /// The user token account for output token
    pub output_token_account: &'a AccountView,
    /// The vault token account for input token
    pub token_a_vault: &'a AccountView,
    /// The vault token account for output token
    pub token_b_vault: &'a AccountView,
    /// The mint of token a
    pub token_a_mint: &'a AccountView,
    /// The mint of token b
    pub token_b_mint: &'a AccountView,
    /// The user performing the swap
    pub payer: &'a AccountView,
    /// Token a program
    pub token_a_program: &'a AccountView,
    /// Token b program
    pub token_b_program: &'a AccountView,
    /// referral token account
    pub referral_token_account: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> SwapAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 14] {
        [
            InstructionAccount::readonly(self.pool_authority.address()),
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.input_token_account.address()),
            InstructionAccount::writable(self.output_token_account.address()),
            InstructionAccount::writable(self.token_a_vault.address()),
            InstructionAccount::writable(self.token_b_vault.address()),
            InstructionAccount::readonly(self.token_a_mint.address()),
            InstructionAccount::readonly(self.token_b_mint.address()),
            InstructionAccount::readonly_signer(self.payer.address()),
            InstructionAccount::readonly(self.token_a_program.address()),
            InstructionAccount::readonly(self.token_b_program.address()),
            InstructionAccount::writable(self.referral_token_account.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 14] {
        [
            self.pool_authority, self.pool, self.input_token_account, self.output_token_account, self.token_a_vault, self.token_b_vault, self.token_a_mint, self.token_b_mint, self.payer, self.token_a_program, self.token_b_program, self.referral_token_account, self.event_authority, self.program
        ]
    }
}

/// Accounts for `swap2`
pub struct Swap2Accounts<'a> {
    /// pool_authority
    pub pool_authority: &'a AccountView,
    /// Pool account
    pub pool: &'a AccountView,
    /// The user token account for input token
    pub input_token_account: &'a AccountView,
    /// The user token account for output token
    pub output_token_account: &'a AccountView,
    /// The vault token account for input token
    pub token_a_vault: &'a AccountView,
    /// The vault token account for output token
    pub token_b_vault: &'a AccountView,
    /// The mint of token a
    pub token_a_mint: &'a AccountView,
    /// The mint of token b
    pub token_b_mint: &'a AccountView,
    /// The user performing the swap
    pub payer: &'a AccountView,
    /// Token a program
    pub token_a_program: &'a AccountView,
    /// Token b program
    pub token_b_program: &'a AccountView,
    /// referral token account
    pub referral_token_account: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> Swap2Accounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 14] {
        [
            InstructionAccount::readonly(self.pool_authority.address()),
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.input_token_account.address()),
            InstructionAccount::writable(self.output_token_account.address()),
            InstructionAccount::writable(self.token_a_vault.address()),
            InstructionAccount::writable(self.token_b_vault.address()),
            InstructionAccount::readonly(self.token_a_mint.address()),
            InstructionAccount::readonly(self.token_b_mint.address()),
            InstructionAccount::readonly_signer(self.payer.address()),
            InstructionAccount::readonly(self.token_a_program.address()),
            InstructionAccount::readonly(self.token_b_program.address()),
            InstructionAccount::writable(self.referral_token_account.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 14] {
        [
            self.pool_authority, self.pool, self.input_token_account, self.output_token_account, self.token_a_vault, self.token_b_vault, self.token_a_mint, self.token_b_mint, self.payer, self.token_a_program, self.token_b_program, self.referral_token_account, self.event_authority, self.program
        ]
    }
}

/// Accounts for `update_pool_fees`
pub struct UpdatePoolFeesAccounts<'a> {
    /// pool
    pub pool: &'a AccountView,
    /// operator
    pub operator: &'a AccountView,
    /// signer
    pub signer: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> UpdatePoolFeesAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 5] {
        [
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::readonly(self.operator.address()),
            InstructionAccount::readonly_signer(self.signer.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 5] {
        [
            self.pool, self.operator, self.signer, self.event_authority, self.program
        ]
    }
}

/// Accounts for `update_reward_duration`
pub struct UpdateRewardDurationAccounts<'a> {
    /// pool
    pub pool: &'a AccountView,
    /// signer
    pub signer: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> UpdateRewardDurationAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 4] {
        [
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::readonly_signer(self.signer.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 4] {
        [
            self.pool, self.signer, self.event_authority, self.program
        ]
    }
}

/// Accounts for `update_reward_funder`
pub struct UpdateRewardFunderAccounts<'a> {
    /// pool
    pub pool: &'a AccountView,
    /// signer
    pub signer: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> UpdateRewardFunderAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 4] {
        [
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::readonly_signer(self.signer.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 4] {
        [
            self.pool, self.signer, self.event_authority, self.program
        ]
    }
}

/// Accounts for `withdraw_ineligible_reward`
pub struct WithdrawIneligibleRewardAccounts<'a> {
    /// pool_authority
    pub pool_authority: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// reward_vault
    pub reward_vault: &'a AccountView,
    /// reward_mint
    pub reward_mint: &'a AccountView,
    /// funder_token_account
    pub funder_token_account: &'a AccountView,
    /// funder
    pub funder: &'a AccountView,
    /// token_program
    pub token_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> WithdrawIneligibleRewardAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 9] {
        [
            InstructionAccount::readonly(self.pool_authority.address()),
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.reward_vault.address()),
            InstructionAccount::readonly(self.reward_mint.address()),
            InstructionAccount::writable(self.funder_token_account.address()),
            InstructionAccount::readonly_signer(self.funder.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 9] {
        [
            self.pool_authority, self.pool, self.reward_vault, self.reward_mint, self.funder_token_account, self.funder, self.token_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `zap_protocol_fee`
pub struct ZapProtocolFeeAccounts<'a> {
    /// pool_authority
    pub pool_authority: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// token_vault
    pub token_vault: &'a AccountView,
    /// token_mint
    pub token_mint: &'a AccountView,
    /// receiver_token
    pub receiver_token: &'a AccountView,
    /// zap claim fee operator
    pub operator: &'a AccountView,
    /// Operator
    pub signer: &'a AccountView,
    /// Token program
    pub token_program: &'a AccountView,
    /// sysvar_instructions
    pub sysvar_instructions: &'a AccountView,
}

impl<'a> ZapProtocolFeeAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 9] {
        [
            InstructionAccount::readonly(self.pool_authority.address()),
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.token_vault.address()),
            InstructionAccount::readonly(self.token_mint.address()),
            InstructionAccount::writable(self.receiver_token.address()),
            InstructionAccount::readonly(self.operator.address()),
            InstructionAccount::readonly_signer(self.signer.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.sysvar_instructions.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 9] {
        [
            self.pool_authority, self.pool, self.token_vault, self.token_mint, self.receiver_token, self.operator, self.signer, self.token_program, self.sysvar_instructions
        ]
    }
}


// ============================================
// CPI Functions
// ============================================
/// CPI: add_liquidity
#[inline(always)]
pub fn add_liquidity<'a>(
    accounts: &AddLiquidityAccounts<'a>, args: &AddLiquidityArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<AddLiquidityArgs>()];
    data[0..8].copy_from_slice(&ADD_LIQUIDITY);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const AddLiquidityArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<AddLiquidityArgs>(),
            );
        }
    
    // Build InstructionAccount array
    let instruction_accounts = accounts.to_instruction_accounts();
    
    // Build InstructionView
    let instruction = InstructionView {
        program_id: &ID,
        accounts: &instruction_accounts,
        data: &data,
    };
    
    // Execute CPI
    let account_views = accounts.to_views();
    invoke_signed::<14>(&instruction, &account_views, signers)
}

/// CPI: claim_partner_fee
#[inline(always)]
pub fn claim_partner_fee<'a>(
    accounts: &ClaimPartnerFeeAccounts<'a>, args: &ClaimPartnerFeeArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<ClaimPartnerFeeArgs>()];
    data[0..8].copy_from_slice(&CLAIM_PARTNER_FEE);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const ClaimPartnerFeeArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<ClaimPartnerFeeArgs>(),
            );
        }
    
    // Build InstructionAccount array
    let instruction_accounts = accounts.to_instruction_accounts();
    
    // Build InstructionView
    let instruction = InstructionView {
        program_id: &ID,
        accounts: &instruction_accounts,
        data: &data,
    };
    
    // Execute CPI
    let account_views = accounts.to_views();
    invoke_signed::<13>(&instruction, &account_views, signers)
}

/// CPI: claim_position_fee
#[inline(always)]
pub fn claim_position_fee<'a>(
    accounts: &ClaimPositionFeeAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&CLAIM_POSITION_FEE);
    
    
    // Build InstructionAccount array
    let instruction_accounts = accounts.to_instruction_accounts();
    
    // Build InstructionView
    let instruction = InstructionView {
        program_id: &ID,
        accounts: &instruction_accounts,
        data: &data,
    };
    
    // Execute CPI
    let account_views = accounts.to_views();
    invoke_signed::<15>(&instruction, &account_views, signers)
}

/// CPI: claim_protocol_fee
#[inline(always)]
pub fn claim_protocol_fee<'a>(
    accounts: &ClaimProtocolFeeAccounts<'a>, args: &ClaimProtocolFeeArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<ClaimProtocolFeeArgs>()];
    data[0..8].copy_from_slice(&CLAIM_PROTOCOL_FEE);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const ClaimProtocolFeeArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<ClaimProtocolFeeArgs>(),
            );
        }
    
    // Build InstructionAccount array
    let instruction_accounts = accounts.to_instruction_accounts();
    
    // Build InstructionView
    let instruction = InstructionView {
        program_id: &ID,
        accounts: &instruction_accounts,
        data: &data,
    };
    
    // Execute CPI
    let account_views = accounts.to_views();
    invoke_signed::<14>(&instruction, &account_views, signers)
}

/// CPI: claim_reward
#[inline(always)]
pub fn claim_reward<'a>(
    accounts: &ClaimRewardAccounts<'a>, args: &ClaimRewardArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<ClaimRewardArgs>()];
    data[0..8].copy_from_slice(&CLAIM_REWARD);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const ClaimRewardArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<ClaimRewardArgs>(),
            );
        }
    
    // Build InstructionAccount array
    let instruction_accounts = accounts.to_instruction_accounts();
    
    // Build InstructionView
    let instruction = InstructionView {
        program_id: &ID,
        accounts: &instruction_accounts,
        data: &data,
    };
    
    // Execute CPI
    let account_views = accounts.to_views();
    invoke_signed::<11>(&instruction, &account_views, signers)
}

/// CPI: close_config
#[inline(always)]
pub fn close_config<'a>(
    accounts: &CloseConfigAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&CLOSE_CONFIG);
    
    
    // Build InstructionAccount array
    let instruction_accounts = accounts.to_instruction_accounts();
    
    // Build InstructionView
    let instruction = InstructionView {
        program_id: &ID,
        accounts: &instruction_accounts,
        data: &data,
    };
    
    // Execute CPI
    let account_views = accounts.to_views();
    invoke_signed::<6>(&instruction, &account_views, signers)
}

/// CPI: close_operator_account
#[inline(always)]
pub fn close_operator_account<'a>(
    accounts: &CloseOperatorAccountAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&CLOSE_OPERATOR_ACCOUNT);
    
    
    // Build InstructionAccount array
    let instruction_accounts = accounts.to_instruction_accounts();
    
    // Build InstructionView
    let instruction = InstructionView {
        program_id: &ID,
        accounts: &instruction_accounts,
        data: &data,
    };
    
    // Execute CPI
    let account_views = accounts.to_views();
    invoke_signed::<5>(&instruction, &account_views, signers)
}

/// CPI: close_position
#[inline(always)]
pub fn close_position<'a>(
    accounts: &ClosePositionAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&CLOSE_POSITION);
    
    
    // Build InstructionAccount array
    let instruction_accounts = accounts.to_instruction_accounts();
    
    // Build InstructionView
    let instruction = InstructionView {
        program_id: &ID,
        accounts: &instruction_accounts,
        data: &data,
    };
    
    // Execute CPI
    let account_views = accounts.to_views();
    invoke_signed::<10>(&instruction, &account_views, signers)
}

/// CPI: close_token_badge
#[inline(always)]
pub fn close_token_badge<'a>(
    accounts: &CloseTokenBadgeAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&CLOSE_TOKEN_BADGE);
    
    
    // Build InstructionAccount array
    let instruction_accounts = accounts.to_instruction_accounts();
    
    // Build InstructionView
    let instruction = InstructionView {
        program_id: &ID,
        accounts: &instruction_accounts,
        data: &data,
    };
    
    // Execute CPI
    let account_views = accounts.to_views();
    invoke_signed::<6>(&instruction, &account_views, signers)
}

/// CPI: create_config
#[inline(always)]
pub fn create_config<'a>(
    accounts: &CreateConfigAccounts<'a>, args: &CreateConfigArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<CreateConfigArgs>()];
    data[0..8].copy_from_slice(&CREATE_CONFIG);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const CreateConfigArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<CreateConfigArgs>(),
            );
        }
    
    // Build InstructionAccount array
    let instruction_accounts = accounts.to_instruction_accounts();
    
    // Build InstructionView
    let instruction = InstructionView {
        program_id: &ID,
        accounts: &instruction_accounts,
        data: &data,
    };
    
    // Execute CPI
    let account_views = accounts.to_views();
    invoke_signed::<7>(&instruction, &account_views, signers)
}

/// CPI: create_dynamic_config
#[inline(always)]
pub fn create_dynamic_config<'a>(
    accounts: &CreateDynamicConfigAccounts<'a>, args: &CreateDynamicConfigArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<CreateDynamicConfigArgs>()];
    data[0..8].copy_from_slice(&CREATE_DYNAMIC_CONFIG);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const CreateDynamicConfigArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<CreateDynamicConfigArgs>(),
            );
        }
    
    // Build InstructionAccount array
    let instruction_accounts = accounts.to_instruction_accounts();
    
    // Build InstructionView
    let instruction = InstructionView {
        program_id: &ID,
        accounts: &instruction_accounts,
        data: &data,
    };
    
    // Execute CPI
    let account_views = accounts.to_views();
    invoke_signed::<7>(&instruction, &account_views, signers)
}

/// CPI: create_operator_account
#[inline(always)]
pub fn create_operator_account<'a>(
    accounts: &CreateOperatorAccountAccounts<'a>, args: &CreateOperatorAccountArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<CreateOperatorAccountArgs>()];
    data[0..8].copy_from_slice(&CREATE_OPERATOR_ACCOUNT);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const CreateOperatorAccountArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<CreateOperatorAccountArgs>(),
            );
        }
    
    // Build InstructionAccount array
    let instruction_accounts = accounts.to_instruction_accounts();
    
    // Build InstructionView
    let instruction = InstructionView {
        program_id: &ID,
        accounts: &instruction_accounts,
        data: &data,
    };
    
    // Execute CPI
    let account_views = accounts.to_views();
    invoke_signed::<7>(&instruction, &account_views, signers)
}

/// CPI: create_position
#[inline(always)]
pub fn create_position<'a>(
    accounts: &CreatePositionAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&CREATE_POSITION);
    
    
    // Build InstructionAccount array
    let instruction_accounts = accounts.to_instruction_accounts();
    
    // Build InstructionView
    let instruction = InstructionView {
        program_id: &ID,
        accounts: &instruction_accounts,
        data: &data,
    };
    
    // Execute CPI
    let account_views = accounts.to_views();
    invoke_signed::<11>(&instruction, &account_views, signers)
}

/// CPI: create_token_badge
#[inline(always)]
pub fn create_token_badge<'a>(
    accounts: &CreateTokenBadgeAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&CREATE_TOKEN_BADGE);
    
    
    // Build InstructionAccount array
    let instruction_accounts = accounts.to_instruction_accounts();
    
    // Build InstructionView
    let instruction = InstructionView {
        program_id: &ID,
        accounts: &instruction_accounts,
        data: &data,
    };
    
    // Execute CPI
    let account_views = accounts.to_views();
    invoke_signed::<8>(&instruction, &account_views, signers)
}

/// CPI: dummy_ix
#[inline(always)]
pub fn dummy_ix<'a>(
    accounts: &DummyIxAccounts<'a>, args: &DummyIxArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<DummyIxArgs>()];
    data[0..8].copy_from_slice(&DUMMY_IX);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const DummyIxArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<DummyIxArgs>(),
            );
        }
    
    // Build InstructionAccount array
    let instruction_accounts = accounts.to_instruction_accounts();
    
    // Build InstructionView
    let instruction = InstructionView {
        program_id: &ID,
        accounts: &instruction_accounts,
        data: &data,
    };
    
    // Execute CPI
    let account_views = accounts.to_views();
    invoke_signed::<3>(&instruction, &account_views, signers)
}

/// CPI: fund_reward
#[inline(always)]
pub fn fund_reward<'a>(
    accounts: &FundRewardAccounts<'a>, args: &FundRewardArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<FundRewardArgs>()];
    data[0..8].copy_from_slice(&FUND_REWARD);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const FundRewardArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<FundRewardArgs>(),
            );
        }
    
    // Build InstructionAccount array
    let instruction_accounts = accounts.to_instruction_accounts();
    
    // Build InstructionView
    let instruction = InstructionView {
        program_id: &ID,
        accounts: &instruction_accounts,
        data: &data,
    };
    
    // Execute CPI
    let account_views = accounts.to_views();
    invoke_signed::<8>(&instruction, &account_views, signers)
}

/// CPI: initialize_customizable_pool
#[inline(always)]
pub fn initialize_customizable_pool<'a>(
    accounts: &InitializeCustomizablePoolAccounts<'a>, args: &InitializeCustomizablePoolArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<InitializeCustomizablePoolArgs>()];
    data[0..8].copy_from_slice(&INITIALIZE_CUSTOMIZABLE_POOL);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const InitializeCustomizablePoolArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<InitializeCustomizablePoolArgs>(),
            );
        }
    
    // Build InstructionAccount array
    let instruction_accounts = accounts.to_instruction_accounts();
    
    // Build InstructionView
    let instruction = InstructionView {
        program_id: &ID,
        accounts: &instruction_accounts,
        data: &data,
    };
    
    // Execute CPI
    let account_views = accounts.to_views();
    invoke_signed::<19>(&instruction, &account_views, signers)
}

/// CPI: initialize_pool
#[inline(always)]
pub fn initialize_pool<'a>(
    accounts: &InitializePoolAccounts<'a>, args: &InitializePoolArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<InitializePoolArgs>()];
    data[0..8].copy_from_slice(&INITIALIZE_POOL);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const InitializePoolArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<InitializePoolArgs>(),
            );
        }
    
    // Build InstructionAccount array
    let instruction_accounts = accounts.to_instruction_accounts();
    
    // Build InstructionView
    let instruction = InstructionView {
        program_id: &ID,
        accounts: &instruction_accounts,
        data: &data,
    };
    
    // Execute CPI
    let account_views = accounts.to_views();
    invoke_signed::<20>(&instruction, &account_views, signers)
}

/// CPI: initialize_pool_with_dynamic_config
#[inline(always)]
pub fn initialize_pool_with_dynamic_config<'a>(
    accounts: &InitializePoolWithDynamicConfigAccounts<'a>, args: &InitializePoolWithDynamicConfigArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<InitializePoolWithDynamicConfigArgs>()];
    data[0..8].copy_from_slice(&INITIALIZE_POOL_WITH_DYNAMIC_CONFIG);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const InitializePoolWithDynamicConfigArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<InitializePoolWithDynamicConfigArgs>(),
            );
        }
    
    // Build InstructionAccount array
    let instruction_accounts = accounts.to_instruction_accounts();
    
    // Build InstructionView
    let instruction = InstructionView {
        program_id: &ID,
        accounts: &instruction_accounts,
        data: &data,
    };
    
    // Execute CPI
    let account_views = accounts.to_views();
    invoke_signed::<21>(&instruction, &account_views, signers)
}

/// CPI: initialize_reward
#[inline(always)]
pub fn initialize_reward<'a>(
    accounts: &InitializeRewardAccounts<'a>, args: &InitializeRewardArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<InitializeRewardArgs>()];
    data[0..8].copy_from_slice(&INITIALIZE_REWARD);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const InitializeRewardArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<InitializeRewardArgs>(),
            );
        }
    
    // Build InstructionAccount array
    let instruction_accounts = accounts.to_instruction_accounts();
    
    // Build InstructionView
    let instruction = InstructionView {
        program_id: &ID,
        accounts: &instruction_accounts,
        data: &data,
    };
    
    // Execute CPI
    let account_views = accounts.to_views();
    invoke_signed::<10>(&instruction, &account_views, signers)
}

/// CPI: lock_position
#[inline(always)]
pub fn lock_position<'a>(
    accounts: &LockPositionAccounts<'a>, args: &LockPositionArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<LockPositionArgs>()];
    data[0..8].copy_from_slice(&LOCK_POSITION);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const LockPositionArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<LockPositionArgs>(),
            );
        }
    
    // Build InstructionAccount array
    let instruction_accounts = accounts.to_instruction_accounts();
    
    // Build InstructionView
    let instruction = InstructionView {
        program_id: &ID,
        accounts: &instruction_accounts,
        data: &data,
    };
    
    // Execute CPI
    let account_views = accounts.to_views();
    invoke_signed::<9>(&instruction, &account_views, signers)
}

/// CPI: permanent_lock_position
#[inline(always)]
pub fn permanent_lock_position<'a>(
    accounts: &PermanentLockPositionAccounts<'a>, args: &PermanentLockPositionArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<PermanentLockPositionArgs>()];
    data[0..8].copy_from_slice(&PERMANENT_LOCK_POSITION);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const PermanentLockPositionArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<PermanentLockPositionArgs>(),
            );
        }
    
    // Build InstructionAccount array
    let instruction_accounts = accounts.to_instruction_accounts();
    
    // Build InstructionView
    let instruction = InstructionView {
        program_id: &ID,
        accounts: &instruction_accounts,
        data: &data,
    };
    
    // Execute CPI
    let account_views = accounts.to_views();
    invoke_signed::<6>(&instruction, &account_views, signers)
}

/// CPI: refresh_vesting
#[inline(always)]
pub fn refresh_vesting<'a>(
    accounts: &RefreshVestingAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&REFRESH_VESTING);
    
    
    // Build InstructionAccount array
    let instruction_accounts = accounts.to_instruction_accounts();
    
    // Build InstructionView
    let instruction = InstructionView {
        program_id: &ID,
        accounts: &instruction_accounts,
        data: &data,
    };
    
    // Execute CPI
    let account_views = accounts.to_views();
    invoke_signed::<4>(&instruction, &account_views, signers)
}

/// CPI: remove_all_liquidity
#[inline(always)]
pub fn remove_all_liquidity<'a>(
    accounts: &RemoveAllLiquidityAccounts<'a>, args: &RemoveAllLiquidityArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<RemoveAllLiquidityArgs>()];
    data[0..8].copy_from_slice(&REMOVE_ALL_LIQUIDITY);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const RemoveAllLiquidityArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<RemoveAllLiquidityArgs>(),
            );
        }
    
    // Build InstructionAccount array
    let instruction_accounts = accounts.to_instruction_accounts();
    
    // Build InstructionView
    let instruction = InstructionView {
        program_id: &ID,
        accounts: &instruction_accounts,
        data: &data,
    };
    
    // Execute CPI
    let account_views = accounts.to_views();
    invoke_signed::<15>(&instruction, &account_views, signers)
}

/// CPI: remove_liquidity
#[inline(always)]
pub fn remove_liquidity<'a>(
    accounts: &RemoveLiquidityAccounts<'a>, args: &RemoveLiquidityArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<RemoveLiquidityArgs>()];
    data[0..8].copy_from_slice(&REMOVE_LIQUIDITY);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const RemoveLiquidityArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<RemoveLiquidityArgs>(),
            );
        }
    
    // Build InstructionAccount array
    let instruction_accounts = accounts.to_instruction_accounts();
    
    // Build InstructionView
    let instruction = InstructionView {
        program_id: &ID,
        accounts: &instruction_accounts,
        data: &data,
    };
    
    // Execute CPI
    let account_views = accounts.to_views();
    invoke_signed::<15>(&instruction, &account_views, signers)
}

/// CPI: set_pool_status
#[inline(always)]
pub fn set_pool_status<'a>(
    accounts: &SetPoolStatusAccounts<'a>, args: &SetPoolStatusArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SetPoolStatusArgs>()];
    data[0..8].copy_from_slice(&SET_POOL_STATUS);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SetPoolStatusArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SetPoolStatusArgs>(),
            );
        }
    
    // Build InstructionAccount array
    let instruction_accounts = accounts.to_instruction_accounts();
    
    // Build InstructionView
    let instruction = InstructionView {
        program_id: &ID,
        accounts: &instruction_accounts,
        data: &data,
    };
    
    // Execute CPI
    let account_views = accounts.to_views();
    invoke_signed::<5>(&instruction, &account_views, signers)
}

/// CPI: split_position
#[inline(always)]
pub fn split_position<'a>(
    accounts: &SplitPositionAccounts<'a>, args: &SplitPositionArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SplitPositionArgs>()];
    data[0..8].copy_from_slice(&SPLIT_POSITION);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SplitPositionArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SplitPositionArgs>(),
            );
        }
    
    // Build InstructionAccount array
    let instruction_accounts = accounts.to_instruction_accounts();
    
    // Build InstructionView
    let instruction = InstructionView {
        program_id: &ID,
        accounts: &instruction_accounts,
        data: &data,
    };
    
    // Execute CPI
    let account_views = accounts.to_views();
    invoke_signed::<9>(&instruction, &account_views, signers)
}

/// CPI: split_position2
#[inline(always)]
pub fn split_position2<'a>(
    accounts: &SplitPosition2Accounts<'a>, args: &SplitPosition2Args,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SplitPosition2Args>()];
    data[0..8].copy_from_slice(&SPLIT_POSITION2);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SplitPosition2Args as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SplitPosition2Args>(),
            );
        }
    
    // Build InstructionAccount array
    let instruction_accounts = accounts.to_instruction_accounts();
    
    // Build InstructionView
    let instruction = InstructionView {
        program_id: &ID,
        accounts: &instruction_accounts,
        data: &data,
    };
    
    // Execute CPI
    let account_views = accounts.to_views();
    invoke_signed::<9>(&instruction, &account_views, signers)
}

/// CPI: swap
#[inline(always)]
pub fn swap<'a>(
    accounts: &SwapAccounts<'a>, args: &SwapArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SwapArgs>()];
    data[0..8].copy_from_slice(&SWAP);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SwapArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SwapArgs>(),
            );
        }
    
    // Build InstructionAccount array
    let instruction_accounts = accounts.to_instruction_accounts();
    
    // Build InstructionView
    let instruction = InstructionView {
        program_id: &ID,
        accounts: &instruction_accounts,
        data: &data,
    };
    
    // Execute CPI
    let account_views = accounts.to_views();
    invoke_signed::<14>(&instruction, &account_views, signers)
}

/// CPI: swap2
#[inline(always)]
pub fn swap2<'a>(
    accounts: &Swap2Accounts<'a>, args: &Swap2Args,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<Swap2Args>()];
    data[0..8].copy_from_slice(&SWAP2);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const Swap2Args as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<Swap2Args>(),
            );
        }
    
    // Build InstructionAccount array
    let instruction_accounts = accounts.to_instruction_accounts();
    
    // Build InstructionView
    let instruction = InstructionView {
        program_id: &ID,
        accounts: &instruction_accounts,
        data: &data,
    };
    
    // Execute CPI
    let account_views = accounts.to_views();
    invoke_signed::<14>(&instruction, &account_views, signers)
}

/// CPI: update_pool_fees
#[inline(always)]
pub fn update_pool_fees<'a>(
    accounts: &UpdatePoolFeesAccounts<'a>, args: &UpdatePoolFeesArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<UpdatePoolFeesArgs>()];
    data[0..8].copy_from_slice(&UPDATE_POOL_FEES);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const UpdatePoolFeesArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<UpdatePoolFeesArgs>(),
            );
        }
    
    // Build InstructionAccount array
    let instruction_accounts = accounts.to_instruction_accounts();
    
    // Build InstructionView
    let instruction = InstructionView {
        program_id: &ID,
        accounts: &instruction_accounts,
        data: &data,
    };
    
    // Execute CPI
    let account_views = accounts.to_views();
    invoke_signed::<5>(&instruction, &account_views, signers)
}

/// CPI: update_reward_duration
#[inline(always)]
pub fn update_reward_duration<'a>(
    accounts: &UpdateRewardDurationAccounts<'a>, args: &UpdateRewardDurationArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<UpdateRewardDurationArgs>()];
    data[0..8].copy_from_slice(&UPDATE_REWARD_DURATION);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const UpdateRewardDurationArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<UpdateRewardDurationArgs>(),
            );
        }
    
    // Build InstructionAccount array
    let instruction_accounts = accounts.to_instruction_accounts();
    
    // Build InstructionView
    let instruction = InstructionView {
        program_id: &ID,
        accounts: &instruction_accounts,
        data: &data,
    };
    
    // Execute CPI
    let account_views = accounts.to_views();
    invoke_signed::<4>(&instruction, &account_views, signers)
}

/// CPI: update_reward_funder
#[inline(always)]
pub fn update_reward_funder<'a>(
    accounts: &UpdateRewardFunderAccounts<'a>, args: &UpdateRewardFunderArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<UpdateRewardFunderArgs>()];
    data[0..8].copy_from_slice(&UPDATE_REWARD_FUNDER);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const UpdateRewardFunderArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<UpdateRewardFunderArgs>(),
            );
        }
    
    // Build InstructionAccount array
    let instruction_accounts = accounts.to_instruction_accounts();
    
    // Build InstructionView
    let instruction = InstructionView {
        program_id: &ID,
        accounts: &instruction_accounts,
        data: &data,
    };
    
    // Execute CPI
    let account_views = accounts.to_views();
    invoke_signed::<4>(&instruction, &account_views, signers)
}

/// CPI: withdraw_ineligible_reward
#[inline(always)]
pub fn withdraw_ineligible_reward<'a>(
    accounts: &WithdrawIneligibleRewardAccounts<'a>, args: &WithdrawIneligibleRewardArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<WithdrawIneligibleRewardArgs>()];
    data[0..8].copy_from_slice(&WITHDRAW_INELIGIBLE_REWARD);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const WithdrawIneligibleRewardArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<WithdrawIneligibleRewardArgs>(),
            );
        }
    
    // Build InstructionAccount array
    let instruction_accounts = accounts.to_instruction_accounts();
    
    // Build InstructionView
    let instruction = InstructionView {
        program_id: &ID,
        accounts: &instruction_accounts,
        data: &data,
    };
    
    // Execute CPI
    let account_views = accounts.to_views();
    invoke_signed::<9>(&instruction, &account_views, signers)
}

/// CPI: zap_protocol_fee
#[inline(always)]
pub fn zap_protocol_fee<'a>(
    accounts: &ZapProtocolFeeAccounts<'a>, args: &ZapProtocolFeeArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<ZapProtocolFeeArgs>()];
    data[0..8].copy_from_slice(&ZAP_PROTOCOL_FEE);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const ZapProtocolFeeArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<ZapProtocolFeeArgs>(),
            );
        }
    
    // Build InstructionAccount array
    let instruction_accounts = accounts.to_instruction_accounts();
    
    // Build InstructionView
    let instruction = InstructionView {
        program_id: &ID,
        accounts: &instruction_accounts,
        data: &data,
    };
    
    // Execute CPI
    let account_views = accounts.to_views();
    invoke_signed::<9>(&instruction, &account_views, signers)
}

