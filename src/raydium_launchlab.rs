//! CPI module for Raydium Launchlab
//!
//! Program: raydium_launchpad
//! Program ID: LanMV9sAd7wArD4vJFi2qDdfnVhFxYSUg6eADduJ3uj
//! Instructions: 23

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
pub const ID: Address = Address::new_from_array(five8_const::decode_32_const("LanMV9sAd7wArD4vJFi2qDdfnVhFxYSUg6eADduJ3uj"));

// ============================================
// Instruction Discriminators
// ============================================
pub const BUY_EXACT_IN: [u8; 8] = [250, 234, 13, 123, 213, 156, 19, 236];
pub const BUY_EXACT_OUT: [u8; 8] = [24, 211, 116, 40, 105, 3, 153, 56];
pub const CLAIM_CREATOR_FEE: [u8; 8] = [26, 97, 138, 203, 132, 171, 141, 252];
pub const CLAIM_PLATFORM_FEE: [u8; 8] = [156, 39, 208, 135, 76, 237, 61, 72];
pub const CLAIM_PLATFORM_FEE_FROM_VAULT: [u8; 8] = [117, 241, 198, 168, 248, 218, 80, 29];
pub const CLAIM_VESTED_TOKEN: [u8; 8] = [49, 33, 104, 30, 189, 157, 79, 35];
pub const COLLECT_FEE: [u8; 8] = [60, 173, 247, 103, 4, 93, 130, 48];
pub const COLLECT_MIGRATE_FEE: [u8; 8] = [255, 186, 150, 223, 235, 118, 201, 186];
pub const CREATE_CONFIG: [u8; 8] = [201, 207, 243, 114, 75, 111, 47, 189];
pub const CREATE_PLATFORM_CONFIG: [u8; 8] = [176, 90, 196, 175, 253, 113, 220, 20];
pub const CREATE_PLATFORM_VESTING_ACCOUNT: [u8; 8] = [146, 71, 173, 69, 98, 19, 15, 106];
pub const CREATE_VESTING_ACCOUNT: [u8; 8] = [129, 178, 2, 13, 217, 172, 230, 218];
pub const INITIALIZE: [u8; 8] = [175, 175, 109, 31, 13, 152, 155, 237];
pub const INITIALIZE_V2: [u8; 8] = [67, 153, 175, 39, 218, 16, 38, 32];
pub const INITIALIZE_WITH_TOKEN_2022: [u8; 8] = [37, 190, 126, 222, 44, 154, 171, 17];
pub const MIGRATE_TO_AMM: [u8; 8] = [207, 82, 192, 145, 254, 207, 145, 223];
pub const MIGRATE_TO_CPSWAP: [u8; 8] = [136, 92, 200, 103, 28, 218, 144, 140];
pub const REMOVE_PLATFORM_CURVE_PARAM: [u8; 8] = [27, 30, 62, 169, 93, 224, 24, 145];
pub const SELL_EXACT_IN: [u8; 8] = [149, 39, 222, 155, 211, 124, 152, 26];
pub const SELL_EXACT_OUT: [u8; 8] = [95, 200, 71, 34, 8, 9, 11, 166];
pub const UPDATE_CONFIG: [u8; 8] = [29, 158, 252, 191, 10, 83, 219, 99];
pub const UPDATE_PLATFORM_CONFIG: [u8; 8] = [195, 60, 76, 129, 146, 45, 67, 143];
pub const UPDATE_PLATFORM_CURVE_PARAM: [u8; 8] = [138, 144, 138, 250, 220, 128, 4, 57];

// ============================================
// Instruction Arguments
// ============================================
/// Arguments for `buy_exact_in`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct BuyExactInArgs {
    pub amount_in: u64,
    pub minimum_amount_out: u64,
    pub share_fee_rate: u64,
}

/// Arguments for `buy_exact_out`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct BuyExactOutArgs {
    pub amount_out: u64,
    pub maximum_amount_in: u64,
    pub share_fee_rate: u64,
}

/// Arguments for `create_config`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct CreateConfigArgs {
    pub curve_type: u8,
    pub index: u16,
    pub migrate_fee: u64,
    pub trade_fee_rate: u64,
}

/// Arguments for `create_platform_config`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct CreatePlatformConfigArgs {
    pub platform_params: [u8; 32],
}

/// Arguments for `create_vesting_account`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct CreateVestingAccountArgs {
    pub share_amount: u64,
}

/// Arguments for `initialize`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct InitializeArgs {
    pub base_mint_param: [u8; 32],
    pub curve_param: [u8; 32],
    pub vesting_param: [u8; 32],
}

/// Arguments for `initialize_v2`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct InitializeV2Args {
    pub base_mint_param: [u8; 32],
    pub curve_param: [u8; 32],
    pub vesting_param: [u8; 32],
    pub amm_fee_on: [u8; 32],
}

/// Arguments for `initialize_with_token_2022`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct InitializeWithToken2022Args {
    pub base_mint_param: [u8; 32],
    pub curve_param: [u8; 32],
    pub vesting_param: [u8; 32],
    pub amm_fee_on: [u8; 32],
    pub transfer_fee_extension_param: Option<[u8; 32]>,
}

/// Arguments for `migrate_to_amm`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct MigrateToAmmArgs {
    pub base_lot_size: u64,
    pub quote_lot_size: u64,
    pub market_vault_signer_nonce: u8,
}

/// Arguments for `remove_platform_curve_param`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct RemovePlatformCurveParamArgs {
    pub index: u8,
}

/// Arguments for `sell_exact_in`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SellExactInArgs {
    pub amount_in: u64,
    pub minimum_amount_out: u64,
    pub share_fee_rate: u64,
}

/// Arguments for `sell_exact_out`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SellExactOutArgs {
    pub amount_out: u64,
    pub maximum_amount_in: u64,
    pub share_fee_rate: u64,
}

/// Arguments for `update_config`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct UpdateConfigArgs {
    pub param: u8,
    pub value: u64,
}

/// Arguments for `update_platform_config`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct UpdatePlatformConfigArgs {
    pub param: [u8; 32],
}

/// Arguments for `update_platform_curve_param`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct UpdatePlatformCurveParamArgs {
    pub index: u8,
    pub bonding_curve_param: [u8; 32],
}


