//! CPI module for DefiTuna
//!
//! Program: fusionamm
//! Program ID: fUSioN9YKKSa3CUC2YUc4tPkHJ5Y6XW1yz8y6F7qWz9
//! Instructions: 31

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
pub const ID: Address = Address::new_from_array(five8_const::decode_32_const("fUSioN9YKKSa3CUC2YUc4tPkHJ5Y6XW1yz8y6F7qWz9"));

// ============================================
// Instruction Discriminators
// ============================================
pub const CLOSE_BUNDLED_POSITION: [u8; 8] = [41, 36, 216, 245, 27, 85, 103, 67];
pub const CLOSE_LIMIT_ORDER: [u8; 8] = [76, 124, 128, 15, 213, 87, 37, 250];
pub const CLOSE_POSITION: [u8; 8] = [123, 134, 81, 0, 49, 68, 98, 98];
pub const COLLECT_FEES: [u8; 8] = [164, 152, 207, 99, 30, 186, 19, 182];
pub const COLLECT_PROTOCOL_FEES: [u8; 8] = [22, 67, 23, 98, 150, 178, 70, 220];
pub const DECREASE_LIMIT_ORDER: [u8; 8] = [117, 157, 60, 103, 66, 49, 163, 0];
pub const DECREASE_LIQUIDITY: [u8; 8] = [160, 38, 208, 111, 104, 91, 44, 1];
pub const DELETE_POSITION_BUNDLE: [u8; 8] = [100, 25, 99, 2, 217, 239, 124, 173];
pub const DELETE_TOKEN_BADGE: [u8; 8] = [53, 146, 68, 8, 18, 117, 17, 185];
pub const INCREASE_LIMIT_ORDER: [u8; 8] = [177, 144, 89, 236, 250, 186, 125, 99];
pub const INCREASE_LIQUIDITY: [u8; 8] = [46, 156, 243, 118, 13, 205, 251, 178];
pub const INITIALIZE_CONFIG: [u8; 8] = [208, 127, 21, 1, 194, 190, 196, 70];
pub const INITIALIZE_POOL: [u8; 8] = [95, 180, 10, 172, 84, 174, 232, 40];
pub const INITIALIZE_POSITION_BUNDLE: [u8; 8] = [117, 45, 241, 149, 24, 18, 194, 65];
pub const INITIALIZE_POSITION_BUNDLE_WITH_METADATA: [u8; 8] = [93, 124, 16, 179, 249, 131, 115, 245];
pub const INITIALIZE_TICK_ARRAY: [u8; 8] = [11, 188, 193, 214, 141, 91, 149, 184];
pub const INITIALIZE_TOKEN_BADGE: [u8; 8] = [253, 77, 205, 95, 27, 224, 89, 223];
pub const OPEN_BUNDLED_POSITION: [u8; 8] = [169, 113, 126, 171, 213, 172, 212, 49];
pub const OPEN_LIMIT_ORDER: [u8; 8] = [157, 32, 218, 183, 71, 29, 18, 147];
pub const OPEN_POSITION: [u8; 8] = [135, 128, 47, 77, 15, 152, 240, 49];
pub const RESET_POOL_PRICE: [u8; 8] = [93, 158, 158, 189, 131, 42, 15, 22];
pub const SET_COLLECT_PROTOCOL_FEES_AUTHORITY: [u8; 8] = [34, 150, 93, 244, 139, 225, 233, 67];
pub const SET_DEFAULT_PROTOCOL_FEE_RATE: [u8; 8] = [107, 205, 249, 226, 151, 35, 86, 0];
pub const SET_FEE_AUTHORITY: [u8; 8] = [31, 1, 50, 87, 237, 101, 97, 132];
pub const SET_FEE_RATE: [u8; 8] = [53, 243, 137, 65, 8, 140, 158, 6];
pub const SET_POSITION_RANGE: [u8; 8] = [192, 22, 176, 176, 155, 49, 153, 96];
pub const SET_PROTOCOL_FEE_RATE: [u8; 8] = [95, 7, 4, 50, 154, 79, 156, 131];
pub const SET_TOKEN_BADGE_AUTHORITY: [u8; 8] = [207, 202, 4, 32, 205, 79, 13, 178];
pub const SWAP: [u8; 8] = [248, 198, 158, 145, 225, 117, 135, 200];
pub const TWO_HOP_SWAP: [u8; 8] = [195, 96, 237, 108, 68, 162, 219, 230];
pub const UPDATE_FEES: [u8; 8] = [225, 27, 13, 6, 69, 84, 172, 191];

// ============================================
// Instruction Arguments
// ============================================
/// Arguments for `close_bundled_position`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct CloseBundledPositionArgs {
    pub bundle_index: u16,
}

/// Arguments for `collect_fees`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct CollectFeesArgs {
    pub remaining_accounts_info: Option<[u8; 32]>,
}

/// Arguments for `collect_protocol_fees`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct CollectProtocolFeesArgs {
    pub remaining_accounts_info: Option<[u8; 32]>,
}

/// Arguments for `decrease_limit_order`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct DecreaseLimitOrderArgs {
    pub amount: u64,
    pub remaining_accounts_info: Option<[u8; 32]>,
}

/// Arguments for `decrease_liquidity`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct DecreaseLiquidityArgs {
    pub liquidity_amount: u128,
    pub token_min_a: u64,
    pub token_min_b: u64,
    pub remaining_accounts_info: Option<[u8; 32]>,
}

/// Arguments for `increase_limit_order`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct IncreaseLimitOrderArgs {
    pub amount: u64,
    pub remaining_accounts_info: Option<[u8; 32]>,
}

/// Arguments for `increase_liquidity`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct IncreaseLiquidityArgs {
    pub liquidity_amount: u128,
    pub token_max_a: u64,
    pub token_max_b: u64,
    pub remaining_accounts_info: Option<[u8; 32]>,
}

/// Arguments for `initialize_config`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct InitializeConfigArgs {
    pub fee_authority: [u8; 32],
    pub collect_protocol_fees_authority: [u8; 32],
    pub token_badge_authority: [u8; 32],
    pub default_protocol_fee_rate: u16,
}

/// Arguments for `initialize_pool`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct InitializePoolArgs {
    pub tick_spacing: u16,
    pub fee_rate: u16,
    pub initial_sqrt_price: u128,
}

/// Arguments for `initialize_tick_array`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct InitializeTickArrayArgs {
    pub start_tick_index: i32,
}

/// Arguments for `open_bundled_position`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct OpenBundledPositionArgs {
    pub bundle_index: u16,
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
}

/// Arguments for `open_limit_order`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct OpenLimitOrderArgs {
    pub tick_index: i32,
    pub a_to_b: bool,
    pub with_token_metadata_extension: bool,
}

/// Arguments for `open_position`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct OpenPositionArgs {
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
    pub with_token_metadata_extension: bool,
}

/// Arguments for `reset_pool_price`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct ResetPoolPriceArgs {
    pub sqrt_price: u128,
}

/// Arguments for `set_default_protocol_fee_rate`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SetDefaultProtocolFeeRateArgs {
    pub default_protocol_fee_rate: u16,
}

/// Arguments for `set_fee_rate`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SetFeeRateArgs {
    pub fee_rate: u16,
}

/// Arguments for `set_position_range`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SetPositionRangeArgs {
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
}

/// Arguments for `set_protocol_fee_rate`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SetProtocolFeeRateArgs {
    pub protocol_fee_rate: u16,
}

/// Arguments for `swap`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SwapArgs {
    pub amount: u64,
    pub other_amount_threshold: u64,
    pub sqrt_price_limit: u128,
    pub amount_specified_is_input: bool,
    pub a_to_b: bool,
    pub remaining_accounts_info: Option<[u8; 32]>,
}

