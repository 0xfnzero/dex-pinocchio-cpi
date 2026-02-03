//! CPI module for GooseFX GAMMA
//!
//! Program: gamma
//! Program ID: GAMMA7meSFWaBXF25oSUgmGRwaW6sCMFLmBNiMSdbHVT
//! Instructions: 27

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
pub const ID: Address = Address::new_from_array(five8_const::decode_32_const("GAMMA7meSFWaBXF25oSUgmGRwaW6sCMFLmBNiMSdbHVT"));

// ============================================
// Instruction Discriminators
// ============================================
pub const ADD_OR_UPDATE_ACTOR: [u8; 8] = [113, 136, 228, 230, 107, 185, 128, 227];
pub const ADD_PARTNER: [u8; 8] = [180, 111, 45, 157, 241, 187, 234, 88];
pub const CALCULATE_REWARDS: [u8; 8] = [199, 115, 201, 124, 71, 81, 143, 252];
pub const CLAIM_PARTNER_FEES: [u8; 8] = [114, 71, 103, 57, 160, 205, 242, 185];
pub const CLAIM_REWARDS: [u8; 8] = [4, 144, 132, 71, 116, 23, 151, 80];
pub const COLLECT_FUND_FEE: [u8; 8] = [167, 138, 78, 149, 223, 194, 6, 126];
pub const COLLECT_PROTOCOL_FEE: [u8; 8] = [136, 136, 252, 221, 194, 66, 126, 89];
pub const CREATE_AMM_CONFIG: [u8; 8] = [137, 52, 237, 212, 215, 117, 108, 104];
pub const CREATE_REWARDS: [u8; 8] = [124, 251, 145, 232, 5, 173, 159, 223];
pub const CREATE_SWAP_REFERRAL: [u8; 8] = [67, 131, 93, 236, 56, 6, 40, 77];
pub const DEPOSIT: [u8; 8] = [242, 35, 198, 137, 82, 225, 242, 182];
pub const INIT_USER_POOL_LIQUIDITY: [u8; 8] = [227, 221, 200, 212, 36, 107, 149, 36];
pub const INITIALIZE: [u8; 8] = [175, 175, 109, 31, 13, 152, 155, 237];
pub const INITIALIZE_PARTNER: [u8; 8] = [165, 62, 179, 112, 129, 50, 173, 144];
pub const INITIALIZE_POOL_PARTNERS: [u8; 8] = [99, 11, 108, 186, 1, 126, 209, 251];
pub const ORACLE_BASED_SWAP_BASE_INPUT: [u8; 8] = [239, 82, 192, 187, 160, 26, 223, 223];
pub const POOL_CONFIG_UPDATE: [u8; 8] = [196, 212, 99, 63, 59, 117, 248, 223];
pub const POOL_CONFIG_UPDATE_V2: [u8; 8] = [228, 161, 199, 222, 53, 172, 121, 238];
pub const POOL_CONFIG_UPDATE_V3: [u8; 8] = [123, 135, 23, 15, 174, 100, 120, 152];
pub const REBALANCE_KAMINO: [u8; 8] = [153, 94, 34, 16, 92, 181, 147, 215];
pub const SWAP_BASE_INPUT: [u8; 8] = [143, 190, 90, 218, 196, 30, 51, 222];
pub const SWAP_BASE_OUTPUT: [u8; 8] = [55, 217, 98, 86, 163, 74, 180, 173];
pub const UPDATE_AMM_CONFIG: [u8; 8] = [49, 60, 174, 136, 154, 28, 116, 200];
pub const UPDATE_PARTNER: [u8; 8] = [19, 112, 236, 81, 127, 55, 21, 196];
pub const UPDATE_PARTNER_FEES: [u8; 8] = [154, 142, 122, 49, 239, 161, 232, 29];
pub const UPDATE_POOL: [u8; 8] = [239, 214, 170, 78, 36, 35, 30, 34];
pub const WITHDRAW: [u8; 8] = [183, 18, 70, 156, 148, 109, 161, 34];

// ============================================
// Instruction Arguments
// ============================================
/// Arguments for `add_or_update_actor`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct AddOrUpdateActorArgs {
    pub is_admin: bool,
}

/// Arguments for `collect_fund_fee`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct CollectFundFeeArgs {
    pub amount_0_requested: u64,
    pub amount_1_requested: u64,
}

/// Arguments for `collect_protocol_fee`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct CollectProtocolFeeArgs {
    pub amount_0_requested: u64,
    pub amount_1_requested: u64,
}

/// Arguments for `create_amm_config`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct CreateAmmConfigArgs {
    pub index: u16,
    pub trade_fee_rate: u64,
    pub protocol_fee_rate: u64,
    pub fund_fee_rate: u64,
    pub create_pool_fee: u64,
    pub max_open_time: u64,
}

/// Arguments for `create_rewards`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct CreateRewardsArgs {
    pub start_time: u64,
    pub end_time: u64,
    pub reward_amount: u64,
}

/// Arguments for `create_swap_referral`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct CreateSwapReferralArgs {
    pub name: [u8; 32],
    pub default_share_bps: u16,
}

/// Arguments for `deposit`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct DepositArgs {
    pub lp_token_amount: u64,
    pub maximum_token_0_amount: u64,
    pub maximum_token_1_amount: u64,
}

/// Arguments for `init_user_pool_liquidity`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct InitUserPoolLiquidityArgs {
    pub partner: Option<[u8; 32]>,
}

/// Arguments for `initialize`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct InitializeArgs {
    pub init_amount_0: u64,
    pub init_amount_1: u64,
    pub open_time: u64,
    pub max_trade_fee_rate: u64,
    pub volatility_factor: u64,
}

/// Arguments for `initialize_partner`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct InitializePartnerArgs {
    pub name: [u8; 32],
    pub token_0_token_account: [u8; 32],
    pub token_1_token_account: [u8; 32],
}

/// Arguments for `oracle_based_swap_base_input`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct OracleBasedSwapBaseInputArgs {
    pub amount_in: u64,
    pub minimum_amount_out: u64,
}

/// Arguments for `pool_config_update`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct PoolConfigUpdateArgs {
    pub h1: u128,
}

/// Arguments for `pool_config_update_v2`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct PoolConfigUpdateV2Args {
    pub h1: u128,
    pub price_fetched_at: u64,
}

/// Arguments for `pool_config_update_v3`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct PoolConfigUpdateV3Args {
    pub h1: u128,
    pub h2: u128,
    pub price_fetched_at: u64,
    pub min_trade_rate_at_oracle_price: Option<u32>,
    pub oracle_price_delay_fee_rate_per_second: Option<u32>,
    pub max_oracle_price_delay_fee: Option<u32>,
    pub drift_factor: Option<u32>,
    pub max_drift_factor: Option<u32>,
    pub volatility_factor: Option<u64>,
}

