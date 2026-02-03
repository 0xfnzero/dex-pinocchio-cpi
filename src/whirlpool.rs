//! CPI module for Whirlpool
//!
//! Program: whirlpool
//! Program ID: whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc
//! Instructions: 58

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
pub const ID: Address = Address::new_from_array(five8_const::decode_32_const("whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc"));

// ============================================
// Instruction Discriminators
// ============================================
pub const INITIALIZE_CONFIG: [u8; 8] = [208, 127, 21, 1, 194, 190, 196, 70];
pub const INITIALIZE_POOL: [u8; 8] = [95, 180, 10, 172, 84, 174, 232, 40];
pub const INITIALIZE_TICK_ARRAY: [u8; 8] = [11, 188, 193, 214, 141, 91, 149, 184];
pub const INITIALIZE_FEE_TIER: [u8; 8] = [183, 74, 156, 160, 112, 2, 42, 30];
pub const INITIALIZE_REWARD: [u8; 8] = [95, 135, 192, 196, 242, 129, 230, 68];
pub const SET_REWARD_EMISSIONS: [u8; 8] = [13, 197, 86, 168, 109, 176, 27, 244];
pub const OPEN_POSITION: [u8; 8] = [135, 128, 47, 77, 15, 152, 240, 49];
pub const OPEN_POSITION_WITH_METADATA: [u8; 8] = [242, 29, 134, 48, 58, 110, 14, 60];
pub const INCREASE_LIQUIDITY: [u8; 8] = [46, 156, 243, 118, 13, 205, 251, 178];
pub const DECREASE_LIQUIDITY: [u8; 8] = [160, 38, 208, 111, 104, 91, 44, 1];
pub const UPDATE_FEES_AND_REWARDS: [u8; 8] = [154, 230, 250, 13, 236, 209, 75, 223];
pub const COLLECT_FEES: [u8; 8] = [164, 152, 207, 99, 30, 186, 19, 182];
pub const COLLECT_REWARD: [u8; 8] = [70, 5, 132, 87, 86, 235, 177, 34];
pub const COLLECT_PROTOCOL_FEES: [u8; 8] = [22, 67, 23, 98, 150, 178, 70, 220];
pub const SWAP: [u8; 8] = [248, 198, 158, 145, 225, 117, 135, 200];
pub const CLOSE_POSITION: [u8; 8] = [123, 134, 81, 0, 49, 68, 98, 98];
pub const SET_DEFAULT_FEE_RATE: [u8; 8] = [118, 215, 214, 157, 182, 229, 208, 228];
pub const SET_DEFAULT_PROTOCOL_FEE_RATE: [u8; 8] = [107, 205, 249, 226, 151, 35, 86, 0];
pub const SET_FEE_RATE: [u8; 8] = [53, 243, 137, 65, 8, 140, 158, 6];
pub const SET_PROTOCOL_FEE_RATE: [u8; 8] = [95, 7, 4, 50, 154, 79, 156, 131];
pub const SET_FEE_AUTHORITY: [u8; 8] = [31, 1, 50, 87, 237, 101, 97, 132];
pub const SET_COLLECT_PROTOCOL_FEES_AUTHORITY: [u8; 8] = [34, 150, 93, 244, 139, 225, 233, 67];
pub const SET_REWARD_AUTHORITY: [u8; 8] = [34, 39, 183, 252, 83, 28, 85, 127];
pub const SET_REWARD_AUTHORITY_BY_SUPER_AUTHORITY: [u8; 8] = [240, 154, 201, 198, 148, 93, 56, 25];
pub const SET_REWARD_EMISSIONS_SUPER_AUTHORITY: [u8; 8] = [207, 5, 200, 209, 122, 56, 82, 183];
pub const TWO_HOP_SWAP: [u8; 8] = [195, 96, 237, 108, 68, 162, 219, 230];
pub const INITIALIZE_POSITION_BUNDLE: [u8; 8] = [117, 45, 241, 149, 24, 18, 194, 65];
pub const INITIALIZE_POSITION_BUNDLE_WITH_METADATA: [u8; 8] = [93, 124, 16, 179, 249, 131, 115, 245];
pub const DELETE_POSITION_BUNDLE: [u8; 8] = [100, 25, 99, 2, 217, 239, 124, 173];
pub const OPEN_BUNDLED_POSITION: [u8; 8] = [169, 113, 126, 171, 213, 172, 212, 49];
pub const CLOSE_BUNDLED_POSITION: [u8; 8] = [41, 36, 216, 245, 27, 85, 103, 67];
pub const OPEN_POSITION_WITH_TOKEN_EXTENSIONS: [u8; 8] = [212, 47, 95, 92, 114, 102, 131, 250];
pub const CLOSE_POSITION_WITH_TOKEN_EXTENSIONS: [u8; 8] = [1, 182, 135, 59, 155, 25, 99, 223];
pub const LOCK_POSITION: [u8; 8] = [227, 62, 2, 252, 247, 10, 171, 185];
pub const RESET_POSITION_RANGE: [u8; 8] = [164, 123, 180, 141, 194, 100, 160, 175];
pub const TRANSFER_LOCKED_POSITION: [u8; 8] = [179, 121, 229, 46, 67, 138, 194, 138];
pub const INITIALIZE_ADAPTIVE_FEE_TIER: [u8; 8] = [77, 99, 208, 200, 141, 123, 117, 48];
pub const SET_DEFAULT_BASE_FEE_RATE: [u8; 8] = [229, 66, 84, 251, 164, 134, 183, 7];
pub const SET_DELEGATED_FEE_AUTHORITY: [u8; 8] = [193, 234, 231, 147, 138, 57, 3, 122];
pub const SET_INITIALIZE_POOL_AUTHORITY: [u8; 8] = [125, 43, 127, 235, 149, 26, 106, 236];
pub const SET_PRESET_ADAPTIVE_FEE_CONSTANTS: [u8; 8] = [132, 185, 66, 148, 83, 88, 134, 198];
pub const INITIALIZE_POOL_WITH_ADAPTIVE_FEE: [u8; 8] = [143, 94, 96, 76, 172, 124, 119, 199];
pub const SET_FEE_RATE_BY_DELEGATED_FEE_AUTHORITY: [u8; 8] = [121, 121, 54, 114, 131, 230, 162, 104];
pub const COLLECT_FEES_V2: [u8; 8] = [207, 117, 95, 191, 229, 180, 226, 15];
pub const COLLECT_PROTOCOL_FEES_V2: [u8; 8] = [103, 128, 222, 134, 114, 200, 22, 200];
pub const COLLECT_REWARD_V2: [u8; 8] = [177, 107, 37, 180, 160, 19, 49, 209];
pub const DECREASE_LIQUIDITY_V2: [u8; 8] = [58, 127, 188, 62, 79, 82, 196, 96];
pub const INCREASE_LIQUIDITY_V2: [u8; 8] = [133, 29, 89, 223, 69, 238, 176, 10];
pub const INITIALIZE_POOL_V2: [u8; 8] = [207, 45, 87, 242, 27, 63, 204, 67];
pub const INITIALIZE_REWARD_V2: [u8; 8] = [91, 1, 77, 50, 235, 229, 133, 49];
pub const SET_REWARD_EMISSIONS_V2: [u8; 8] = [114, 228, 72, 32, 193, 48, 160, 102];
pub const SWAP_V2: [u8; 8] = [43, 4, 237, 11, 26, 201, 30, 98];
pub const TWO_HOP_SWAP_V2: [u8; 8] = [186, 143, 209, 29, 254, 2, 194, 117];
pub const INITIALIZE_CONFIG_EXTENSION: [u8; 8] = [55, 9, 53, 9, 114, 57, 209, 52];
pub const SET_CONFIG_EXTENSION_AUTHORITY: [u8; 8] = [44, 94, 241, 116, 24, 188, 60, 143];
pub const SET_TOKEN_BADGE_AUTHORITY: [u8; 8] = [207, 202, 4, 32, 205, 79, 13, 178];
pub const INITIALIZE_TOKEN_BADGE: [u8; 8] = [253, 77, 205, 95, 27, 224, 89, 223];
pub const DELETE_TOKEN_BADGE: [u8; 8] = [53, 146, 68, 8, 18, 117, 17, 185];

// ============================================
// Instruction Arguments
// ============================================
/// Arguments for `initializeConfig`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct InitializeconfigArgs {
    pub fee_authority: [u8; 32],
    pub collect_protocol_fees_authority: [u8; 32],
    pub reward_emissions_super_authority: [u8; 32],
    pub default_protocol_fee_rate: u16,
}

/// Arguments for `initializePool`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct InitializepoolArgs {
    pub bumps: [u8; 32],
    pub tick_spacing: u16,
    pub initial_sqrt_price: u128,
}

/// Arguments for `initializeTickArray`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct InitializetickarrayArgs {
    pub start_tick_index: i32,
}

/// Arguments for `initializeFeeTier`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct InitializefeetierArgs {
    pub tick_spacing: u16,
    pub default_fee_rate: u16,
}

/// Arguments for `initializeReward`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct InitializerewardArgs {
    pub reward_index: u8,
}

/// Arguments for `setRewardEmissions`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SetrewardemissionsArgs {
    pub reward_index: u8,
    pub emissions_per_second_x64: u128,
}

/// Arguments for `openPosition`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct OpenpositionArgs {
    pub bumps: [u8; 32],
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
}

/// Arguments for `openPositionWithMetadata`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct OpenpositionwithmetadataArgs {
    pub bumps: [u8; 32],
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
}

/// Arguments for `increaseLiquidity`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct IncreaseliquidityArgs {
    pub liquidity_amount: u128,
    pub token_max_a: u64,
    pub token_max_b: u64,
}

/// Arguments for `decreaseLiquidity`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct DecreaseliquidityArgs {
    pub liquidity_amount: u128,
    pub token_min_a: u64,
    pub token_min_b: u64,
}

/// Arguments for `collectReward`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct CollectrewardArgs {
    pub reward_index: u8,
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
}

/// Arguments for `setDefaultFeeRate`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SetdefaultfeerateArgs {
    pub default_fee_rate: u16,
}

/// Arguments for `setDefaultProtocolFeeRate`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SetdefaultprotocolfeerateArgs {
    pub default_protocol_fee_rate: u16,
}

/// Arguments for `setFeeRate`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SetfeerateArgs {
    pub fee_rate: u16,
}

/// Arguments for `setProtocolFeeRate`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SetprotocolfeerateArgs {
    pub protocol_fee_rate: u16,
}

/// Arguments for `setRewardAuthority`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SetrewardauthorityArgs {
    pub reward_index: u8,
}

/// Arguments for `setRewardAuthorityBySuperAuthority`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SetrewardauthoritybysuperauthorityArgs {
    pub reward_index: u8,
}

/// Arguments for `twoHopSwap`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct TwohopswapArgs {
    pub amount: u64,
    pub other_amount_threshold: u64,
    pub amount_specified_is_input: bool,
    pub a_to_b_one: bool,
    pub a_to_b_two: bool,
    pub sqrt_price_limit_one: u128,
    pub sqrt_price_limit_two: u128,
}

/// Arguments for `openBundledPosition`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct OpenbundledpositionArgs {
    pub bundle_index: u16,
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
}

/// Arguments for `closeBundledPosition`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct ClosebundledpositionArgs {
    pub bundle_index: u16,
}

/// Arguments for `openPositionWithTokenExtensions`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct OpenpositionwithtokenextensionsArgs {
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
    pub with_token_metadata_extension: bool,
}

/// Arguments for `lockPosition`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct LockpositionArgs {
    pub lock_type: [u8; 32],
}

/// Arguments for `resetPositionRange`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct ResetpositionrangeArgs {
    pub new_tick_lower_index: i32,
    pub new_tick_upper_index: i32,
}

/// Arguments for `initializeAdaptiveFeeTier`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct InitializeadaptivefeetierArgs {
    pub fee_tier_index: u16,
    pub tick_spacing: u16,
    pub initialize_pool_authority: [u8; 32],
    pub delegated_fee_authority: [u8; 32],
    pub default_base_fee_rate: u16,
    pub filter_period: u16,
    pub decay_period: u16,
    pub reduction_factor: u16,
    pub adaptive_fee_control_factor: u32,
    pub max_volatility_accumulator: u32,
    pub tick_group_size: u16,
    pub major_swap_threshold_ticks: u16,
}

/// Arguments for `setDefaultBaseFeeRate`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SetdefaultbasefeerateArgs {
    pub default_base_fee_rate: u16,
}

/// Arguments for `setPresetAdaptiveFeeConstants`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SetpresetadaptivefeeconstantsArgs {
    pub filter_period: u16,
    pub decay_period: u16,
    pub reduction_factor: u16,
    pub adaptive_fee_control_factor: u32,
    pub max_volatility_accumulator: u32,
    pub tick_group_size: u16,
    pub major_swap_threshold_ticks: u16,
}

/// Arguments for `initializePoolWithAdaptiveFee`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct InitializepoolwithadaptivefeeArgs {
    pub initial_sqrt_price: u128,
    pub trade_enable_timestamp: Option<u64>,
}