// ============================================
// Instruction Accounts
// ============================================
/// Accounts for `buy_exact_in`
pub struct BuyExactInAccounts<'a> {
    /// The user performing the swap operation
    pub payer: &'a AccountView,
    /// PDA that acts as the authority for pool vault operations
    pub authority: &'a AccountView,
    /// Global configuration account containing protocol-wide settings
    pub global_config: &'a AccountView,
    /// Platform configuration account containing platform-wide settings
    pub platform_config: &'a AccountView,
    /// The pool state account where the swap will be performed
    pub pool_state: &'a AccountView,
    /// The user's token account for base tokens (tokens being bought)
    pub user_base_token: &'a AccountView,
    /// The user's token account for quote tokens (tokens being sold)
    pub user_quote_token: &'a AccountView,
    /// The pool's vault for base tokens
    pub base_vault: &'a AccountView,
    /// The pool's vault for quote tokens
    pub quote_vault: &'a AccountView,
    /// The mint of the base token
    pub base_token_mint: &'a AccountView,
    /// The mint of the quote token
    pub quote_token_mint: &'a AccountView,
    /// SPL Token program for base token transfers
    pub base_token_program: &'a AccountView,
    /// SPL Token program for quote token transfers
    pub quote_token_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> BuyExactInAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 15] {
        [
            InstructionAccount::writable_signer(self.payer.address()),
            InstructionAccount::readonly(self.authority.address()),
            InstructionAccount::readonly(self.global_config.address()),
            InstructionAccount::readonly(self.platform_config.address()),
            InstructionAccount::writable(self.pool_state.address()),
            InstructionAccount::writable(self.user_base_token.address()),
            InstructionAccount::writable(self.user_quote_token.address()),
            InstructionAccount::writable(self.base_vault.address()),
            InstructionAccount::writable(self.quote_vault.address()),
            InstructionAccount::readonly(self.base_token_mint.address()),
            InstructionAccount::readonly(self.quote_token_mint.address()),
            InstructionAccount::readonly(self.base_token_program.address()),
            InstructionAccount::readonly(self.quote_token_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 15] {
        [
            self.payer, self.authority, self.global_config, self.platform_config, self.pool_state, self.user_base_token, self.user_quote_token, self.base_vault, self.quote_vault, self.base_token_mint, self.quote_token_mint, self.base_token_program, self.quote_token_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `buy_exact_out`
pub struct BuyExactOutAccounts<'a> {
    /// The user performing the swap operation
    pub payer: &'a AccountView,
    /// PDA that acts as the authority for pool vault operations
    pub authority: &'a AccountView,
    /// Global configuration account containing protocol-wide settings
    pub global_config: &'a AccountView,
    /// Platform configuration account containing platform-wide settings
    pub platform_config: &'a AccountView,
    /// The pool state account where the swap will be performed
    pub pool_state: &'a AccountView,
    /// The user's token account for base tokens (tokens being bought)
    pub user_base_token: &'a AccountView,
    /// The user's token account for quote tokens (tokens being sold)
    pub user_quote_token: &'a AccountView,
    /// The pool's vault for base tokens
    pub base_vault: &'a AccountView,
    /// The pool's vault for quote tokens
    pub quote_vault: &'a AccountView,
    /// The mint of the base token
    pub base_token_mint: &'a AccountView,
    /// The mint of the quote token
    pub quote_token_mint: &'a AccountView,
    /// SPL Token program for base token transfers
    pub base_token_program: &'a AccountView,
    /// SPL Token program for quote token transfers
    pub quote_token_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> BuyExactOutAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 15] {
        [
            InstructionAccount::writable_signer(self.payer.address()),
            InstructionAccount::readonly(self.authority.address()),
            InstructionAccount::readonly(self.global_config.address()),
            InstructionAccount::readonly(self.platform_config.address()),
            InstructionAccount::writable(self.pool_state.address()),
            InstructionAccount::writable(self.user_base_token.address()),
            InstructionAccount::writable(self.user_quote_token.address()),
            InstructionAccount::writable(self.base_vault.address()),
            InstructionAccount::writable(self.quote_vault.address()),
            InstructionAccount::readonly(self.base_token_mint.address()),
            InstructionAccount::readonly(self.quote_token_mint.address()),
            InstructionAccount::readonly(self.base_token_program.address()),
            InstructionAccount::readonly(self.quote_token_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 15] {
        [
            self.payer, self.authority, self.global_config, self.platform_config, self.pool_state, self.user_base_token, self.user_quote_token, self.base_vault, self.quote_vault, self.base_token_mint, self.quote_token_mint, self.base_token_program, self.quote_token_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `claim_creator_fee`
pub struct ClaimCreatorFeeAccounts<'a> {
    /// The pool creator
    pub creator: &'a AccountView,
    /// fee_vault_authority
    pub fee_vault_authority: &'a AccountView,
    /// The creator fee vault
    pub creator_fee_vault: &'a AccountView,
    /// recipient_token_account
    pub recipient_token_account: &'a AccountView,
    /// The mint for the quote token
    pub quote_mint: &'a AccountView,
    /// SPL Token program for the quote token
    pub token_program: &'a AccountView,
    /// Required for account creation
    pub system_program: &'a AccountView,
    /// Required for associated token program
    pub associated_token_program: &'a AccountView,
}

impl<'a> ClaimCreatorFeeAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 8] {
        [
            InstructionAccount::writable_signer(self.creator.address()),
            InstructionAccount::readonly(self.fee_vault_authority.address()),
            InstructionAccount::writable(self.creator_fee_vault.address()),
            InstructionAccount::writable(self.recipient_token_account.address()),
            InstructionAccount::readonly(self.quote_mint.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 8] {
        [
            self.creator, self.fee_vault_authority, self.creator_fee_vault, self.recipient_token_account, self.quote_mint, self.token_program, self.system_program, self.associated_token_program
        ]
    }
}

/// Accounts for `claim_platform_fee`
pub struct ClaimPlatformFeeAccounts<'a> {
    /// Only the wallet stored in platform_config can collect platform fees
    pub platform_fee_wallet: &'a AccountView,
    /// PDA that acts as the authority for pool vault and mint operations
    pub authority: &'a AccountView,
    /// Account that stores the pool's state and parameters
    pub pool_state: &'a AccountView,
    /// The platform config account
    pub platform_config: &'a AccountView,
    /// quote_vault
    pub quote_vault: &'a AccountView,
    /// The address that receives the collected quote token fees
    pub recipient_token_account: &'a AccountView,
    /// The mint of quote token vault
    pub quote_mint: &'a AccountView,
    /// SPL program for input token transfers
    pub token_program: &'a AccountView,
    /// Required for account creation
    pub system_program: &'a AccountView,
    /// Required for associated token program
    pub associated_token_program: &'a AccountView,
}

impl<'a> ClaimPlatformFeeAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 10] {
        [
            InstructionAccount::writable_signer(self.platform_fee_wallet.address()),
            InstructionAccount::readonly(self.authority.address()),
            InstructionAccount::writable(self.pool_state.address()),
            InstructionAccount::readonly(self.platform_config.address()),
            InstructionAccount::writable(self.quote_vault.address()),
            InstructionAccount::writable(self.recipient_token_account.address()),
            InstructionAccount::readonly(self.quote_mint.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 10] {
        [
            self.platform_fee_wallet, self.authority, self.pool_state, self.platform_config, self.quote_vault, self.recipient_token_account, self.quote_mint, self.token_program, self.system_program, self.associated_token_program
        ]
    }
}

/// Accounts for `claim_platform_fee_from_vault`
pub struct ClaimPlatformFeeFromVaultAccounts<'a> {
    /// Only the wallet stored in platform_config can collect platform fees
    pub platform_fee_wallet: &'a AccountView,
    /// fee_vault_authority
    pub fee_vault_authority: &'a AccountView,
    /// The platform config account
    pub platform_config: &'a AccountView,
    /// The platform fee vault
    pub platform_fee_vault: &'a AccountView,
    /// The address that receives the collected quote token fees
    pub recipient_token_account: &'a AccountView,
    /// The mint of quote token vault
    pub quote_mint: &'a AccountView,
    /// SPL program for input token transfers
    pub token_program: &'a AccountView,
    /// Required for account creation
    pub system_program: &'a AccountView,
    /// Required for associated token program
    pub associated_token_program: &'a AccountView,
}

impl<'a> ClaimPlatformFeeFromVaultAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 9] {
        [
            InstructionAccount::writable_signer(self.platform_fee_wallet.address()),
            InstructionAccount::readonly(self.fee_vault_authority.address()),
            InstructionAccount::readonly(self.platform_config.address()),
            InstructionAccount::writable(self.platform_fee_vault.address()),
            InstructionAccount::writable(self.recipient_token_account.address()),
            InstructionAccount::readonly(self.quote_mint.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 9] {
        [
            self.platform_fee_wallet, self.fee_vault_authority, self.platform_config, self.platform_fee_vault, self.recipient_token_account, self.quote_mint, self.token_program, self.system_program, self.associated_token_program
        ]
    }
}

/// Accounts for `claim_vested_token`
pub struct ClaimVestedTokenAccounts<'a> {
    /// The beneficiary of the vesting account
    pub beneficiary: &'a AccountView,
    /// PDA that acts as the authority for pool vault and mint operations
    pub authority: &'a AccountView,
    /// Account that stores the pool's state and parameters
    pub pool_state: &'a AccountView,
    /// The vesting record account
    pub vesting_record: &'a AccountView,
    /// The pool's vault for base tokens
    pub base_vault: &'a AccountView,
    /// user_base_token
    pub user_base_token: &'a AccountView,
    /// The mint for the base token (token being sold)
    pub base_token_mint: &'a AccountView,
    /// SPL Token program for the base token
    pub base_token_program: &'a AccountView,
    /// Required for account creation
    pub system_program: &'a AccountView,
    /// Required for associated token program
    pub associated_token_program: &'a AccountView,
}

impl<'a> ClaimVestedTokenAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 10] {
        [
            InstructionAccount::writable_signer(self.beneficiary.address()),
            InstructionAccount::readonly(self.authority.address()),
            InstructionAccount::writable(self.pool_state.address()),
            InstructionAccount::writable(self.vesting_record.address()),
            InstructionAccount::writable(self.base_vault.address()),
            InstructionAccount::writable(self.user_base_token.address()),
            InstructionAccount::readonly(self.base_token_mint.address()),
            InstructionAccount::readonly(self.base_token_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 10] {
        [
            self.beneficiary, self.authority, self.pool_state, self.vesting_record, self.base_vault, self.user_base_token, self.base_token_mint, self.base_token_program, self.system_program, self.associated_token_program
        ]
    }
}

/// Accounts for `collect_fee`
pub struct CollectFeeAccounts<'a> {
    /// Only protocol_fee_owner saved in global_config can collect protocol fee now
    pub owner: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
    /// Pool state stores accumulated protocol fee amount
    pub pool_state: &'a AccountView,
    /// Global config account stores owner
    pub global_config: &'a AccountView,
    /// The address that holds pool tokens for quote token
    pub quote_vault: &'a AccountView,
    /// The mint of quote token vault
    pub quote_mint: &'a AccountView,
    /// The address that receives the collected quote token fees
    pub recipient_token_account: &'a AccountView,
    /// SPL program for input token transfers
    pub token_program: &'a AccountView,
}

impl<'a> CollectFeeAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 8] {
        [
            InstructionAccount::readonly_signer(self.owner.address()),
            InstructionAccount::readonly(self.authority.address()),
            InstructionAccount::writable(self.pool_state.address()),
            InstructionAccount::readonly(self.global_config.address()),
            InstructionAccount::writable(self.quote_vault.address()),
            InstructionAccount::readonly(self.quote_mint.address()),
            InstructionAccount::writable(self.recipient_token_account.address()),
            InstructionAccount::readonly(self.token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 8] {
        [
            self.owner, self.authority, self.pool_state, self.global_config, self.quote_vault, self.quote_mint, self.recipient_token_account, self.token_program
        ]
    }
}

/// Accounts for `collect_migrate_fee`
pub struct CollectMigrateFeeAccounts<'a> {
    /// Only migrate_fee_owner saved in global_config can collect migrate fee now
    pub owner: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
    /// Pool state stores accumulated protocol fee amount
    pub pool_state: &'a AccountView,
    /// Global config account stores owner
    pub global_config: &'a AccountView,
    /// The address that holds pool tokens for quote token
    pub quote_vault: &'a AccountView,
    /// The mint of quote token vault
    pub quote_mint: &'a AccountView,
    /// The address that receives the collected quote token fees
    pub recipient_token_account: &'a AccountView,
    /// SPL program for input token transfers
    pub token_program: &'a AccountView,
}

impl<'a> CollectMigrateFeeAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 8] {
        [
            InstructionAccount::readonly_signer(self.owner.address()),
            InstructionAccount::readonly(self.authority.address()),
            InstructionAccount::writable(self.pool_state.address()),
            InstructionAccount::readonly(self.global_config.address()),
            InstructionAccount::writable(self.quote_vault.address()),
            InstructionAccount::readonly(self.quote_mint.address()),
            InstructionAccount::writable(self.recipient_token_account.address()),
            InstructionAccount::readonly(self.token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 8] {
        [
            self.owner, self.authority, self.pool_state, self.global_config, self.quote_vault, self.quote_mint, self.recipient_token_account, self.token_program
        ]
    }
}

/// Accounts for `create_config`
pub struct CreateConfigAccounts<'a> {
    /// The protocol owner/admin account
    pub owner: &'a AccountView,
    /// Global configuration account that stores protocol-wide settings
    pub global_config: &'a AccountView,
    /// The mint address of the quote token (token used for buying)
    pub quote_token_mint: &'a AccountView,
    /// Account that will receive protocol fees
    pub protocol_fee_owner: &'a AccountView,
    /// Account that will receive migrate fees
    pub migrate_fee_owner: &'a AccountView,
    /// The control wallet address for migrating to amm
    pub migrate_to_amm_wallet: &'a AccountView,
    /// The control wallet address for migrating to cpswap
    pub migrate_to_cpswap_wallet: &'a AccountView,
    /// Required for account creation
    pub system_program: &'a AccountView,
}