/// Arguments for `swap_base_input`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SwapBaseInputArgs {
    pub amount_in: u64,
    pub minimum_amount_out: u64,
}

/// Arguments for `swap_base_output`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SwapBaseOutputArgs {
    pub max_amount_in: u64,
    pub amount_out: u64,
}

/// Arguments for `update_amm_config`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct UpdateAmmConfigArgs {
    pub param: u16,
    pub value: u64,
}

/// Arguments for `update_partner`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct UpdatePartnerArgs {
    pub token_account_0: Option<[u8; 32]>,
    pub token_account_1: Option<[u8; 32]>,
}

/// Arguments for `update_pool`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct UpdatePoolArgs {
    pub param: u32,
    pub value: u64,
}

/// Arguments for `withdraw`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct WithdrawArgs {
    pub lp_token_amount: u64,
    pub minimum_token_0_amount: u64,
    pub minimum_token_1_amount: u64,
}


// ============================================
// Instruction Accounts
// ============================================
/// Accounts for `add_or_update_actor`
pub struct AddOrUpdateActorAccounts<'a> {
    /// Only admin(multisig) can add or update actor
    pub owner: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
    /// actor
    pub actor: &'a AccountView,
    /// Account for which the actor is being created.
    pub authority: &'a AccountView,
}

impl<'a> AddOrUpdateActorAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 4] {
        [
            InstructionAccount::writable_signer(self.owner.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::writable(self.actor.address()),
            InstructionAccount::readonly(self.authority.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 4] {
        [
            self.owner, self.system_program, self.actor, self.authority
        ]
    }
}

/// Accounts for `add_partner`
pub struct AddPartnerAccounts<'a> {
    /// authority
    pub authority: &'a AccountView,
    /// amm_config
    pub amm_config: &'a AccountView,
    /// pool_state
    pub pool_state: &'a AccountView,
    /// pool_partners
    pub pool_partners: &'a AccountView,
    /// partner
    pub partner: &'a AccountView,
}

impl<'a> AddPartnerAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 5] {
        [
            InstructionAccount::readonly_signer(self.authority.address()),
            InstructionAccount::readonly(self.amm_config.address()),
            InstructionAccount::writable(self.pool_state.address()),
            InstructionAccount::writable(self.pool_partners.address()),
            InstructionAccount::readonly(self.partner.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 5] {
        [
            self.authority, self.amm_config, self.pool_state, self.pool_partners, self.partner
        ]
    }
}

/// Accounts for `calculate_rewards`
pub struct CalculateRewardsAccounts<'a> {
    /// signer
    pub signer: &'a AccountView,
    /// User for which we are calculating rewards
    pub user: &'a AccountView,
    /// pool_state
    pub pool_state: &'a AccountView,
    /// reward_info
    pub reward_info: &'a AccountView,
    /// user_reward_info
    pub user_reward_info: &'a AccountView,
    /// User pool liquidity account
    pub user_pool_liquidity: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
}

impl<'a> CalculateRewardsAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 7] {
        [
            InstructionAccount::writable_signer(self.signer.address()),
            InstructionAccount::readonly(self.user.address()),
            InstructionAccount::readonly(self.pool_state.address()),
            InstructionAccount::writable(self.reward_info.address()),
            InstructionAccount::writable(self.user_reward_info.address()),
            InstructionAccount::readonly(self.user_pool_liquidity.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 7] {
        [
            self.signer, self.user, self.pool_state, self.reward_info, self.user_reward_info, self.user_pool_liquidity, self.system_program
        ]
    }
}

/// Accounts for `claim_partner_fees`
pub struct ClaimPartnerFeesAccounts<'a> {
    /// partner
    pub partner: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
    /// pool_state
    pub pool_state: &'a AccountView,
    /// The address that holds pool tokens for token_0
    pub token_0_vault: &'a AccountView,
    /// The address that holds pool tokens for token_1
    pub token_1_vault: &'a AccountView,
    /// The mint of token_0 vault
    pub vault_0_mint: &'a AccountView,
    /// The mint of token_1 vault
    pub vault_1_mint: &'a AccountView,
    /// pool_partners
    pub pool_partners: &'a AccountView,
    /// token_0_token_account
    pub token_0_token_account: &'a AccountView,
    /// token_1_token_account
    pub token_1_token_account: &'a AccountView,
    /// The SPL program to perform token transfers
    pub token_program: &'a AccountView,
    /// The SPL program 2022 to perform token transfers
    pub token_program_2022: &'a AccountView,
}

impl<'a> ClaimPartnerFeesAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 12] {
        [
            InstructionAccount::readonly(self.partner.address()),
            InstructionAccount::readonly(self.authority.address()),
            InstructionAccount::readonly(self.pool_state.address()),
            InstructionAccount::writable(self.token_0_vault.address()),
            InstructionAccount::writable(self.token_1_vault.address()),
            InstructionAccount::readonly(self.vault_0_mint.address()),
            InstructionAccount::readonly(self.vault_1_mint.address()),
            InstructionAccount::writable(self.pool_partners.address()),
            InstructionAccount::writable(self.token_0_token_account.address()),
            InstructionAccount::writable(self.token_1_token_account.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.token_program_2022.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 12] {
        [
            self.partner, self.authority, self.pool_state, self.token_0_vault, self.token_1_vault, self.vault_0_mint, self.vault_1_mint, self.pool_partners, self.token_0_token_account, self.token_1_token_account, self.token_program, self.token_program_2022
        ]
    }
}

/// Accounts for `claim_rewards`
pub struct ClaimRewardsAccounts<'a> {
    /// user
    pub user: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
    /// pool_state
    pub pool_state: &'a AccountView,
    /// reward_info
    pub reward_info: &'a AccountView,
    /// reward_vault
    pub reward_vault: &'a AccountView,
    /// user_token_account
    pub user_token_account: &'a AccountView,
    /// user_reward_info
    pub user_reward_info: &'a AccountView,
    /// reward_mint
    pub reward_mint: &'a AccountView,
    /// token Program
    pub token_program: &'a AccountView,
    /// Token program 2022
    pub token_program_2022: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
}

impl<'a> ClaimRewardsAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 11] {
        [
            InstructionAccount::writable_signer(self.user.address()),
            InstructionAccount::readonly(self.authority.address()),
            InstructionAccount::readonly(self.pool_state.address()),
            InstructionAccount::readonly(self.reward_info.address()),
            InstructionAccount::writable(self.reward_vault.address()),
            InstructionAccount::writable(self.user_token_account.address()),
            InstructionAccount::writable(self.user_reward_info.address()),
            InstructionAccount::writable(self.reward_mint.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.token_program_2022.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 11] {
        [
            self.user, self.authority, self.pool_state, self.reward_info, self.reward_vault, self.user_token_account, self.user_reward_info, self.reward_mint, self.token_program, self.token_program_2022, self.system_program
        ]
    }
}