/// Arguments for `two_hop_swap`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct TwoHopSwapArgs {
    pub amount: u64,
    pub other_amount_threshold: u64,
    pub amount_specified_is_input: bool,
    pub a_to_b_one: bool,
    pub a_to_b_two: bool,
    pub sqrt_price_limit_one: u128,
    pub sqrt_price_limit_two: u128,
    pub remaining_accounts_info: Option<[u8; 32]>,
}


// ============================================
// Instruction Accounts
// ============================================
/// Accounts for `close_bundled_position`
pub struct CloseBundledPositionAccounts<'a> {
    /// bundled_position
    pub bundled_position: &'a AccountView,
    /// position_bundle
    pub position_bundle: &'a AccountView,
    /// position_bundle_token_account
    pub position_bundle_token_account: &'a AccountView,
    /// position_bundle_authority
    pub position_bundle_authority: &'a AccountView,
    /// receiver
    pub receiver: &'a AccountView,
}

impl<'a> CloseBundledPositionAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 5] {
        [
            InstructionAccount::writable(self.bundled_position.address()),
            InstructionAccount::writable(self.position_bundle.address()),
            InstructionAccount::readonly(self.position_bundle_token_account.address()),
            InstructionAccount::readonly_signer(self.position_bundle_authority.address()),
            InstructionAccount::writable(self.receiver.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 5] {
        [
            self.bundled_position, self.position_bundle, self.position_bundle_token_account, self.position_bundle_authority, self.receiver
        ]
    }
}

/// Accounts for `close_limit_order`
pub struct CloseLimitOrderAccounts<'a> {
    /// limit_order_authority
    pub limit_order_authority: &'a AccountView,
    /// receiver
    pub receiver: &'a AccountView,
    /// limit_order
    pub limit_order: &'a AccountView,
    /// limit_order_mint
    pub limit_order_mint: &'a AccountView,
    /// limit_order_token_account
    pub limit_order_token_account: &'a AccountView,
    /// token_2022_program
    pub token_2022_program: &'a AccountView,
}

impl<'a> CloseLimitOrderAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 6] {
        [
            InstructionAccount::readonly_signer(self.limit_order_authority.address()),
            InstructionAccount::writable(self.receiver.address()),
            InstructionAccount::writable(self.limit_order.address()),
            InstructionAccount::writable(self.limit_order_mint.address()),
            InstructionAccount::writable(self.limit_order_token_account.address()),
            InstructionAccount::readonly(self.token_2022_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 6] {
        [
            self.limit_order_authority, self.receiver, self.limit_order, self.limit_order_mint, self.limit_order_token_account, self.token_2022_program
        ]
    }
}

/// Accounts for `close_position`
pub struct ClosePositionAccounts<'a> {
    /// position_authority
    pub position_authority: &'a AccountView,
    /// receiver
    pub receiver: &'a AccountView,
    /// position
    pub position: &'a AccountView,
    /// position_mint
    pub position_mint: &'a AccountView,
    /// position_token_account
    pub position_token_account: &'a AccountView,
    /// token_2022_program
    pub token_2022_program: &'a AccountView,
}

impl<'a> ClosePositionAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 6] {
        [
            InstructionAccount::readonly_signer(self.position_authority.address()),
            InstructionAccount::writable(self.receiver.address()),
            InstructionAccount::writable(self.position.address()),
            InstructionAccount::writable(self.position_mint.address()),
            InstructionAccount::writable(self.position_token_account.address()),
            InstructionAccount::readonly(self.token_2022_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 6] {
        [
            self.position_authority, self.receiver, self.position, self.position_mint, self.position_token_account, self.token_2022_program
        ]
    }
}

/// Accounts for `collect_fees`
pub struct CollectFeesAccounts<'a> {
    /// fusion_pool
    pub fusion_pool: &'a AccountView,
    /// position_authority
    pub position_authority: &'a AccountView,
    /// position
    pub position: &'a AccountView,
    /// position_token_account
    pub position_token_account: &'a AccountView,
    /// token_mint_a
    pub token_mint_a: &'a AccountView,
    /// token_mint_b
    pub token_mint_b: &'a AccountView,
    /// token_owner_account_a
    pub token_owner_account_a: &'a AccountView,
    /// token_owner_account_b
    pub token_owner_account_b: &'a AccountView,
    /// token_vault_a
    pub token_vault_a: &'a AccountView,
    /// token_vault_b
    pub token_vault_b: &'a AccountView,
    /// token_program_a
    pub token_program_a: &'a AccountView,
    /// token_program_b
    pub token_program_b: &'a AccountView,
    /// memo_program
    pub memo_program: &'a AccountView,
}

impl<'a> CollectFeesAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 13] {
        [
            InstructionAccount::readonly(self.fusion_pool.address()),
            InstructionAccount::readonly_signer(self.position_authority.address()),
            InstructionAccount::writable(self.position.address()),
            InstructionAccount::readonly(self.position_token_account.address()),
            InstructionAccount::readonly(self.token_mint_a.address()),
            InstructionAccount::readonly(self.token_mint_b.address()),
            InstructionAccount::writable(self.token_owner_account_a.address()),
            InstructionAccount::writable(self.token_owner_account_b.address()),
            InstructionAccount::writable(self.token_vault_a.address()),
            InstructionAccount::writable(self.token_vault_b.address()),
            InstructionAccount::readonly(self.token_program_a.address()),
            InstructionAccount::readonly(self.token_program_b.address()),
            InstructionAccount::readonly(self.memo_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 13] {
        [
            self.fusion_pool, self.position_authority, self.position, self.position_token_account, self.token_mint_a, self.token_mint_b, self.token_owner_account_a, self.token_owner_account_b, self.token_vault_a, self.token_vault_b, self.token_program_a, self.token_program_b, self.memo_program
        ]
    }
}

/// Accounts for `collect_protocol_fees`
pub struct CollectProtocolFeesAccounts<'a> {
    /// fusion_pools_config
    pub fusion_pools_config: &'a AccountView,
    /// fusion_pool
    pub fusion_pool: &'a AccountView,
    /// collect_protocol_fees_authority
    pub collect_protocol_fees_authority: &'a AccountView,
    /// token_mint_a
    pub token_mint_a: &'a AccountView,
    /// token_mint_b
    pub token_mint_b: &'a AccountView,
    /// token_vault_a
    pub token_vault_a: &'a AccountView,
    /// token_vault_b
    pub token_vault_b: &'a AccountView,
    /// token_destination_a
    pub token_destination_a: &'a AccountView,
    /// token_destination_b
    pub token_destination_b: &'a AccountView,
    /// token_program_a
    pub token_program_a: &'a AccountView,
    /// token_program_b
    pub token_program_b: &'a AccountView,
    /// memo_program
    pub memo_program: &'a AccountView,
}

impl<'a> CollectProtocolFeesAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 12] {
        [
            InstructionAccount::readonly(self.fusion_pools_config.address()),
            InstructionAccount::writable(self.fusion_pool.address()),
            InstructionAccount::readonly_signer(self.collect_protocol_fees_authority.address()),
            InstructionAccount::readonly(self.token_mint_a.address()),
            InstructionAccount::readonly(self.token_mint_b.address()),
            InstructionAccount::writable(self.token_vault_a.address()),
            InstructionAccount::writable(self.token_vault_b.address()),
            InstructionAccount::writable(self.token_destination_a.address()),
            InstructionAccount::writable(self.token_destination_b.address()),
            InstructionAccount::readonly(self.token_program_a.address()),
            InstructionAccount::readonly(self.token_program_b.address()),
            InstructionAccount::readonly(self.memo_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 12] {
        [
            self.fusion_pools_config, self.fusion_pool, self.collect_protocol_fees_authority, self.token_mint_a, self.token_mint_b, self.token_vault_a, self.token_vault_b, self.token_destination_a, self.token_destination_b, self.token_program_a, self.token_program_b, self.memo_program
        ]
    }
}