/// Arguments for `setFeeRateByDelegatedFeeAuthority`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SetfeeratebydelegatedfeeauthorityArgs {
    pub fee_rate: u16,
}

/// Arguments for `collectFeesV2`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct Collectfeesv2Args {
    pub remaining_accounts_info: Option<[u8; 32]>,
}

/// Arguments for `collectProtocolFeesV2`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct Collectprotocolfeesv2Args {
    pub remaining_accounts_info: Option<[u8; 32]>,
}

/// Arguments for `collectRewardV2`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct Collectrewardv2Args {
    pub reward_index: u8,
    pub remaining_accounts_info: Option<[u8; 32]>,
}

/// Arguments for `decreaseLiquidityV2`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct Decreaseliquidityv2Args {
    pub liquidity_amount: u128,
    pub token_min_a: u64,
    pub token_min_b: u64,
    pub remaining_accounts_info: Option<[u8; 32]>,
}

/// Arguments for `increaseLiquidityV2`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct Increaseliquidityv2Args {
    pub liquidity_amount: u128,
    pub token_max_a: u64,
    pub token_max_b: u64,
    pub remaining_accounts_info: Option<[u8; 32]>,
}

/// Arguments for `initializePoolV2`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct Initializepoolv2Args {
    pub tick_spacing: u16,
    pub initial_sqrt_price: u128,
}

/// Arguments for `initializeRewardV2`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct Initializerewardv2Args {
    pub reward_index: u8,
}

/// Arguments for `setRewardEmissionsV2`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct Setrewardemissionsv2Args {
    pub reward_index: u8,
    pub emissions_per_second_x64: u128,
}

/// Arguments for `swapV2`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct Swapv2Args {
    pub amount: u64,
    pub other_amount_threshold: u64,
    pub sqrt_price_limit: u128,
    pub amount_specified_is_input: bool,
    pub a_to_b: bool,
    pub remaining_accounts_info: Option<[u8; 32]>,
}

/// Arguments for `twoHopSwapV2`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct Twohopswapv2Args {
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
/// Accounts for `initializeConfig`
pub struct InitializeconfigAccounts<'a> {
    /// config
    pub config: &'a AccountView,
    /// funder
    pub funder: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
}

impl<'a> InitializeconfigAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 3] {
        [
            InstructionAccount::writable_signer(self.config.address()),
            InstructionAccount::writable_signer(self.funder.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 3] {
        [
            self.config, self.funder, self.system_program
        ]
    }
}

/// Accounts for `initializePool`
pub struct InitializepoolAccounts<'a> {
    /// whirlpoolsConfig
    pub whirlpools_config: &'a AccountView,
    /// tokenMintA
    pub token_mint_a: &'a AccountView,
    /// tokenMintB
    pub token_mint_b: &'a AccountView,
    /// funder
    pub funder: &'a AccountView,
    /// whirlpool
    pub whirlpool: &'a AccountView,
    /// tokenVaultA
    pub token_vault_a: &'a AccountView,
    /// tokenVaultB
    pub token_vault_b: &'a AccountView,
    /// feeTier
    pub fee_tier: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
    /// rent
    pub rent: &'a AccountView,
}

impl<'a> InitializepoolAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 11] {
        [
            InstructionAccount::readonly(self.whirlpools_config.address()),
            InstructionAccount::readonly(self.token_mint_a.address()),
            InstructionAccount::readonly(self.token_mint_b.address()),
            InstructionAccount::writable_signer(self.funder.address()),
            InstructionAccount::writable(self.whirlpool.address()),
            InstructionAccount::writable_signer(self.token_vault_a.address()),
            InstructionAccount::writable_signer(self.token_vault_b.address()),
            InstructionAccount::readonly(self.fee_tier.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.rent.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 11] {
        [
            self.whirlpools_config, self.token_mint_a, self.token_mint_b, self.funder, self.whirlpool, self.token_vault_a, self.token_vault_b, self.fee_tier, self.token_program, self.system_program, self.rent
        ]
    }
}

/// Accounts for `initializeTickArray`
pub struct InitializetickarrayAccounts<'a> {
    /// whirlpool
    pub whirlpool: &'a AccountView,
    /// funder
    pub funder: &'a AccountView,
    /// tickArray
    pub tick_array: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
}

impl<'a> InitializetickarrayAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 4] {
        [
            InstructionAccount::readonly(self.whirlpool.address()),
            InstructionAccount::writable_signer(self.funder.address()),
            InstructionAccount::writable(self.tick_array.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 4] {
        [
            self.whirlpool, self.funder, self.tick_array, self.system_program
        ]
    }
}

/// Accounts for `initializeFeeTier`
pub struct InitializefeetierAccounts<'a> {
    /// config
    pub config: &'a AccountView,
    /// feeTier
    pub fee_tier: &'a AccountView,
    /// funder
    pub funder: &'a AccountView,
    /// feeAuthority
    pub fee_authority: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
}

impl<'a> InitializefeetierAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 5] {
        [
            InstructionAccount::readonly(self.config.address()),
            InstructionAccount::writable(self.fee_tier.address()),
            InstructionAccount::writable_signer(self.funder.address()),
            InstructionAccount::readonly_signer(self.fee_authority.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 5] {
        [
            self.config, self.fee_tier, self.funder, self.fee_authority, self.system_program
        ]
    }
}

/// Accounts for `initializeReward`
pub struct InitializerewardAccounts<'a> {
    /// rewardAuthority
    pub reward_authority: &'a AccountView,
    /// funder
    pub funder: &'a AccountView,
    /// whirlpool
    pub whirlpool: &'a AccountView,
    /// rewardMint
    pub reward_mint: &'a AccountView,
    /// rewardVault
    pub reward_vault: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
    /// rent
    pub rent: &'a AccountView,
}

impl<'a> InitializerewardAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 8] {
        [
            InstructionAccount::readonly_signer(self.reward_authority.address()),
            InstructionAccount::writable_signer(self.funder.address()),
            InstructionAccount::writable(self.whirlpool.address()),
            InstructionAccount::readonly(self.reward_mint.address()),
            InstructionAccount::writable_signer(self.reward_vault.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.rent.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 8] {
        [
            self.reward_authority, self.funder, self.whirlpool, self.reward_mint, self.reward_vault, self.token_program, self.system_program, self.rent
        ]
    }
}

/// Accounts for `setRewardEmissions`
pub struct SetrewardemissionsAccounts<'a> {
    /// whirlpool
    pub whirlpool: &'a AccountView,
    /// rewardAuthority
    pub reward_authority: &'a AccountView,
    /// rewardVault
    pub reward_vault: &'a AccountView,
}

impl<'a> SetrewardemissionsAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 3] {
        [
            InstructionAccount::writable(self.whirlpool.address()),
            InstructionAccount::readonly_signer(self.reward_authority.address()),
            InstructionAccount::readonly(self.reward_vault.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 3] {
        [
            self.whirlpool, self.reward_authority, self.reward_vault
        ]
    }
}

/// Accounts for `openPosition`
pub struct OpenpositionAccounts<'a> {
    /// funder
    pub funder: &'a AccountView,
    /// owner
    pub owner: &'a AccountView,
    /// position
    pub position: &'a AccountView,
    /// positionMint
    pub position_mint: &'a AccountView,
    /// positionTokenAccount
    pub position_token_account: &'a AccountView,
    /// whirlpool
    pub whirlpool: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
    /// rent
    pub rent: &'a AccountView,
    /// associatedTokenProgram
    pub associated_token_program: &'a AccountView,
}

impl<'a> OpenpositionAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 10] {
        [
            InstructionAccount::writable_signer(self.funder.address()),
            InstructionAccount::readonly(self.owner.address()),
            InstructionAccount::writable(self.position.address()),
            InstructionAccount::writable_signer(self.position_mint.address()),
            InstructionAccount::writable(self.position_token_account.address()),
            InstructionAccount::readonly(self.whirlpool.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.rent.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 10] {
        [
            self.funder, self.owner, self.position, self.position_mint, self.position_token_account, self.whirlpool, self.token_program, self.system_program, self.rent, self.associated_token_program
        ]
    }
}

/// Accounts for `openPositionWithMetadata`
pub struct OpenpositionwithmetadataAccounts<'a> {
    /// funder
    pub funder: &'a AccountView,
    /// owner
    pub owner: &'a AccountView,
    /// position
    pub position: &'a AccountView,
    /// positionMint
    pub position_mint: &'a AccountView,
    /// positionMetadataAccount
    pub position_metadata_account: &'a AccountView,
    /// positionTokenAccount
    pub position_token_account: &'a AccountView,
    /// whirlpool
    pub whirlpool: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
    /// rent
    pub rent: &'a AccountView,
    /// associatedTokenProgram
    pub associated_token_program: &'a AccountView,
    /// metadataProgram
    pub metadata_program: &'a AccountView,
    /// metadataUpdateAuth
    pub metadata_update_auth: &'a AccountView,
}

impl<'a> OpenpositionwithmetadataAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 13] {
        [
            InstructionAccount::writable_signer(self.funder.address()),
            InstructionAccount::readonly(self.owner.address()),
            InstructionAccount::writable(self.position.address()),
            InstructionAccount::writable_signer(self.position_mint.address()),
            InstructionAccount::writable(self.position_metadata_account.address()),
            InstructionAccount::writable(self.position_token_account.address()),
            InstructionAccount::readonly(self.whirlpool.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.rent.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
            InstructionAccount::readonly(self.metadata_program.address()),
            InstructionAccount::readonly(self.metadata_update_auth.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 13] {
        [
            self.funder, self.owner, self.position, self.position_mint, self.position_metadata_account, self.position_token_account, self.whirlpool, self.token_program, self.system_program, self.rent, self.associated_token_program, self.metadata_program, self.metadata_update_auth
        ]
    }
}

/// Accounts for `increaseLiquidity`
pub struct IncreaseliquidityAccounts<'a> {
    /// whirlpool
    pub whirlpool: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// positionAuthority
    pub position_authority: &'a AccountView,
    /// position
    pub position: &'a AccountView,
    /// positionTokenAccount
    pub position_token_account: &'a AccountView,
    /// tokenOwnerAccountA
    pub token_owner_account_a: &'a AccountView,
    /// tokenOwnerAccountB
    pub token_owner_account_b: &'a AccountView,
    /// tokenVaultA
    pub token_vault_a: &'a AccountView,
    /// tokenVaultB
    pub token_vault_b: &'a AccountView,
    /// tickArrayLower
    pub tick_array_lower: &'a AccountView,
    /// tickArrayUpper
    pub tick_array_upper: &'a AccountView,
}

impl<'a> IncreaseliquidityAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 11] {
        [
            InstructionAccount::writable(self.whirlpool.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly_signer(self.position_authority.address()),
            InstructionAccount::writable(self.position.address()),
            InstructionAccount::readonly(self.position_token_account.address()),
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
    pub fn to_views(&self) -> [&'a AccountView; 11] {
        [
            self.whirlpool, self.token_program, self.position_authority, self.position, self.position_token_account, self.token_owner_account_a, self.token_owner_account_b, self.token_vault_a, self.token_vault_b, self.tick_array_lower, self.tick_array_upper
        ]
    }
}

/// Accounts for `decreaseLiquidity`
pub struct DecreaseliquidityAccounts<'a> {
    /// whirlpool
    pub whirlpool: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// positionAuthority
    pub position_authority: &'a AccountView,
    /// position
    pub position: &'a AccountView,
    /// positionTokenAccount
    pub position_token_account: &'a AccountView,
    /// tokenOwnerAccountA
    pub token_owner_account_a: &'a AccountView,
    /// tokenOwnerAccountB
    pub token_owner_account_b: &'a AccountView,
    /// tokenVaultA
    pub token_vault_a: &'a AccountView,
    /// tokenVaultB
    pub token_vault_b: &'a AccountView,
    /// tickArrayLower
    pub tick_array_lower: &'a AccountView,
    /// tickArrayUpper
    pub tick_array_upper: &'a AccountView,
}

impl<'a> DecreaseliquidityAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 11] {
        [
            InstructionAccount::writable(self.whirlpool.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly_signer(self.position_authority.address()),
            InstructionAccount::writable(self.position.address()),
            InstructionAccount::readonly(self.position_token_account.address()),
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
    pub fn to_views(&self) -> [&'a AccountView; 11] {
        [
            self.whirlpool, self.token_program, self.position_authority, self.position, self.position_token_account, self.token_owner_account_a, self.token_owner_account_b, self.token_vault_a, self.token_vault_b, self.tick_array_lower, self.tick_array_upper
        ]
    }
}

/// Accounts for `updateFeesAndRewards`
pub struct UpdatefeesandrewardsAccounts<'a> {
    /// whirlpool
    pub whirlpool: &'a AccountView,
    /// position
    pub position: &'a AccountView,
    /// tickArrayLower
    pub tick_array_lower: &'a AccountView,
    /// tickArrayUpper
    pub tick_array_upper: &'a AccountView,
}

impl<'a> UpdatefeesandrewardsAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 4] {
        [
            InstructionAccount::writable(self.whirlpool.address()),
            InstructionAccount::writable(self.position.address()),
            InstructionAccount::readonly(self.tick_array_lower.address()),
            InstructionAccount::readonly(self.tick_array_upper.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 4] {
        [
            self.whirlpool, self.position, self.tick_array_lower, self.tick_array_upper
        ]
    }
}

/// Accounts for `collectFees`
pub struct CollectfeesAccounts<'a> {
    /// whirlpool
    pub whirlpool: &'a AccountView,
    /// positionAuthority
    pub position_authority: &'a AccountView,
    /// position
    pub position: &'a AccountView,
    /// positionTokenAccount
    pub position_token_account: &'a AccountView,
    /// tokenOwnerAccountA
    pub token_owner_account_a: &'a AccountView,
    /// tokenVaultA
    pub token_vault_a: &'a AccountView,
    /// tokenOwnerAccountB
    pub token_owner_account_b: &'a AccountView,
    /// tokenVaultB
    pub token_vault_b: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
}

impl<'a> CollectfeesAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 9] {
        [
            InstructionAccount::readonly(self.whirlpool.address()),
            InstructionAccount::readonly_signer(self.position_authority.address()),
            InstructionAccount::writable(self.position.address()),
            InstructionAccount::readonly(self.position_token_account.address()),
            InstructionAccount::writable(self.token_owner_account_a.address()),
            InstructionAccount::writable(self.token_vault_a.address()),
            InstructionAccount::writable(self.token_owner_account_b.address()),
            InstructionAccount::writable(self.token_vault_b.address()),
            InstructionAccount::readonly(self.token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 9] {
        [
            self.whirlpool, self.position_authority, self.position, self.position_token_account, self.token_owner_account_a, self.token_vault_a, self.token_owner_account_b, self.token_vault_b, self.token_program
        ]
    }
}

/// Accounts for `collectReward`
pub struct CollectrewardAccounts<'a> {
    /// whirlpool
    pub whirlpool: &'a AccountView,
    /// positionAuthority
    pub position_authority: &'a AccountView,
    /// position
    pub position: &'a AccountView,
    /// positionTokenAccount
    pub position_token_account: &'a AccountView,
    /// rewardOwnerAccount
    pub reward_owner_account: &'a AccountView,
    /// rewardVault
    pub reward_vault: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
}

impl<'a> CollectrewardAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 7] {
        [
            InstructionAccount::readonly(self.whirlpool.address()),
            InstructionAccount::readonly_signer(self.position_authority.address()),
            InstructionAccount::writable(self.position.address()),
            InstructionAccount::readonly(self.position_token_account.address()),
            InstructionAccount::writable(self.reward_owner_account.address()),
            InstructionAccount::writable(self.reward_vault.address()),
            InstructionAccount::readonly(self.token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 7] {
        [
            self.whirlpool, self.position_authority, self.position, self.position_token_account, self.reward_owner_account, self.reward_vault, self.token_program
        ]
    }
}

/// Accounts for `collectProtocolFees`
pub struct CollectprotocolfeesAccounts<'a> {
    /// whirlpoolsConfig
    pub whirlpools_config: &'a AccountView,
    /// whirlpool
    pub whirlpool: &'a AccountView,
    /// collectProtocolFeesAuthority
    pub collect_protocol_fees_authority: &'a AccountView,
    /// tokenVaultA
    pub token_vault_a: &'a AccountView,
    /// tokenVaultB
    pub token_vault_b: &'a AccountView,
    /// tokenDestinationA
    pub token_destination_a: &'a AccountView,
    /// tokenDestinationB
    pub token_destination_b: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
}

impl<'a> CollectprotocolfeesAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 8] {
        [
            InstructionAccount::readonly(self.whirlpools_config.address()),
            InstructionAccount::writable(self.whirlpool.address()),
            InstructionAccount::readonly_signer(self.collect_protocol_fees_authority.address()),
            InstructionAccount::writable(self.token_vault_a.address()),
            InstructionAccount::writable(self.token_vault_b.address()),
            InstructionAccount::writable(self.token_destination_a.address()),
            InstructionAccount::writable(self.token_destination_b.address()),
            InstructionAccount::readonly(self.token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 8] {
        [
            self.whirlpools_config, self.whirlpool, self.collect_protocol_fees_authority, self.token_vault_a, self.token_vault_b, self.token_destination_a, self.token_destination_b, self.token_program
        ]
    }
}

/// Accounts for `swap`
pub struct SwapAccounts<'a> {
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// tokenAuthority
    pub token_authority: &'a AccountView,
    /// whirlpool
    pub whirlpool: &'a AccountView,
    /// tokenOwnerAccountA
    pub token_owner_account_a: &'a AccountView,
    /// tokenVaultA
    pub token_vault_a: &'a AccountView,
    /// tokenOwnerAccountB
    pub token_owner_account_b: &'a AccountView,
    /// tokenVaultB
    pub token_vault_b: &'a AccountView,
    /// tickArray0
    pub tick_array0: &'a AccountView,
    /// tickArray1
    pub tick_array1: &'a AccountView,
    /// tickArray2
    pub tick_array2: &'a AccountView,
    /// oracle
    pub oracle: &'a AccountView,
}

impl<'a> SwapAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 11] {
        [
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly_signer(self.token_authority.address()),
            InstructionAccount::writable(self.whirlpool.address()),
            InstructionAccount::writable(self.token_owner_account_a.address()),
            InstructionAccount::writable(self.token_vault_a.address()),
            InstructionAccount::writable(self.token_owner_account_b.address()),
            InstructionAccount::writable(self.token_vault_b.address()),
            InstructionAccount::writable(self.tick_array0.address()),
            InstructionAccount::writable(self.tick_array1.address()),
            InstructionAccount::writable(self.tick_array2.address()),
            InstructionAccount::readonly(self.oracle.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 11] {
        [
            self.token_program, self.token_authority, self.whirlpool, self.token_owner_account_a, self.token_vault_a, self.token_owner_account_b, self.token_vault_b, self.tick_array0, self.tick_array1, self.tick_array2, self.oracle
        ]
    }
}

/// Accounts for `closePosition`
pub struct ClosepositionAccounts<'a> {
    /// positionAuthority
    pub position_authority: &'a AccountView,
    /// receiver
    pub receiver: &'a AccountView,
    /// position
    pub position: &'a AccountView,
    /// positionMint
    pub position_mint: &'a AccountView,
    /// positionTokenAccount
    pub position_token_account: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
}

impl<'a> ClosepositionAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 6] {
        [
            InstructionAccount::readonly_signer(self.position_authority.address()),
            InstructionAccount::writable(self.receiver.address()),
            InstructionAccount::writable(self.position.address()),
            InstructionAccount::writable(self.position_mint.address()),
            InstructionAccount::writable(self.position_token_account.address()),
            InstructionAccount::readonly(self.token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 6] {
        [
            self.position_authority, self.receiver, self.position, self.position_mint, self.position_token_account, self.token_program
        ]
    }
}

/// Accounts for `setDefaultFeeRate`
pub struct SetdefaultfeerateAccounts<'a> {
    /// whirlpoolsConfig
    pub whirlpools_config: &'a AccountView,
    /// feeTier
    pub fee_tier: &'a AccountView,
    /// feeAuthority
    pub fee_authority: &'a AccountView,
}

impl<'a> SetdefaultfeerateAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 3] {
        [
            InstructionAccount::readonly(self.whirlpools_config.address()),
            InstructionAccount::writable(self.fee_tier.address()),
            InstructionAccount::readonly_signer(self.fee_authority.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 3] {
        [
            self.whirlpools_config, self.fee_tier, self.fee_authority
        ]
    }
}

/// Accounts for `setDefaultProtocolFeeRate`
pub struct SetdefaultprotocolfeerateAccounts<'a> {
    /// whirlpoolsConfig
    pub whirlpools_config: &'a AccountView,
    /// feeAuthority
    pub fee_authority: &'a AccountView,
}

impl<'a> SetdefaultprotocolfeerateAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 2] {
        [
            InstructionAccount::writable(self.whirlpools_config.address()),
            InstructionAccount::readonly_signer(self.fee_authority.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 2] {
        [
            self.whirlpools_config, self.fee_authority
        ]
    }
}

/// Accounts for `setFeeRate`
pub struct SetfeerateAccounts<'a> {
    /// whirlpoolsConfig
    pub whirlpools_config: &'a AccountView,
    /// whirlpool
    pub whirlpool: &'a AccountView,
    /// feeAuthority
    pub fee_authority: &'a AccountView,
}

impl<'a> SetfeerateAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 3] {
        [
            InstructionAccount::readonly(self.whirlpools_config.address()),
            InstructionAccount::writable(self.whirlpool.address()),
            InstructionAccount::readonly_signer(self.fee_authority.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 3] {
        [
            self.whirlpools_config, self.whirlpool, self.fee_authority
        ]
    }
}

/// Accounts for `setProtocolFeeRate`
pub struct SetprotocolfeerateAccounts<'a> {
    /// whirlpoolsConfig
    pub whirlpools_config: &'a AccountView,
    /// whirlpool
    pub whirlpool: &'a AccountView,
    /// feeAuthority
    pub fee_authority: &'a AccountView,
}

impl<'a> SetprotocolfeerateAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 3] {
        [
            InstructionAccount::readonly(self.whirlpools_config.address()),
            InstructionAccount::writable(self.whirlpool.address()),
            InstructionAccount::readonly_signer(self.fee_authority.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 3] {
        [
            self.whirlpools_config, self.whirlpool, self.fee_authority
        ]
    }
}

/// Accounts for `setFeeAuthority`
pub struct SetfeeauthorityAccounts<'a> {
    /// whirlpoolsConfig
    pub whirlpools_config: &'a AccountView,
    /// feeAuthority
    pub fee_authority: &'a AccountView,
    /// newFeeAuthority
    pub new_fee_authority: &'a AccountView,
}

impl<'a> SetfeeauthorityAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 3] {
        [
            InstructionAccount::writable(self.whirlpools_config.address()),
            InstructionAccount::readonly_signer(self.fee_authority.address()),
            InstructionAccount::readonly(self.new_fee_authority.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 3] {
        [
            self.whirlpools_config, self.fee_authority, self.new_fee_authority
        ]
    }
}

/// Accounts for `setCollectProtocolFeesAuthority`
pub struct SetcollectprotocolfeesauthorityAccounts<'a> {
    /// whirlpoolsConfig
    pub whirlpools_config: &'a AccountView,
    /// collectProtocolFeesAuthority
    pub collect_protocol_fees_authority: &'a AccountView,
    /// newCollectProtocolFeesAuthority
    pub new_collect_protocol_fees_authority: &'a AccountView,
}

impl<'a> SetcollectprotocolfeesauthorityAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 3] {
        [
            InstructionAccount::writable(self.whirlpools_config.address()),
            InstructionAccount::readonly_signer(self.collect_protocol_fees_authority.address()),
            InstructionAccount::readonly(self.new_collect_protocol_fees_authority.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 3] {
        [
            self.whirlpools_config, self.collect_protocol_fees_authority, self.new_collect_protocol_fees_authority
        ]
    }
}

/// Accounts for `setRewardAuthority`
pub struct SetrewardauthorityAccounts<'a> {
    /// whirlpool
    pub whirlpool: &'a AccountView,
    /// rewardAuthority
    pub reward_authority: &'a AccountView,
    /// newRewardAuthority
    pub new_reward_authority: &'a AccountView,
}

impl<'a> SetrewardauthorityAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 3] {
        [
            InstructionAccount::writable(self.whirlpool.address()),
            InstructionAccount::readonly_signer(self.reward_authority.address()),
            InstructionAccount::readonly(self.new_reward_authority.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 3] {
        [
            self.whirlpool, self.reward_authority, self.new_reward_authority
        ]
    }
}

/// Accounts for `setRewardAuthorityBySuperAuthority`
pub struct SetrewardauthoritybysuperauthorityAccounts<'a> {
    /// whirlpoolsConfig
    pub whirlpools_config: &'a AccountView,
    /// whirlpool
    pub whirlpool: &'a AccountView,
    /// rewardEmissionsSuperAuthority
    pub reward_emissions_super_authority: &'a AccountView,
    /// newRewardAuthority
    pub new_reward_authority: &'a AccountView,
}

impl<'a> SetrewardauthoritybysuperauthorityAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 4] {
        [
            InstructionAccount::readonly(self.whirlpools_config.address()),
            InstructionAccount::writable(self.whirlpool.address()),
            InstructionAccount::readonly_signer(self.reward_emissions_super_authority.address()),
            InstructionAccount::readonly(self.new_reward_authority.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 4] {
        [
            self.whirlpools_config, self.whirlpool, self.reward_emissions_super_authority, self.new_reward_authority
        ]
    }
}

/// Accounts for `setRewardEmissionsSuperAuthority`
pub struct SetrewardemissionssuperauthorityAccounts<'a> {
    /// whirlpoolsConfig
    pub whirlpools_config: &'a AccountView,
    /// rewardEmissionsSuperAuthority
    pub reward_emissions_super_authority: &'a AccountView,
    /// newRewardEmissionsSuperAuthority
    pub new_reward_emissions_super_authority: &'a AccountView,
}

impl<'a> SetrewardemissionssuperauthorityAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 3] {
        [
            InstructionAccount::writable(self.whirlpools_config.address()),
            InstructionAccount::readonly_signer(self.reward_emissions_super_authority.address()),
            InstructionAccount::readonly(self.new_reward_emissions_super_authority.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 3] {
        [
            self.whirlpools_config, self.reward_emissions_super_authority, self.new_reward_emissions_super_authority
        ]
    }
}

/// Accounts for `twoHopSwap`
pub struct TwohopswapAccounts<'a> {
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// tokenAuthority
    pub token_authority: &'a AccountView,
    /// whirlpoolOne
    pub whirlpool_one: &'a AccountView,
    /// whirlpoolTwo
    pub whirlpool_two: &'a AccountView,
    /// tokenOwnerAccountOneA
    pub token_owner_account_one_a: &'a AccountView,
    /// tokenVaultOneA
    pub token_vault_one_a: &'a AccountView,
    /// tokenOwnerAccountOneB
    pub token_owner_account_one_b: &'a AccountView,
    /// tokenVaultOneB
    pub token_vault_one_b: &'a AccountView,
    /// tokenOwnerAccountTwoA
    pub token_owner_account_two_a: &'a AccountView,
    /// tokenVaultTwoA
    pub token_vault_two_a: &'a AccountView,
    /// tokenOwnerAccountTwoB
    pub token_owner_account_two_b: &'a AccountView,
    /// tokenVaultTwoB
    pub token_vault_two_b: &'a AccountView,
    /// tickArrayOne0
    pub tick_array_one0: &'a AccountView,
    /// tickArrayOne1
    pub tick_array_one1: &'a AccountView,
    /// tickArrayOne2
    pub tick_array_one2: &'a AccountView,
    /// tickArrayTwo0
    pub tick_array_two0: &'a AccountView,
    /// tickArrayTwo1
    pub tick_array_two1: &'a AccountView,
    /// tickArrayTwo2
    pub tick_array_two2: &'a AccountView,
    /// oracleOne
    pub oracle_one: &'a AccountView,
    /// oracleTwo
    pub oracle_two: &'a AccountView,
}

impl<'a> TwohopswapAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 20] {
        [
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly_signer(self.token_authority.address()),
            InstructionAccount::writable(self.whirlpool_one.address()),
            InstructionAccount::writable(self.whirlpool_two.address()),
            InstructionAccount::writable(self.token_owner_account_one_a.address()),
            InstructionAccount::writable(self.token_vault_one_a.address()),
            InstructionAccount::writable(self.token_owner_account_one_b.address()),
            InstructionAccount::writable(self.token_vault_one_b.address()),
            InstructionAccount::writable(self.token_owner_account_two_a.address()),
            InstructionAccount::writable(self.token_vault_two_a.address()),
            InstructionAccount::writable(self.token_owner_account_two_b.address()),
            InstructionAccount::writable(self.token_vault_two_b.address()),
            InstructionAccount::writable(self.tick_array_one0.address()),
            InstructionAccount::writable(self.tick_array_one1.address()),
            InstructionAccount::writable(self.tick_array_one2.address()),
            InstructionAccount::writable(self.tick_array_two0.address()),
            InstructionAccount::writable(self.tick_array_two1.address()),
            InstructionAccount::writable(self.tick_array_two2.address()),
            InstructionAccount::readonly(self.oracle_one.address()),
            InstructionAccount::readonly(self.oracle_two.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 20] {
        [
            self.token_program, self.token_authority, self.whirlpool_one, self.whirlpool_two, self.token_owner_account_one_a, self.token_vault_one_a, self.token_owner_account_one_b, self.token_vault_one_b, self.token_owner_account_two_a, self.token_vault_two_a, self.token_owner_account_two_b, self.token_vault_two_b, self.tick_array_one0, self.tick_array_one1, self.tick_array_one2, self.tick_array_two0, self.tick_array_two1, self.tick_array_two2, self.oracle_one, self.oracle_two
        ]
    }
}

/// Accounts for `initializePositionBundle`
pub struct InitializepositionbundleAccounts<'a> {
    /// positionBundle
    pub position_bundle: &'a AccountView,
    /// positionBundleMint
    pub position_bundle_mint: &'a AccountView,
    /// positionBundleTokenAccount
    pub position_bundle_token_account: &'a AccountView,
    /// positionBundleOwner
    pub position_bundle_owner: &'a AccountView,
    /// funder
    pub funder: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
    /// rent
    pub rent: &'a AccountView,
    /// associatedTokenProgram
    pub associated_token_program: &'a AccountView,
}

impl<'a> InitializepositionbundleAccounts<'a> {
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

/// Accounts for `initializePositionBundleWithMetadata`
pub struct InitializepositionbundlewithmetadataAccounts<'a> {
    /// positionBundle
    pub position_bundle: &'a AccountView,
    /// positionBundleMint
    pub position_bundle_mint: &'a AccountView,
    /// positionBundleMetadata
    pub position_bundle_metadata: &'a AccountView,
    /// positionBundleTokenAccount
    pub position_bundle_token_account: &'a AccountView,
    /// positionBundleOwner
    pub position_bundle_owner: &'a AccountView,
    /// funder
    pub funder: &'a AccountView,
    /// metadataUpdateAuth
    pub metadata_update_auth: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
    /// rent
    pub rent: &'a AccountView,
    /// associatedTokenProgram
    pub associated_token_program: &'a AccountView,
    /// metadataProgram
    pub metadata_program: &'a AccountView,
}

impl<'a> InitializepositionbundlewithmetadataAccounts<'a> {
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

/// Accounts for `deletePositionBundle`
pub struct DeletepositionbundleAccounts<'a> {
    /// positionBundle
    pub position_bundle: &'a AccountView,
    /// positionBundleMint
    pub position_bundle_mint: &'a AccountView,
    /// positionBundleTokenAccount
    pub position_bundle_token_account: &'a AccountView,
    /// positionBundleOwner
    pub position_bundle_owner: &'a AccountView,
    /// receiver
    pub receiver: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
}

impl<'a> DeletepositionbundleAccounts<'a> {
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

/// Accounts for `openBundledPosition`
pub struct OpenbundledpositionAccounts<'a> {
    /// bundledPosition
    pub bundled_position: &'a AccountView,
    /// positionBundle
    pub position_bundle: &'a AccountView,
    /// positionBundleTokenAccount
    pub position_bundle_token_account: &'a AccountView,
    /// positionBundleAuthority
    pub position_bundle_authority: &'a AccountView,
    /// whirlpool
    pub whirlpool: &'a AccountView,
    /// funder
    pub funder: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
    /// rent
    pub rent: &'a AccountView,
}

impl<'a> OpenbundledpositionAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 8] {
        [
            InstructionAccount::writable(self.bundled_position.address()),
            InstructionAccount::writable(self.position_bundle.address()),
            InstructionAccount::readonly(self.position_bundle_token_account.address()),
            InstructionAccount::readonly_signer(self.position_bundle_authority.address()),
            InstructionAccount::readonly(self.whirlpool.address()),
            InstructionAccount::writable_signer(self.funder.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.rent.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 8] {
        [
            self.bundled_position, self.position_bundle, self.position_bundle_token_account, self.position_bundle_authority, self.whirlpool, self.funder, self.system_program, self.rent
        ]
    }
}

/// Accounts for `closeBundledPosition`
pub struct ClosebundledpositionAccounts<'a> {
    /// bundledPosition
    pub bundled_position: &'a AccountView,
    /// positionBundle
    pub position_bundle: &'a AccountView,
    /// positionBundleTokenAccount
    pub position_bundle_token_account: &'a AccountView,
    /// positionBundleAuthority
    pub position_bundle_authority: &'a AccountView,
    /// receiver
    pub receiver: &'a AccountView,
}

impl<'a> ClosebundledpositionAccounts<'a> {
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

/// Accounts for `openPositionWithTokenExtensions`
pub struct OpenpositionwithtokenextensionsAccounts<'a> {
    /// funder
    pub funder: &'a AccountView,
    /// owner
    pub owner: &'a AccountView,
    /// position
    pub position: &'a AccountView,
    /// positionMint
    pub position_mint: &'a AccountView,
    /// positionTokenAccount
    pub position_token_account: &'a AccountView,
    /// whirlpool
    pub whirlpool: &'a AccountView,
    /// token2022Program
    pub token2022_program: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
    /// associatedTokenProgram
    pub associated_token_program: &'a AccountView,
    /// metadataUpdateAuth
    pub metadata_update_auth: &'a AccountView,
}

impl<'a> OpenpositionwithtokenextensionsAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 10] {
        [
            InstructionAccount::writable_signer(self.funder.address()),
            InstructionAccount::readonly(self.owner.address()),
            InstructionAccount::writable(self.position.address()),
            InstructionAccount::writable_signer(self.position_mint.address()),
            InstructionAccount::writable(self.position_token_account.address()),
            InstructionAccount::readonly(self.whirlpool.address()),
            InstructionAccount::readonly(self.token2022_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
            InstructionAccount::readonly(self.metadata_update_auth.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 10] {
        [
            self.funder, self.owner, self.position, self.position_mint, self.position_token_account, self.whirlpool, self.token2022_program, self.system_program, self.associated_token_program, self.metadata_update_auth
        ]
    }
}

/// Accounts for `closePositionWithTokenExtensions`
pub struct ClosepositionwithtokenextensionsAccounts<'a> {
    /// positionAuthority
    pub position_authority: &'a AccountView,
    /// receiver
    pub receiver: &'a AccountView,
    /// position
    pub position: &'a AccountView,
    /// positionMint
    pub position_mint: &'a AccountView,
    /// positionTokenAccount
    pub position_token_account: &'a AccountView,
    /// token2022Program
    pub token2022_program: &'a AccountView,
}

impl<'a> ClosepositionwithtokenextensionsAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 6] {
        [
            InstructionAccount::readonly_signer(self.position_authority.address()),
            InstructionAccount::writable(self.receiver.address()),
            InstructionAccount::writable(self.position.address()),
            InstructionAccount::writable(self.position_mint.address()),
            InstructionAccount::writable(self.position_token_account.address()),
            InstructionAccount::readonly(self.token2022_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 6] {
        [
            self.position_authority, self.receiver, self.position, self.position_mint, self.position_token_account, self.token2022_program
        ]
    }
}

/// Accounts for `lockPosition`
pub struct LockpositionAccounts<'a> {
    /// funder
    pub funder: &'a AccountView,
    /// positionAuthority
    pub position_authority: &'a AccountView,
    /// position
    pub position: &'a AccountView,
    /// positionMint
    pub position_mint: &'a AccountView,
    /// positionTokenAccount
    pub position_token_account: &'a AccountView,
    /// lockConfig
    pub lock_config: &'a AccountView,
    /// whirlpool
    pub whirlpool: &'a AccountView,
    /// token2022Program
    pub token2022_program: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
}

impl<'a> LockpositionAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 9] {
        [
            InstructionAccount::writable_signer(self.funder.address()),
            InstructionAccount::readonly_signer(self.position_authority.address()),
            InstructionAccount::readonly(self.position.address()),
            InstructionAccount::readonly(self.position_mint.address()),
            InstructionAccount::writable(self.position_token_account.address()),
            InstructionAccount::writable(self.lock_config.address()),
            InstructionAccount::readonly(self.whirlpool.address()),
            InstructionAccount::readonly(self.token2022_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 9] {
        [
            self.funder, self.position_authority, self.position, self.position_mint, self.position_token_account, self.lock_config, self.whirlpool, self.token2022_program, self.system_program
        ]
    }
}

/// Accounts for `resetPositionRange`
pub struct ResetpositionrangeAccounts<'a> {
    /// funder
    pub funder: &'a AccountView,
    /// positionAuthority
    pub position_authority: &'a AccountView,
    /// whirlpool
    pub whirlpool: &'a AccountView,
    /// position
    pub position: &'a AccountView,
    /// positionTokenAccount
    pub position_token_account: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
}

impl<'a> ResetpositionrangeAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 6] {
        [
            InstructionAccount::writable_signer(self.funder.address()),
            InstructionAccount::readonly_signer(self.position_authority.address()),
            InstructionAccount::readonly(self.whirlpool.address()),
            InstructionAccount::writable(self.position.address()),
            InstructionAccount::readonly(self.position_token_account.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 6] {
        [
            self.funder, self.position_authority, self.whirlpool, self.position, self.position_token_account, self.system_program
        ]
    }
}

/// Accounts for `transferLockedPosition`
pub struct TransferlockedpositionAccounts<'a> {
    /// positionAuthority
    pub position_authority: &'a AccountView,
    /// receiver
    pub receiver: &'a AccountView,
    /// position
    pub position: &'a AccountView,
    /// positionMint
    pub position_mint: &'a AccountView,
    /// positionTokenAccount
    pub position_token_account: &'a AccountView,
    /// destinationTokenAccount
    pub destination_token_account: &'a AccountView,
    /// lockConfig
    pub lock_config: &'a AccountView,
    /// token2022Program
    pub token2022_program: &'a AccountView,
}