impl<'a> CreateConfigAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 8] {
        [
            InstructionAccount::writable_signer(self.owner.address()),
            InstructionAccount::writable(self.global_config.address()),
            InstructionAccount::readonly(self.quote_token_mint.address()),
            InstructionAccount::readonly(self.protocol_fee_owner.address()),
            InstructionAccount::readonly(self.migrate_fee_owner.address()),
            InstructionAccount::readonly(self.migrate_to_amm_wallet.address()),
            InstructionAccount::readonly(self.migrate_to_cpswap_wallet.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 8] {
        [
            self.owner, self.global_config, self.quote_token_mint, self.protocol_fee_owner, self.migrate_fee_owner, self.migrate_to_amm_wallet, self.migrate_to_cpswap_wallet, self.system_program
        ]
    }
}

/// Accounts for `create_platform_config`
pub struct CreatePlatformConfigAccounts<'a> {
    /// The account paying for the initialization costs
    pub platform_admin: &'a AccountView,
    /// platform_fee_wallet
    pub platform_fee_wallet: &'a AccountView,
    /// platform_nft_wallet
    pub platform_nft_wallet: &'a AccountView,
    /// The platform config account
    pub platform_config: &'a AccountView,
    /// cpswap_config
    pub cpswap_config: &'a AccountView,
    /// Required for account creation
    pub system_program: &'a AccountView,
    /// transfer_fee_extension_authority
    pub transfer_fee_extension_authority: &'a AccountView,
    /// platform_vesting_wallet
    pub platform_vesting_wallet: &'a AccountView,
}