/// Accounts for `collect_fund_fee`
pub struct CollectFundFeeAccounts<'a> {
    /// Only admin or fund_owner can collect fee now
    pub owner: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
    /// Pool state stores accumulated protocol fee amount
    pub pool_state: &'a AccountView,
    /// Amm config account stores fund_owner
    pub amm_config: &'a AccountView,
    /// The address that holds pool tokens for token_0
    pub token_0_vault: &'a AccountView,
    /// The address that holds pool tokens for token_1
    pub token_1_vault: &'a AccountView,
    /// The mint of token_0 vault
    pub vault_0_mint: &'a AccountView,
    /// The mint of token_1 vault
    pub vault_1_mint: &'a AccountView,
    /// The address that receives the collected token_0 fund fees
    pub recipient_token_0_account: &'a AccountView,
    /// The address that receives the collected token_1 fund fees
    pub recipient_token_1_account: &'a AccountView,
    /// The SPL program to perform token transfers
    pub token_program: &'a AccountView,
    /// The SPL program 2022 to perform token transfers
    pub token_program_2022: &'a AccountView,
}

impl<'a> CollectFundFeeAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 12] {
        [
            InstructionAccount::readonly_signer(self.owner.address()),
            InstructionAccount::readonly(self.authority.address()),
            InstructionAccount::writable(self.pool_state.address()),
            InstructionAccount::readonly(self.amm_config.address()),
            InstructionAccount::writable(self.token_0_vault.address()),
            InstructionAccount::writable(self.token_1_vault.address()),
            InstructionAccount::readonly(self.vault_0_mint.address()),
            InstructionAccount::readonly(self.vault_1_mint.address()),
            InstructionAccount::writable(self.recipient_token_0_account.address()),
            InstructionAccount::writable(self.recipient_token_1_account.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.token_program_2022.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 12] {
        [
            self.owner, self.authority, self.pool_state, self.amm_config, self.token_0_vault, self.token_1_vault, self.vault_0_mint, self.vault_1_mint, self.recipient_token_0_account, self.recipient_token_1_account, self.token_program, self.token_program_2022
        ]
    }
}

/// Accounts for `collect_protocol_fee`
pub struct CollectProtocolFeeAccounts<'a> {
    /// Only admin or owner can collect fee now
    pub owner: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
    /// Pool state stores accumulated protocol fee amount
    pub pool_state: &'a AccountView,
    /// Amm config account stores owner
    pub amm_config: &'a AccountView,
    /// The address that holds pool tokens for token_0
    pub token_0_vault: &'a AccountView,
    /// The address that holds pool tokens for token_1
    pub token_1_vault: &'a AccountView,
    /// The mint of token_0 vault
    pub vault_0_mint: &'a AccountView,
    /// The mint of token_1 vault
    pub vault_1_mint: &'a AccountView,
    /// The address that receives the collected token_0 protocol fees
    pub recipient_token_0_account: &'a AccountView,
    /// The address that receives the collected token_1 protocol fees
    pub recipient_token_1_account: &'a AccountView,
    /// The SPL program to perform token transfers
    pub token_program: &'a AccountView,
    /// The SPL program 2022 to perform token transfers
    pub token_program_2022: &'a AccountView,
}

impl<'a> CollectProtocolFeeAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 12] {
        [
            InstructionAccount::readonly_signer(self.owner.address()),
            InstructionAccount::readonly(self.authority.address()),
            InstructionAccount::writable(self.pool_state.address()),
            InstructionAccount::readonly(self.amm_config.address()),
            InstructionAccount::writable(self.token_0_vault.address()),
            InstructionAccount::writable(self.token_1_vault.address()),
            InstructionAccount::readonly(self.vault_0_mint.address()),
            InstructionAccount::readonly(self.vault_1_mint.address()),
            InstructionAccount::writable(self.recipient_token_0_account.address()),
            InstructionAccount::writable(self.recipient_token_1_account.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.token_program_2022.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 12] {
        [
            self.owner, self.authority, self.pool_state, self.amm_config, self.token_0_vault, self.token_1_vault, self.vault_0_mint, self.vault_1_mint, self.recipient_token_0_account, self.recipient_token_1_account, self.token_program, self.token_program_2022
        ]
    }
}

/// Accounts for `create_amm_config`
pub struct CreateAmmConfigAccounts<'a> {
    /// Address to be set as protocol owner.
    pub owner: &'a AccountView,
    /// Initialize AmmConfig state account to store protocol owner address and fee rates
    pub amm_config: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
}

impl<'a> CreateAmmConfigAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 3] {
        [
            InstructionAccount::writable_signer(self.owner.address()),
            InstructionAccount::writable(self.amm_config.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 3] {
        [
            self.owner, self.amm_config, self.system_program
        ]
    }
}

/// Accounts for `create_rewards`
pub struct CreateRewardsAccounts<'a> {
    /// reward_provider
    pub reward_provider: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
    /// pool_state
    pub pool_state: &'a AccountView,
    /// reward_info
    pub reward_info: &'a AccountView,
    /// reward_providers_token_account
    pub reward_providers_token_account: &'a AccountView,
    /// For reward to deposit into.
    pub reward_vault: &'a AccountView,
    /// reward_mint
    pub reward_mint: &'a AccountView,
    /// token Program
    pub token_program: &'a AccountView,
    /// Token program 2022
    pub token_program_2022: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
}

impl<'a> CreateRewardsAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 10] {
        [
            InstructionAccount::writable_signer(self.reward_provider.address()),
            InstructionAccount::readonly(self.authority.address()),
            InstructionAccount::readonly(self.pool_state.address()),
            InstructionAccount::writable(self.reward_info.address()),
            InstructionAccount::writable(self.reward_providers_token_account.address()),
            InstructionAccount::writable(self.reward_vault.address()),
            InstructionAccount::writable(self.reward_mint.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.token_program_2022.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 10] {
        [
            self.reward_provider, self.authority, self.pool_state, self.reward_info, self.reward_providers_token_account, self.reward_vault, self.reward_mint, self.token_program, self.token_program_2022, self.system_program
        ]
    }
}

/// Accounts for `create_swap_referral`
pub struct CreateSwapReferralAccounts<'a> {
    /// Admin signer for this operation
    pub admin: &'a AccountView,
    /// owner
    pub owner: &'a AccountView,
    /// payer
    pub payer: &'a AccountView,
    /// The config acts as the base for its referral project
    pub amm_config: &'a AccountView,
    /// project
    pub project: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
    /// referral_program
    pub referral_program: &'a AccountView,
}