impl<'a> TransferlockedpositionAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 8] {
        [
            InstructionAccount::readonly_signer(self.position_authority.address()),
            InstructionAccount::writable(self.receiver.address()),
            InstructionAccount::readonly(self.position.address()),
            InstructionAccount::readonly(self.position_mint.address()),
            InstructionAccount::writable(self.position_token_account.address()),
            InstructionAccount::writable(self.destination_token_account.address()),
            InstructionAccount::writable(self.lock_config.address()),
            InstructionAccount::readonly(self.token2022_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 8] {
        [
            self.position_authority, self.receiver, self.position, self.position_mint, self.position_token_account, self.destination_token_account, self.lock_config, self.token2022_program
        ]
    }
}

/// Accounts for `initializeAdaptiveFeeTier`
pub struct InitializeadaptivefeetierAccounts<'a> {
    /// whirlpoolsConfig
    pub whirlpools_config: &'a AccountView,
    /// adaptiveFeeTier
    pub adaptive_fee_tier: &'a AccountView,
    /// funder
    pub funder: &'a AccountView,
    /// feeAuthority
    pub fee_authority: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
}

impl<'a> InitializeadaptivefeetierAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 5] {
        [
            InstructionAccount::readonly(self.whirlpools_config.address()),
            InstructionAccount::writable(self.adaptive_fee_tier.address()),
            InstructionAccount::writable_signer(self.funder.address()),
            InstructionAccount::readonly_signer(self.fee_authority.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 5] {
        [
            self.whirlpools_config, self.adaptive_fee_tier, self.funder, self.fee_authority, self.system_program
        ]
    }
}

/// Accounts for `setDefaultBaseFeeRate`
pub struct SetdefaultbasefeerateAccounts<'a> {
    /// whirlpoolsConfig
    pub whirlpools_config: &'a AccountView,
    /// adaptiveFeeTier
    pub adaptive_fee_tier: &'a AccountView,
    /// feeAuthority
    pub fee_authority: &'a AccountView,
}

impl<'a> SetdefaultbasefeerateAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 3] {
        [
            InstructionAccount::readonly(self.whirlpools_config.address()),
            InstructionAccount::writable(self.adaptive_fee_tier.address()),
            InstructionAccount::readonly_signer(self.fee_authority.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 3] {
        [
            self.whirlpools_config, self.adaptive_fee_tier, self.fee_authority
        ]
    }
}

/// Accounts for `setDelegatedFeeAuthority`
pub struct SetdelegatedfeeauthorityAccounts<'a> {
    /// whirlpoolsConfig
    pub whirlpools_config: &'a AccountView,
    /// adaptiveFeeTier
    pub adaptive_fee_tier: &'a AccountView,
    /// feeAuthority
    pub fee_authority: &'a AccountView,
    /// newDelegatedFeeAuthority
    pub new_delegated_fee_authority: &'a AccountView,
}

impl<'a> SetdelegatedfeeauthorityAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 4] {
        [
            InstructionAccount::readonly(self.whirlpools_config.address()),
            InstructionAccount::writable(self.adaptive_fee_tier.address()),
            InstructionAccount::readonly_signer(self.fee_authority.address()),
            InstructionAccount::readonly(self.new_delegated_fee_authority.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 4] {
        [
            self.whirlpools_config, self.adaptive_fee_tier, self.fee_authority, self.new_delegated_fee_authority
        ]
    }
}

/// Accounts for `setInitializePoolAuthority`
pub struct SetinitializepoolauthorityAccounts<'a> {
    /// whirlpoolsConfig
    pub whirlpools_config: &'a AccountView,
    /// adaptiveFeeTier
    pub adaptive_fee_tier: &'a AccountView,
    /// feeAuthority
    pub fee_authority: &'a AccountView,
    /// newInitializePoolAuthority
    pub new_initialize_pool_authority: &'a AccountView,
}

impl<'a> SetinitializepoolauthorityAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 4] {
        [
            InstructionAccount::readonly(self.whirlpools_config.address()),
            InstructionAccount::writable(self.adaptive_fee_tier.address()),
            InstructionAccount::readonly_signer(self.fee_authority.address()),
            InstructionAccount::readonly(self.new_initialize_pool_authority.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 4] {
        [
            self.whirlpools_config, self.adaptive_fee_tier, self.fee_authority, self.new_initialize_pool_authority
        ]
    }
}

/// Accounts for `setPresetAdaptiveFeeConstants`
pub struct SetpresetadaptivefeeconstantsAccounts<'a> {
    /// whirlpoolsConfig
    pub whirlpools_config: &'a AccountView,
    /// adaptiveFeeTier
    pub adaptive_fee_tier: &'a AccountView,
    /// feeAuthority
    pub fee_authority: &'a AccountView,
}

impl<'a> SetpresetadaptivefeeconstantsAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 3] {
        [
            InstructionAccount::readonly(self.whirlpools_config.address()),
            InstructionAccount::writable(self.adaptive_fee_tier.address()),
            InstructionAccount::readonly_signer(self.fee_authority.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 3] {
        [
            self.whirlpools_config, self.adaptive_fee_tier, self.fee_authority
        ]
    }
}

/// Accounts for `initializePoolWithAdaptiveFee`
pub struct InitializepoolwithadaptivefeeAccounts<'a> {
    /// whirlpoolsConfig
    pub whirlpools_config: &'a AccountView,
    /// tokenMintA
    pub token_mint_a: &'a AccountView,
    /// tokenMintB
    pub token_mint_b: &'a AccountView,
    /// tokenBadgeA
    pub token_badge_a: &'a AccountView,
    /// tokenBadgeB
    pub token_badge_b: &'a AccountView,
    /// funder
    pub funder: &'a AccountView,
    /// initializePoolAuthority
    pub initialize_pool_authority: &'a AccountView,
    /// whirlpool
    pub whirlpool: &'a AccountView,
    /// oracle
    pub oracle: &'a AccountView,
    /// tokenVaultA
    pub token_vault_a: &'a AccountView,
    /// tokenVaultB
    pub token_vault_b: &'a AccountView,
    /// adaptiveFeeTier
    pub adaptive_fee_tier: &'a AccountView,
    /// tokenProgramA
    pub token_program_a: &'a AccountView,
    /// tokenProgramB
    pub token_program_b: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
    /// rent
    pub rent: &'a AccountView,
}

impl<'a> InitializepoolwithadaptivefeeAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 16] {
        [
            InstructionAccount::readonly(self.whirlpools_config.address()),
            InstructionAccount::readonly(self.token_mint_a.address()),
            InstructionAccount::readonly(self.token_mint_b.address()),
            InstructionAccount::readonly(self.token_badge_a.address()),
            InstructionAccount::readonly(self.token_badge_b.address()),
            InstructionAccount::writable_signer(self.funder.address()),
            InstructionAccount::readonly_signer(self.initialize_pool_authority.address()),
            InstructionAccount::writable(self.whirlpool.address()),
            InstructionAccount::writable(self.oracle.address()),
            InstructionAccount::writable_signer(self.token_vault_a.address()),
            InstructionAccount::writable_signer(self.token_vault_b.address()),
            InstructionAccount::readonly(self.adaptive_fee_tier.address()),
            InstructionAccount::readonly(self.token_program_a.address()),
            InstructionAccount::readonly(self.token_program_b.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.rent.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 16] {
        [
            self.whirlpools_config, self.token_mint_a, self.token_mint_b, self.token_badge_a, self.token_badge_b, self.funder, self.initialize_pool_authority, self.whirlpool, self.oracle, self.token_vault_a, self.token_vault_b, self.adaptive_fee_tier, self.token_program_a, self.token_program_b, self.system_program, self.rent
        ]
    }
}

/// Accounts for `setFeeRateByDelegatedFeeAuthority`
pub struct SetfeeratebydelegatedfeeauthorityAccounts<'a> {
    /// whirlpool
    pub whirlpool: &'a AccountView,
    /// adaptiveFeeTier
    pub adaptive_fee_tier: &'a AccountView,
    /// delegatedFeeAuthority
    pub delegated_fee_authority: &'a AccountView,
}

impl<'a> SetfeeratebydelegatedfeeauthorityAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 3] {
        [
            InstructionAccount::writable(self.whirlpool.address()),
            InstructionAccount::readonly(self.adaptive_fee_tier.address()),
            InstructionAccount::readonly_signer(self.delegated_fee_authority.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 3] {
        [
            self.whirlpool, self.adaptive_fee_tier, self.delegated_fee_authority
        ]
    }
}

/// Accounts for `collectFeesV2`
pub struct Collectfeesv2Accounts<'a> {
    /// whirlpool
    pub whirlpool: &'a AccountView,
    /// positionAuthority
    pub position_authority: &'a AccountView,
    /// position
    pub position: &'a AccountView,
    /// positionTokenAccount
    pub position_token_account: &'a AccountView,
    /// tokenMintA
    pub token_mint_a: &'a AccountView,
    /// tokenMintB
    pub token_mint_b: &'a AccountView,
    /// tokenOwnerAccountA
    pub token_owner_account_a: &'a AccountView,
    /// tokenVaultA
    pub token_vault_a: &'a AccountView,
    /// tokenOwnerAccountB
    pub token_owner_account_b: &'a AccountView,
    /// tokenVaultB
    pub token_vault_b: &'a AccountView,
    /// tokenProgramA
    pub token_program_a: &'a AccountView,
    /// tokenProgramB
    pub token_program_b: &'a AccountView,
    /// memoProgram
    pub memo_program: &'a AccountView,
}

impl<'a> Collectfeesv2Accounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 13] {
        [
            InstructionAccount::readonly(self.whirlpool.address()),
            InstructionAccount::readonly_signer(self.position_authority.address()),
            InstructionAccount::writable(self.position.address()),
            InstructionAccount::readonly(self.position_token_account.address()),
            InstructionAccount::readonly(self.token_mint_a.address()),
            InstructionAccount::readonly(self.token_mint_b.address()),
            InstructionAccount::writable(self.token_owner_account_a.address()),
            InstructionAccount::writable(self.token_vault_a.address()),
            InstructionAccount::writable(self.token_owner_account_b.address()),
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
            self.whirlpool, self.position_authority, self.position, self.position_token_account, self.token_mint_a, self.token_mint_b, self.token_owner_account_a, self.token_vault_a, self.token_owner_account_b, self.token_vault_b, self.token_program_a, self.token_program_b, self.memo_program
        ]
    }
}

/// Accounts for `collectProtocolFeesV2`
pub struct Collectprotocolfeesv2Accounts<'a> {
    /// whirlpoolsConfig
    pub whirlpools_config: &'a AccountView,
    /// whirlpool
    pub whirlpool: &'a AccountView,
    /// collectProtocolFeesAuthority
    pub collect_protocol_fees_authority: &'a AccountView,
    /// tokenMintA
    pub token_mint_a: &'a AccountView,
    /// tokenMintB
    pub token_mint_b: &'a AccountView,
    /// tokenVaultA
    pub token_vault_a: &'a AccountView,
    /// tokenVaultB
    pub token_vault_b: &'a AccountView,
    /// tokenDestinationA
    pub token_destination_a: &'a AccountView,
    /// tokenDestinationB
    pub token_destination_b: &'a AccountView,
    /// tokenProgramA
    pub token_program_a: &'a AccountView,
    /// tokenProgramB
    pub token_program_b: &'a AccountView,
    /// memoProgram
    pub memo_program: &'a AccountView,
}

impl<'a> Collectprotocolfeesv2Accounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 12] {
        [
            InstructionAccount::readonly(self.whirlpools_config.address()),
            InstructionAccount::writable(self.whirlpool.address()),
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
            self.whirlpools_config, self.whirlpool, self.collect_protocol_fees_authority, self.token_mint_a, self.token_mint_b, self.token_vault_a, self.token_vault_b, self.token_destination_a, self.token_destination_b, self.token_program_a, self.token_program_b, self.memo_program
        ]
    }
}

/// Accounts for `collectRewardV2`
pub struct Collectrewardv2Accounts<'a> {
    /// whirlpool
    pub whirlpool: &'a AccountView,
    /// positionAuthority
    pub position_authority: &'a AccountView,
    /// position
    pub position: &'a AccountView,
    /// positionTokenAccount
    pub position_token_account: &'a AccountView,
    /// rewardOwnerAccount
    pub reward_owner_account: &'a AccountView,
    /// rewardMint
    pub reward_mint: &'a AccountView,
    /// rewardVault
    pub reward_vault: &'a AccountView,
    /// rewardTokenProgram
    pub reward_token_program: &'a AccountView,
    /// memoProgram
    pub memo_program: &'a AccountView,
}