impl<'a> CreatePlatformConfigAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 8] {
        [
            InstructionAccount::writable_signer(self.platform_admin.address()),
            InstructionAccount::readonly(self.platform_fee_wallet.address()),
            InstructionAccount::readonly(self.platform_nft_wallet.address()),
            InstructionAccount::writable(self.platform_config.address()),
            InstructionAccount::readonly(self.cpswap_config.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.transfer_fee_extension_authority.address()),
            InstructionAccount::readonly(self.platform_vesting_wallet.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 8] {
        [
            self.platform_admin, self.platform_fee_wallet, self.platform_nft_wallet, self.platform_config, self.cpswap_config, self.system_program, self.transfer_fee_extension_authority, self.platform_vesting_wallet
        ]
    }
}

/// Accounts for `create_platform_vesting_account`
pub struct CreatePlatformVestingAccountAccounts<'a> {
    /// The account paying for the initialization costs
    pub platform_vesting_wallet: &'a AccountView,
    /// The beneficiary is used to receive the allocated linear release of tokens.
    pub beneficiary: &'a AccountView,
    /// Platform config account to be changed
    pub platform_config: &'a AccountView,
    /// The pool state account
    pub pool_state: &'a AccountView,
    /// The vesting record account
    pub platform_vesting_record: &'a AccountView,
    /// Required for account creation
    pub system_program: &'a AccountView,
}

impl<'a> CreatePlatformVestingAccountAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 6] {
        [
            InstructionAccount::writable_signer(self.platform_vesting_wallet.address()),
            InstructionAccount::writable(self.beneficiary.address()),
            InstructionAccount::readonly(self.platform_config.address()),
            InstructionAccount::writable(self.pool_state.address()),
            InstructionAccount::writable(self.platform_vesting_record.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 6] {
        [
            self.platform_vesting_wallet, self.beneficiary, self.platform_config, self.pool_state, self.platform_vesting_record, self.system_program
        ]
    }
}

/// Accounts for `create_vesting_account`
pub struct CreateVestingAccountAccounts<'a> {
    /// The account paying for the initialization costs
    pub creator: &'a AccountView,
    /// The beneficiary is used to receive the allocated linear release of tokens.
    pub beneficiary: &'a AccountView,
    /// The pool state account
    pub pool_state: &'a AccountView,
    /// The vesting record account
    pub vesting_record: &'a AccountView,
    /// Required for account creation
    pub system_program: &'a AccountView,
}

impl<'a> CreateVestingAccountAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 5] {
        [
            InstructionAccount::writable_signer(self.creator.address()),
            InstructionAccount::writable(self.beneficiary.address()),
            InstructionAccount::writable(self.pool_state.address()),
            InstructionAccount::writable(self.vesting_record.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 5] {
        [
            self.creator, self.beneficiary, self.pool_state, self.vesting_record, self.system_program
        ]
    }
}