/// Accounts for `decrease_limit_order`
pub struct DecreaseLimitOrderAccounts<'a> {
    /// limit_order_authority
    pub limit_order_authority: &'a AccountView,
    /// fusion_pool
    pub fusion_pool: &'a AccountView,
    /// limit_order
    pub limit_order: &'a AccountView,
    /// limit_order_token_account
    pub limit_order_token_account: &'a AccountView,
    /// token_mint_a
    pub token_mint_a: &'a AccountView,
    /// token_mint_b
    pub token_mint_b: &'a AccountView,
    /// token_owner_account_a
    pub token_owner_account_a: &'a AccountView,
    /// token_owner_account_b
    pub token_owner_account_b: &'a AccountView,
    /// token_vault_a
    pub token_vault_a: &'a AccountView,
    /// token_vault_b
    pub token_vault_b: &'a AccountView,
    /// tick_array
    pub tick_array: &'a AccountView,
    /// token_program_a
    pub token_program_a: &'a AccountView,
    /// token_program_b
    pub token_program_b: &'a AccountView,
    /// memo_program
    pub memo_program: &'a AccountView,
}

impl<'a> DecreaseLimitOrderAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 14] {
        [
            InstructionAccount::readonly_signer(self.limit_order_authority.address()),
            InstructionAccount::writable(self.fusion_pool.address()),
            InstructionAccount::writable(self.limit_order.address()),
            InstructionAccount::readonly(self.limit_order_token_account.address()),
            InstructionAccount::readonly(self.token_mint_a.address()),
            InstructionAccount::readonly(self.token_mint_b.address()),
            InstructionAccount::writable(self.token_owner_account_a.address()),
            InstructionAccount::writable(self.token_owner_account_b.address()),
            InstructionAccount::writable(self.token_vault_a.address()),
            InstructionAccount::writable(self.token_vault_b.address()),
            InstructionAccount::writable(self.tick_array.address()),
            InstructionAccount::readonly(self.token_program_a.address()),
            InstructionAccount::readonly(self.token_program_b.address()),
            InstructionAccount::readonly(self.memo_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 14] {
        [
            self.limit_order_authority, self.fusion_pool, self.limit_order, self.limit_order_token_account, self.token_mint_a, self.token_mint_b, self.token_owner_account_a, self.token_owner_account_b, self.token_vault_a, self.token_vault_b, self.tick_array, self.token_program_a, self.token_program_b, self.memo_program
        ]
    }
}

/// Accounts for `decrease_liquidity`
pub struct DecreaseLiquidityAccounts<'a> {
    /// fusion_pool
    pub fusion_pool: &'a AccountView,
    /// token_program_a
    pub token_program_a: &'a AccountView,
    /// token_program_b
    pub token_program_b: &'a AccountView,
    /// memo_program
    pub memo_program: &'a AccountView,
    /// position_authority
    pub position_authority: &'a AccountView,
    /// position
    pub position: &'a AccountView,
    /// position_token_account
    pub position_token_account: &'a AccountView,
    /// token_mint_a
    pub token_mint_a: &'a AccountView,
    /// token_mint_b
    pub token_mint_b: &'a AccountView,
    /// token_owner_account_a
    pub token_owner_account_a: &'a AccountView,
    /// token_owner_account_b
    pub token_owner_account_b: &'a AccountView,
    /// token_vault_a
    pub token_vault_a: &'a AccountView,
    /// token_vault_b
    pub token_vault_b: &'a AccountView,
    /// tick_array_lower
    pub tick_array_lower: &'a AccountView,
    /// tick_array_upper
    pub tick_array_upper: &'a AccountView,
}

impl<'a> DecreaseLiquidityAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 15] {
        [
            InstructionAccount::writable(self.fusion_pool.address()),
            InstructionAccount::readonly(self.token_program_a.address()),
            InstructionAccount::readonly(self.token_program_b.address()),
            InstructionAccount::readonly(self.memo_program.address()),
            InstructionAccount::readonly_signer(self.position_authority.address()),
            InstructionAccount::writable(self.position.address()),
            InstructionAccount::readonly(self.position_token_account.address()),
            InstructionAccount::readonly(self.token_mint_a.address()),
            InstructionAccount::readonly(self.token_mint_b.address()),
            InstructionAccount::writable(self.token_owner_account_a.address()),
            InstructionAccount::writable(self.token_owner_account_b.address()),
            InstructionAccount::writable(self.token_vault_a.address()),
            InstructionAccount::writable(self.token_vault_b.address()),
            InstructionAccount::writable(self.tick_array_lower.address()),
            InstructionAccount::writable(self.tick_array_upper.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 15] {
        [
            self.fusion_pool, self.token_program_a, self.token_program_b, self.memo_program, self.position_authority, self.position, self.position_token_account, self.token_mint_a, self.token_mint_b, self.token_owner_account_a, self.token_owner_account_b, self.token_vault_a, self.token_vault_b, self.tick_array_lower, self.tick_array_upper
        ]
    }
}

/// Accounts for `delete_position_bundle`
pub struct DeletePositionBundleAccounts<'a> {
    /// position_bundle
    pub position_bundle: &'a AccountView,
    /// position_bundle_mint
    pub position_bundle_mint: &'a AccountView,
    /// position_bundle_token_account
    pub position_bundle_token_account: &'a AccountView,
    /// position_bundle_owner
    pub position_bundle_owner: &'a AccountView,
    /// receiver
    pub receiver: &'a AccountView,
    /// token_program
    pub token_program: &'a AccountView,
}

impl<'a> DeletePositionBundleAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 6] {
        [
            InstructionAccount::writable(self.position_bundle.address()),
            InstructionAccount::writable(self.position_bundle_mint.address()),
            InstructionAccount::writable(self.position_bundle_token_account.address()),
            InstructionAccount::readonly_signer(self.position_bundle_owner.address()),
            InstructionAccount::writable(self.receiver.address()),
            InstructionAccount::readonly(self.token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 6] {
        [
            self.position_bundle, self.position_bundle_mint, self.position_bundle_token_account, self.position_bundle_owner, self.receiver, self.token_program
        ]
    }
}

/// Accounts for `delete_token_badge`
pub struct DeleteTokenBadgeAccounts<'a> {
    /// fusion_pools_config
    pub fusion_pools_config: &'a AccountView,
    /// token_badge_authority
    pub token_badge_authority: &'a AccountView,
    /// token_mint
    pub token_mint: &'a AccountView,
    /// token_badge
    pub token_badge: &'a AccountView,
    /// receiver
    pub receiver: &'a AccountView,
}

impl<'a> DeleteTokenBadgeAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 5] {
        [
            InstructionAccount::readonly(self.fusion_pools_config.address()),
            InstructionAccount::readonly_signer(self.token_badge_authority.address()),
            InstructionAccount::readonly(self.token_mint.address()),
            InstructionAccount::writable(self.token_badge.address()),
            InstructionAccount::writable(self.receiver.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 5] {
        [
            self.fusion_pools_config, self.token_badge_authority, self.token_mint, self.token_badge, self.receiver
        ]
    }
}