impl<'a> CreateSwapReferralAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 7] {
        [
            InstructionAccount::readonly_signer(self.admin.address()),
            InstructionAccount::readonly(self.owner.address()),
            InstructionAccount::writable_signer(self.payer.address()),
            InstructionAccount::writable(self.amm_config.address()),
            InstructionAccount::writable(self.project.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.referral_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 7] {
        [
            self.admin, self.owner, self.payer, self.amm_config, self.project, self.system_program, self.referral_program
        ]
    }
}

/// Accounts for `deposit`
pub struct DepositAccounts<'a> {
    /// Owner of the liquidity provided
    pub owner: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
    /// Pool state the owner is depositing into
    pub pool_state: &'a AccountView,
    /// user_pool_liquidity
    pub user_pool_liquidity: &'a AccountView,
    /// The payer's token account to deposit token_0
    pub token_0_account: &'a AccountView,
    /// The payer's token account to deposit token_1
    pub token_1_account: &'a AccountView,
    /// Pool vault for token_0 to deposit into
    pub token_0_vault: &'a AccountView,
    /// Pool vault for token_1 to deposit into
    pub token_1_vault: &'a AccountView,
    /// token Program
    pub token_program: &'a AccountView,
    /// Token program 2022
    pub token_program_2022: &'a AccountView,
    /// The mint of token_0 vault
    pub vault_0_mint: &'a AccountView,
    /// The mint of token_1 vault
    pub vault_1_mint: &'a AccountView,
    /// pool_partners
    pub pool_partners: &'a AccountView,
}

impl<'a> DepositAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 13] {
        [
            InstructionAccount::readonly_signer(self.owner.address()),
            InstructionAccount::readonly(self.authority.address()),
            InstructionAccount::writable(self.pool_state.address()),
            InstructionAccount::writable(self.user_pool_liquidity.address()),
            InstructionAccount::writable(self.token_0_account.address()),
            InstructionAccount::writable(self.token_1_account.address()),
            InstructionAccount::writable(self.token_0_vault.address()),
            InstructionAccount::writable(self.token_1_vault.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.token_program_2022.address()),
            InstructionAccount::readonly(self.vault_0_mint.address()),
            InstructionAccount::readonly(self.vault_1_mint.address()),
            InstructionAccount::writable(self.pool_partners.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 13] {
        [
            self.owner, self.authority, self.pool_state, self.user_pool_liquidity, self.token_0_account, self.token_1_account, self.token_0_vault, self.token_1_vault, self.token_program, self.token_program_2022, self.vault_0_mint, self.vault_1_mint, self.pool_partners
        ]
    }
}

/// Accounts for `init_user_pool_liquidity`
pub struct InitUserPoolLiquidityAccounts<'a> {
    /// user
    pub user: &'a AccountView,
    /// pool_state
    pub pool_state: &'a AccountView,
    /// user_pool_liquidity
    pub user_pool_liquidity: &'a AccountView,
    /// pool_partners
    pub pool_partners: &'a AccountView,
    /// To create a new program account
    pub system_program: &'a AccountView,
}

impl<'a> InitUserPoolLiquidityAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 5] {
        [
            InstructionAccount::writable_signer(self.user.address()),
            InstructionAccount::readonly(self.pool_state.address()),
            InstructionAccount::writable(self.user_pool_liquidity.address()),
            InstructionAccount::writable(self.pool_partners.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 5] {
        [
            self.user, self.pool_state, self.user_pool_liquidity, self.pool_partners, self.system_program
        ]
    }
}

/// Accounts for `initialize`
pub struct InitializeAccounts<'a> {
    /// Address paying to create the pool. It can be anyone.
    pub creator: &'a AccountView,
    /// Which amm config the pool belongs to
    pub amm_config: &'a AccountView,
    /// sign transactions on behalf of the pool
    pub authority: &'a AccountView,
    /// Initialize an account to store the pool state
    pub pool_state: &'a AccountView,
    /// user_pool_liquidity
    pub user_pool_liquidity: &'a AccountView,
    /// Token_0 mint, the key must smaller than token_1 mint.
    pub token_0_mint: &'a AccountView,
    /// Token_1 mint, the key must greater than token_0 mint.
    pub token_1_mint: &'a AccountView,
    /// creator token 0 account
    pub creator_token_0: &'a AccountView,
    /// creator token 1 account
    pub creator_token_1: &'a AccountView,
    /// token_0_vault
    pub token_0_vault: &'a AccountView,
    /// token_1_vault
    pub token_1_vault: &'a AccountView,
    /// create pool fee account
    pub create_pool_fee: &'a AccountView,
    /// an account to store oracle observations
    pub observation_state: &'a AccountView,
    /// Program to create mint account and mint tokens
    pub token_program: &'a AccountView,
    /// Spl token program or token program 2022
    pub token_0_program: &'a AccountView,
    /// Spl token program or token program 2022
    pub token_1_program: &'a AccountView,
    /// Program to create an ATA for receiving position NFT
    pub associated_token_program: &'a AccountView,
    /// To create a new program account
    pub system_program: &'a AccountView,
    /// Sysvar for program account
    pub rent: &'a AccountView,
}

impl<'a> InitializeAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 19] {
        [
            InstructionAccount::writable_signer(self.creator.address()),
            InstructionAccount::readonly(self.amm_config.address()),
            InstructionAccount::readonly(self.authority.address()),
            InstructionAccount::writable(self.pool_state.address()),
            InstructionAccount::writable(self.user_pool_liquidity.address()),
            InstructionAccount::readonly(self.token_0_mint.address()),
            InstructionAccount::readonly(self.token_1_mint.address()),
            InstructionAccount::writable(self.creator_token_0.address()),
            InstructionAccount::writable(self.creator_token_1.address()),
            InstructionAccount::writable(self.token_0_vault.address()),
            InstructionAccount::writable(self.token_1_vault.address()),
            InstructionAccount::writable(self.create_pool_fee.address()),
            InstructionAccount::writable(self.observation_state.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.token_0_program.address()),
            InstructionAccount::readonly(self.token_1_program.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.rent.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 19] {
        [
            self.creator, self.amm_config, self.authority, self.pool_state, self.user_pool_liquidity, self.token_0_mint, self.token_1_mint, self.creator_token_0, self.creator_token_1, self.token_0_vault, self.token_1_vault, self.create_pool_fee, self.observation_state, self.token_program, self.token_0_program, self.token_1_program, self.associated_token_program, self.system_program, self.rent
        ]
    }
}