impl<'a> Collectrewardv2Accounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 9] {
        [
            InstructionAccount::readonly(self.whirlpool.address()),
            InstructionAccount::readonly_signer(self.position_authority.address()),
            InstructionAccount::writable(self.position.address()),
            InstructionAccount::readonly(self.position_token_account.address()),
            InstructionAccount::writable(self.reward_owner_account.address()),
            InstructionAccount::readonly(self.reward_mint.address()),
            InstructionAccount::writable(self.reward_vault.address()),
            InstructionAccount::readonly(self.reward_token_program.address()),
            InstructionAccount::readonly(self.memo_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 9] {
        [
            self.whirlpool, self.position_authority, self.position, self.position_token_account, self.reward_owner_account, self.reward_mint, self.reward_vault, self.reward_token_program, self.memo_program
        ]
    }
}

/// Accounts for `decreaseLiquidityV2`
pub struct Decreaseliquidityv2Accounts<'a> {
    /// whirlpool
    pub whirlpool: &'a AccountView,
    /// tokenProgramA
    pub token_program_a: &'a AccountView,
    /// tokenProgramB
    pub token_program_b: &'a AccountView,
    /// memoProgram
    pub memo_program: &'a AccountView,
    /// positionAuthority
    pub position_authority: &'a AccountView,
    /// position
    pub position: &'a AccountView,
    /// positionTokenAccount
    pub position_token_account: &'a AccountView,
    /// tokenMintA
    pub token_mint_a: &'a AccountView,
    /// tokenMintB
    pub token_mint_b: &'a AccountView,
    /// tokenOwnerAccountA
    pub token_owner_account_a: &'a AccountView,
    /// tokenOwnerAccountB
    pub token_owner_account_b: &'a AccountView,
    /// tokenVaultA
    pub token_vault_a: &'a AccountView,
    /// tokenVaultB
    pub token_vault_b: &'a AccountView,
    /// tickArrayLower
    pub tick_array_lower: &'a AccountView,
    /// tickArrayUpper
    pub tick_array_upper: &'a AccountView,
}

impl<'a> Decreaseliquidityv2Accounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 15] {
        [
            InstructionAccount::writable(self.whirlpool.address()),
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
            self.whirlpool, self.token_program_a, self.token_program_b, self.memo_program, self.position_authority, self.position, self.position_token_account, self.token_mint_a, self.token_mint_b, self.token_owner_account_a, self.token_owner_account_b, self.token_vault_a, self.token_vault_b, self.tick_array_lower, self.tick_array_upper
        ]
    }
}

/// Accounts for `increaseLiquidityV2`
pub struct Increaseliquidityv2Accounts<'a> {
    /// whirlpool
    pub whirlpool: &'a AccountView,
    /// tokenProgramA
    pub token_program_a: &'a AccountView,
    /// tokenProgramB
    pub token_program_b: &'a AccountView,
    /// memoProgram
    pub memo_program: &'a AccountView,
    /// positionAuthority
    pub position_authority: &'a AccountView,
    /// position
    pub position: &'a AccountView,
    /// positionTokenAccount
    pub position_token_account: &'a AccountView,
    /// tokenMintA
    pub token_mint_a: &'a AccountView,
    /// tokenMintB
    pub token_mint_b: &'a AccountView,
    /// tokenOwnerAccountA
    pub token_owner_account_a: &'a AccountView,
    /// tokenOwnerAccountB
    pub token_owner_account_b: &'a AccountView,
    /// tokenVaultA
    pub token_vault_a: &'a AccountView,
    /// tokenVaultB
    pub token_vault_b: &'a AccountView,
    /// tickArrayLower
    pub tick_array_lower: &'a AccountView,
    /// tickArrayUpper
    pub tick_array_upper: &'a AccountView,
}

impl<'a> Increaseliquidityv2Accounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 15] {
        [
            InstructionAccount::writable(self.whirlpool.address()),
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
            self.whirlpool, self.token_program_a, self.token_program_b, self.memo_program, self.position_authority, self.position, self.position_token_account, self.token_mint_a, self.token_mint_b, self.token_owner_account_a, self.token_owner_account_b, self.token_vault_a, self.token_vault_b, self.tick_array_lower, self.tick_array_upper
        ]
    }
}

/// Accounts for `initializePoolV2`
pub struct Initializepoolv2Accounts<'a> {
    /// whirlpoolsConfig
    pub whirlpools_config: &'a AccountView,
    /// tokenMintA
    pub token_mint_a: &'a AccountView,
    /// tokenMintB
    pub token_mint_b: &'a AccountView,
    /// tokenBadgeA
    pub token_badge_a: &'a AccountView,
    /// tokenBadgeB
    pub token_badge_b: &'a AccountView,
    /// funder
    pub funder: &'a AccountView,
    /// whirlpool
    pub whirlpool: &'a AccountView,
    /// tokenVaultA
    pub token_vault_a: &'a AccountView,
    /// tokenVaultB
    pub token_vault_b: &'a AccountView,
    /// feeTier
    pub fee_tier: &'a AccountView,
    /// tokenProgramA
    pub token_program_a: &'a AccountView,
    /// tokenProgramB
    pub token_program_b: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
    /// rent
    pub rent: &'a AccountView,
}

impl<'a> Initializepoolv2Accounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 14] {
        [
            InstructionAccount::readonly(self.whirlpools_config.address()),
            InstructionAccount::readonly(self.token_mint_a.address()),
            InstructionAccount::readonly(self.token_mint_b.address()),
            InstructionAccount::readonly(self.token_badge_a.address()),
            InstructionAccount::readonly(self.token_badge_b.address()),
            InstructionAccount::writable_signer(self.funder.address()),
            InstructionAccount::writable(self.whirlpool.address()),
            InstructionAccount::writable_signer(self.token_vault_a.address()),
            InstructionAccount::writable_signer(self.token_vault_b.address()),
            InstructionAccount::readonly(self.fee_tier.address()),
            InstructionAccount::readonly(self.token_program_a.address()),
            InstructionAccount::readonly(self.token_program_b.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.rent.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 14] {
        [
            self.whirlpools_config, self.token_mint_a, self.token_mint_b, self.token_badge_a, self.token_badge_b, self.funder, self.whirlpool, self.token_vault_a, self.token_vault_b, self.fee_tier, self.token_program_a, self.token_program_b, self.system_program, self.rent
        ]
    }
}

/// Accounts for `initializeRewardV2`
pub struct Initializerewardv2Accounts<'a> {
    /// rewardAuthority
    pub reward_authority: &'a AccountView,
    /// funder
    pub funder: &'a AccountView,
    /// whirlpool
    pub whirlpool: &'a AccountView,
    /// rewardMint
    pub reward_mint: &'a AccountView,
    /// rewardTokenBadge
    pub reward_token_badge: &'a AccountView,
    /// rewardVault
    pub reward_vault: &'a AccountView,
    /// rewardTokenProgram
    pub reward_token_program: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
    /// rent
    pub rent: &'a AccountView,
}

impl<'a> Initializerewardv2Accounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 9] {
        [
            InstructionAccount::readonly_signer(self.reward_authority.address()),
            InstructionAccount::writable_signer(self.funder.address()),
            InstructionAccount::writable(self.whirlpool.address()),
            InstructionAccount::readonly(self.reward_mint.address()),
            InstructionAccount::readonly(self.reward_token_badge.address()),
            InstructionAccount::writable_signer(self.reward_vault.address()),
            InstructionAccount::readonly(self.reward_token_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.rent.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 9] {
        [
            self.reward_authority, self.funder, self.whirlpool, self.reward_mint, self.reward_token_badge, self.reward_vault, self.reward_token_program, self.system_program, self.rent
        ]
    }
}

/// Accounts for `setRewardEmissionsV2`
pub struct Setrewardemissionsv2Accounts<'a> {
    /// whirlpool
    pub whirlpool: &'a AccountView,
    /// rewardAuthority
    pub reward_authority: &'a AccountView,
    /// rewardVault
    pub reward_vault: &'a AccountView,
}

impl<'a> Setrewardemissionsv2Accounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 3] {
        [
            InstructionAccount::writable(self.whirlpool.address()),
            InstructionAccount::readonly_signer(self.reward_authority.address()),
            InstructionAccount::readonly(self.reward_vault.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 3] {
        [
            self.whirlpool, self.reward_authority, self.reward_vault
        ]
    }
}

/// Accounts for `swapV2`
pub struct Swapv2Accounts<'a> {
    /// tokenProgramA
    pub token_program_a: &'a AccountView,
    /// tokenProgramB
    pub token_program_b: &'a AccountView,
    /// memoProgram
    pub memo_program: &'a AccountView,
    /// tokenAuthority
    pub token_authority: &'a AccountView,
    /// whirlpool
    pub whirlpool: &'a AccountView,
    /// tokenMintA
    pub token_mint_a: &'a AccountView,
    /// tokenMintB
    pub token_mint_b: &'a AccountView,
    /// tokenOwnerAccountA
    pub token_owner_account_a: &'a AccountView,
    /// tokenVaultA
    pub token_vault_a: &'a AccountView,
    /// tokenOwnerAccountB
    pub token_owner_account_b: &'a AccountView,
    /// tokenVaultB
    pub token_vault_b: &'a AccountView,
    /// tickArray0
    pub tick_array0: &'a AccountView,
    /// tickArray1
    pub tick_array1: &'a AccountView,
    /// tickArray2
    pub tick_array2: &'a AccountView,
    /// oracle
    pub oracle: &'a AccountView,
}

impl<'a> Swapv2Accounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 15] {
        [
            InstructionAccount::readonly(self.token_program_a.address()),
            InstructionAccount::readonly(self.token_program_b.address()),
            InstructionAccount::readonly(self.memo_program.address()),
            InstructionAccount::readonly_signer(self.token_authority.address()),
            InstructionAccount::writable(self.whirlpool.address()),
            InstructionAccount::readonly(self.token_mint_a.address()),
            InstructionAccount::readonly(self.token_mint_b.address()),
            InstructionAccount::writable(self.token_owner_account_a.address()),
            InstructionAccount::writable(self.token_vault_a.address()),
            InstructionAccount::writable(self.token_owner_account_b.address()),
            InstructionAccount::writable(self.token_vault_b.address()),
            InstructionAccount::writable(self.tick_array0.address()),
            InstructionAccount::writable(self.tick_array1.address()),
            InstructionAccount::writable(self.tick_array2.address()),
            InstructionAccount::writable(self.oracle.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 15] {
        [
            self.token_program_a, self.token_program_b, self.memo_program, self.token_authority, self.whirlpool, self.token_mint_a, self.token_mint_b, self.token_owner_account_a, self.token_vault_a, self.token_owner_account_b, self.token_vault_b, self.tick_array0, self.tick_array1, self.tick_array2, self.oracle
        ]
    }
}

/// Accounts for `twoHopSwapV2`
pub struct Twohopswapv2Accounts<'a> {
    /// whirlpoolOne
    pub whirlpool_one: &'a AccountView,
    /// whirlpoolTwo
    pub whirlpool_two: &'a AccountView,
    /// tokenMintInput
    pub token_mint_input: &'a AccountView,
    /// tokenMintIntermediate
    pub token_mint_intermediate: &'a AccountView,
    /// tokenMintOutput
    pub token_mint_output: &'a AccountView,
    /// tokenProgramInput
    pub token_program_input: &'a AccountView,
    /// tokenProgramIntermediate
    pub token_program_intermediate: &'a AccountView,
    /// tokenProgramOutput
    pub token_program_output: &'a AccountView,
    /// tokenOwnerAccountInput
    pub token_owner_account_input: &'a AccountView,
    /// tokenVaultOneInput
    pub token_vault_one_input: &'a AccountView,
    /// tokenVaultOneIntermediate
    pub token_vault_one_intermediate: &'a AccountView,
    /// tokenVaultTwoIntermediate
    pub token_vault_two_intermediate: &'a AccountView,
    /// tokenVaultTwoOutput
    pub token_vault_two_output: &'a AccountView,
    /// tokenOwnerAccountOutput
    pub token_owner_account_output: &'a AccountView,
    /// tokenAuthority
    pub token_authority: &'a AccountView,
    /// tickArrayOne0
    pub tick_array_one0: &'a AccountView,
    /// tickArrayOne1
    pub tick_array_one1: &'a AccountView,
    /// tickArrayOne2
    pub tick_array_one2: &'a AccountView,
    /// tickArrayTwo0
    pub tick_array_two0: &'a AccountView,
    /// tickArrayTwo1
    pub tick_array_two1: &'a AccountView,
    /// tickArrayTwo2
    pub tick_array_two2: &'a AccountView,
    /// oracleOne
    pub oracle_one: &'a AccountView,
    /// oracleTwo
    pub oracle_two: &'a AccountView,
    /// memoProgram
    pub memo_program: &'a AccountView,
}