/// Accounts for `initialize`
pub struct InitializeAccounts<'a> {
    /// The account paying for the initialization costs
    pub payer: &'a AccountView,
    /// creator
    pub creator: &'a AccountView,
    /// Global configuration account containing protocol-wide settings
    pub global_config: &'a AccountView,
    /// Platform configuration account containing platform info
    pub platform_config: &'a AccountView,
    /// PDA that acts as the authority for pool vault and mint operations
    pub authority: &'a AccountView,
    /// Account that stores the pool's state and parameters
    pub pool_state: &'a AccountView,
    /// The mint for the base token (token being sold)
    pub base_mint: &'a AccountView,
    /// The mint for the quote token (token used to buy)
    pub quote_mint: &'a AccountView,
    /// Token account that holds the pool's base tokens
    pub base_vault: &'a AccountView,
    /// Token account that holds the pool's quote tokens
    pub quote_vault: &'a AccountView,
    /// Account to store the base token's metadata
    pub metadata_account: &'a AccountView,
    /// SPL Token program for the base token
    pub base_token_program: &'a AccountView,
    /// SPL Token program for the quote token
    pub quote_token_program: &'a AccountView,
    /// Metaplex Token Metadata program
    pub metadata_program: &'a AccountView,
    /// Required for account creation
    pub system_program: &'a AccountView,
    /// Required for rent exempt calculations
    pub rent_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> InitializeAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 18] {
        [
            InstructionAccount::writable_signer(self.payer.address()),
            InstructionAccount::readonly(self.creator.address()),
            InstructionAccount::readonly(self.global_config.address()),
            InstructionAccount::readonly(self.platform_config.address()),
            InstructionAccount::readonly(self.authority.address()),
            InstructionAccount::writable(self.pool_state.address()),
            InstructionAccount::writable_signer(self.base_mint.address()),
            InstructionAccount::readonly(self.quote_mint.address()),
            InstructionAccount::writable(self.base_vault.address()),
            InstructionAccount::writable(self.quote_vault.address()),
            InstructionAccount::writable(self.metadata_account.address()),
            InstructionAccount::readonly(self.base_token_program.address()),
            InstructionAccount::readonly(self.quote_token_program.address()),
            InstructionAccount::readonly(self.metadata_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.rent_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 18] {
        [
            self.payer, self.creator, self.global_config, self.platform_config, self.authority, self.pool_state, self.base_mint, self.quote_mint, self.base_vault, self.quote_vault, self.metadata_account, self.base_token_program, self.quote_token_program, self.metadata_program, self.system_program, self.rent_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `initialize_v2`
pub struct InitializeV2Accounts<'a> {
    /// The account paying for the initialization costs
    pub payer: &'a AccountView,
    /// creator
    pub creator: &'a AccountView,
    /// Global configuration account containing protocol-wide settings
    pub global_config: &'a AccountView,
    /// Platform configuration account containing platform info
    pub platform_config: &'a AccountView,
    /// PDA that acts as the authority for pool vault and mint operations
    pub authority: &'a AccountView,
    /// Account that stores the pool's state and parameters
    pub pool_state: &'a AccountView,
    /// The mint for the base token (token being sold)
    pub base_mint: &'a AccountView,
    /// The mint for the quote token (token used to buy)
    pub quote_mint: &'a AccountView,
    /// Token account that holds the pool's base tokens
    pub base_vault: &'a AccountView,
    /// Token account that holds the pool's quote tokens
    pub quote_vault: &'a AccountView,
    /// Account to store the base token's metadata
    pub metadata_account: &'a AccountView,
    /// SPL Token program for the base token
    pub base_token_program: &'a AccountView,
    /// SPL Token program for the quote token
    pub quote_token_program: &'a AccountView,
    /// Metaplex Token Metadata program
    pub metadata_program: &'a AccountView,
    /// Required for account creation
    pub system_program: &'a AccountView,
    /// Required for rent exempt calculations
    pub rent_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> InitializeV2Accounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 18] {
        [
            InstructionAccount::writable_signer(self.payer.address()),
            InstructionAccount::readonly(self.creator.address()),
            InstructionAccount::readonly(self.global_config.address()),
            InstructionAccount::readonly(self.platform_config.address()),
            InstructionAccount::readonly(self.authority.address()),
            InstructionAccount::writable(self.pool_state.address()),
            InstructionAccount::writable_signer(self.base_mint.address()),
            InstructionAccount::readonly(self.quote_mint.address()),
            InstructionAccount::writable(self.base_vault.address()),
            InstructionAccount::writable(self.quote_vault.address()),
            InstructionAccount::writable(self.metadata_account.address()),
            InstructionAccount::readonly(self.base_token_program.address()),
            InstructionAccount::readonly(self.quote_token_program.address()),
            InstructionAccount::readonly(self.metadata_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.rent_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 18] {
        [
            self.payer, self.creator, self.global_config, self.platform_config, self.authority, self.pool_state, self.base_mint, self.quote_mint, self.base_vault, self.quote_vault, self.metadata_account, self.base_token_program, self.quote_token_program, self.metadata_program, self.system_program, self.rent_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `initialize_with_token_2022`
pub struct InitializeWithToken2022Accounts<'a> {
    /// The account paying for the initialization costs
    pub payer: &'a AccountView,
    /// creator
    pub creator: &'a AccountView,
    /// Global configuration account containing protocol-wide settings
    pub global_config: &'a AccountView,
    /// Platform configuration account containing platform info
    pub platform_config: &'a AccountView,
    /// PDA that acts as the authority for pool vault and mint operations
    pub authority: &'a AccountView,
    /// Account that stores the pool's state and parameters
    pub pool_state: &'a AccountView,
    /// The mint for the base token (token being sold)
    pub base_mint: &'a AccountView,
    /// The mint for the quote token (token used to buy)
    pub quote_mint: &'a AccountView,
    /// Token account that holds the pool's base tokens
    pub base_vault: &'a AccountView,
    /// Token account that holds the pool's quote tokens
    pub quote_vault: &'a AccountView,
    /// SPL Token program for the base token
    pub base_token_program: &'a AccountView,
    /// SPL Token program for the quote token
    pub quote_token_program: &'a AccountView,
    /// Required for account creation
    pub system_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> InitializeWithToken2022Accounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 15] {
        [
            InstructionAccount::writable_signer(self.payer.address()),
            InstructionAccount::readonly(self.creator.address()),
            InstructionAccount::readonly(self.global_config.address()),
            InstructionAccount::readonly(self.platform_config.address()),
            InstructionAccount::readonly(self.authority.address()),
            InstructionAccount::writable(self.pool_state.address()),
            InstructionAccount::writable_signer(self.base_mint.address()),
            InstructionAccount::readonly(self.quote_mint.address()),
            InstructionAccount::writable(self.base_vault.address()),
            InstructionAccount::writable(self.quote_vault.address()),
            InstructionAccount::readonly(self.base_token_program.address()),
            InstructionAccount::readonly(self.quote_token_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 15] {
        [
            self.payer, self.creator, self.global_config, self.platform_config, self.authority, self.pool_state, self.base_mint, self.quote_mint, self.base_vault, self.quote_vault, self.base_token_program, self.quote_token_program, self.system_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `migrate_to_amm`
pub struct MigrateToAmmAccounts<'a> {
    /// Only migrate_to_amm_wallet can migrate to cpswap pool
    pub payer: &'a AccountView,
    /// The mint for the base token (token being sold)
    pub base_mint: &'a AccountView,
    /// The mint for the quote token (token used to buy)
    pub quote_mint: &'a AccountView,
    /// openbook_program
    pub openbook_program: &'a AccountView,
    /// Account created and asigned to openbook_program but not been initialized
    pub market: &'a AccountView,
    /// Account created and asigned to openbook_program but not been initialized
    pub request_queue: &'a AccountView,
    /// Account created and asigned to openbook_program but not been initialized
    pub event_queue: &'a AccountView,
    /// Account created and asigned to openbook_program but not been initialized
    pub bids: &'a AccountView,
    /// Account created and asigned to openbook_program but not been initialized
    pub asks: &'a AccountView,
    /// market_vault_signer
    pub market_vault_signer: &'a AccountView,
    /// Token account that holds the market's base tokens
    pub market_base_vault: &'a AccountView,
    /// Token account that holds the market's quote tokens
    pub market_quote_vault: &'a AccountView,
    /// amm_program
    pub amm_program: &'a AccountView,
    /// amm_pool
    pub amm_pool: &'a AccountView,
    /// amm_authority
    pub amm_authority: &'a AccountView,
    /// amm_open_orders
    pub amm_open_orders: &'a AccountView,
    /// amm_lp_mint
    pub amm_lp_mint: &'a AccountView,
    /// amm_base_vault
    pub amm_base_vault: &'a AccountView,
    /// amm_quote_vault
    pub amm_quote_vault: &'a AccountView,
    /// amm_target_orders
    pub amm_target_orders: &'a AccountView,
    /// amm_config
    pub amm_config: &'a AccountView,
    /// amm_create_fee_destination
    pub amm_create_fee_destination: &'a AccountView,
    /// PDA that acts as the authority for pool vault operations
    pub authority: &'a AccountView,
    /// Account that stores the pool's state and parameters
    pub pool_state: &'a AccountView,
    /// Global config account stores owner
    pub global_config: &'a AccountView,
    /// The pool's vault for base tokens
    pub base_vault: &'a AccountView,
    /// The pool's vault for quote tokens
    pub quote_vault: &'a AccountView,
    /// pool_lp_token
    pub pool_lp_token: &'a AccountView,
    /// SPL Token program for the base token
    pub spl_token_program: &'a AccountView,
    /// Program to create an ATA for receiving fee NFT
    pub associated_token_program: &'a AccountView,
    /// Required for account creation
    pub system_program: &'a AccountView,
    /// Required for rent exempt calculations
    pub rent_program: &'a AccountView,
}

impl<'a> MigrateToAmmAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 32] {
        [
            InstructionAccount::writable_signer(self.payer.address()),
            InstructionAccount::readonly(self.base_mint.address()),
            InstructionAccount::readonly(self.quote_mint.address()),
            InstructionAccount::readonly(self.openbook_program.address()),
            InstructionAccount::writable(self.market.address()),
            InstructionAccount::writable(self.request_queue.address()),
            InstructionAccount::writable(self.event_queue.address()),
            InstructionAccount::writable(self.bids.address()),
            InstructionAccount::writable(self.asks.address()),
            InstructionAccount::readonly(self.market_vault_signer.address()),
            InstructionAccount::writable(self.market_base_vault.address()),
            InstructionAccount::writable(self.market_quote_vault.address()),
            InstructionAccount::readonly(self.amm_program.address()),
            InstructionAccount::writable(self.amm_pool.address()),
            InstructionAccount::readonly(self.amm_authority.address()),
            InstructionAccount::writable(self.amm_open_orders.address()),
            InstructionAccount::writable(self.amm_lp_mint.address()),
            InstructionAccount::writable(self.amm_base_vault.address()),
            InstructionAccount::writable(self.amm_quote_vault.address()),
            InstructionAccount::writable(self.amm_target_orders.address()),
            InstructionAccount::readonly(self.amm_config.address()),
            InstructionAccount::writable(self.amm_create_fee_destination.address()),
            InstructionAccount::writable(self.authority.address()),
            InstructionAccount::writable(self.pool_state.address()),
            InstructionAccount::readonly(self.global_config.address()),
            InstructionAccount::writable(self.base_vault.address()),
            InstructionAccount::writable(self.quote_vault.address()),
            InstructionAccount::writable(self.pool_lp_token.address()),
            InstructionAccount::readonly(self.spl_token_program.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.rent_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 32] {
        [
            self.payer, self.base_mint, self.quote_mint, self.openbook_program, self.market, self.request_queue, self.event_queue, self.bids, self.asks, self.market_vault_signer, self.market_base_vault, self.market_quote_vault, self.amm_program, self.amm_pool, self.amm_authority, self.amm_open_orders, self.amm_lp_mint, self.amm_base_vault, self.amm_quote_vault, self.amm_target_orders, self.amm_config, self.amm_create_fee_destination, self.authority, self.pool_state, self.global_config, self.base_vault, self.quote_vault, self.pool_lp_token, self.spl_token_program, self.associated_token_program, self.system_program, self.rent_program
        ]
    }
}

/// Accounts for `migrate_to_cpswap`
pub struct MigrateToCpswapAccounts<'a> {
    /// Only migrate_to_cpswap_wallet can migrate to cpswap pool
    pub payer: &'a AccountView,
    /// The mint for the base token (token being sold)
    pub base_mint: &'a AccountView,
    /// The mint for the quote token (token used to buy)
    pub quote_mint: &'a AccountView,
    /// Platform configuration account containing platform-wide settings
    pub platform_config: &'a AccountView,
    /// cpswap_program
    pub cpswap_program: &'a AccountView,
    /// PDA account:
    pub cpswap_pool: &'a AccountView,
    /// cpswap_authority
    pub cpswap_authority: &'a AccountView,
    /// cpswap_lp_mint
    pub cpswap_lp_mint: &'a AccountView,
    /// cpswap_base_vault
    pub cpswap_base_vault: &'a AccountView,
    /// cpswap_quote_vault
    pub cpswap_quote_vault: &'a AccountView,
    /// cpswap_config
    pub cpswap_config: &'a AccountView,
    /// cpswap_create_pool_fee
    pub cpswap_create_pool_fee: &'a AccountView,
    /// cpswap_observation
    pub cpswap_observation: &'a AccountView,
    /// lock_program
    pub lock_program: &'a AccountView,
    /// lock_authority
    pub lock_authority: &'a AccountView,
    /// lock_lp_vault
    pub lock_lp_vault: &'a AccountView,
    /// PDA that acts as the authority for pool vault operations
    pub authority: &'a AccountView,
    /// Account that stores the pool's state and parameters
    pub pool_state: &'a AccountView,
    /// Global config account stores owner
    pub global_config: &'a AccountView,
    /// The pool's vault for base tokens
    pub base_vault: &'a AccountView,
    /// The pool's vault for quote tokens
    pub quote_vault: &'a AccountView,
    /// pool_lp_token
    pub pool_lp_token: &'a AccountView,
    /// SPL Token program for the base token
    pub base_token_program: &'a AccountView,
    /// SPL Token program for the quote token
    pub quote_token_program: &'a AccountView,
    /// Program to create an ATA for receiving fee NFT
    pub associated_token_program: &'a AccountView,
    /// Required for account creation
    pub system_program: &'a AccountView,
    /// Required for rent exempt calculations
    pub rent_program: &'a AccountView,
    /// Program to create NFT metadata accunt
    pub metadata_program: &'a AccountView,
}

impl<'a> MigrateToCpswapAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 28] {
        [
            InstructionAccount::writable_signer(self.payer.address()),
            InstructionAccount::writable(self.base_mint.address()),
            InstructionAccount::readonly(self.quote_mint.address()),
            InstructionAccount::readonly(self.platform_config.address()),
            InstructionAccount::readonly(self.cpswap_program.address()),
            InstructionAccount::writable(self.cpswap_pool.address()),
            InstructionAccount::readonly(self.cpswap_authority.address()),
            InstructionAccount::writable(self.cpswap_lp_mint.address()),
            InstructionAccount::writable(self.cpswap_base_vault.address()),
            InstructionAccount::writable(self.cpswap_quote_vault.address()),
            InstructionAccount::readonly(self.cpswap_config.address()),
            InstructionAccount::writable(self.cpswap_create_pool_fee.address()),
            InstructionAccount::writable(self.cpswap_observation.address()),
            InstructionAccount::readonly(self.lock_program.address()),
            InstructionAccount::readonly(self.lock_authority.address()),
            InstructionAccount::writable(self.lock_lp_vault.address()),
            InstructionAccount::writable(self.authority.address()),
            InstructionAccount::writable(self.pool_state.address()),
            InstructionAccount::readonly(self.global_config.address()),
            InstructionAccount::writable(self.base_vault.address()),
            InstructionAccount::writable(self.quote_vault.address()),
            InstructionAccount::writable(self.pool_lp_token.address()),
            InstructionAccount::readonly(self.base_token_program.address()),
            InstructionAccount::readonly(self.quote_token_program.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.rent_program.address()),
            InstructionAccount::readonly(self.metadata_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 28] {
        [
            self.payer, self.base_mint, self.quote_mint, self.platform_config, self.cpswap_program, self.cpswap_pool, self.cpswap_authority, self.cpswap_lp_mint, self.cpswap_base_vault, self.cpswap_quote_vault, self.cpswap_config, self.cpswap_create_pool_fee, self.cpswap_observation, self.lock_program, self.lock_authority, self.lock_lp_vault, self.authority, self.pool_state, self.global_config, self.base_vault, self.quote_vault, self.pool_lp_token, self.base_token_program, self.quote_token_program, self.associated_token_program, self.system_program, self.rent_program, self.metadata_program
        ]
    }
}

/// Accounts for `remove_platform_curve_param`
pub struct RemovePlatformCurveParamAccounts<'a> {
    /// The account paying for the initialization costs
    pub platform_admin: &'a AccountView,
    /// Platform config account to be changed
    pub platform_config: &'a AccountView,
}

impl<'a> RemovePlatformCurveParamAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 2] {
        [
            InstructionAccount::readonly_signer(self.platform_admin.address()),
            InstructionAccount::writable(self.platform_config.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 2] {
        [
            self.platform_admin, self.platform_config
        ]
    }
}

/// Accounts for `sell_exact_in`
pub struct SellExactInAccounts<'a> {
    /// The user performing the swap operation
    pub payer: &'a AccountView,
    /// PDA that acts as the authority for pool vault operations
    pub authority: &'a AccountView,
    /// Global configuration account containing protocol-wide settings
    pub global_config: &'a AccountView,
    /// Platform configuration account containing platform-wide settings
    pub platform_config: &'a AccountView,
    /// The pool state account where the swap will be performed
    pub pool_state: &'a AccountView,
    /// The user's token account for base tokens (tokens being bought)
    pub user_base_token: &'a AccountView,
    /// The user's token account for quote tokens (tokens being sold)
    pub user_quote_token: &'a AccountView,
    /// The pool's vault for base tokens
    pub base_vault: &'a AccountView,
    /// The pool's vault for quote tokens
    pub quote_vault: &'a AccountView,
    /// The mint of the base token
    pub base_token_mint: &'a AccountView,
    /// The mint of the quote token
    pub quote_token_mint: &'a AccountView,
    /// SPL Token program for base token transfers
    pub base_token_program: &'a AccountView,
    /// SPL Token program for quote token transfers
    pub quote_token_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> SellExactInAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 15] {
        [
            InstructionAccount::writable_signer(self.payer.address()),
            InstructionAccount::readonly(self.authority.address()),
            InstructionAccount::readonly(self.global_config.address()),
            InstructionAccount::readonly(self.platform_config.address()),
            InstructionAccount::writable(self.pool_state.address()),
            InstructionAccount::writable(self.user_base_token.address()),
            InstructionAccount::writable(self.user_quote_token.address()),
            InstructionAccount::writable(self.base_vault.address()),
            InstructionAccount::writable(self.quote_vault.address()),
            InstructionAccount::readonly(self.base_token_mint.address()),
            InstructionAccount::readonly(self.quote_token_mint.address()),
            InstructionAccount::readonly(self.base_token_program.address()),
            InstructionAccount::readonly(self.quote_token_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 15] {
        [
            self.payer, self.authority, self.global_config, self.platform_config, self.pool_state, self.user_base_token, self.user_quote_token, self.base_vault, self.quote_vault, self.base_token_mint, self.quote_token_mint, self.base_token_program, self.quote_token_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `sell_exact_out`
pub struct SellExactOutAccounts<'a> {
    /// The user performing the swap operation
    pub payer: &'a AccountView,
    /// PDA that acts as the authority for pool vault operations
    pub authority: &'a AccountView,
    /// Global configuration account containing protocol-wide settings
    pub global_config: &'a AccountView,
    /// Platform configuration account containing platform-wide settings
    pub platform_config: &'a AccountView,
    /// The pool state account where the swap will be performed
    pub pool_state: &'a AccountView,
    /// The user's token account for base tokens (tokens being bought)
    pub user_base_token: &'a AccountView,
    /// The user's token account for quote tokens (tokens being sold)
    pub user_quote_token: &'a AccountView,
    /// The pool's vault for base tokens
    pub base_vault: &'a AccountView,
    /// The pool's vault for quote tokens
    pub quote_vault: &'a AccountView,
    /// The mint of the base token
    pub base_token_mint: &'a AccountView,
    /// The mint of the quote token
    pub quote_token_mint: &'a AccountView,
    /// SPL Token program for base token transfers
    pub base_token_program: &'a AccountView,
    /// SPL Token program for quote token transfers
    pub quote_token_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> SellExactOutAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 15] {
        [
            InstructionAccount::writable_signer(self.payer.address()),
            InstructionAccount::readonly(self.authority.address()),
            InstructionAccount::readonly(self.global_config.address()),
            InstructionAccount::readonly(self.platform_config.address()),
            InstructionAccount::writable(self.pool_state.address()),
            InstructionAccount::writable(self.user_base_token.address()),
            InstructionAccount::writable(self.user_quote_token.address()),
            InstructionAccount::writable(self.base_vault.address()),
            InstructionAccount::writable(self.quote_vault.address()),
            InstructionAccount::readonly(self.base_token_mint.address()),
            InstructionAccount::readonly(self.quote_token_mint.address()),
            InstructionAccount::readonly(self.base_token_program.address()),
            InstructionAccount::readonly(self.quote_token_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 15] {
        [
            self.payer, self.authority, self.global_config, self.platform_config, self.pool_state, self.user_base_token, self.user_quote_token, self.base_vault, self.quote_vault, self.base_token_mint, self.quote_token_mint, self.base_token_program, self.quote_token_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `update_config`
pub struct UpdateConfigAccounts<'a> {
    /// The global config owner or admin
    pub owner: &'a AccountView,
    /// Global config account to be changed
    pub global_config: &'a AccountView,
}

impl<'a> UpdateConfigAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 2] {
        [
            InstructionAccount::readonly_signer(self.owner.address()),
            InstructionAccount::writable(self.global_config.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 2] {
        [
            self.owner, self.global_config
        ]
    }
}

/// Accounts for `update_platform_config`
pub struct UpdatePlatformConfigAccounts<'a> {
    /// The account paying for the initialization costs
    pub platform_admin: &'a AccountView,
    /// Platform config account to be changed
    pub platform_config: &'a AccountView,
}

impl<'a> UpdatePlatformConfigAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 2] {
        [
            InstructionAccount::readonly_signer(self.platform_admin.address()),
            InstructionAccount::writable(self.platform_config.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 2] {
        [
            self.platform_admin, self.platform_config
        ]
    }
}

/// Accounts for `update_platform_curve_param`
pub struct UpdatePlatformCurveParamAccounts<'a> {
    /// The account paying for the initialization costs
    pub platform_admin: &'a AccountView,
    /// Platform config account to be changed
    pub platform_config: &'a AccountView,
    /// Global configuration account containing protocol-wide settings
    pub global_config: &'a AccountView,
    /// System program for lamport transfers
    pub system_program: &'a AccountView,
}

impl<'a> UpdatePlatformCurveParamAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 4] {
        [
            InstructionAccount::writable_signer(self.platform_admin.address()),
            InstructionAccount::writable(self.platform_config.address()),
            InstructionAccount::readonly(self.global_config.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 4] {
        [
            self.platform_admin, self.platform_config, self.global_config, self.system_program
        ]
    }
}


// ============================================
// CPI Functions
// ============================================
/// CPI: buy_exact_in
#[inline(always)]
pub fn buy_exact_in<'a>(
    accounts: &BuyExactInAccounts<'a>, args: &BuyExactInArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<BuyExactInArgs>()];
    data[0..8].copy_from_slice(&BUY_EXACT_IN);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const BuyExactInArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<BuyExactInArgs>(),
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

/// CPI: buy_exact_out
#[inline(always)]
pub fn buy_exact_out<'a>(
    accounts: &BuyExactOutAccounts<'a>, args: &BuyExactOutArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<BuyExactOutArgs>()];
    data[0..8].copy_from_slice(&BUY_EXACT_OUT);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const BuyExactOutArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<BuyExactOutArgs>(),
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

/// CPI: claim_creator_fee
#[inline(always)]
pub fn claim_creator_fee<'a>(
    accounts: &ClaimCreatorFeeAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&CLAIM_CREATOR_FEE);
    
    
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

/// CPI: claim_platform_fee
#[inline(always)]
pub fn claim_platform_fee<'a>(
    accounts: &ClaimPlatformFeeAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&CLAIM_PLATFORM_FEE);
    
    
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

/// CPI: claim_platform_fee_from_vault
#[inline(always)]
pub fn claim_platform_fee_from_vault<'a>(
    accounts: &ClaimPlatformFeeFromVaultAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&CLAIM_PLATFORM_FEE_FROM_VAULT);
    
    
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

/// CPI: claim_vested_token
#[inline(always)]
pub fn claim_vested_token<'a>(
    accounts: &ClaimVestedTokenAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&CLAIM_VESTED_TOKEN);
    
    
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

/// CPI: collect_fee
#[inline(always)]
pub fn collect_fee<'a>(
    accounts: &CollectFeeAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&COLLECT_FEE);
    
    
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

/// CPI: collect_migrate_fee
#[inline(always)]
pub fn collect_migrate_fee<'a>(
    accounts: &CollectMigrateFeeAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&COLLECT_MIGRATE_FEE);
    
    
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
    invoke_signed::<8>(&instruction, &account_views, signers)
}

/// CPI: create_platform_config
#[inline(always)]
pub fn create_platform_config<'a>(
    accounts: &CreatePlatformConfigAccounts<'a>, args: &CreatePlatformConfigArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<CreatePlatformConfigArgs>()];
    data[0..8].copy_from_slice(&CREATE_PLATFORM_CONFIG);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const CreatePlatformConfigArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<CreatePlatformConfigArgs>(),
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

/// CPI: create_platform_vesting_account
#[inline(always)]
pub fn create_platform_vesting_account<'a>(
    accounts: &CreatePlatformVestingAccountAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&CREATE_PLATFORM_VESTING_ACCOUNT);
    
    
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

/// CPI: create_vesting_account
#[inline(always)]
pub fn create_vesting_account<'a>(
    accounts: &CreateVestingAccountAccounts<'a>, args: &CreateVestingAccountArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<CreateVestingAccountArgs>()];
    data[0..8].copy_from_slice(&CREATE_VESTING_ACCOUNT);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const CreateVestingAccountArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<CreateVestingAccountArgs>(),
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
    invoke_signed::<18>(&instruction, &account_views, signers)
}

/// CPI: initialize_v2
#[inline(always)]
pub fn initialize_v2<'a>(
    accounts: &InitializeV2Accounts<'a>, args: &InitializeV2Args,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<InitializeV2Args>()];
    data[0..8].copy_from_slice(&INITIALIZE_V2);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const InitializeV2Args as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<InitializeV2Args>(),
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
    invoke_signed::<18>(&instruction, &account_views, signers)
}