/// Accounts for `initialize_partner`
pub struct InitializePartnerAccounts<'a> {
    /// payer
    pub payer: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
    /// pool_state
    pub pool_state: &'a AccountView,
    /// pool_partners
    pub pool_partners: &'a AccountView,
    /// partner
    pub partner: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
}

impl<'a> InitializePartnerAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 6] {
        [
            InstructionAccount::writable_signer(self.payer.address()),
            InstructionAccount::readonly_signer(self.authority.address()),
            InstructionAccount::readonly(self.pool_state.address()),
            InstructionAccount::writable(self.pool_partners.address()),
            InstructionAccount::writable_signer(self.partner.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 6] {
        [
            self.payer, self.authority, self.pool_state, self.pool_partners, self.partner, self.system_program
        ]
    }
}

/// Accounts for `initialize_pool_partners`
pub struct InitializePoolPartnersAccounts<'a> {
    /// payer
    pub payer: &'a AccountView,
    /// pool_state
    pub pool_state: &'a AccountView,
    /// pool_partners
    pub pool_partners: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
}

impl<'a> InitializePoolPartnersAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 4] {
        [
            InstructionAccount::writable_signer(self.payer.address()),
            InstructionAccount::readonly(self.pool_state.address()),
            InstructionAccount::writable(self.pool_partners.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 4] {
        [
            self.payer, self.pool_state, self.pool_partners, self.system_program
        ]
    }
}

/// Accounts for `oracle_based_swap_base_input`
pub struct OracleBasedSwapBaseInputAccounts<'a> {
    /// The user performing the swap
    pub payer: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
    /// The factory state to read protocol fees
    pub amm_config: &'a AccountView,
    /// The program account of the pool in which the swap will be performed
    pub pool_state: &'a AccountView,
    /// The user token account for input token
    pub input_token_account: &'a AccountView,
    /// The user token account for output token
    pub output_token_account: &'a AccountView,
    /// The vault token account for input token
    pub input_vault: &'a AccountView,
    /// The vault token account for output token
    pub output_vault: &'a AccountView,
    /// SPL program for input token transfers
    pub input_token_program: &'a AccountView,
    /// SPL program for output token transfers
    pub output_token_program: &'a AccountView,
    /// The mint of input token
    pub input_token_mint: &'a AccountView,
    /// The mint of output token
    pub output_token_mint: &'a AccountView,
    /// The program account for the most recent oracle observation
    pub observation_state: &'a AccountView,
}

impl<'a> OracleBasedSwapBaseInputAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 13] {
        [
            InstructionAccount::readonly_signer(self.payer.address()),
            InstructionAccount::readonly(self.authority.address()),
            InstructionAccount::readonly(self.amm_config.address()),
            InstructionAccount::writable(self.pool_state.address()),
            InstructionAccount::writable(self.input_token_account.address()),
            InstructionAccount::writable(self.output_token_account.address()),
            InstructionAccount::writable(self.input_vault.address()),
            InstructionAccount::writable(self.output_vault.address()),
            InstructionAccount::readonly(self.input_token_program.address()),
            InstructionAccount::readonly(self.output_token_program.address()),
            InstructionAccount::readonly(self.input_token_mint.address()),
            InstructionAccount::readonly(self.output_token_mint.address()),
            InstructionAccount::writable(self.observation_state.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 13] {
        [
            self.payer, self.authority, self.amm_config, self.pool_state, self.input_token_account, self.output_token_account, self.input_vault, self.output_vault, self.input_token_program, self.output_token_program, self.input_token_mint, self.output_token_mint, self.observation_state
        ]
    }
}

/// Accounts for `pool_config_update`
pub struct PoolConfigUpdateAccounts<'a> {
    /// authority
    pub authority: &'a AccountView,
    /// pool_state
    pub pool_state: &'a AccountView,
    /// amm_config
    pub amm_config: &'a AccountView,
}

impl<'a> PoolConfigUpdateAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 3] {
        [
            InstructionAccount::readonly_signer(self.authority.address()),
            InstructionAccount::writable(self.pool_state.address()),
            InstructionAccount::readonly(self.amm_config.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 3] {
        [
            self.authority, self.pool_state, self.amm_config
        ]
    }
}

/// Accounts for `pool_config_update_v2`
pub struct PoolConfigUpdateV2Accounts<'a> {
    /// authority
    pub authority: &'a AccountView,
    /// pool_state
    pub pool_state: &'a AccountView,
    /// amm_config
    pub amm_config: &'a AccountView,
}

impl<'a> PoolConfigUpdateV2Accounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 3] {
        [
            InstructionAccount::readonly_signer(self.authority.address()),
            InstructionAccount::writable(self.pool_state.address()),
            InstructionAccount::readonly(self.amm_config.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 3] {
        [
            self.authority, self.pool_state, self.amm_config
        ]
    }
}

/// Accounts for `pool_config_update_v3`
pub struct PoolConfigUpdateV3Accounts<'a> {
    /// authority
    pub authority: &'a AccountView,
    /// pool_state
    pub pool_state: &'a AccountView,
    /// amm_config
    pub amm_config: &'a AccountView,
    /// observation_state
    pub observation_state: &'a AccountView,
    /// actor
    pub actor: &'a AccountView,
}

impl<'a> PoolConfigUpdateV3Accounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 5] {
        [
            InstructionAccount::readonly_signer(self.authority.address()),
            InstructionAccount::writable(self.pool_state.address()),
            InstructionAccount::readonly(self.amm_config.address()),
            InstructionAccount::writable(self.observation_state.address()),
            InstructionAccount::readonly(self.actor.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 5] {
        [
            self.authority, self.pool_state, self.amm_config, self.observation_state, self.actor
        ]
    }
}

/// Accounts for `rebalance_kamino`
pub struct RebalanceKaminoAccounts<'a> {
    /// signer
    pub signer: &'a AccountView,
    /// gamma_authority
    pub gamma_authority: &'a AccountView,
    /// The program account of the pool in which the swap will be performed
    pub pool_state: &'a AccountView,
    /// The vault token account for token 0
    pub token_vault: &'a AccountView,
    /// token_mint
    pub token_mint: &'a AccountView,
    /// kamino_reserve
    pub kamino_reserve: &'a AccountView,
    /// kamino_lending_market
    pub kamino_lending_market: &'a AccountView,
    /// lending_market_authority
    pub lending_market_authority: &'a AccountView,
    /// reserve_liquidity_supply
    pub reserve_liquidity_supply: &'a AccountView,
    /// reserve_collateral_mint
    pub reserve_collateral_mint: &'a AccountView,
    /// gamma_pool_destination_collateral
    pub gamma_pool_destination_collateral: &'a AccountView,
    /// instruction_sysvar_account
    pub instruction_sysvar_account: &'a AccountView,
    /// liquidity_token_program
    pub liquidity_token_program: &'a AccountView,
    /// collateral_token_program
    pub collateral_token_program: &'a AccountView,
    /// kamino_program
    pub kamino_program: &'a AccountView,
    /// token_program
    pub token_program: &'a AccountView,
    /// token_program_2022
    pub token_program_2022: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
}