/// Accounts for `increase_limit_order`
pub struct IncreaseLimitOrderAccounts<'a> {
    /// limit_order_authority
    pub limit_order_authority: &'a AccountView,
    /// fusion_pool
    pub fusion_pool: &'a AccountView,
    /// limit_order
    pub limit_order: &'a AccountView,
    /// limit_order_token_account
    pub limit_order_token_account: &'a AccountView,
    /// token_mint
    pub token_mint: &'a AccountView,
    /// token_owner_account
    pub token_owner_account: &'a AccountView,
    /// token_vault
    pub token_vault: &'a AccountView,
    /// tick_array
    pub tick_array: &'a AccountView,
    /// token_program
    pub token_program: &'a AccountView,
    /// memo_program
    pub memo_program: &'a AccountView,
}

impl<'a> IncreaseLimitOrderAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 10] {
        [
            InstructionAccount::readonly_signer(self.limit_order_authority.address()),
            InstructionAccount::writable(self.fusion_pool.address()),
            InstructionAccount::writable(self.limit_order.address()),
            InstructionAccount::readonly(self.limit_order_token_account.address()),
            InstructionAccount::readonly(self.token_mint.address()),
            InstructionAccount::writable(self.token_owner_account.address()),
            InstructionAccount::writable(self.token_vault.address()),
            InstructionAccount::writable(self.tick_array.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.memo_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 10] {
        [
            self.limit_order_authority, self.fusion_pool, self.limit_order, self.limit_order_token_account, self.token_mint, self.token_owner_account, self.token_vault, self.tick_array, self.token_program, self.memo_program
        ]
    }
}

/// Accounts for `increase_liquidity`
pub struct IncreaseLiquidityAccounts<'a> {
    /// fusion_pool
    pub fusion_pool: &'a AccountView,
    /// token_program_a
    pub token_program_a: &'a AccountView,
    /// token_program_b
    pub token_program_b: &'a AccountView,
    /// memo_program
    pub memo_program: &'a AccountView,
    /// position_authority
    pub position_authority: &'a AccountView,
    /// position
    pub position: &'a AccountView,
    /// position_token_account
    pub position_token_account: &'a AccountView,
    /// token_mint_a
    pub token_mint_a: &'a AccountView,
    /// token_mint_b
    pub token_mint_b: &'a AccountView,
    /// token_owner_account_a
    pub token_owner_account_a: &'a AccountView,
    /// token_owner_account_b
    pub token_owner_account_b: &'a AccountView,
    /// token_vault_a
    pub token_vault_a: &'a AccountView,
    /// token_vault_b
    pub token_vault_b: &'a AccountView,
    /// tick_array_lower
    pub tick_array_lower: &'a AccountView,
    /// tick_array_upper
    pub tick_array_upper: &'a AccountView,
}

impl<'a> IncreaseLiquidityAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 15] {
        [
            InstructionAccount::writable(self.fusion_pool.address()),
            InstructionAccount::readonly(self.token_program_a.address()),
            InstructionAccount::readonly(self.token_program_b.address()),
            InstructionAccount::readonly(self.memo_program.address()),
            InstructionAccount::readonly_signer(self.position_authority.address()),
            InstructionAccount::writable(self.position.address()),
            InstructionAccount::readonly(self.position_token_account.address()),
            InstructionAccount::readonly(self.token_mint_a.address()),
            InstructionAccount::readonly(self.token_mint_b.address()),
            InstructionAccount::writable(self.token_owner_account_a.address()),
            InstructionAccount::writable(self.token_owner_account_b.address()),
            InstructionAccount::writable(self.token_vault_a.address()),
            InstructionAccount::writable(self.token_vault_b.address()),
            InstructionAccount::writable(self.tick_array_lower.address()),
            InstructionAccount::writable(self.tick_array_upper.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 15] {
        [
            self.fusion_pool, self.token_program_a, self.token_program_b, self.memo_program, self.position_authority, self.position, self.position_token_account, self.token_mint_a, self.token_mint_b, self.token_owner_account_a, self.token_owner_account_b, self.token_vault_a, self.token_vault_b, self.tick_array_lower, self.tick_array_upper
        ]
    }
}

/// Accounts for `initialize_config`
pub struct InitializeConfigAccounts<'a> {
    /// fusion_pools_config
    pub fusion_pools_config: &'a AccountView,
    /// funder
    pub funder: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
}

impl<'a> InitializeConfigAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 3] {
        [
            InstructionAccount::writable(self.fusion_pools_config.address()),
            InstructionAccount::writable_signer(self.funder.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 3] {
        [
            self.fusion_pools_config, self.funder, self.system_program
        ]
    }
}

/// Accounts for `initialize_pool`
pub struct InitializePoolAccounts<'a> {
    /// fusion_pools_config
    pub fusion_pools_config: &'a AccountView,
    /// token_mint_a
    pub token_mint_a: &'a AccountView,
    /// token_mint_b
    pub token_mint_b: &'a AccountView,
    /// token_badge_a
    pub token_badge_a: &'a AccountView,
    /// token_badge_b
    pub token_badge_b: &'a AccountView,
    /// funder
    pub funder: &'a AccountView,
    /// fusion_pool
    pub fusion_pool: &'a AccountView,
    /// token_vault_a
    pub token_vault_a: &'a AccountView,
    /// token_vault_b
    pub token_vault_b: &'a AccountView,
    /// token_program_a
    pub token_program_a: &'a AccountView,
    /// token_program_b
    pub token_program_b: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
    /// rent
    pub rent: &'a AccountView,
}

impl<'a> InitializePoolAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 13] {
        [
            InstructionAccount::readonly(self.fusion_pools_config.address()),
            InstructionAccount::readonly(self.token_mint_a.address()),
            InstructionAccount::readonly(self.token_mint_b.address()),
            InstructionAccount::readonly(self.token_badge_a.address()),
            InstructionAccount::readonly(self.token_badge_b.address()),
            InstructionAccount::writable_signer(self.funder.address()),
            InstructionAccount::writable(self.fusion_pool.address()),
            InstructionAccount::writable_signer(self.token_vault_a.address()),
            InstructionAccount::writable_signer(self.token_vault_b.address()),
            InstructionAccount::readonly(self.token_program_a.address()),
            InstructionAccount::readonly(self.token_program_b.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.rent.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 13] {
        [
            self.fusion_pools_config, self.token_mint_a, self.token_mint_b, self.token_badge_a, self.token_badge_b, self.funder, self.fusion_pool, self.token_vault_a, self.token_vault_b, self.token_program_a, self.token_program_b, self.system_program, self.rent
        ]
    }
}

/// Accounts for `initialize_position_bundle`
pub struct InitializePositionBundleAccounts<'a> {
    /// position_bundle
    pub position_bundle: &'a AccountView,
    /// position_bundle_mint
    pub position_bundle_mint: &'a AccountView,
    /// position_bundle_token_account
    pub position_bundle_token_account: &'a AccountView,
    /// position_bundle_owner
    pub position_bundle_owner: &'a AccountView,
    /// funder
    pub funder: &'a AccountView,
    /// token_program
    pub token_program: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
    /// rent
    pub rent: &'a AccountView,
    /// associated_token_program
    pub associated_token_program: &'a AccountView,
}

impl<'a> InitializePositionBundleAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 9] {
        [
            InstructionAccount::writable(self.position_bundle.address()),
            InstructionAccount::writable_signer(self.position_bundle_mint.address()),
            InstructionAccount::writable(self.position_bundle_token_account.address()),
            InstructionAccount::readonly(self.position_bundle_owner.address()),
            InstructionAccount::writable_signer(self.funder.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.rent.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 9] {
        [
            self.position_bundle, self.position_bundle_mint, self.position_bundle_token_account, self.position_bundle_owner, self.funder, self.token_program, self.system_program, self.rent, self.associated_token_program
        ]
    }
}

