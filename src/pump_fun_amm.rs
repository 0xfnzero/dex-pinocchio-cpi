//! CPI module for Pump fun Amm
//!
//! Program: pump_amm
//! Program ID: pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA
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
pub const ID: Address = Address::new_from_array(five8_const::decode_32_const("pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA"));

// ============================================
// Instruction Discriminators
// ============================================
pub const ADMIN_SET_COIN_CREATOR: [u8; 8] = [242, 40, 117, 145, 73, 96, 105, 104];
pub const ADMIN_UPDATE_TOKEN_INCENTIVES: [u8; 8] = [209, 11, 115, 87, 213, 23, 124, 204];
pub const BUY: [u8; 8] = [102, 6, 61, 18, 1, 218, 235, 234];
pub const BUY_EXACT_QUOTE_IN: [u8; 8] = [198, 46, 21, 82, 180, 217, 232, 112];
pub const CLAIM_TOKEN_INCENTIVES: [u8; 8] = [16, 4, 71, 28, 204, 1, 40, 27];
pub const CLOSE_USER_VOLUME_ACCUMULATOR: [u8; 8] = [249, 69, 164, 218, 150, 103, 84, 138];
pub const COLLECT_COIN_CREATOR_FEE: [u8; 8] = [160, 57, 89, 42, 181, 139, 43, 66];
pub const CREATE_CONFIG: [u8; 8] = [201, 207, 243, 114, 75, 111, 47, 189];
pub const CREATE_POOL: [u8; 8] = [233, 146, 209, 142, 207, 104, 64, 188];
pub const DEPOSIT: [u8; 8] = [242, 35, 198, 137, 82, 225, 242, 182];
pub const DISABLE: [u8; 8] = [185, 173, 187, 90, 216, 15, 238, 233];
pub const EXTEND_ACCOUNT: [u8; 8] = [234, 102, 194, 203, 150, 72, 62, 229];
pub const INIT_USER_VOLUME_ACCUMULATOR: [u8; 8] = [94, 6, 202, 115, 255, 96, 232, 183];
pub const MIGRATE_POOL_COIN_CREATOR: [u8; 8] = [208, 8, 159, 4, 74, 175, 16, 58];
pub const SELL: [u8; 8] = [51, 230, 133, 164, 1, 127, 131, 173];
pub const SET_COIN_CREATOR: [u8; 8] = [210, 149, 128, 45, 188, 58, 78, 175];
pub const SET_RESERVED_FEE_RECIPIENTS: [u8; 8] = [111, 172, 162, 232, 114, 89, 213, 142];
pub const SYNC_USER_VOLUME_ACCUMULATOR: [u8; 8] = [86, 31, 192, 87, 163, 87, 79, 238];
pub const TOGGLE_MAYHEM_MODE: [u8; 8] = [1, 9, 111, 208, 100, 31, 255, 163];
pub const TRANSFER_CREATOR_FEES_TO_PUMP: [u8; 8] = [139, 52, 134, 85, 228, 229, 108, 241];
pub const UPDATE_ADMIN: [u8; 8] = [161, 176, 40, 213, 60, 184, 179, 228];
pub const UPDATE_FEE_CONFIG: [u8; 8] = [104, 184, 103, 242, 88, 151, 107, 20];
pub const WITHDRAW: [u8; 8] = [183, 18, 70, 156, 148, 109, 161, 34];

// ============================================
// Instruction Arguments
// ============================================
/// Arguments for `admin_set_coin_creator`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct AdminSetCoinCreatorArgs {
    pub coin_creator: [u8; 32],
}

/// Arguments for `admin_update_token_incentives`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct AdminUpdateTokenIncentivesArgs {
    pub start_time: i64,
    pub end_time: i64,
    pub seconds_in_a_day: i64,
    pub day_number: u64,
    pub token_supply_per_day: u64,
}

/// Arguments for `buy`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct BuyArgs {
    pub base_amount_out: u64,
    pub max_quote_amount_in: u64,
    pub track_volume: [u8; 32],
}

/// Arguments for `buy_exact_quote_in`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct BuyExactQuoteInArgs {
    pub spendable_quote_in: u64,
    pub min_base_amount_out: u64,
    pub track_volume: [u8; 32],
}

/// Arguments for `create_config`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct CreateConfigArgs {
    pub lp_fee_basis_points: u64,
    pub protocol_fee_basis_points: u64,
    pub protocol_fee_recipients: [[u8; 32]; 8],
    pub coin_creator_fee_basis_points: u64,
    pub admin_set_coin_creator_authority: [u8; 32],
}

/// Arguments for `create_pool`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct CreatePoolArgs {
    pub index: u16,
    pub base_amount_in: u64,
    pub quote_amount_in: u64,
    pub coin_creator: [u8; 32],
    pub is_mayhem_mode: bool,
}

/// Arguments for `deposit`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct DepositArgs {
    pub lp_token_amount_out: u64,
    pub max_base_amount_in: u64,
    pub max_quote_amount_in: u64,
}