impl<'a> RebalanceKaminoAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 18] {
        [
            InstructionAccount::writable_signer(self.signer.address()),
            InstructionAccount::readonly(self.gamma_authority.address()),
            InstructionAccount::writable(self.pool_state.address()),
            InstructionAccount::writable(self.token_vault.address()),
            InstructionAccount::writable(self.token_mint.address()),
            InstructionAccount::writable(self.kamino_reserve.address()),
            InstructionAccount::writable(self.kamino_lending_market.address()),
            InstructionAccount::readonly(self.lending_market_authority.address()),
            InstructionAccount::writable(self.reserve_liquidity_supply.address()),
            InstructionAccount::writable(self.reserve_collateral_mint.address()),
            InstructionAccount::writable(self.gamma_pool_destination_collateral.address()),
            InstructionAccount::readonly(self.instruction_sysvar_account.address()),
            InstructionAccount::readonly(self.liquidity_token_program.address()),
            InstructionAccount::readonly(self.collateral_token_program.address()),
            InstructionAccount::readonly(self.kamino_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.token_program_2022.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 18] {
        [
            self.signer, self.gamma_authority, self.pool_state, self.token_vault, self.token_mint, self.kamino_reserve, self.kamino_lending_market, self.lending_market_authority, self.reserve_liquidity_supply, self.reserve_collateral_mint, self.gamma_pool_destination_collateral, self.instruction_sysvar_account, self.liquidity_token_program, self.collateral_token_program, self.kamino_program, self.token_program, self.token_program_2022, self.system_program
        ]
    }
}

/// Accounts for `swap_base_input`
pub struct SwapBaseInputAccounts<'a> {
    /// The user performing the swap
    pub payer: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
    /// The factory state to read protocol fees
    pub amm_config: &'a AccountView,
    /// The program account of the pool in which the swap will be performed
    pub pool_state: &'a AccountView,
    /// The user token account for input token
    pub input_token_account: &'a AccountView,
    /// The user token account for output token
    pub output_token_account: &'a AccountView,
    /// The vault token account for input token
    pub input_vault: &'a AccountView,
    /// The vault token account for output token
    pub output_vault: &'a AccountView,
    /// SPL program for input token transfers
    pub input_token_program: &'a AccountView,
    /// SPL program for output token transfers
    pub output_token_program: &'a AccountView,
    /// The mint of input token
    pub input_token_mint: &'a AccountView,
    /// The mint of output token
    pub output_token_mint: &'a AccountView,
    /// The program account for the most recent oracle observation
    pub observation_state: &'a AccountView,
}

impl<'a> SwapBaseInputAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 13] {
        [
            InstructionAccount::readonly_signer(self.payer.address()),
            InstructionAccount::readonly(self.authority.address()),
            InstructionAccount::readonly(self.amm_config.address()),
            InstructionAccount::writable(self.pool_state.address()),
            InstructionAccount::writable(self.input_token_account.address()),
            InstructionAccount::writable(self.output_token_account.address()),
            InstructionAccount::writable(self.input_vault.address()),
            InstructionAccount::writable(self.output_vault.address()),
            InstructionAccount::readonly(self.input_token_program.address()),
            InstructionAccount::readonly(self.output_token_program.address()),
            InstructionAccount::readonly(self.input_token_mint.address()),
            InstructionAccount::readonly(self.output_token_mint.address()),
            InstructionAccount::writable(self.observation_state.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 13] {
        [
            self.payer, self.authority, self.amm_config, self.pool_state, self.input_token_account, self.output_token_account, self.input_vault, self.output_vault, self.input_token_program, self.output_token_program, self.input_token_mint, self.output_token_mint, self.observation_state
        ]
    }
}

/// Accounts for `swap_base_output`
pub struct SwapBaseOutputAccounts<'a> {
    /// The user performing the swap
    pub payer: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
    /// The factory state to read protocol fees
    pub amm_config: &'a AccountView,
    /// The program account of the pool in which the swap will be performed
    pub pool_state: &'a AccountView,
    /// The user token account for input token
    pub input_token_account: &'a AccountView,
    /// The user token account for output token
    pub output_token_account: &'a AccountView,
    /// The vault token account for input token
    pub input_vault: &'a AccountView,
    /// The vault token account for output token
    pub output_vault: &'a AccountView,
    /// SPL program for input token transfers
    pub input_token_program: &'a AccountView,
    /// SPL program for output token transfers
    pub output_token_program: &'a AccountView,
    /// The mint of input token
    pub input_token_mint: &'a AccountView,
    /// The mint of output token
    pub output_token_mint: &'a AccountView,
    /// The program account for the most recent oracle observation
    pub observation_state: &'a AccountView,
}

impl<'a> SwapBaseOutputAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 13] {
        [
            InstructionAccount::readonly_signer(self.payer.address()),
            InstructionAccount::readonly(self.authority.address()),
            InstructionAccount::readonly(self.amm_config.address()),
            InstructionAccount::writable(self.pool_state.address()),
            InstructionAccount::writable(self.input_token_account.address()),
            InstructionAccount::writable(self.output_token_account.address()),
            InstructionAccount::writable(self.input_vault.address()),
            InstructionAccount::writable(self.output_vault.address()),
            InstructionAccount::readonly(self.input_token_program.address()),
            InstructionAccount::readonly(self.output_token_program.address()),
            InstructionAccount::readonly(self.input_token_mint.address()),
            InstructionAccount::readonly(self.output_token_mint.address()),
            InstructionAccount::writable(self.observation_state.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 13] {
        [
            self.payer, self.authority, self.amm_config, self.pool_state, self.input_token_account, self.output_token_account, self.input_vault, self.output_vault, self.input_token_program, self.output_token_program, self.input_token_mint, self.output_token_mint, self.observation_state
        ]
    }
}

/// Accounts for `update_amm_config`
pub struct UpdateAmmConfigAccounts<'a> {
    /// The amm config owner or admin
    pub owner: &'a AccountView,
    /// The amm config account to update
    pub amm_config: &'a AccountView,
}

impl<'a> UpdateAmmConfigAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 2] {
        [
            InstructionAccount::readonly_signer(self.owner.address()),
            InstructionAccount::writable(self.amm_config.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 2] {
        [
            self.owner, self.amm_config
        ]
    }
}