/// Accounts for `initialize_position_bundle_with_metadata`
pub struct InitializePositionBundleWithMetadataAccounts<'a> {
    /// position_bundle
    pub position_bundle: &'a AccountView,
    /// position_bundle_mint
    pub position_bundle_mint: &'a AccountView,
    /// https://github.com/metaplex-foundation/metaplex-program-library/blob/773a574c4b34e5b9f248a81306ec24db064e255f/token-metadata/program/src/utils/metadata.rs#L100
    pub position_bundle_metadata: &'a AccountView,
    /// position_bundle_token_account
    pub position_bundle_token_account: &'a AccountView,
    /// position_bundle_owner
    pub position_bundle_owner: &'a AccountView,
    /// funder
    pub funder: &'a AccountView,
    /// metadata_update_auth
    pub metadata_update_auth: &'a AccountView,
    /// token_program
    pub token_program: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
    /// rent
    pub rent: &'a AccountView,
    /// associated_token_program
    pub associated_token_program: &'a AccountView,
    /// metadata_program
    pub metadata_program: &'a AccountView,
}

impl<'a> InitializePositionBundleWithMetadataAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 12] {
        [
            InstructionAccount::writable(self.position_bundle.address()),
            InstructionAccount::writable_signer(self.position_bundle_mint.address()),
            InstructionAccount::writable(self.position_bundle_metadata.address()),
            InstructionAccount::writable(self.position_bundle_token_account.address()),
            InstructionAccount::readonly(self.position_bundle_owner.address()),
            InstructionAccount::writable_signer(self.funder.address()),
            InstructionAccount::readonly(self.metadata_update_auth.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.rent.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
            InstructionAccount::readonly(self.metadata_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 12] {
        [
            self.position_bundle, self.position_bundle_mint, self.position_bundle_metadata, self.position_bundle_token_account, self.position_bundle_owner, self.funder, self.metadata_update_auth, self.token_program, self.system_program, self.rent, self.associated_token_program, self.metadata_program
        ]
    }
}

/// Accounts for `initialize_tick_array`
pub struct InitializeTickArrayAccounts<'a> {
    /// fusion_pool
    pub fusion_pool: &'a AccountView,
    /// funder
    pub funder: &'a AccountView,
    /// tick_array
    pub tick_array: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
}

impl<'a> InitializeTickArrayAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 4] {
        [
            InstructionAccount::readonly(self.fusion_pool.address()),
            InstructionAccount::writable_signer(self.funder.address()),
            InstructionAccount::writable(self.tick_array.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 4] {
        [
            self.fusion_pool, self.funder, self.tick_array, self.system_program
        ]
    }
}

/// Accounts for `initialize_token_badge`
pub struct InitializeTokenBadgeAccounts<'a> {
    /// fusion_pools_config
    pub fusion_pools_config: &'a AccountView,
    /// token_badge_authority
    pub token_badge_authority: &'a AccountView,
    /// token_mint
    pub token_mint: &'a AccountView,
    /// token_badge
    pub token_badge: &'a AccountView,
    /// funder
    pub funder: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
}

impl<'a> InitializeTokenBadgeAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 6] {
        [
            InstructionAccount::readonly(self.fusion_pools_config.address()),
            InstructionAccount::readonly_signer(self.token_badge_authority.address()),
            InstructionAccount::readonly(self.token_mint.address()),
            InstructionAccount::writable(self.token_badge.address()),
            InstructionAccount::writable_signer(self.funder.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 6] {
        [
            self.fusion_pools_config, self.token_badge_authority, self.token_mint, self.token_badge, self.funder, self.system_program
        ]
    }
}

/// Accounts for `open_bundled_position`
pub struct OpenBundledPositionAccounts<'a> {
    /// bundled_position
    pub bundled_position: &'a AccountView,
    /// position_bundle
    pub position_bundle: &'a AccountView,
    /// position_bundle_token_account
    pub position_bundle_token_account: &'a AccountView,
    /// position_bundle_authority
    pub position_bundle_authority: &'a AccountView,
    /// fusion_pool
    pub fusion_pool: &'a AccountView,
    /// funder
    pub funder: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
    /// rent
    pub rent: &'a AccountView,
}

impl<'a> OpenBundledPositionAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 8] {
        [
            InstructionAccount::writable(self.bundled_position.address()),
            InstructionAccount::writable(self.position_bundle.address()),
            InstructionAccount::readonly(self.position_bundle_token_account.address()),
            InstructionAccount::readonly_signer(self.position_bundle_authority.address()),
            InstructionAccount::readonly(self.fusion_pool.address()),
            InstructionAccount::writable_signer(self.funder.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.rent.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 8] {
        [
            self.bundled_position, self.position_bundle, self.position_bundle_token_account, self.position_bundle_authority, self.fusion_pool, self.funder, self.system_program, self.rent
        ]
    }
}

/// Accounts for `open_limit_order`
pub struct OpenLimitOrderAccounts<'a> {
    /// funder
    pub funder: &'a AccountView,
    /// owner
    pub owner: &'a AccountView,
    /// limit_order
    pub limit_order: &'a AccountView,
    /// limit_order_mint
    pub limit_order_mint: &'a AccountView,
    /// limit_order_token_account
    pub limit_order_token_account: &'a AccountView,
    /// fusion_pool
    pub fusion_pool: &'a AccountView,
    /// token_2022_program
    pub token_2022_program: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
    /// associated_token_program
    pub associated_token_program: &'a AccountView,
    /// metadata_update_auth
    pub metadata_update_auth: &'a AccountView,
}

impl<'a> OpenLimitOrderAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 10] {
        [
            InstructionAccount::writable_signer(self.funder.address()),
            InstructionAccount::readonly(self.owner.address()),
            InstructionAccount::writable(self.limit_order.address()),
            InstructionAccount::writable_signer(self.limit_order_mint.address()),
            InstructionAccount::writable(self.limit_order_token_account.address()),
            InstructionAccount::readonly(self.fusion_pool.address()),
            InstructionAccount::readonly(self.token_2022_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
            InstructionAccount::readonly(self.metadata_update_auth.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 10] {
        [
            self.funder, self.owner, self.limit_order, self.limit_order_mint, self.limit_order_token_account, self.fusion_pool, self.token_2022_program, self.system_program, self.associated_token_program, self.metadata_update_auth
        ]
    }
}

/// Accounts for `open_position`
pub struct OpenPositionAccounts<'a> {
    /// funder
    pub funder: &'a AccountView,
    /// owner
    pub owner: &'a AccountView,
    /// position
    pub position: &'a AccountView,
    /// position_mint
    pub position_mint: &'a AccountView,
    /// position_token_account
    pub position_token_account: &'a AccountView,
    /// fusion_pool
    pub fusion_pool: &'a AccountView,
    /// token_2022_program
    pub token_2022_program: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
    /// associated_token_program
    pub associated_token_program: &'a AccountView,
    /// metadata_update_auth
    pub metadata_update_auth: &'a AccountView,
}

impl<'a> OpenPositionAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 10] {
        [
            InstructionAccount::writable_signer(self.funder.address()),
            InstructionAccount::readonly(self.owner.address()),
            InstructionAccount::writable(self.position.address()),
            InstructionAccount::writable_signer(self.position_mint.address()),
            InstructionAccount::writable(self.position_token_account.address()),
            InstructionAccount::readonly(self.fusion_pool.address()),
            InstructionAccount::readonly(self.token_2022_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
            InstructionAccount::readonly(self.metadata_update_auth.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 10] {
        [
            self.funder, self.owner, self.position, self.position_mint, self.position_token_account, self.fusion_pool, self.token_2022_program, self.system_program, self.associated_token_program, self.metadata_update_auth
        ]
    }
}

/// Accounts for `reset_pool_price`
pub struct ResetPoolPriceAccounts<'a> {
    /// fee_authority
    pub fee_authority: &'a AccountView,
    /// fusion_pools_config
    pub fusion_pools_config: &'a AccountView,
    /// token_vault_a
    pub token_vault_a: &'a AccountView,
    /// token_vault_b
    pub token_vault_b: &'a AccountView,
    /// fusion_pool
    pub fusion_pool: &'a AccountView,
}