impl<'a> Twohopswapv2Accounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 24] {
        [
            InstructionAccount::writable(self.whirlpool_one.address()),
            InstructionAccount::writable(self.whirlpool_two.address()),
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
            InstructionAccount::writable(self.tick_array_one0.address()),
            InstructionAccount::writable(self.tick_array_one1.address()),
            InstructionAccount::writable(self.tick_array_one2.address()),
            InstructionAccount::writable(self.tick_array_two0.address()),
            InstructionAccount::writable(self.tick_array_two1.address()),
            InstructionAccount::writable(self.tick_array_two2.address()),
            InstructionAccount::writable(self.oracle_one.address()),
            InstructionAccount::writable(self.oracle_two.address()),
            InstructionAccount::readonly(self.memo_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 24] {
        [
            self.whirlpool_one, self.whirlpool_two, self.token_mint_input, self.token_mint_intermediate, self.token_mint_output, self.token_program_input, self.token_program_intermediate, self.token_program_output, self.token_owner_account_input, self.token_vault_one_input, self.token_vault_one_intermediate, self.token_vault_two_intermediate, self.token_vault_two_output, self.token_owner_account_output, self.token_authority, self.tick_array_one0, self.tick_array_one1, self.tick_array_one2, self.tick_array_two0, self.tick_array_two1, self.tick_array_two2, self.oracle_one, self.oracle_two, self.memo_program
        ]
    }
}

/// Accounts for `initializeConfigExtension`
pub struct InitializeconfigextensionAccounts<'a> {
    /// config
    pub config: &'a AccountView,
    /// configExtension
    pub config_extension: &'a AccountView,
    /// funder
    pub funder: &'a AccountView,
    /// feeAuthority
    pub fee_authority: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
}

impl<'a> InitializeconfigextensionAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 5] {
        [
            InstructionAccount::readonly(self.config.address()),
            InstructionAccount::writable(self.config_extension.address()),
            InstructionAccount::writable_signer(self.funder.address()),
            InstructionAccount::readonly_signer(self.fee_authority.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 5] {
        [
            self.config, self.config_extension, self.funder, self.fee_authority, self.system_program
        ]
    }
}

/// Accounts for `setConfigExtensionAuthority`
pub struct SetconfigextensionauthorityAccounts<'a> {
    /// whirlpoolsConfig
    pub whirlpools_config: &'a AccountView,
    /// whirlpoolsConfigExtension
    pub whirlpools_config_extension: &'a AccountView,
    /// configExtensionAuthority
    pub config_extension_authority: &'a AccountView,
    /// newConfigExtensionAuthority
    pub new_config_extension_authority: &'a AccountView,
}

impl<'a> SetconfigextensionauthorityAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 4] {
        [
            InstructionAccount::readonly(self.whirlpools_config.address()),
            InstructionAccount::writable(self.whirlpools_config_extension.address()),
            InstructionAccount::readonly_signer(self.config_extension_authority.address()),
            InstructionAccount::readonly(self.new_config_extension_authority.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 4] {
        [
            self.whirlpools_config, self.whirlpools_config_extension, self.config_extension_authority, self.new_config_extension_authority
        ]
    }
}

/// Accounts for `setTokenBadgeAuthority`
pub struct SettokenbadgeauthorityAccounts<'a> {
    /// whirlpoolsConfig
    pub whirlpools_config: &'a AccountView,
    /// whirlpoolsConfigExtension
    pub whirlpools_config_extension: &'a AccountView,
    /// configExtensionAuthority
    pub config_extension_authority: &'a AccountView,
    /// newTokenBadgeAuthority
    pub new_token_badge_authority: &'a AccountView,
}

impl<'a> SettokenbadgeauthorityAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 4] {
        [
            InstructionAccount::readonly(self.whirlpools_config.address()),
            InstructionAccount::writable(self.whirlpools_config_extension.address()),
            InstructionAccount::readonly_signer(self.config_extension_authority.address()),
            InstructionAccount::readonly(self.new_token_badge_authority.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 4] {
        [
            self.whirlpools_config, self.whirlpools_config_extension, self.config_extension_authority, self.new_token_badge_authority
        ]
    }
}

/// Accounts for `initializeTokenBadge`
pub struct InitializetokenbadgeAccounts<'a> {
    /// whirlpoolsConfig
    pub whirlpools_config: &'a AccountView,
    /// whirlpoolsConfigExtension
    pub whirlpools_config_extension: &'a AccountView,
    /// tokenBadgeAuthority
    pub token_badge_authority: &'a AccountView,
    /// tokenMint
    pub token_mint: &'a AccountView,
    /// tokenBadge
    pub token_badge: &'a AccountView,
    /// funder
    pub funder: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
}

impl<'a> InitializetokenbadgeAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 7] {
        [
            InstructionAccount::readonly(self.whirlpools_config.address()),
            InstructionAccount::readonly(self.whirlpools_config_extension.address()),
            InstructionAccount::readonly_signer(self.token_badge_authority.address()),
            InstructionAccount::readonly(self.token_mint.address()),
            InstructionAccount::writable(self.token_badge.address()),
            InstructionAccount::writable_signer(self.funder.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 7] {
        [
            self.whirlpools_config, self.whirlpools_config_extension, self.token_badge_authority, self.token_mint, self.token_badge, self.funder, self.system_program
        ]
    }
}

/// Accounts for `deleteTokenBadge`
pub struct DeletetokenbadgeAccounts<'a> {
    /// whirlpoolsConfig
    pub whirlpools_config: &'a AccountView,
    /// whirlpoolsConfigExtension
    pub whirlpools_config_extension: &'a AccountView,
    /// tokenBadgeAuthority
    pub token_badge_authority: &'a AccountView,
    /// tokenMint
    pub token_mint: &'a AccountView,
    /// tokenBadge
    pub token_badge: &'a AccountView,
    /// receiver
    pub receiver: &'a AccountView,
}

impl<'a> DeletetokenbadgeAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 6] {
        [
            InstructionAccount::readonly(self.whirlpools_config.address()),
            InstructionAccount::readonly(self.whirlpools_config_extension.address()),
            InstructionAccount::readonly_signer(self.token_badge_authority.address()),
            InstructionAccount::readonly(self.token_mint.address()),
            InstructionAccount::writable(self.token_badge.address()),
            InstructionAccount::writable(self.receiver.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 6] {
        [
            self.whirlpools_config, self.whirlpools_config_extension, self.token_badge_authority, self.token_mint, self.token_badge, self.receiver
        ]
    }
}


// ============================================
// CPI Functions
// ============================================
/// CPI: initializeConfig
#[inline(always)]
pub fn initialize_config<'a>(
    accounts: &InitializeconfigAccounts<'a>, args: &InitializeconfigArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<InitializeconfigArgs>()];
    data[0..8].copy_from_slice(&INITIALIZE_CONFIG);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const InitializeconfigArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<InitializeconfigArgs>(),
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

/// CPI: initializePool
#[inline(always)]
pub fn initialize_pool<'a>(
    accounts: &InitializepoolAccounts<'a>, args: &InitializepoolArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<InitializepoolArgs>()];
    data[0..8].copy_from_slice(&INITIALIZE_POOL);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const InitializepoolArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<InitializepoolArgs>(),
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

/// CPI: initializeTickArray
#[inline(always)]
pub fn initialize_tick_array<'a>(
    accounts: &InitializetickarrayAccounts<'a>, args: &InitializetickarrayArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<InitializetickarrayArgs>()];
    data[0..8].copy_from_slice(&INITIALIZE_TICK_ARRAY);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const InitializetickarrayArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<InitializetickarrayArgs>(),
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

/// CPI: initializeFeeTier
#[inline(always)]
pub fn initialize_fee_tier<'a>(
    accounts: &InitializefeetierAccounts<'a>, args: &InitializefeetierArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<InitializefeetierArgs>()];
    data[0..8].copy_from_slice(&INITIALIZE_FEE_TIER);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const InitializefeetierArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<InitializefeetierArgs>(),
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

/// CPI: initializeReward
#[inline(always)]
pub fn initialize_reward<'a>(
    accounts: &InitializerewardAccounts<'a>, args: &InitializerewardArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<InitializerewardArgs>()];
    data[0..8].copy_from_slice(&INITIALIZE_REWARD);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const InitializerewardArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<InitializerewardArgs>(),
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

/// CPI: setRewardEmissions
#[inline(always)]
pub fn set_reward_emissions<'a>(
    accounts: &SetrewardemissionsAccounts<'a>, args: &SetrewardemissionsArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SetrewardemissionsArgs>()];
    data[0..8].copy_from_slice(&SET_REWARD_EMISSIONS);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SetrewardemissionsArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SetrewardemissionsArgs>(),
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

/// CPI: openPosition
#[inline(always)]
pub fn open_position<'a>(
    accounts: &OpenpositionAccounts<'a>, args: &OpenpositionArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<OpenpositionArgs>()];
    data[0..8].copy_from_slice(&OPEN_POSITION);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const OpenpositionArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<OpenpositionArgs>(),
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

/// CPI: openPositionWithMetadata
#[inline(always)]
pub fn open_position_with_metadata<'a>(
    accounts: &OpenpositionwithmetadataAccounts<'a>, args: &OpenpositionwithmetadataArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<OpenpositionwithmetadataArgs>()];
    data[0..8].copy_from_slice(&OPEN_POSITION_WITH_METADATA);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const OpenpositionwithmetadataArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<OpenpositionwithmetadataArgs>(),
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

/// CPI: increaseLiquidity
#[inline(always)]
pub fn increase_liquidity<'a>(
    accounts: &IncreaseliquidityAccounts<'a>, args: &IncreaseliquidityArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<IncreaseliquidityArgs>()];
    data[0..8].copy_from_slice(&INCREASE_LIQUIDITY);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const IncreaseliquidityArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<IncreaseliquidityArgs>(),
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

/// CPI: decreaseLiquidity
#[inline(always)]
pub fn decrease_liquidity<'a>(
    accounts: &DecreaseliquidityAccounts<'a>, args: &DecreaseliquidityArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<DecreaseliquidityArgs>()];
    data[0..8].copy_from_slice(&DECREASE_LIQUIDITY);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const DecreaseliquidityArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<DecreaseliquidityArgs>(),
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

/// CPI: updateFeesAndRewards
#[inline(always)]
pub fn update_fees_and_rewards<'a>(
    accounts: &UpdatefeesandrewardsAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&UPDATE_FEES_AND_REWARDS);
    
    
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

/// CPI: collectFees
#[inline(always)]
pub fn collect_fees<'a>(
    accounts: &CollectfeesAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&COLLECT_FEES);
    
    
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

/// CPI: collectReward
#[inline(always)]
pub fn collect_reward<'a>(
    accounts: &CollectrewardAccounts<'a>, args: &CollectrewardArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<CollectrewardArgs>()];
    data[0..8].copy_from_slice(&COLLECT_REWARD);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const CollectrewardArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<CollectrewardArgs>(),
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

/// CPI: collectProtocolFees
#[inline(always)]
pub fn collect_protocol_fees<'a>(
    accounts: &CollectprotocolfeesAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&COLLECT_PROTOCOL_FEES);
    
    
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
    invoke_signed::<11>(&instruction, &account_views, signers)
}

/// CPI: closePosition
#[inline(always)]
pub fn close_position<'a>(
    accounts: &ClosepositionAccounts<'a>,
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

/// CPI: setDefaultFeeRate
#[inline(always)]
pub fn set_default_fee_rate<'a>(
    accounts: &SetdefaultfeerateAccounts<'a>, args: &SetdefaultfeerateArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SetdefaultfeerateArgs>()];
    data[0..8].copy_from_slice(&SET_DEFAULT_FEE_RATE);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SetdefaultfeerateArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SetdefaultfeerateArgs>(),
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

/// CPI: setDefaultProtocolFeeRate
#[inline(always)]
pub fn set_default_protocol_fee_rate<'a>(
    accounts: &SetdefaultprotocolfeerateAccounts<'a>, args: &SetdefaultprotocolfeerateArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SetdefaultprotocolfeerateArgs>()];
    data[0..8].copy_from_slice(&SET_DEFAULT_PROTOCOL_FEE_RATE);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SetdefaultprotocolfeerateArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SetdefaultprotocolfeerateArgs>(),
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

/// CPI: setFeeRate
#[inline(always)]
pub fn set_fee_rate<'a>(
    accounts: &SetfeerateAccounts<'a>, args: &SetfeerateArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SetfeerateArgs>()];
    data[0..8].copy_from_slice(&SET_FEE_RATE);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SetfeerateArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SetfeerateArgs>(),
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

/// CPI: setProtocolFeeRate
#[inline(always)]
pub fn set_protocol_fee_rate<'a>(
    accounts: &SetprotocolfeerateAccounts<'a>, args: &SetprotocolfeerateArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SetprotocolfeerateArgs>()];
    data[0..8].copy_from_slice(&SET_PROTOCOL_FEE_RATE);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SetprotocolfeerateArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SetprotocolfeerateArgs>(),
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

/// CPI: setFeeAuthority
#[inline(always)]
pub fn set_fee_authority<'a>(
    accounts: &SetfeeauthorityAccounts<'a>,
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

/// CPI: setCollectProtocolFeesAuthority
#[inline(always)]
pub fn set_collect_protocol_fees_authority<'a>(
    accounts: &SetcollectprotocolfeesauthorityAccounts<'a>,
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