/// Accounts for `update_partner`
pub struct UpdatePartnerAccounts<'a> {
    /// authority
    pub authority: &'a AccountView,
    /// partner
    pub partner: &'a AccountView,
}

impl<'a> UpdatePartnerAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 2] {
        [
            InstructionAccount::readonly_signer(self.authority.address()),
            InstructionAccount::writable(self.partner.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 2] {
        [
            self.authority, self.partner
        ]
    }
}

/// Accounts for `update_partner_fees`
pub struct UpdatePartnerFeesAccounts<'a> {
    /// pool_state
    pub pool_state: &'a AccountView,
    /// pool_partners
    pub pool_partners: &'a AccountView,
}

impl<'a> UpdatePartnerFeesAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 2] {
        [
            InstructionAccount::writable(self.pool_state.address()),
            InstructionAccount::writable(self.pool_partners.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 2] {
        [
            self.pool_state, self.pool_partners
        ]
    }
}

/// Accounts for `update_pool`
pub struct UpdatePoolAccounts<'a> {
    /// authority
    pub authority: &'a AccountView,
    /// pool_state
    pub pool_state: &'a AccountView,
    /// actor
    pub actor: &'a AccountView,
}

impl<'a> UpdatePoolAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 3] {
        [
            InstructionAccount::readonly_signer(self.authority.address()),
            InstructionAccount::writable(self.pool_state.address()),
            InstructionAccount::readonly(self.actor.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 3] {
        [
            self.authority, self.pool_state, self.actor
        ]
    }
}

/// Accounts for `withdraw`
pub struct WithdrawAccounts<'a> {
    /// Owner of the liquidity provided
    pub owner: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
    /// Pool state account
    pub pool_state: &'a AccountView,
    /// User pool liquidity account
    pub user_pool_liquidity: &'a AccountView,
    /// The owner's token account for receive token_0
    pub token_0_account: &'a AccountView,
    /// The owner's token account for receive token_1
    pub token_1_account: &'a AccountView,
    /// The address that holds pool tokens for token_0
    pub token_0_vault: &'a AccountView,
    /// The address that holds pool tokens for token_1
    pub token_1_vault: &'a AccountView,
    /// token Program
    pub token_program: &'a AccountView,
    /// Token program 2022
    pub token_program_2022: &'a AccountView,
    /// The mint of token_0 vault
    pub vault_0_mint: &'a AccountView,
    /// The mint of token_1 vault
    pub vault_1_mint: &'a AccountView,
    /// pool_partners
    pub pool_partners: &'a AccountView,
    /// memo program
    pub memo_program: &'a AccountView,
    /// kamino_program
    pub kamino_program: &'a AccountView,
    /// instruction_sysvar_account
    pub instruction_sysvar_account: &'a AccountView,
}

impl<'a> WithdrawAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 16] {
        [
            InstructionAccount::readonly_signer(self.owner.address()),
            InstructionAccount::readonly(self.authority.address()),
            InstructionAccount::writable(self.pool_state.address()),
            InstructionAccount::writable(self.user_pool_liquidity.address()),
            InstructionAccount::writable(self.token_0_account.address()),
            InstructionAccount::writable(self.token_1_account.address()),
            InstructionAccount::writable(self.token_0_vault.address()),
            InstructionAccount::writable(self.token_1_vault.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.token_program_2022.address()),
            InstructionAccount::writable(self.vault_0_mint.address()),
            InstructionAccount::writable(self.vault_1_mint.address()),
            InstructionAccount::writable(self.pool_partners.address()),
            InstructionAccount::readonly(self.memo_program.address()),
            InstructionAccount::readonly(self.kamino_program.address()),
            InstructionAccount::readonly(self.instruction_sysvar_account.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 16] {
        [
            self.owner, self.authority, self.pool_state, self.user_pool_liquidity, self.token_0_account, self.token_1_account, self.token_0_vault, self.token_1_vault, self.token_program, self.token_program_2022, self.vault_0_mint, self.vault_1_mint, self.pool_partners, self.memo_program, self.kamino_program, self.instruction_sysvar_account
        ]
    }
}


// ============================================
// CPI Functions
// ============================================
/// CPI: add_or_update_actor
#[inline(always)]
pub fn add_or_update_actor<'a>(
    accounts: &AddOrUpdateActorAccounts<'a>, args: &AddOrUpdateActorArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<AddOrUpdateActorArgs>()];
    data[0..8].copy_from_slice(&ADD_OR_UPDATE_ACTOR);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const AddOrUpdateActorArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<AddOrUpdateActorArgs>(),
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

/// CPI: add_partner
#[inline(always)]
pub fn add_partner<'a>(
    accounts: &AddPartnerAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&ADD_PARTNER);
    
    
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

/// CPI: calculate_rewards
#[inline(always)]
pub fn calculate_rewards<'a>(
    accounts: &CalculateRewardsAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&CALCULATE_REWARDS);
    
    
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

/// CPI: claim_partner_fees
#[inline(always)]
pub fn claim_partner_fees<'a>(
    accounts: &ClaimPartnerFeesAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&CLAIM_PARTNER_FEES);
    
    
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

/// CPI: claim_rewards
#[inline(always)]
pub fn claim_rewards<'a>(
    accounts: &ClaimRewardsAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&CLAIM_REWARDS);
    
    
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

/// CPI: collect_fund_fee
#[inline(always)]
pub fn collect_fund_fee<'a>(
    accounts: &CollectFundFeeAccounts<'a>, args: &CollectFundFeeArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<CollectFundFeeArgs>()];
    data[0..8].copy_from_slice(&COLLECT_FUND_FEE);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const CollectFundFeeArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<CollectFundFeeArgs>(),
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

/// CPI: collect_protocol_fee
#[inline(always)]
pub fn collect_protocol_fee<'a>(
    accounts: &CollectProtocolFeeAccounts<'a>, args: &CollectProtocolFeeArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<CollectProtocolFeeArgs>()];
    data[0..8].copy_from_slice(&COLLECT_PROTOCOL_FEE);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const CollectProtocolFeeArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<CollectProtocolFeeArgs>(),
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

/// CPI: create_amm_config
#[inline(always)]
pub fn create_amm_config<'a>(
    accounts: &CreateAmmConfigAccounts<'a>, args: &CreateAmmConfigArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<CreateAmmConfigArgs>()];
    data[0..8].copy_from_slice(&CREATE_AMM_CONFIG);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const CreateAmmConfigArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<CreateAmmConfigArgs>(),
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

/// CPI: create_rewards
#[inline(always)]
pub fn create_rewards<'a>(
    accounts: &CreateRewardsAccounts<'a>, args: &CreateRewardsArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<CreateRewardsArgs>()];
    data[0..8].copy_from_slice(&CREATE_REWARDS);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const CreateRewardsArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<CreateRewardsArgs>(),
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