impl<'a> ResetPoolPriceAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 5] {
        [
            InstructionAccount::readonly_signer(self.fee_authority.address()),
            InstructionAccount::readonly(self.fusion_pools_config.address()),
            InstructionAccount::writable(self.token_vault_a.address()),
            InstructionAccount::writable(self.token_vault_b.address()),
            InstructionAccount::writable(self.fusion_pool.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 5] {
        [
            self.fee_authority, self.fusion_pools_config, self.token_vault_a, self.token_vault_b, self.fusion_pool
        ]
    }
}

/// Accounts for `set_collect_protocol_fees_authority`
pub struct SetCollectProtocolFeesAuthorityAccounts<'a> {
    /// fusion_pools_config
    pub fusion_pools_config: &'a AccountView,
    /// collect_protocol_fees_authority
    pub collect_protocol_fees_authority: &'a AccountView,
    /// new_collect_protocol_fees_authority
    pub new_collect_protocol_fees_authority: &'a AccountView,
}

impl<'a> SetCollectProtocolFeesAuthorityAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 3] {
        [
            InstructionAccount::writable(self.fusion_pools_config.address()),
            InstructionAccount::readonly_signer(self.collect_protocol_fees_authority.address()),
            InstructionAccount::readonly(self.new_collect_protocol_fees_authority.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 3] {
        [
            self.fusion_pools_config, self.collect_protocol_fees_authority, self.new_collect_protocol_fees_authority
        ]
    }
}

/// Accounts for `set_default_protocol_fee_rate`
pub struct SetDefaultProtocolFeeRateAccounts<'a> {
    /// fusion_pools_config
    pub fusion_pools_config: &'a AccountView,
    /// fee_authority
    pub fee_authority: &'a AccountView,
}

impl<'a> SetDefaultProtocolFeeRateAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 2] {
        [
            InstructionAccount::writable(self.fusion_pools_config.address()),
            InstructionAccount::readonly_signer(self.fee_authority.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 2] {
        [
            self.fusion_pools_config, self.fee_authority
        ]
    }
}

/// Accounts for `set_fee_authority`
pub struct SetFeeAuthorityAccounts<'a> {
    /// fusion_pools_config
    pub fusion_pools_config: &'a AccountView,
    /// fee_authority
    pub fee_authority: &'a AccountView,
    /// new_fee_authority
    pub new_fee_authority: &'a AccountView,
}

impl<'a> SetFeeAuthorityAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 3] {
        [
            InstructionAccount::writable(self.fusion_pools_config.address()),
            InstructionAccount::readonly_signer(self.fee_authority.address()),
            InstructionAccount::readonly(self.new_fee_authority.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 3] {
        [
            self.fusion_pools_config, self.fee_authority, self.new_fee_authority
        ]
    }
}

/// Accounts for `set_fee_rate`
pub struct SetFeeRateAccounts<'a> {
    /// fusion_pools_config
    pub fusion_pools_config: &'a AccountView,
    /// fusion_pool
    pub fusion_pool: &'a AccountView,
    /// fee_authority
    pub fee_authority: &'a AccountView,
}

impl<'a> SetFeeRateAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 3] {
        [
            InstructionAccount::readonly(self.fusion_pools_config.address()),
            InstructionAccount::writable(self.fusion_pool.address()),
            InstructionAccount::readonly_signer(self.fee_authority.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 3] {
        [
            self.fusion_pools_config, self.fusion_pool, self.fee_authority
        ]
    }
}

/// Accounts for `set_position_range`
pub struct SetPositionRangeAccounts<'a> {
    /// position_authority
    pub position_authority: &'a AccountView,
    /// position
    pub position: &'a AccountView,
    /// position_token_account
    pub position_token_account: &'a AccountView,
    /// fusion_pool
    pub fusion_pool: &'a AccountView,
}

impl<'a> SetPositionRangeAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 4] {
        [
            InstructionAccount::readonly_signer(self.position_authority.address()),
            InstructionAccount::writable(self.position.address()),
            InstructionAccount::writable(self.position_token_account.address()),
            InstructionAccount::readonly(self.fusion_pool.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 4] {
        [
            self.position_authority, self.position, self.position_token_account, self.fusion_pool
        ]
    }
}

/// Accounts for `set_protocol_fee_rate`
pub struct SetProtocolFeeRateAccounts<'a> {
    /// fusion_pools_config
    pub fusion_pools_config: &'a AccountView,
    /// fusion_pool
    pub fusion_pool: &'a AccountView,
    /// fee_authority
    pub fee_authority: &'a AccountView,
}

impl<'a> SetProtocolFeeRateAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 3] {
        [
            InstructionAccount::readonly(self.fusion_pools_config.address()),
            InstructionAccount::writable(self.fusion_pool.address()),
            InstructionAccount::readonly_signer(self.fee_authority.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 3] {
        [
            self.fusion_pools_config, self.fusion_pool, self.fee_authority
        ]
    }
}

/// Accounts for `set_token_badge_authority`
pub struct SetTokenBadgeAuthorityAccounts<'a> {
    /// fusion_pools_config
    pub fusion_pools_config: &'a AccountView,
    /// fee_authority
    pub fee_authority: &'a AccountView,
    /// new_token_badge_authority
    pub new_token_badge_authority: &'a AccountView,
}

impl<'a> SetTokenBadgeAuthorityAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 3] {
        [
            InstructionAccount::writable(self.fusion_pools_config.address()),
            InstructionAccount::readonly_signer(self.fee_authority.address()),
            InstructionAccount::readonly(self.new_token_badge_authority.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 3] {
        [
            self.fusion_pools_config, self.fee_authority, self.new_token_badge_authority
        ]
    }
}

/// Accounts for `swap`
pub struct SwapAccounts<'a> {
    /// token_program_a
    pub token_program_a: &'a AccountView,
    /// token_program_b
    pub token_program_b: &'a AccountView,
    /// memo_program
    pub memo_program: &'a AccountView,
    /// token_authority
    pub token_authority: &'a AccountView,
    /// fusion_pool
    pub fusion_pool: &'a AccountView,
    /// token_mint_a
    pub token_mint_a: &'a AccountView,
    /// token_mint_b
    pub token_mint_b: &'a AccountView,
    /// token_owner_account_a
    pub token_owner_account_a: &'a AccountView,
    /// token_owner_account_b
    pub token_owner_account_b: &'a AccountView,
    /// token_vault_a
    pub token_vault_a: &'a AccountView,
    /// token_vault_b
    pub token_vault_b: &'a AccountView,
    /// tick_array_0
    pub tick_array_0: &'a AccountView,
    /// tick_array_1
    pub tick_array_1: &'a AccountView,
    /// tick_array_2
    pub tick_array_2: &'a AccountView,
}