/// CPI: setRewardAuthority
#[inline(always)]
pub fn set_reward_authority<'a>(
    accounts: &SetrewardauthorityAccounts<'a>, args: &SetrewardauthorityArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SetrewardauthorityArgs>()];
    data[0..8].copy_from_slice(&SET_REWARD_AUTHORITY);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SetrewardauthorityArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SetrewardauthorityArgs>(),
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

/// CPI: setRewardAuthorityBySuperAuthority
#[inline(always)]
pub fn set_reward_authority_by_super_authority<'a>(
    accounts: &SetrewardauthoritybysuperauthorityAccounts<'a>, args: &SetrewardauthoritybysuperauthorityArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SetrewardauthoritybysuperauthorityArgs>()];
    data[0..8].copy_from_slice(&SET_REWARD_AUTHORITY_BY_SUPER_AUTHORITY);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SetrewardauthoritybysuperauthorityArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SetrewardauthoritybysuperauthorityArgs>(),
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

/// CPI: setRewardEmissionsSuperAuthority
#[inline(always)]
pub fn set_reward_emissions_super_authority<'a>(
    accounts: &SetrewardemissionssuperauthorityAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&SET_REWARD_EMISSIONS_SUPER_AUTHORITY);
    
    
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

/// CPI: twoHopSwap
#[inline(always)]
pub fn two_hop_swap<'a>(
    accounts: &TwohopswapAccounts<'a>, args: &TwohopswapArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<TwohopswapArgs>()];
    data[0..8].copy_from_slice(&TWO_HOP_SWAP);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const TwohopswapArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<TwohopswapArgs>(),
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

/// CPI: initializePositionBundle
#[inline(always)]
pub fn initialize_position_bundle<'a>(
    accounts: &InitializepositionbundleAccounts<'a>,
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

/// CPI: initializePositionBundleWithMetadata
#[inline(always)]
pub fn initialize_position_bundle_with_metadata<'a>(
    accounts: &InitializepositionbundlewithmetadataAccounts<'a>,
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

/// CPI: deletePositionBundle
#[inline(always)]
pub fn delete_position_bundle<'a>(
    accounts: &DeletepositionbundleAccounts<'a>,
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

/// CPI: openBundledPosition
#[inline(always)]
pub fn open_bundled_position<'a>(
    accounts: &OpenbundledpositionAccounts<'a>, args: &OpenbundledpositionArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<OpenbundledpositionArgs>()];
    data[0..8].copy_from_slice(&OPEN_BUNDLED_POSITION);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const OpenbundledpositionArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<OpenbundledpositionArgs>(),
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

/// CPI: closeBundledPosition
#[inline(always)]
pub fn close_bundled_position<'a>(
    accounts: &ClosebundledpositionAccounts<'a>, args: &ClosebundledpositionArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<ClosebundledpositionArgs>()];
    data[0..8].copy_from_slice(&CLOSE_BUNDLED_POSITION);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const ClosebundledpositionArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<ClosebundledpositionArgs>(),
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

/// CPI: openPositionWithTokenExtensions
#[inline(always)]
pub fn open_position_with_token_extensions<'a>(
    accounts: &OpenpositionwithtokenextensionsAccounts<'a>, args: &OpenpositionwithtokenextensionsArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<OpenpositionwithtokenextensionsArgs>()];
    data[0..8].copy_from_slice(&OPEN_POSITION_WITH_TOKEN_EXTENSIONS);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const OpenpositionwithtokenextensionsArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<OpenpositionwithtokenextensionsArgs>(),
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

/// CPI: closePositionWithTokenExtensions
#[inline(always)]
pub fn close_position_with_token_extensions<'a>(
    accounts: &ClosepositionwithtokenextensionsAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&CLOSE_POSITION_WITH_TOKEN_EXTENSIONS);
    
    
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

/// CPI: lockPosition
#[inline(always)]
pub fn lock_position<'a>(
    accounts: &LockpositionAccounts<'a>, args: &LockpositionArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<LockpositionArgs>()];
    data[0..8].copy_from_slice(&LOCK_POSITION);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const LockpositionArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<LockpositionArgs>(),
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

/// CPI: resetPositionRange
#[inline(always)]
pub fn reset_position_range<'a>(
    accounts: &ResetpositionrangeAccounts<'a>, args: &ResetpositionrangeArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<ResetpositionrangeArgs>()];
    data[0..8].copy_from_slice(&RESET_POSITION_RANGE);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const ResetpositionrangeArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<ResetpositionrangeArgs>(),
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

/// CPI: transferLockedPosition
#[inline(always)]
pub fn transfer_locked_position<'a>(
    accounts: &TransferlockedpositionAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&TRANSFER_LOCKED_POSITION);
    
    
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

/// CPI: initializeAdaptiveFeeTier
#[inline(always)]
pub fn initialize_adaptive_fee_tier<'a>(
    accounts: &InitializeadaptivefeetierAccounts<'a>, args: &InitializeadaptivefeetierArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<InitializeadaptivefeetierArgs>()];
    data[0..8].copy_from_slice(&INITIALIZE_ADAPTIVE_FEE_TIER);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const InitializeadaptivefeetierArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<InitializeadaptivefeetierArgs>(),
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

/// CPI: setDefaultBaseFeeRate
#[inline(always)]
pub fn set_default_base_fee_rate<'a>(
    accounts: &SetdefaultbasefeerateAccounts<'a>, args: &SetdefaultbasefeerateArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SetdefaultbasefeerateArgs>()];
    data[0..8].copy_from_slice(&SET_DEFAULT_BASE_FEE_RATE);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SetdefaultbasefeerateArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SetdefaultbasefeerateArgs>(),
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

/// CPI: setDelegatedFeeAuthority
#[inline(always)]
pub fn set_delegated_fee_authority<'a>(
    accounts: &SetdelegatedfeeauthorityAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&SET_DELEGATED_FEE_AUTHORITY);
    
    
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

/// CPI: setInitializePoolAuthority
#[inline(always)]
pub fn set_initialize_pool_authority<'a>(
    accounts: &SetinitializepoolauthorityAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&SET_INITIALIZE_POOL_AUTHORITY);
    
    
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

/// CPI: setPresetAdaptiveFeeConstants
#[inline(always)]
pub fn set_preset_adaptive_fee_constants<'a>(
    accounts: &SetpresetadaptivefeeconstantsAccounts<'a>, args: &SetpresetadaptivefeeconstantsArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SetpresetadaptivefeeconstantsArgs>()];
    data[0..8].copy_from_slice(&SET_PRESET_ADAPTIVE_FEE_CONSTANTS);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SetpresetadaptivefeeconstantsArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SetpresetadaptivefeeconstantsArgs>(),
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

/// CPI: initializePoolWithAdaptiveFee
#[inline(always)]
pub fn initialize_pool_with_adaptive_fee<'a>(
    accounts: &InitializepoolwithadaptivefeeAccounts<'a>, args: &InitializepoolwithadaptivefeeArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<InitializepoolwithadaptivefeeArgs>()];
    data[0..8].copy_from_slice(&INITIALIZE_POOL_WITH_ADAPTIVE_FEE);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const InitializepoolwithadaptivefeeArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<InitializepoolwithadaptivefeeArgs>(),
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
    invoke_signed::<16>(&instruction, &account_views, signers)
}

/// CPI: setFeeRateByDelegatedFeeAuthority
#[inline(always)]
pub fn set_fee_rate_by_delegated_fee_authority<'a>(
    accounts: &SetfeeratebydelegatedfeeauthorityAccounts<'a>, args: &SetfeeratebydelegatedfeeauthorityArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SetfeeratebydelegatedfeeauthorityArgs>()];
    data[0..8].copy_from_slice(&SET_FEE_RATE_BY_DELEGATED_FEE_AUTHORITY);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SetfeeratebydelegatedfeeauthorityArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SetfeeratebydelegatedfeeauthorityArgs>(),
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

/// CPI: collectFeesV2
#[inline(always)]
pub fn collect_fees_v2<'a>(
    accounts: &Collectfeesv2Accounts<'a>, args: &Collectfeesv2Args,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<Collectfeesv2Args>()];
    data[0..8].copy_from_slice(&COLLECT_FEES_V2);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const Collectfeesv2Args as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<Collectfeesv2Args>(),
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

/// CPI: collectProtocolFeesV2
#[inline(always)]
pub fn collect_protocol_fees_v2<'a>(
    accounts: &Collectprotocolfeesv2Accounts<'a>, args: &Collectprotocolfeesv2Args,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<Collectprotocolfeesv2Args>()];
    data[0..8].copy_from_slice(&COLLECT_PROTOCOL_FEES_V2);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const Collectprotocolfeesv2Args as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<Collectprotocolfeesv2Args>(),
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

/// CPI: collectRewardV2
#[inline(always)]
pub fn collect_reward_v2<'a>(
    accounts: &Collectrewardv2Accounts<'a>, args: &Collectrewardv2Args,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<Collectrewardv2Args>()];
    data[0..8].copy_from_slice(&COLLECT_REWARD_V2);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const Collectrewardv2Args as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<Collectrewardv2Args>(),
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

/// CPI: decreaseLiquidityV2
#[inline(always)]
pub fn decrease_liquidity_v2<'a>(
    accounts: &Decreaseliquidityv2Accounts<'a>, args: &Decreaseliquidityv2Args,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<Decreaseliquidityv2Args>()];
    data[0..8].copy_from_slice(&DECREASE_LIQUIDITY_V2);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const Decreaseliquidityv2Args as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<Decreaseliquidityv2Args>(),
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

/// CPI: increaseLiquidityV2
#[inline(always)]
pub fn increase_liquidity_v2<'a>(
    accounts: &Increaseliquidityv2Accounts<'a>, args: &Increaseliquidityv2Args,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<Increaseliquidityv2Args>()];
    data[0..8].copy_from_slice(&INCREASE_LIQUIDITY_V2);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const Increaseliquidityv2Args as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<Increaseliquidityv2Args>(),
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

/// CPI: initializePoolV2
#[inline(always)]
pub fn initialize_pool_v2<'a>(
    accounts: &Initializepoolv2Accounts<'a>, args: &Initializepoolv2Args,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<Initializepoolv2Args>()];
    data[0..8].copy_from_slice(&INITIALIZE_POOL_V2);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const Initializepoolv2Args as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<Initializepoolv2Args>(),
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

/// CPI: initializeRewardV2
#[inline(always)]
pub fn initialize_reward_v2<'a>(
    accounts: &Initializerewardv2Accounts<'a>, args: &Initializerewardv2Args,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<Initializerewardv2Args>()];
    data[0..8].copy_from_slice(&INITIALIZE_REWARD_V2);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const Initializerewardv2Args as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<Initializerewardv2Args>(),
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

/// CPI: setRewardEmissionsV2
#[inline(always)]
pub fn set_reward_emissions_v2<'a>(
    accounts: &Setrewardemissionsv2Accounts<'a>, args: &Setrewardemissionsv2Args,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<Setrewardemissionsv2Args>()];
    data[0..8].copy_from_slice(&SET_REWARD_EMISSIONS_V2);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const Setrewardemissionsv2Args as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<Setrewardemissionsv2Args>(),
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

/// CPI: swapV2
#[inline(always)]
pub fn swap_v2<'a>(
    accounts: &Swapv2Accounts<'a>, args: &Swapv2Args,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<Swapv2Args>()];
    data[0..8].copy_from_slice(&SWAP_V2);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const Swapv2Args as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<Swapv2Args>(),
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

/// CPI: twoHopSwapV2
#[inline(always)]
pub fn two_hop_swap_v2<'a>(
    accounts: &Twohopswapv2Accounts<'a>, args: &Twohopswapv2Args,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<Twohopswapv2Args>()];
    data[0..8].copy_from_slice(&TWO_HOP_SWAP_V2);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const Twohopswapv2Args as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<Twohopswapv2Args>(),
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
    invoke_signed::<24>(&instruction, &account_views, signers)
}

/// CPI: initializeConfigExtension
#[inline(always)]
pub fn initialize_config_extension<'a>(
    accounts: &InitializeconfigextensionAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&INITIALIZE_CONFIG_EXTENSION);
    
    
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

/// CPI: setConfigExtensionAuthority
#[inline(always)]
pub fn set_config_extension_authority<'a>(
    accounts: &SetconfigextensionauthorityAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&SET_CONFIG_EXTENSION_AUTHORITY);
    
    
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

/// CPI: setTokenBadgeAuthority
#[inline(always)]
pub fn set_token_badge_authority<'a>(
    accounts: &SettokenbadgeauthorityAccounts<'a>,
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
    invoke_signed::<4>(&instruction, &account_views, signers)
}

/// CPI: initializeTokenBadge
#[inline(always)]
pub fn initialize_token_badge<'a>(
    accounts: &InitializetokenbadgeAccounts<'a>,
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
    invoke_signed::<7>(&instruction, &account_views, signers)
}

/// CPI: deleteTokenBadge
#[inline(always)]
pub fn delete_token_badge<'a>(
    accounts: &DeletetokenbadgeAccounts<'a>,
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
    invoke_signed::<6>(&instruction, &account_views, signers)
}