/// CPI: create_swap_referral
#[inline(always)]
pub fn create_swap_referral<'a>(
    accounts: &CreateSwapReferralAccounts<'a>, args: &CreateSwapReferralArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<CreateSwapReferralArgs>()];
    data[0..8].copy_from_slice(&CREATE_SWAP_REFERRAL);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const CreateSwapReferralArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<CreateSwapReferralArgs>(),
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

/// CPI: deposit
#[inline(always)]
pub fn deposit<'a>(
    accounts: &DepositAccounts<'a>, args: &DepositArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<DepositArgs>()];
    data[0..8].copy_from_slice(&DEPOSIT);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const DepositArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<DepositArgs>(),
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

/// CPI: init_user_pool_liquidity
#[inline(always)]
pub fn init_user_pool_liquidity<'a>(
    accounts: &InitUserPoolLiquidityAccounts<'a>, args: &InitUserPoolLiquidityArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<InitUserPoolLiquidityArgs>()];
    data[0..8].copy_from_slice(&INIT_USER_POOL_LIQUIDITY);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const InitUserPoolLiquidityArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<InitUserPoolLiquidityArgs>(),
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

/// CPI: initialize
#[inline(always)]
pub fn initialize<'a>(
    accounts: &InitializeAccounts<'a>, args: &InitializeArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<InitializeArgs>()];
    data[0..8].copy_from_slice(&INITIALIZE);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const InitializeArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<InitializeArgs>(),
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

/// CPI: initialize_partner
#[inline(always)]
pub fn initialize_partner<'a>(
    accounts: &InitializePartnerAccounts<'a>, args: &InitializePartnerArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<InitializePartnerArgs>()];
    data[0..8].copy_from_slice(&INITIALIZE_PARTNER);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const InitializePartnerArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<InitializePartnerArgs>(),
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

/// CPI: initialize_pool_partners
#[inline(always)]
pub fn initialize_pool_partners<'a>(
    accounts: &InitializePoolPartnersAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&INITIALIZE_POOL_PARTNERS);
    
    
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

/// CPI: oracle_based_swap_base_input
#[inline(always)]
pub fn oracle_based_swap_base_input<'a>(
    accounts: &OracleBasedSwapBaseInputAccounts<'a>, args: &OracleBasedSwapBaseInputArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<OracleBasedSwapBaseInputArgs>()];
    data[0..8].copy_from_slice(&ORACLE_BASED_SWAP_BASE_INPUT);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const OracleBasedSwapBaseInputArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<OracleBasedSwapBaseInputArgs>(),
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

/// CPI: pool_config_update
#[inline(always)]
pub fn pool_config_update<'a>(
    accounts: &PoolConfigUpdateAccounts<'a>, args: &PoolConfigUpdateArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<PoolConfigUpdateArgs>()];
    data[0..8].copy_from_slice(&POOL_CONFIG_UPDATE);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const PoolConfigUpdateArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<PoolConfigUpdateArgs>(),
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

/// CPI: pool_config_update_v2
#[inline(always)]
pub fn pool_config_update_v2<'a>(
    accounts: &PoolConfigUpdateV2Accounts<'a>, args: &PoolConfigUpdateV2Args,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<PoolConfigUpdateV2Args>()];
    data[0..8].copy_from_slice(&POOL_CONFIG_UPDATE_V2);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const PoolConfigUpdateV2Args as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<PoolConfigUpdateV2Args>(),
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

/// CPI: pool_config_update_v3
#[inline(always)]
pub fn pool_config_update_v3<'a>(
    accounts: &PoolConfigUpdateV3Accounts<'a>, args: &PoolConfigUpdateV3Args,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<PoolConfigUpdateV3Args>()];
    data[0..8].copy_from_slice(&POOL_CONFIG_UPDATE_V3);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const PoolConfigUpdateV3Args as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<PoolConfigUpdateV3Args>(),
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

/// CPI: rebalance_kamino
#[inline(always)]
pub fn rebalance_kamino<'a>(
    accounts: &RebalanceKaminoAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&REBALANCE_KAMINO);
    
    
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
    invoke_signed::<18>(&instruction, &account_views, signers)
}

/// CPI: swap_base_input
#[inline(always)]
pub fn swap_base_input<'a>(
    accounts: &SwapBaseInputAccounts<'a>, args: &SwapBaseInputArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SwapBaseInputArgs>()];
    data[0..8].copy_from_slice(&SWAP_BASE_INPUT);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SwapBaseInputArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SwapBaseInputArgs>(),
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

/// CPI: swap_base_output
#[inline(always)]
pub fn swap_base_output<'a>(
    accounts: &SwapBaseOutputAccounts<'a>, args: &SwapBaseOutputArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SwapBaseOutputArgs>()];
    data[0..8].copy_from_slice(&SWAP_BASE_OUTPUT);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SwapBaseOutputArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SwapBaseOutputArgs>(),
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

/// CPI: update_amm_config
#[inline(always)]
pub fn update_amm_config<'a>(
    accounts: &UpdateAmmConfigAccounts<'a>, args: &UpdateAmmConfigArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<UpdateAmmConfigArgs>()];
    data[0..8].copy_from_slice(&UPDATE_AMM_CONFIG);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const UpdateAmmConfigArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<UpdateAmmConfigArgs>(),
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

/// CPI: update_partner
#[inline(always)]
pub fn update_partner<'a>(
    accounts: &UpdatePartnerAccounts<'a>, args: &UpdatePartnerArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<UpdatePartnerArgs>()];
    data[0..8].copy_from_slice(&UPDATE_PARTNER);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const UpdatePartnerArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<UpdatePartnerArgs>(),
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

/// CPI: update_partner_fees
#[inline(always)]
pub fn update_partner_fees<'a>(
    accounts: &UpdatePartnerFeesAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&UPDATE_PARTNER_FEES);
    
    
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

/// CPI: update_pool
#[inline(always)]
pub fn update_pool<'a>(
    accounts: &UpdatePoolAccounts<'a>, args: &UpdatePoolArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<UpdatePoolArgs>()];
    data[0..8].copy_from_slice(&UPDATE_POOL);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const UpdatePoolArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<UpdatePoolArgs>(),
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

/// CPI: withdraw
#[inline(always)]
pub fn withdraw<'a>(
    accounts: &WithdrawAccounts<'a>, args: &WithdrawArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<WithdrawArgs>()];
    data[0..8].copy_from_slice(&WITHDRAW);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const WithdrawArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<WithdrawArgs>(),
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