/// Arguments for `disable`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct DisableArgs {
    pub disable_create_pool: bool,
    pub disable_deposit: bool,
    pub disable_withdraw: bool,
    pub disable_buy: bool,
    pub disable_sell: bool,
}

/// Arguments for `sell`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SellArgs {
    pub base_amount_in: u64,
    pub min_quote_amount_out: u64,
}

/// Arguments for `set_reserved_fee_recipients`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SetReservedFeeRecipientsArgs {
    pub whitelist_pda: [u8; 32],
}

/// Arguments for `toggle_mayhem_mode`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct ToggleMayhemModeArgs {
    pub enabled: bool,
}

/// Arguments for `update_fee_config`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct UpdateFeeConfigArgs {
    pub lp_fee_basis_points: u64,
    pub protocol_fee_basis_points: u64,
    pub protocol_fee_recipients: [[u8; 32]; 8],
    pub coin_creator_fee_basis_points: u64,
    pub admin_set_coin_creator_authority: [u8; 32],
}

/// Arguments for `withdraw`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct WithdrawArgs {
    pub lp_token_amount_in: u64,
    pub min_base_amount_out: u64,
    pub min_quote_amount_out: u64,
}


// ============================================
// Instruction Accounts
// ============================================
/// Accounts for `admin_set_coin_creator`
pub struct AdminSetCoinCreatorAccounts<'a> {
    /// admin_set_coin_creator_authority
    pub admin_set_coin_creator_authority: &'a AccountView,
    /// global_config
    pub global_config: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> AdminSetCoinCreatorAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 5] {
        [
            InstructionAccount::readonly_signer(self.admin_set_coin_creator_authority.address()),
            InstructionAccount::readonly(self.global_config.address()),
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 5] {
        [
            self.admin_set_coin_creator_authority, self.global_config, self.pool, self.event_authority, self.program
        ]
    }
}