/// CPI: initialize_with_token_2022
#[inline(always)]
pub fn initialize_with_token_2022<'a>(
    accounts: &InitializeWithToken2022Accounts<'a>, args: &InitializeWithToken2022Args,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<InitializeWithToken2022Args>()];
    data[0..8].copy_from_slice(&INITIALIZE_WITH_TOKEN_2022);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const InitializeWithToken2022Args as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<InitializeWithToken2022Args>(),
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

/// CPI: migrate_to_amm
#[inline(always)]
pub fn migrate_to_amm<'a>(
    accounts: &MigrateToAmmAccounts<'a>, args: &MigrateToAmmArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<MigrateToAmmArgs>()];
    data[0..8].copy_from_slice(&MIGRATE_TO_AMM);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const MigrateToAmmArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<MigrateToAmmArgs>(),
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
    invoke_signed::<32>(&instruction, &account_views, signers)
}

/// CPI: migrate_to_cpswap
#[inline(always)]
pub fn migrate_to_cpswap<'a>(
    accounts: &MigrateToCpswapAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&MIGRATE_TO_CPSWAP);
    
    
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
    invoke_signed::<28>(&instruction, &account_views, signers)
}

/// CPI: remove_platform_curve_param
#[inline(always)]
pub fn remove_platform_curve_param<'a>(
    accounts: &RemovePlatformCurveParamAccounts<'a>, args: &RemovePlatformCurveParamArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<RemovePlatformCurveParamArgs>()];
    data[0..8].copy_from_slice(&REMOVE_PLATFORM_CURVE_PARAM);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const RemovePlatformCurveParamArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<RemovePlatformCurveParamArgs>(),
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