impl<'a> SwapAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 14] {
        [
            InstructionAccount::readonly(self.token_program_a.address()),
            InstructionAccount::readonly(self.token_program_b.address()),
            InstructionAccount::readonly(self.memo_program.address()),
            InstructionAccount::readonly_signer(self.token_authority.address()),
            InstructionAccount::writable(self.fusion_pool.address()),
            InstructionAccount::readonly(self.token_mint_a.address()),
            InstructionAccount::readonly(self.token_mint_b.address()),
            InstructionAccount::writable(self.token_owner_account_a.address()),
            InstructionAccount::writable(self.token_owner_account_b.address()),
            InstructionAccount::writable(self.token_vault_a.address()),
            InstructionAccount::writable(self.token_vault_b.address()),
            InstructionAccount::writable(self.tick_array_0.address()),
            InstructionAccount::writable(self.tick_array_1.address()),
            InstructionAccount::writable(self.tick_array_2.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 14] {
        [
            self.token_program_a, self.token_program_b, self.memo_program, self.token_authority, self.fusion_pool, self.token_mint_a, self.token_mint_b, self.token_owner_account_a, self.token_owner_account_b, self.token_vault_a, self.token_vault_b, self.tick_array_0, self.tick_array_1, self.tick_array_2
        ]
    }
}

/// Accounts for `two_hop_swap`
pub struct TwoHopSwapAccounts<'a> {
    /// fusion_pool_one
    pub fusion_pool_one: &'a AccountView,
    /// fusion_pool_two
    pub fusion_pool_two: &'a AccountView,
    /// token_mint_input
    pub token_mint_input: &'a AccountView,
    /// token_mint_intermediate
    pub token_mint_intermediate: &'a AccountView,
    /// token_mint_output
    pub token_mint_output: &'a AccountView,
    /// token_program_input
    pub token_program_input: &'a AccountView,
    /// token_program_intermediate
    pub token_program_intermediate: &'a AccountView,
    /// token_program_output
    pub token_program_output: &'a AccountView,
    /// token_owner_account_input
    pub token_owner_account_input: &'a AccountView,
    /// token_vault_one_input
    pub token_vault_one_input: &'a AccountView,
    /// token_vault_one_intermediate
    pub token_vault_one_intermediate: &'a AccountView,
    /// token_vault_two_intermediate
    pub token_vault_two_intermediate: &'a AccountView,
    /// token_vault_two_output
    pub token_vault_two_output: &'a AccountView,
    /// token_owner_account_output
    pub token_owner_account_output: &'a AccountView,
    /// token_authority
    pub token_authority: &'a AccountView,
    /// tick_array_one_0
    pub tick_array_one_0: &'a AccountView,
    /// tick_array_one_1
    pub tick_array_one_1: &'a AccountView,
    /// tick_array_one_2
    pub tick_array_one_2: &'a AccountView,
    /// tick_array_two_0
    pub tick_array_two_0: &'a AccountView,
    /// tick_array_two_1
    pub tick_array_two_1: &'a AccountView,
    /// tick_array_two_2
    pub tick_array_two_2: &'a AccountView,
    /// memo_program
    pub memo_program: &'a AccountView,
}

impl<'a> TwoHopSwapAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 22] {
        [
            InstructionAccount::writable(self.fusion_pool_one.address()),
            InstructionAccount::writable(self.fusion_pool_two.address()),
            InstructionAccount::readonly(self.token_mint_input.address()),
            InstructionAccount::readonly(self.token_mint_intermediate.address()),
            InstructionAccount::readonly(self.token_mint_output.address()),
            InstructionAccount::readonly(self.token_program_input.address()),
            InstructionAccount::readonly(self.token_program_intermediate.address()),
            InstructionAccount::readonly(self.token_program_output.address()),
            InstructionAccount::writable(self.token_owner_account_input.address()),
            InstructionAccount::writable(self.token_vault_one_input.address()),
            InstructionAccount::writable(self.token_vault_one_intermediate.address()),
            InstructionAccount::writable(self.token_vault_two_intermediate.address()),
            InstructionAccount::writable(self.token_vault_two_output.address()),
            InstructionAccount::writable(self.token_owner_account_output.address()),
            InstructionAccount::readonly_signer(self.token_authority.address()),
            InstructionAccount::writable(self.tick_array_one_0.address()),
            InstructionAccount::writable(self.tick_array_one_1.address()),
            InstructionAccount::writable(self.tick_array_one_2.address()),
            InstructionAccount::writable(self.tick_array_two_0.address()),
            InstructionAccount::writable(self.tick_array_two_1.address()),
            InstructionAccount::writable(self.tick_array_two_2.address()),
            InstructionAccount::readonly(self.memo_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 22] {
        [
            self.fusion_pool_one, self.fusion_pool_two, self.token_mint_input, self.token_mint_intermediate, self.token_mint_output, self.token_program_input, self.token_program_intermediate, self.token_program_output, self.token_owner_account_input, self.token_vault_one_input, self.token_vault_one_intermediate, self.token_vault_two_intermediate, self.token_vault_two_output, self.token_owner_account_output, self.token_authority, self.tick_array_one_0, self.tick_array_one_1, self.tick_array_one_2, self.tick_array_two_0, self.tick_array_two_1, self.tick_array_two_2, self.memo_program
        ]
    }
}

/// Accounts for `update_fees`
pub struct UpdateFeesAccounts<'a> {
    /// fusion_pool
    pub fusion_pool: &'a AccountView,
    /// position
    pub position: &'a AccountView,
    /// tick_array_lower
    pub tick_array_lower: &'a AccountView,
    /// tick_array_upper
    pub tick_array_upper: &'a AccountView,
}

impl<'a> UpdateFeesAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 4] {
        [
            InstructionAccount::writable(self.fusion_pool.address()),
            InstructionAccount::writable(self.position.address()),
            InstructionAccount::readonly(self.tick_array_lower.address()),
            InstructionAccount::readonly(self.tick_array_upper.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 4] {
        [
            self.fusion_pool, self.position, self.tick_array_lower, self.tick_array_upper
        ]
    }
}


// ============================================
// CPI Functions
// ============================================
/// CPI: close_bundled_position
#[inline(always)]
pub fn close_bundled_position<'a>(
    accounts: &CloseBundledPositionAccounts<'a>, args: &CloseBundledPositionArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<CloseBundledPositionArgs>()];
    data[0..8].copy_from_slice(&CLOSE_BUNDLED_POSITION);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const CloseBundledPositionArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<CloseBundledPositionArgs>(),
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

/// CPI: close_limit_order
#[inline(always)]
pub fn close_limit_order<'a>(
    accounts: &CloseLimitOrderAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&CLOSE_LIMIT_ORDER);
    
    
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
    invoke_signed::<6>(&instruction, &account_views, signers)
}

/// CPI: collect_fees
#[inline(always)]
pub fn collect_fees<'a>(
    accounts: &CollectFeesAccounts<'a>, args: &CollectFeesArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<CollectFeesArgs>()];
    data[0..8].copy_from_slice(&COLLECT_FEES);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const CollectFeesArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<CollectFeesArgs>(),
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

/// CPI: collect_protocol_fees
#[inline(always)]
pub fn collect_protocol_fees<'a>(
    accounts: &CollectProtocolFeesAccounts<'a>, args: &CollectProtocolFeesArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<CollectProtocolFeesArgs>()];
    data[0..8].copy_from_slice(&COLLECT_PROTOCOL_FEES);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const CollectProtocolFeesArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<CollectProtocolFeesArgs>(),
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
    invoke_signed::<12>(&instruction, &account_views, signers)
}

/// CPI: decrease_limit_order
#[inline(always)]
pub fn decrease_limit_order<'a>(
    accounts: &DecreaseLimitOrderAccounts<'a>, args: &DecreaseLimitOrderArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<DecreaseLimitOrderArgs>()];
    data[0..8].copy_from_slice(&DECREASE_LIMIT_ORDER);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const DecreaseLimitOrderArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<DecreaseLimitOrderArgs>(),
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

/// CPI: decrease_liquidity
#[inline(always)]
pub fn decrease_liquidity<'a>(
    accounts: &DecreaseLiquidityAccounts<'a>, args: &DecreaseLiquidityArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<DecreaseLiquidityArgs>()];
    data[0..8].copy_from_slice(&DECREASE_LIQUIDITY);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const DecreaseLiquidityArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<DecreaseLiquidityArgs>(),
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