/// Accounts for `admin_update_token_incentives`
pub struct AdminUpdateTokenIncentivesAccounts<'a> {
    /// admin
    pub admin: &'a AccountView,
    /// global_config
    pub global_config: &'a AccountView,
    /// global_volume_accumulator
    pub global_volume_accumulator: &'a AccountView,
    /// mint
    pub mint: &'a AccountView,
    /// global_incentive_token_account
    pub global_incentive_token_account: &'a AccountView,
    /// associated_token_program
    pub associated_token_program: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
    /// token_program
    pub token_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> AdminUpdateTokenIncentivesAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 10] {
        [
            InstructionAccount::writable_signer(self.admin.address()),
            InstructionAccount::readonly(self.global_config.address()),
            InstructionAccount::writable(self.global_volume_accumulator.address()),
            InstructionAccount::readonly(self.mint.address()),
            InstructionAccount::writable(self.global_incentive_token_account.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 10] {
        [
            self.admin, self.global_config, self.global_volume_accumulator, self.mint, self.global_incentive_token_account, self.associated_token_program, self.system_program, self.token_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `buy`
pub struct BuyAccounts<'a> {
    /// pool
    pub pool: &'a AccountView,
    /// user
    pub user: &'a AccountView,
    /// global_config
    pub global_config: &'a AccountView,
    /// base_mint
    pub base_mint: &'a AccountView,
    /// quote_mint
    pub quote_mint: &'a AccountView,
    /// user_base_token_account
    pub user_base_token_account: &'a AccountView,
    /// user_quote_token_account
    pub user_quote_token_account: &'a AccountView,
    /// pool_base_token_account
    pub pool_base_token_account: &'a AccountView,
    /// pool_quote_token_account
    pub pool_quote_token_account: &'a AccountView,
    /// protocol_fee_recipient
    pub protocol_fee_recipient: &'a AccountView,
    /// protocol_fee_recipient_token_account
    pub protocol_fee_recipient_token_account: &'a AccountView,
    /// base_token_program
    pub base_token_program: &'a AccountView,
    /// quote_token_program
    pub quote_token_program: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
    /// associated_token_program
    pub associated_token_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
    /// coin_creator_vault_ata
    pub coin_creator_vault_ata: &'a AccountView,
    /// coin_creator_vault_authority
    pub coin_creator_vault_authority: &'a AccountView,
    /// global_volume_accumulator
    pub global_volume_accumulator: &'a AccountView,
    /// user_volume_accumulator
    pub user_volume_accumulator: &'a AccountView,
    /// fee_config
    pub fee_config: &'a AccountView,
    /// fee_program
    pub fee_program: &'a AccountView,
}

impl<'a> BuyAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 23] {
        [
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable_signer(self.user.address()),
            InstructionAccount::readonly(self.global_config.address()),
            InstructionAccount::readonly(self.base_mint.address()),
            InstructionAccount::readonly(self.quote_mint.address()),
            InstructionAccount::writable(self.user_base_token_account.address()),
            InstructionAccount::writable(self.user_quote_token_account.address()),
            InstructionAccount::writable(self.pool_base_token_account.address()),
            InstructionAccount::writable(self.pool_quote_token_account.address()),
            InstructionAccount::readonly(self.protocol_fee_recipient.address()),
            InstructionAccount::writable(self.protocol_fee_recipient_token_account.address()),
            InstructionAccount::readonly(self.base_token_program.address()),
            InstructionAccount::readonly(self.quote_token_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
            InstructionAccount::writable(self.coin_creator_vault_ata.address()),
            InstructionAccount::readonly(self.coin_creator_vault_authority.address()),
            InstructionAccount::readonly(self.global_volume_accumulator.address()),
            InstructionAccount::writable(self.user_volume_accumulator.address()),
            InstructionAccount::readonly(self.fee_config.address()),
            InstructionAccount::readonly(self.fee_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 23] {
        [
            self.pool, self.user, self.global_config, self.base_mint, self.quote_mint, self.user_base_token_account, self.user_quote_token_account, self.pool_base_token_account, self.pool_quote_token_account, self.protocol_fee_recipient, self.protocol_fee_recipient_token_account, self.base_token_program, self.quote_token_program, self.system_program, self.associated_token_program, self.event_authority, self.program, self.coin_creator_vault_ata, self.coin_creator_vault_authority, self.global_volume_accumulator, self.user_volume_accumulator, self.fee_config, self.fee_program
        ]
    }
}

/// Accounts for `buy_exact_quote_in`
pub struct BuyExactQuoteInAccounts<'a> {
    /// pool
    pub pool: &'a AccountView,
    /// user
    pub user: &'a AccountView,
    /// global_config
    pub global_config: &'a AccountView,
    /// base_mint
    pub base_mint: &'a AccountView,
    /// quote_mint
    pub quote_mint: &'a AccountView,
    /// user_base_token_account
    pub user_base_token_account: &'a AccountView,
    /// user_quote_token_account
    pub user_quote_token_account: &'a AccountView,
    /// pool_base_token_account
    pub pool_base_token_account: &'a AccountView,
    /// pool_quote_token_account
    pub pool_quote_token_account: &'a AccountView,
    /// protocol_fee_recipient
    pub protocol_fee_recipient: &'a AccountView,
    /// protocol_fee_recipient_token_account
    pub protocol_fee_recipient_token_account: &'a AccountView,
    /// base_token_program
    pub base_token_program: &'a AccountView,
    /// quote_token_program
    pub quote_token_program: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
    /// associated_token_program
    pub associated_token_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
    /// coin_creator_vault_ata
    pub coin_creator_vault_ata: &'a AccountView,
    /// coin_creator_vault_authority
    pub coin_creator_vault_authority: &'a AccountView,
    /// global_volume_accumulator
    pub global_volume_accumulator: &'a AccountView,
    /// user_volume_accumulator
    pub user_volume_accumulator: &'a AccountView,
    /// fee_config
    pub fee_config: &'a AccountView,
    /// fee_program
    pub fee_program: &'a AccountView,
}

impl<'a> BuyExactQuoteInAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 23] {
        [
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable_signer(self.user.address()),
            InstructionAccount::readonly(self.global_config.address()),
            InstructionAccount::readonly(self.base_mint.address()),
            InstructionAccount::readonly(self.quote_mint.address()),
            InstructionAccount::writable(self.user_base_token_account.address()),
            InstructionAccount::writable(self.user_quote_token_account.address()),
            InstructionAccount::writable(self.pool_base_token_account.address()),
            InstructionAccount::writable(self.pool_quote_token_account.address()),
            InstructionAccount::readonly(self.protocol_fee_recipient.address()),
            InstructionAccount::writable(self.protocol_fee_recipient_token_account.address()),
            InstructionAccount::readonly(self.base_token_program.address()),
            InstructionAccount::readonly(self.quote_token_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
            InstructionAccount::writable(self.coin_creator_vault_ata.address()),
            InstructionAccount::readonly(self.coin_creator_vault_authority.address()),
            InstructionAccount::readonly(self.global_volume_accumulator.address()),
            InstructionAccount::writable(self.user_volume_accumulator.address()),
            InstructionAccount::readonly(self.fee_config.address()),
            InstructionAccount::readonly(self.fee_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 23] {
        [
            self.pool, self.user, self.global_config, self.base_mint, self.quote_mint, self.user_base_token_account, self.user_quote_token_account, self.pool_base_token_account, self.pool_quote_token_account, self.protocol_fee_recipient, self.protocol_fee_recipient_token_account, self.base_token_program, self.quote_token_program, self.system_program, self.associated_token_program, self.event_authority, self.program, self.coin_creator_vault_ata, self.coin_creator_vault_authority, self.global_volume_accumulator, self.user_volume_accumulator, self.fee_config, self.fee_program
        ]
    }
}

/// Accounts for `claim_token_incentives`
pub struct ClaimTokenIncentivesAccounts<'a> {
    /// user
    pub user: &'a AccountView,
    /// user_ata
    pub user_ata: &'a AccountView,
    /// global_volume_accumulator
    pub global_volume_accumulator: &'a AccountView,
    /// global_incentive_token_account
    pub global_incentive_token_account: &'a AccountView,
    /// user_volume_accumulator
    pub user_volume_accumulator: &'a AccountView,
    /// mint
    pub mint: &'a AccountView,
    /// token_program
    pub token_program: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
    /// associated_token_program
    pub associated_token_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
    /// payer
    pub payer: &'a AccountView,
}

impl<'a> ClaimTokenIncentivesAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 12] {
        [
            InstructionAccount::readonly(self.user.address()),
            InstructionAccount::writable(self.user_ata.address()),
            InstructionAccount::readonly(self.global_volume_accumulator.address()),
            InstructionAccount::writable(self.global_incentive_token_account.address()),
            InstructionAccount::writable(self.user_volume_accumulator.address()),
            InstructionAccount::readonly(self.mint.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
            InstructionAccount::writable_signer(self.payer.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 12] {
        [
            self.user, self.user_ata, self.global_volume_accumulator, self.global_incentive_token_account, self.user_volume_accumulator, self.mint, self.token_program, self.system_program, self.associated_token_program, self.event_authority, self.program, self.payer
        ]
    }
}

/// Accounts for `close_user_volume_accumulator`
pub struct CloseUserVolumeAccumulatorAccounts<'a> {
    /// user
    pub user: &'a AccountView,
    /// user_volume_accumulator
    pub user_volume_accumulator: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> CloseUserVolumeAccumulatorAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 4] {
        [
            InstructionAccount::writable_signer(self.user.address()),
            InstructionAccount::writable(self.user_volume_accumulator.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 4] {
        [
            self.user, self.user_volume_accumulator, self.event_authority, self.program
        ]
    }
}

/// Accounts for `collect_coin_creator_fee`
pub struct CollectCoinCreatorFeeAccounts<'a> {
    /// quote_mint
    pub quote_mint: &'a AccountView,
    /// quote_token_program
    pub quote_token_program: &'a AccountView,
    /// coin_creator
    pub coin_creator: &'a AccountView,
    /// coin_creator_vault_authority
    pub coin_creator_vault_authority: &'a AccountView,
    /// coin_creator_vault_ata
    pub coin_creator_vault_ata: &'a AccountView,
    /// coin_creator_token_account
    pub coin_creator_token_account: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> CollectCoinCreatorFeeAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 8] {
        [
            InstructionAccount::readonly(self.quote_mint.address()),
            InstructionAccount::readonly(self.quote_token_program.address()),
            InstructionAccount::readonly(self.coin_creator.address()),
            InstructionAccount::readonly(self.coin_creator_vault_authority.address()),
            InstructionAccount::writable(self.coin_creator_vault_ata.address()),
            InstructionAccount::writable(self.coin_creator_token_account.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 8] {
        [
            self.quote_mint, self.quote_token_program, self.coin_creator, self.coin_creator_vault_authority, self.coin_creator_vault_ata, self.coin_creator_token_account, self.event_authority, self.program
        ]
    }
}

/// Accounts for `create_config`
pub struct CreateConfigAccounts<'a> {
    /// admin
    pub admin: &'a AccountView,
    /// global_config
    pub global_config: &'a AccountView,
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
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 5] {
        [
            InstructionAccount::writable_signer(self.admin.address()),
            InstructionAccount::writable(self.global_config.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 5] {
        [
            self.admin, self.global_config, self.system_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `create_pool`
pub struct CreatePoolAccounts<'a> {
    /// pool
    pub pool: &'a AccountView,
    /// global_config
    pub global_config: &'a AccountView,
    /// creator
    pub creator: &'a AccountView,
    /// base_mint
    pub base_mint: &'a AccountView,
    /// quote_mint
    pub quote_mint: &'a AccountView,
    /// lp_mint
    pub lp_mint: &'a AccountView,
    /// user_base_token_account
    pub user_base_token_account: &'a AccountView,
    /// user_quote_token_account
    pub user_quote_token_account: &'a AccountView,
    /// user_pool_token_account
    pub user_pool_token_account: &'a AccountView,
    /// pool_base_token_account
    pub pool_base_token_account: &'a AccountView,
    /// pool_quote_token_account
    pub pool_quote_token_account: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
    /// token_2022_program
    pub token_2022_program: &'a AccountView,
    /// base_token_program
    pub base_token_program: &'a AccountView,
    /// quote_token_program
    pub quote_token_program: &'a AccountView,
    /// associated_token_program
    pub associated_token_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> CreatePoolAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 18] {
        [
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::readonly(self.global_config.address()),
            InstructionAccount::writable_signer(self.creator.address()),
            InstructionAccount::readonly(self.base_mint.address()),
            InstructionAccount::readonly(self.quote_mint.address()),
            InstructionAccount::writable(self.lp_mint.address()),
            InstructionAccount::writable(self.user_base_token_account.address()),
            InstructionAccount::writable(self.user_quote_token_account.address()),
            InstructionAccount::writable(self.user_pool_token_account.address()),
            InstructionAccount::writable(self.pool_base_token_account.address()),
            InstructionAccount::writable(self.pool_quote_token_account.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.token_2022_program.address()),
            InstructionAccount::readonly(self.base_token_program.address()),
            InstructionAccount::readonly(self.quote_token_program.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 18] {
        [
            self.pool, self.global_config, self.creator, self.base_mint, self.quote_mint, self.lp_mint, self.user_base_token_account, self.user_quote_token_account, self.user_pool_token_account, self.pool_base_token_account, self.pool_quote_token_account, self.system_program, self.token_2022_program, self.base_token_program, self.quote_token_program, self.associated_token_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `deposit`
pub struct DepositAccounts<'a> {
    /// pool
    pub pool: &'a AccountView,
    /// global_config
    pub global_config: &'a AccountView,
    /// user
    pub user: &'a AccountView,
    /// base_mint
    pub base_mint: &'a AccountView,
    /// quote_mint
    pub quote_mint: &'a AccountView,
    /// lp_mint
    pub lp_mint: &'a AccountView,
    /// user_base_token_account
    pub user_base_token_account: &'a AccountView,
    /// user_quote_token_account
    pub user_quote_token_account: &'a AccountView,
    /// user_pool_token_account
    pub user_pool_token_account: &'a AccountView,
    /// pool_base_token_account
    pub pool_base_token_account: &'a AccountView,
    /// pool_quote_token_account
    pub pool_quote_token_account: &'a AccountView,
    /// token_program
    pub token_program: &'a AccountView,
    /// token_2022_program
    pub token_2022_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> DepositAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 15] {
        [
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::readonly(self.global_config.address()),
            InstructionAccount::readonly_signer(self.user.address()),
            InstructionAccount::readonly(self.base_mint.address()),
            InstructionAccount::readonly(self.quote_mint.address()),
            InstructionAccount::writable(self.lp_mint.address()),
            InstructionAccount::writable(self.user_base_token_account.address()),
            InstructionAccount::writable(self.user_quote_token_account.address()),
            InstructionAccount::writable(self.user_pool_token_account.address()),
            InstructionAccount::writable(self.pool_base_token_account.address()),
            InstructionAccount::writable(self.pool_quote_token_account.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.token_2022_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 15] {
        [
            self.pool, self.global_config, self.user, self.base_mint, self.quote_mint, self.lp_mint, self.user_base_token_account, self.user_quote_token_account, self.user_pool_token_account, self.pool_base_token_account, self.pool_quote_token_account, self.token_program, self.token_2022_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `disable`
pub struct DisableAccounts<'a> {
    /// admin
    pub admin: &'a AccountView,
    /// global_config
    pub global_config: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> DisableAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 4] {
        [
            InstructionAccount::readonly_signer(self.admin.address()),
            InstructionAccount::writable(self.global_config.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 4] {
        [
            self.admin, self.global_config, self.event_authority, self.program
        ]
    }
}

/// Accounts for `extend_account`
pub struct ExtendAccountAccounts<'a> {
    /// account
    pub account: &'a AccountView,
    /// user
    pub user: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> ExtendAccountAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 5] {
        [
            InstructionAccount::writable(self.account.address()),
            InstructionAccount::readonly_signer(self.user.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 5] {
        [
            self.account, self.user, self.system_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `init_user_volume_accumulator`
pub struct InitUserVolumeAccumulatorAccounts<'a> {
    /// payer
    pub payer: &'a AccountView,
    /// user
    pub user: &'a AccountView,
    /// user_volume_accumulator
    pub user_volume_accumulator: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> InitUserVolumeAccumulatorAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 6] {
        [
            InstructionAccount::writable_signer(self.payer.address()),
            InstructionAccount::readonly(self.user.address()),
            InstructionAccount::writable(self.user_volume_accumulator.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 6] {
        [
            self.payer, self.user, self.user_volume_accumulator, self.system_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `migrate_pool_coin_creator`
pub struct MigratePoolCoinCreatorAccounts<'a> {
    /// pool
    pub pool: &'a AccountView,
    /// sharing_config
    pub sharing_config: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> MigratePoolCoinCreatorAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 4] {
        [
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::readonly(self.sharing_config.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 4] {
        [
            self.pool, self.sharing_config, self.event_authority, self.program
        ]
    }
}

/// Accounts for `sell`
pub struct SellAccounts<'a> {
    /// pool
    pub pool: &'a AccountView,
    /// user
    pub user: &'a AccountView,
    /// global_config
    pub global_config: &'a AccountView,
    /// base_mint
    pub base_mint: &'a AccountView,
    /// quote_mint
    pub quote_mint: &'a AccountView,
    /// user_base_token_account
    pub user_base_token_account: &'a AccountView,
    /// user_quote_token_account
    pub user_quote_token_account: &'a AccountView,
    /// pool_base_token_account
    pub pool_base_token_account: &'a AccountView,
    /// pool_quote_token_account
    pub pool_quote_token_account: &'a AccountView,
    /// protocol_fee_recipient
    pub protocol_fee_recipient: &'a AccountView,
    /// protocol_fee_recipient_token_account
    pub protocol_fee_recipient_token_account: &'a AccountView,
    /// base_token_program
    pub base_token_program: &'a AccountView,
    /// quote_token_program
    pub quote_token_program: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
    /// associated_token_program
    pub associated_token_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
    /// coin_creator_vault_ata
    pub coin_creator_vault_ata: &'a AccountView,
    /// coin_creator_vault_authority
    pub coin_creator_vault_authority: &'a AccountView,
    /// fee_config
    pub fee_config: &'a AccountView,
    /// fee_program
    pub fee_program: &'a AccountView,
}

impl<'a> SellAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 21] {
        [
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable_signer(self.user.address()),
            InstructionAccount::readonly(self.global_config.address()),
            InstructionAccount::readonly(self.base_mint.address()),
            InstructionAccount::readonly(self.quote_mint.address()),
            InstructionAccount::writable(self.user_base_token_account.address()),
            InstructionAccount::writable(self.user_quote_token_account.address()),
            InstructionAccount::writable(self.pool_base_token_account.address()),
            InstructionAccount::writable(self.pool_quote_token_account.address()),
            InstructionAccount::readonly(self.protocol_fee_recipient.address()),
            InstructionAccount::writable(self.protocol_fee_recipient_token_account.address()),
            InstructionAccount::readonly(self.base_token_program.address()),
            InstructionAccount::readonly(self.quote_token_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
            InstructionAccount::writable(self.coin_creator_vault_ata.address()),
            InstructionAccount::readonly(self.coin_creator_vault_authority.address()),
            InstructionAccount::readonly(self.fee_config.address()),
            InstructionAccount::readonly(self.fee_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 21] {
        [
            self.pool, self.user, self.global_config, self.base_mint, self.quote_mint, self.user_base_token_account, self.user_quote_token_account, self.pool_base_token_account, self.pool_quote_token_account, self.protocol_fee_recipient, self.protocol_fee_recipient_token_account, self.base_token_program, self.quote_token_program, self.system_program, self.associated_token_program, self.event_authority, self.program, self.coin_creator_vault_ata, self.coin_creator_vault_authority, self.fee_config, self.fee_program
        ]
    }
}

/// Accounts for `set_coin_creator`
pub struct SetCoinCreatorAccounts<'a> {
    /// pool
    pub pool: &'a AccountView,
    /// metadata
    pub metadata: &'a AccountView,
    /// bonding_curve
    pub bonding_curve: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> SetCoinCreatorAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 5] {
        [
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::readonly(self.metadata.address()),
            InstructionAccount::readonly(self.bonding_curve.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 5] {
        [
            self.pool, self.metadata, self.bonding_curve, self.event_authority, self.program
        ]
    }
}

/// Accounts for `set_reserved_fee_recipients`
pub struct SetReservedFeeRecipientsAccounts<'a> {
    /// global_config
    pub global_config: &'a AccountView,
    /// admin
    pub admin: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> SetReservedFeeRecipientsAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 4] {
        [
            InstructionAccount::writable(self.global_config.address()),
            InstructionAccount::readonly_signer(self.admin.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 4] {
        [
            self.global_config, self.admin, self.event_authority, self.program
        ]
    }
}

/// Accounts for `sync_user_volume_accumulator`
pub struct SyncUserVolumeAccumulatorAccounts<'a> {
    /// user
    pub user: &'a AccountView,
    /// global_volume_accumulator
    pub global_volume_accumulator: &'a AccountView,
    /// user_volume_accumulator
    pub user_volume_accumulator: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> SyncUserVolumeAccumulatorAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 5] {
        [
            InstructionAccount::readonly(self.user.address()),
            InstructionAccount::readonly(self.global_volume_accumulator.address()),
            InstructionAccount::writable(self.user_volume_accumulator.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 5] {
        [
            self.user, self.global_volume_accumulator, self.user_volume_accumulator, self.event_authority, self.program
        ]
    }
}

/// Accounts for `toggle_mayhem_mode`
pub struct ToggleMayhemModeAccounts<'a> {
    /// admin
    pub admin: &'a AccountView,
    /// global_config
    pub global_config: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> ToggleMayhemModeAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 4] {
        [
            InstructionAccount::readonly_signer(self.admin.address()),
            InstructionAccount::writable(self.global_config.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 4] {
        [
            self.admin, self.global_config, self.event_authority, self.program
        ]
    }
}

/// Accounts for `transfer_creator_fees_to_pump`
pub struct TransferCreatorFeesToPumpAccounts<'a> {
    /// Pump Canonical Pool are quoted in wSOL
    pub wsol_mint: &'a AccountView,
    /// token_program
    pub token_program: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
    /// associated_token_program
    pub associated_token_program: &'a AccountView,
    /// coin_creator
    pub coin_creator: &'a AccountView,
    /// coin_creator_vault_authority
    pub coin_creator_vault_authority: &'a AccountView,
    /// coin_creator_vault_ata
    pub coin_creator_vault_ata: &'a AccountView,
    /// pump_creator_vault
    pub pump_creator_vault: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> TransferCreatorFeesToPumpAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 10] {
        [
            InstructionAccount::readonly(self.wsol_mint.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
            InstructionAccount::readonly(self.coin_creator.address()),
            InstructionAccount::writable(self.coin_creator_vault_authority.address()),
            InstructionAccount::writable(self.coin_creator_vault_ata.address()),
            InstructionAccount::writable(self.pump_creator_vault.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 10] {
        [
            self.wsol_mint, self.token_program, self.system_program, self.associated_token_program, self.coin_creator, self.coin_creator_vault_authority, self.coin_creator_vault_ata, self.pump_creator_vault, self.event_authority, self.program
        ]
    }
}

/// Accounts for `update_admin`
pub struct UpdateAdminAccounts<'a> {
    /// admin
    pub admin: &'a AccountView,
    /// global_config
    pub global_config: &'a AccountView,
    /// new_admin
    pub new_admin: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> UpdateAdminAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 5] {
        [
            InstructionAccount::readonly_signer(self.admin.address()),
            InstructionAccount::writable(self.global_config.address()),
            InstructionAccount::readonly(self.new_admin.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 5] {
        [
            self.admin, self.global_config, self.new_admin, self.event_authority, self.program
        ]
    }
}

/// Accounts for `update_fee_config`
pub struct UpdateFeeConfigAccounts<'a> {
    /// admin
    pub admin: &'a AccountView,
    /// global_config
    pub global_config: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> UpdateFeeConfigAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 4] {
        [
            InstructionAccount::readonly_signer(self.admin.address()),
            InstructionAccount::writable(self.global_config.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 4] {
        [
            self.admin, self.global_config, self.event_authority, self.program
        ]
    }
}

/// Accounts for `withdraw`
pub struct WithdrawAccounts<'a> {
    /// pool
    pub pool: &'a AccountView,
    /// global_config
    pub global_config: &'a AccountView,
    /// user
    pub user: &'a AccountView,
    /// base_mint
    pub base_mint: &'a AccountView,
    /// quote_mint
    pub quote_mint: &'a AccountView,
    /// lp_mint
    pub lp_mint: &'a AccountView,
    /// user_base_token_account
    pub user_base_token_account: &'a AccountView,
    /// user_quote_token_account
    pub user_quote_token_account: &'a AccountView,
    /// user_pool_token_account
    pub user_pool_token_account: &'a AccountView,
    /// pool_base_token_account
    pub pool_base_token_account: &'a AccountView,
    /// pool_quote_token_account
    pub pool_quote_token_account: &'a AccountView,
    /// token_program
    pub token_program: &'a AccountView,
    /// token_2022_program
    pub token_2022_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> WithdrawAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 15] {
        [
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::readonly(self.global_config.address()),
            InstructionAccount::readonly_signer(self.user.address()),
            InstructionAccount::readonly(self.base_mint.address()),
            InstructionAccount::readonly(self.quote_mint.address()),
            InstructionAccount::writable(self.lp_mint.address()),
            InstructionAccount::writable(self.user_base_token_account.address()),
            InstructionAccount::writable(self.user_quote_token_account.address()),
            InstructionAccount::writable(self.user_pool_token_account.address()),
            InstructionAccount::writable(self.pool_base_token_account.address()),
            InstructionAccount::writable(self.pool_quote_token_account.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.token_2022_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 15] {
        [
            self.pool, self.global_config, self.user, self.base_mint, self.quote_mint, self.lp_mint, self.user_base_token_account, self.user_quote_token_account, self.user_pool_token_account, self.pool_base_token_account, self.pool_quote_token_account, self.token_program, self.token_2022_program, self.event_authority, self.program
        ]
    }
}


// ============================================
// CPI Functions
// ============================================
/// CPI: admin_set_coin_creator
#[inline(always)]
pub fn admin_set_coin_creator<'a>(
    accounts: &AdminSetCoinCreatorAccounts<'a>, args: &AdminSetCoinCreatorArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<AdminSetCoinCreatorArgs>()];
    data[0..8].copy_from_slice(&ADMIN_SET_COIN_CREATOR);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const AdminSetCoinCreatorArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<AdminSetCoinCreatorArgs>(),
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

/// CPI: admin_update_token_incentives
#[inline(always)]
pub fn admin_update_token_incentives<'a>(
    accounts: &AdminUpdateTokenIncentivesAccounts<'a>, args: &AdminUpdateTokenIncentivesArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<AdminUpdateTokenIncentivesArgs>()];
    data[0..8].copy_from_slice(&ADMIN_UPDATE_TOKEN_INCENTIVES);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const AdminUpdateTokenIncentivesArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<AdminUpdateTokenIncentivesArgs>(),
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

/// CPI: buy
#[inline(always)]
pub fn buy<'a>(
    accounts: &BuyAccounts<'a>, args: &BuyArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<BuyArgs>()];
    data[0..8].copy_from_slice(&BUY);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const BuyArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<BuyArgs>(),
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
    invoke_signed::<23>(&instruction, &account_views, signers)
}

/// CPI: buy_exact_quote_in
#[inline(always)]
pub fn buy_exact_quote_in<'a>(
    accounts: &BuyExactQuoteInAccounts<'a>, args: &BuyExactQuoteInArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<BuyExactQuoteInArgs>()];
    data[0..8].copy_from_slice(&BUY_EXACT_QUOTE_IN);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const BuyExactQuoteInArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<BuyExactQuoteInArgs>(),
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
    invoke_signed::<23>(&instruction, &account_views, signers)
}

/// CPI: claim_token_incentives
#[inline(always)]
pub fn claim_token_incentives<'a>(
    accounts: &ClaimTokenIncentivesAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&CLAIM_TOKEN_INCENTIVES);
    
    
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

/// CPI: close_user_volume_accumulator
#[inline(always)]
pub fn close_user_volume_accumulator<'a>(
    accounts: &CloseUserVolumeAccumulatorAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&CLOSE_USER_VOLUME_ACCUMULATOR);
    
    
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

/// CPI: collect_coin_creator_fee
#[inline(always)]
pub fn collect_coin_creator_fee<'a>(
    accounts: &CollectCoinCreatorFeeAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&COLLECT_COIN_CREATOR_FEE);
    
    
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
    invoke_signed::<5>(&instruction, &account_views, signers)
}

/// CPI: create_pool
#[inline(always)]
pub fn create_pool<'a>(
    accounts: &CreatePoolAccounts<'a>, args: &CreatePoolArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<CreatePoolArgs>()];
    data[0..8].copy_from_slice(&CREATE_POOL);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const CreatePoolArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<CreatePoolArgs>(),
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
    invoke_signed::<15>(&instruction, &account_views, signers)
}

/// CPI: disable
#[inline(always)]
pub fn disable<'a>(
    accounts: &DisableAccounts<'a>, args: &DisableArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<DisableArgs>()];
    data[0..8].copy_from_slice(&DISABLE);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const DisableArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<DisableArgs>(),
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

/// CPI: extend_account
#[inline(always)]
pub fn extend_account<'a>(
    accounts: &ExtendAccountAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&EXTEND_ACCOUNT);
    
    
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

/// CPI: init_user_volume_accumulator
#[inline(always)]
pub fn init_user_volume_accumulator<'a>(
    accounts: &InitUserVolumeAccumulatorAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&INIT_USER_VOLUME_ACCUMULATOR);
    
    
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

/// CPI: migrate_pool_coin_creator
#[inline(always)]
pub fn migrate_pool_coin_creator<'a>(
    accounts: &MigratePoolCoinCreatorAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&MIGRATE_POOL_COIN_CREATOR);
    
    
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

/// CPI: sell
#[inline(always)]
pub fn sell<'a>(
    accounts: &SellAccounts<'a>, args: &SellArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SellArgs>()];
    data[0..8].copy_from_slice(&SELL);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SellArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SellArgs>(),
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

/// CPI: set_coin_creator
#[inline(always)]
pub fn set_coin_creator<'a>(
    accounts: &SetCoinCreatorAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&SET_COIN_CREATOR);
    
    
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

/// CPI: set_reserved_fee_recipients
#[inline(always)]
pub fn set_reserved_fee_recipients<'a>(
    accounts: &SetReservedFeeRecipientsAccounts<'a>, args: &SetReservedFeeRecipientsArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SetReservedFeeRecipientsArgs>()];
    data[0..8].copy_from_slice(&SET_RESERVED_FEE_RECIPIENTS);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SetReservedFeeRecipientsArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SetReservedFeeRecipientsArgs>(),
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

/// CPI: sync_user_volume_accumulator
#[inline(always)]
pub fn sync_user_volume_accumulator<'a>(
    accounts: &SyncUserVolumeAccumulatorAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&SYNC_USER_VOLUME_ACCUMULATOR);
    
    
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

/// CPI: toggle_mayhem_mode
#[inline(always)]
pub fn toggle_mayhem_mode<'a>(
    accounts: &ToggleMayhemModeAccounts<'a>, args: &ToggleMayhemModeArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<ToggleMayhemModeArgs>()];
    data[0..8].copy_from_slice(&TOGGLE_MAYHEM_MODE);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const ToggleMayhemModeArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<ToggleMayhemModeArgs>(),
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

/// CPI: transfer_creator_fees_to_pump
#[inline(always)]
pub fn transfer_creator_fees_to_pump<'a>(
    accounts: &TransferCreatorFeesToPumpAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&TRANSFER_CREATOR_FEES_TO_PUMP);
    
    
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

/// CPI: update_admin
#[inline(always)]
pub fn update_admin<'a>(
    accounts: &UpdateAdminAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&UPDATE_ADMIN);
    
    
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

/// CPI: update_fee_config
#[inline(always)]
pub fn update_fee_config<'a>(
    accounts: &UpdateFeeConfigAccounts<'a>, args: &UpdateFeeConfigArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<UpdateFeeConfigArgs>()];
    data[0..8].copy_from_slice(&UPDATE_FEE_CONFIG);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const UpdateFeeConfigArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<UpdateFeeConfigArgs>(),
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
    invoke_signed::<15>(&instruction, &account_views, signers)
}