/// CPI: sell_exact_in
#[inline(always)]
pub fn sell_exact_in<'a>(
    accounts: &SellExactInAccounts<'a>, args: &SellExactInArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SellExactInArgs>()];
    data[0..8].copy_from_slice(&SELL_EXACT_IN);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SellExactInArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SellExactInArgs>(),
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

/// CPI: sell_exact_out
#[inline(always)]
pub fn sell_exact_out<'a>(
    accounts: &SellExactOutAccounts<'a>, args: &SellExactOutArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SellExactOutArgs>()];
    data[0..8].copy_from_slice(&SELL_EXACT_OUT);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SellExactOutArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SellExactOutArgs>(),
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

/// CPI: update_config
#[inline(always)]
pub fn update_config<'a>(
    accounts: &UpdateConfigAccounts<'a>, args: &UpdateConfigArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<UpdateConfigArgs>()];
    data[0..8].copy_from_slice(&UPDATE_CONFIG);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const UpdateConfigArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<UpdateConfigArgs>(),
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

/// CPI: update_platform_config
#[inline(always)]
pub fn update_platform_config<'a>(
    accounts: &UpdatePlatformConfigAccounts<'a>, args: &UpdatePlatformConfigArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<UpdatePlatformConfigArgs>()];
    data[0..8].copy_from_slice(&UPDATE_PLATFORM_CONFIG);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const UpdatePlatformConfigArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<UpdatePlatformConfigArgs>(),
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

/// CPI: update_platform_curve_param
#[inline(always)]
pub fn update_platform_curve_param<'a>(
    accounts: &UpdatePlatformCurveParamAccounts<'a>, args: &UpdatePlatformCurveParamArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<UpdatePlatformCurveParamArgs>()];
    data[0..8].copy_from_slice(&UPDATE_PLATFORM_CURVE_PARAM);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const UpdatePlatformCurveParamArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<UpdatePlatformCurveParamArgs>(),
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