/// CPI: delete_position_bundle
#[inline(always)]
pub fn delete_position_bundle<'a>(
    accounts: &DeletePositionBundleAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&DELETE_POSITION_BUNDLE);
    
    
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

/// CPI: delete_token_badge
#[inline(always)]
pub fn delete_token_badge<'a>(
    accounts: &DeleteTokenBadgeAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&DELETE_TOKEN_BADGE);
    
    
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

/// CPI: increase_limit_order
#[inline(always)]
pub fn increase_limit_order<'a>(
    accounts: &IncreaseLimitOrderAccounts<'a>, args: &IncreaseLimitOrderArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<IncreaseLimitOrderArgs>()];
    data[0..8].copy_from_slice(&INCREASE_LIMIT_ORDER);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const IncreaseLimitOrderArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<IncreaseLimitOrderArgs>(),
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

/// CPI: increase_liquidity
#[inline(always)]
pub fn increase_liquidity<'a>(
    accounts: &IncreaseLiquidityAccounts<'a>, args: &IncreaseLiquidityArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<IncreaseLiquidityArgs>()];
    data[0..8].copy_from_slice(&INCREASE_LIQUIDITY);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const IncreaseLiquidityArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<IncreaseLiquidityArgs>(),
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

/// CPI: initialize_config
#[inline(always)]
pub fn initialize_config<'a>(
    accounts: &InitializeConfigAccounts<'a>, args: &InitializeConfigArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<InitializeConfigArgs>()];
    data[0..8].copy_from_slice(&INITIALIZE_CONFIG);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const InitializeConfigArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<InitializeConfigArgs>(),
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
    invoke_signed::<13>(&instruction, &account_views, signers)
}

/// CPI: initialize_position_bundle
#[inline(always)]
pub fn initialize_position_bundle<'a>(
    accounts: &InitializePositionBundleAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&INITIALIZE_POSITION_BUNDLE);
    
    
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

/// CPI: initialize_position_bundle_with_metadata
#[inline(always)]
pub fn initialize_position_bundle_with_metadata<'a>(
    accounts: &InitializePositionBundleWithMetadataAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&INITIALIZE_POSITION_BUNDLE_WITH_METADATA);
    
    
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
    invoke_signed::<12>(&instruction, &account_views, signers)
}

/// CPI: initialize_tick_array
#[inline(always)]
pub fn initialize_tick_array<'a>(
    accounts: &InitializeTickArrayAccounts<'a>, args: &InitializeTickArrayArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<InitializeTickArrayArgs>()];
    data[0..8].copy_from_slice(&INITIALIZE_TICK_ARRAY);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const InitializeTickArrayArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<InitializeTickArrayArgs>(),
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

/// CPI: initialize_token_badge
#[inline(always)]
pub fn initialize_token_badge<'a>(
    accounts: &InitializeTokenBadgeAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&INITIALIZE_TOKEN_BADGE);
    
    
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

/// CPI: open_bundled_position
#[inline(always)]
pub fn open_bundled_position<'a>(
    accounts: &OpenBundledPositionAccounts<'a>, args: &OpenBundledPositionArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<OpenBundledPositionArgs>()];
    data[0..8].copy_from_slice(&OPEN_BUNDLED_POSITION);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const OpenBundledPositionArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<OpenBundledPositionArgs>(),
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

/// CPI: open_limit_order
#[inline(always)]
pub fn open_limit_order<'a>(
    accounts: &OpenLimitOrderAccounts<'a>, args: &OpenLimitOrderArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<OpenLimitOrderArgs>()];
    data[0..8].copy_from_slice(&OPEN_LIMIT_ORDER);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const OpenLimitOrderArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<OpenLimitOrderArgs>(),
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

/// CPI: open_position
#[inline(always)]
pub fn open_position<'a>(
    accounts: &OpenPositionAccounts<'a>, args: &OpenPositionArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<OpenPositionArgs>()];
    data[0..8].copy_from_slice(&OPEN_POSITION);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const OpenPositionArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<OpenPositionArgs>(),
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

/// CPI: reset_pool_price
#[inline(always)]
pub fn reset_pool_price<'a>(
    accounts: &ResetPoolPriceAccounts<'a>, args: &ResetPoolPriceArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<ResetPoolPriceArgs>()];
    data[0..8].copy_from_slice(&RESET_POOL_PRICE);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const ResetPoolPriceArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<ResetPoolPriceArgs>(),
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

/// CPI: set_collect_protocol_fees_authority
#[inline(always)]
pub fn set_collect_protocol_fees_authority<'a>(
    accounts: &SetCollectProtocolFeesAuthorityAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&SET_COLLECT_PROTOCOL_FEES_AUTHORITY);
    
    
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

/// CPI: set_default_protocol_fee_rate
#[inline(always)]
pub fn set_default_protocol_fee_rate<'a>(
    accounts: &SetDefaultProtocolFeeRateAccounts<'a>, args: &SetDefaultProtocolFeeRateArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SetDefaultProtocolFeeRateArgs>()];
    data[0..8].copy_from_slice(&SET_DEFAULT_PROTOCOL_FEE_RATE);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SetDefaultProtocolFeeRateArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SetDefaultProtocolFeeRateArgs>(),
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
    invoke_signed::<2>(&instruction, &account_views, signers)
}

/// CPI: set_fee_authority
#[inline(always)]
pub fn set_fee_authority<'a>(
    accounts: &SetFeeAuthorityAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&SET_FEE_AUTHORITY);
    
    
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

/// CPI: set_fee_rate
#[inline(always)]
pub fn set_fee_rate<'a>(
    accounts: &SetFeeRateAccounts<'a>, args: &SetFeeRateArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SetFeeRateArgs>()];
    data[0..8].copy_from_slice(&SET_FEE_RATE);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SetFeeRateArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SetFeeRateArgs>(),
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

/// CPI: set_position_range
#[inline(always)]
pub fn set_position_range<'a>(
    accounts: &SetPositionRangeAccounts<'a>, args: &SetPositionRangeArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SetPositionRangeArgs>()];
    data[0..8].copy_from_slice(&SET_POSITION_RANGE);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SetPositionRangeArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SetPositionRangeArgs>(),
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

/// CPI: set_protocol_fee_rate
#[inline(always)]
pub fn set_protocol_fee_rate<'a>(
    accounts: &SetProtocolFeeRateAccounts<'a>, args: &SetProtocolFeeRateArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SetProtocolFeeRateArgs>()];
    data[0..8].copy_from_slice(&SET_PROTOCOL_FEE_RATE);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SetProtocolFeeRateArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SetProtocolFeeRateArgs>(),
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

/// CPI: set_token_badge_authority
#[inline(always)]
pub fn set_token_badge_authority<'a>(
    accounts: &SetTokenBadgeAuthorityAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&SET_TOKEN_BADGE_AUTHORITY);
    
    
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

/// CPI: two_hop_swap
#[inline(always)]
pub fn two_hop_swap<'a>(
    accounts: &TwoHopSwapAccounts<'a>, args: &TwoHopSwapArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<TwoHopSwapArgs>()];
    data[0..8].copy_from_slice(&TWO_HOP_SWAP);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const TwoHopSwapArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<TwoHopSwapArgs>(),
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
    invoke_signed::<22>(&instruction, &account_views, signers)
}

/// CPI: update_fees
#[inline(always)]
pub fn update_fees<'a>(
    accounts: &UpdateFeesAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&UPDATE_FEES);
    
    
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

