//! CPI module for Pump fun
//!
//! Program: pump
//! Program ID: 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P
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
pub const ID: Address = Address::new_from_array(five8_const::decode_32_const("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P"));

// ============================================
// Instruction Discriminators
// ============================================
pub const ADMIN_SET_CREATOR: [u8; 8] = [69, 25, 171, 142, 57, 239, 13, 4];
pub const ADMIN_SET_IDL_AUTHORITY: [u8; 8] = [8, 217, 96, 231, 144, 104, 192, 5];
pub const ADMIN_UPDATE_TOKEN_INCENTIVES: [u8; 8] = [209, 11, 115, 87, 213, 23, 124, 204];
pub const BUY: [u8; 8] = [102, 6, 61, 18, 1, 218, 235, 234];
pub const BUY_EXACT_SOL_IN: [u8; 8] = [56, 252, 116, 8, 158, 223, 205, 95];
pub const CLAIM_TOKEN_INCENTIVES: [u8; 8] = [16, 4, 71, 28, 204, 1, 40, 27];
pub const CLOSE_USER_VOLUME_ACCUMULATOR: [u8; 8] = [249, 69, 164, 218, 150, 103, 84, 138];
pub const COLLECT_CREATOR_FEE: [u8; 8] = [20, 22, 86, 123, 198, 28, 219, 132];
pub const CREATE: [u8; 8] = [24, 30, 200, 40, 5, 28, 7, 119];
pub const CREATE_V2: [u8; 8] = [214, 144, 76, 236, 95, 139, 49, 180];
pub const DISTRIBUTE_CREATOR_FEES: [u8; 8] = [165, 114, 103, 0, 121, 206, 247, 81];
pub const EXTEND_ACCOUNT: [u8; 8] = [234, 102, 194, 203, 150, 72, 62, 229];
pub const GET_MINIMUM_DISTRIBUTABLE_FEE: [u8; 8] = [117, 225, 127, 202, 134, 95, 68, 35];
pub const INIT_USER_VOLUME_ACCUMULATOR: [u8; 8] = [94, 6, 202, 115, 255, 96, 232, 183];
pub const INITIALIZE: [u8; 8] = [175, 175, 109, 31, 13, 152, 155, 237];
pub const MIGRATE: [u8; 8] = [155, 234, 231, 146, 236, 158, 162, 30];
pub const MIGRATE_BONDING_CURVE_CREATOR: [u8; 8] = [87, 124, 52, 191, 52, 38, 214, 232];
pub const SELL: [u8; 8] = [51, 230, 133, 164, 1, 127, 131, 173];
pub const SET_CREATOR: [u8; 8] = [254, 148, 255, 112, 207, 142, 170, 165];
pub const SET_MAYHEM_VIRTUAL_PARAMS: [u8; 8] = [61, 169, 188, 191, 153, 149, 42, 97];
pub const SET_METAPLEX_CREATOR: [u8; 8] = [138, 96, 174, 217, 48, 85, 197, 246];
pub const SET_PARAMS: [u8; 8] = [27, 234, 178, 52, 147, 2, 187, 141];
pub const SET_RESERVED_FEE_RECIPIENTS: [u8; 8] = [111, 172, 162, 232, 114, 89, 213, 142];
pub const SYNC_USER_VOLUME_ACCUMULATOR: [u8; 8] = [86, 31, 192, 87, 163, 87, 79, 238];
pub const TOGGLE_CREATE_V2: [u8; 8] = [28, 255, 230, 240, 172, 107, 203, 171];
pub const TOGGLE_MAYHEM_MODE: [u8; 8] = [1, 9, 111, 208, 100, 31, 255, 163];
pub const UPDATE_GLOBAL_AUTHORITY: [u8; 8] = [227, 181, 74, 196, 208, 21, 97, 213];

// ============================================
// Instruction Arguments
// ============================================
/// Arguments for `admin_set_creator`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct AdminSetCreatorArgs {
    pub creator: [u8; 32],
}

/// Arguments for `admin_set_idl_authority`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct AdminSetIdlAuthorityArgs {
    pub idl_authority: [u8; 32],
}

/// Arguments for `admin_update_token_incentives`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct AdminUpdateTokenIncentivesArgs {
    pub start_time: i64,
    pub end_time: i64,
    pub seconds_in_a_day: i64,
    pub day_number: u64,
    pub pump_token_supply_per_day: u64,
}

/// Arguments for `buy`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct BuyArgs {
    pub amount: u64,
    pub max_sol_cost: u64,
    pub track_volume: [u8; 32],
}

/// Arguments for `buy_exact_sol_in`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct BuyExactSolInArgs {
    pub spendable_sol_in: u64,
    pub min_tokens_out: u64,
    pub track_volume: [u8; 32],
}

/// Arguments for `create`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct CreateArgs {
    pub name: [u8; 32],
    pub symbol: [u8; 32],
    pub uri: [u8; 32],
    pub creator: [u8; 32],
}

/// Arguments for `create_v2`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct CreateV2Args {
    pub name: [u8; 32],
    pub symbol: [u8; 32],
    pub uri: [u8; 32],
    pub creator: [u8; 32],
    pub is_mayhem_mode: bool,
}

/// Arguments for `sell`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SellArgs {
    pub amount: u64,
    pub min_sol_output: u64,
}

/// Arguments for `set_creator`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SetCreatorArgs {
    pub creator: [u8; 32],
}

/// Arguments for `set_params`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SetParamsArgs {
    pub initial_virtual_token_reserves: u64,
    pub initial_virtual_sol_reserves: u64,
    pub initial_real_token_reserves: u64,
    pub token_total_supply: u64,
    pub fee_basis_points: u64,
    pub withdraw_authority: [u8; 32],
    pub enable_migrate: bool,
    pub pool_migration_fee: u64,
    pub creator_fee_basis_points: u64,
    pub set_creator_authority: [u8; 32],
    pub admin_set_creator_authority: [u8; 32],
}

/// Arguments for `set_reserved_fee_recipients`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SetReservedFeeRecipientsArgs {
    pub whitelist_pda: [u8; 32],
}

/// Arguments for `toggle_create_v2`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct ToggleCreateV2Args {
    pub enabled: bool,
}

/// Arguments for `toggle_mayhem_mode`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct ToggleMayhemModeArgs {
    pub enabled: bool,
}


// ============================================
// Instruction Accounts
// ============================================
/// Accounts for `admin_set_creator`
pub struct AdminSetCreatorAccounts<'a> {
    /// admin_set_creator_authority
    pub admin_set_creator_authority: &'a AccountView,
    /// global
    pub global: &'a AccountView,
    /// mint
    pub mint: &'a AccountView,
    /// bonding_curve
    pub bonding_curve: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> AdminSetCreatorAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 6] {
        [
            InstructionAccount::readonly_signer(self.admin_set_creator_authority.address()),
            InstructionAccount::readonly(self.global.address()),
            InstructionAccount::readonly(self.mint.address()),
            InstructionAccount::writable(self.bonding_curve.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 6] {
        [
            self.admin_set_creator_authority, self.global, self.mint, self.bonding_curve, self.event_authority, self.program
        ]
    }
}

/// Accounts for `admin_set_idl_authority`
pub struct AdminSetIdlAuthorityAccounts<'a> {
    /// authority
    pub authority: &'a AccountView,
    /// global
    pub global: &'a AccountView,
    /// idl_account
    pub idl_account: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
    /// program_signer
    pub program_signer: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> AdminSetIdlAuthorityAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 7] {
        [
            InstructionAccount::readonly_signer(self.authority.address()),
            InstructionAccount::readonly(self.global.address()),
            InstructionAccount::writable(self.idl_account.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.program_signer.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 7] {
        [
            self.authority, self.global, self.idl_account, self.system_program, self.program_signer, self.event_authority, self.program
        ]
    }
}

/// Accounts for `admin_update_token_incentives`
pub struct AdminUpdateTokenIncentivesAccounts<'a> {
    /// authority
    pub authority: &'a AccountView,
    /// global
    pub global: &'a AccountView,
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
            InstructionAccount::writable_signer(self.authority.address()),
            InstructionAccount::readonly(self.global.address()),
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
            self.authority, self.global, self.global_volume_accumulator, self.mint, self.global_incentive_token_account, self.associated_token_program, self.system_program, self.token_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `buy`
pub struct BuyAccounts<'a> {
    /// global
    pub global: &'a AccountView,
    /// fee_recipient
    pub fee_recipient: &'a AccountView,
    /// mint
    pub mint: &'a AccountView,
    /// bonding_curve
    pub bonding_curve: &'a AccountView,
    /// associated_bonding_curve
    pub associated_bonding_curve: &'a AccountView,
    /// associated_user
    pub associated_user: &'a AccountView,
    /// user
    pub user: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
    /// token_program
    pub token_program: &'a AccountView,
    /// creator_vault
    pub creator_vault: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
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
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 16] {
        [
            InstructionAccount::readonly(self.global.address()),
            InstructionAccount::writable(self.fee_recipient.address()),
            InstructionAccount::readonly(self.mint.address()),
            InstructionAccount::writable(self.bonding_curve.address()),
            InstructionAccount::writable(self.associated_bonding_curve.address()),
            InstructionAccount::writable(self.associated_user.address()),
            InstructionAccount::writable_signer(self.user.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::writable(self.creator_vault.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
            InstructionAccount::readonly(self.global_volume_accumulator.address()),
            InstructionAccount::writable(self.user_volume_accumulator.address()),
            InstructionAccount::readonly(self.fee_config.address()),
            InstructionAccount::readonly(self.fee_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 16] {
        [
            self.global, self.fee_recipient, self.mint, self.bonding_curve, self.associated_bonding_curve, self.associated_user, self.user, self.system_program, self.token_program, self.creator_vault, self.event_authority, self.program, self.global_volume_accumulator, self.user_volume_accumulator, self.fee_config, self.fee_program
        ]
    }
}

/// Accounts for `buy_exact_sol_in`
pub struct BuyExactSolInAccounts<'a> {
    /// global
    pub global: &'a AccountView,
    /// fee_recipient
    pub fee_recipient: &'a AccountView,
    /// mint
    pub mint: &'a AccountView,
    /// bonding_curve
    pub bonding_curve: &'a AccountView,
    /// associated_bonding_curve
    pub associated_bonding_curve: &'a AccountView,
    /// associated_user
    pub associated_user: &'a AccountView,
    /// user
    pub user: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
    /// token_program
    pub token_program: &'a AccountView,
    /// creator_vault
    pub creator_vault: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
    /// global_volume_accumulator
    pub global_volume_accumulator: &'a AccountView,
    /// user_volume_accumulator
    pub user_volume_accumulator: &'a AccountView,
    /// fee_config
    pub fee_config: &'a AccountView,
    /// fee_program
    pub fee_program: &'a AccountView,
}

impl<'a> BuyExactSolInAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 16] {
        [
            InstructionAccount::readonly(self.global.address()),
            InstructionAccount::writable(self.fee_recipient.address()),
            InstructionAccount::readonly(self.mint.address()),
            InstructionAccount::writable(self.bonding_curve.address()),
            InstructionAccount::writable(self.associated_bonding_curve.address()),
            InstructionAccount::writable(self.associated_user.address()),
            InstructionAccount::writable_signer(self.user.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::writable(self.creator_vault.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
            InstructionAccount::readonly(self.global_volume_accumulator.address()),
            InstructionAccount::writable(self.user_volume_accumulator.address()),
            InstructionAccount::readonly(self.fee_config.address()),
            InstructionAccount::readonly(self.fee_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 16] {
        [
            self.global, self.fee_recipient, self.mint, self.bonding_curve, self.associated_bonding_curve, self.associated_user, self.user, self.system_program, self.token_program, self.creator_vault, self.event_authority, self.program, self.global_volume_accumulator, self.user_volume_accumulator, self.fee_config, self.fee_program
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

/// Accounts for `collect_creator_fee`
pub struct CollectCreatorFeeAccounts<'a> {
    /// creator
    pub creator: &'a AccountView,
    /// creator_vault
    pub creator_vault: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> CollectCreatorFeeAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 5] {
        [
            InstructionAccount::writable(self.creator.address()),
            InstructionAccount::writable(self.creator_vault.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 5] {
        [
            self.creator, self.creator_vault, self.system_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `create`
pub struct CreateAccounts<'a> {
    /// mint
    pub mint: &'a AccountView,
    /// mint_authority
    pub mint_authority: &'a AccountView,
    /// bonding_curve
    pub bonding_curve: &'a AccountView,
    /// associated_bonding_curve
    pub associated_bonding_curve: &'a AccountView,
    /// global
    pub global: &'a AccountView,
    /// mpl_token_metadata
    pub mpl_token_metadata: &'a AccountView,
    /// metadata
    pub metadata: &'a AccountView,
    /// user
    pub user: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
    /// token_program
    pub token_program: &'a AccountView,
    /// associated_token_program
    pub associated_token_program: &'a AccountView,
    /// rent
    pub rent: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> CreateAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 14] {
        [
            InstructionAccount::writable_signer(self.mint.address()),
            InstructionAccount::readonly(self.mint_authority.address()),
            InstructionAccount::writable(self.bonding_curve.address()),
            InstructionAccount::writable(self.associated_bonding_curve.address()),
            InstructionAccount::readonly(self.global.address()),
            InstructionAccount::readonly(self.mpl_token_metadata.address()),
            InstructionAccount::writable(self.metadata.address()),
            InstructionAccount::writable_signer(self.user.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
            InstructionAccount::readonly(self.rent.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 14] {
        [
            self.mint, self.mint_authority, self.bonding_curve, self.associated_bonding_curve, self.global, self.mpl_token_metadata, self.metadata, self.user, self.system_program, self.token_program, self.associated_token_program, self.rent, self.event_authority, self.program
        ]
    }
}

/// Accounts for `create_v2`
pub struct CreateV2Accounts<'a> {
    /// mint
    pub mint: &'a AccountView,
    /// mint_authority
    pub mint_authority: &'a AccountView,
    /// bonding_curve
    pub bonding_curve: &'a AccountView,
    /// associated_bonding_curve
    pub associated_bonding_curve: &'a AccountView,
    /// global
    pub global: &'a AccountView,
    /// user
    pub user: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
    /// token_program
    pub token_program: &'a AccountView,
    /// associated_token_program
    pub associated_token_program: &'a AccountView,
    /// mayhem_program_id
    pub mayhem_program_id: &'a AccountView,
    /// global_params
    pub global_params: &'a AccountView,
    /// sol_vault
    pub sol_vault: &'a AccountView,
    /// mayhem_state
    pub mayhem_state: &'a AccountView,
    /// mayhem_token_vault
    pub mayhem_token_vault: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> CreateV2Accounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 16] {
        [
            InstructionAccount::writable_signer(self.mint.address()),
            InstructionAccount::readonly(self.mint_authority.address()),
            InstructionAccount::writable(self.bonding_curve.address()),
            InstructionAccount::writable(self.associated_bonding_curve.address()),
            InstructionAccount::readonly(self.global.address()),
            InstructionAccount::writable_signer(self.user.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
            InstructionAccount::writable(self.mayhem_program_id.address()),
            InstructionAccount::readonly(self.global_params.address()),
            InstructionAccount::writable(self.sol_vault.address()),
            InstructionAccount::writable(self.mayhem_state.address()),
            InstructionAccount::writable(self.mayhem_token_vault.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 16] {
        [
            self.mint, self.mint_authority, self.bonding_curve, self.associated_bonding_curve, self.global, self.user, self.system_program, self.token_program, self.associated_token_program, self.mayhem_program_id, self.global_params, self.sol_vault, self.mayhem_state, self.mayhem_token_vault, self.event_authority, self.program
        ]
    }
}

/// Accounts for `distribute_creator_fees`
pub struct DistributeCreatorFeesAccounts<'a> {
    /// mint
    pub mint: &'a AccountView,
    /// bonding_curve
    pub bonding_curve: &'a AccountView,
    /// sharing_config
    pub sharing_config: &'a AccountView,
    /// creator_vault
    pub creator_vault: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> DistributeCreatorFeesAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 7] {
        [
            InstructionAccount::readonly(self.mint.address()),
            InstructionAccount::readonly(self.bonding_curve.address()),
            InstructionAccount::readonly(self.sharing_config.address()),
            InstructionAccount::writable(self.creator_vault.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 7] {
        [
            self.mint, self.bonding_curve, self.sharing_config, self.creator_vault, self.system_program, self.event_authority, self.program
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

/// Accounts for `get_minimum_distributable_fee`
pub struct GetMinimumDistributableFeeAccounts<'a> {
    /// mint
    pub mint: &'a AccountView,
    /// bonding_curve
    pub bonding_curve: &'a AccountView,
    /// sharing_config
    pub sharing_config: &'a AccountView,
    /// creator_vault
    pub creator_vault: &'a AccountView,
}

impl<'a> GetMinimumDistributableFeeAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 4] {
        [
            InstructionAccount::readonly(self.mint.address()),
            InstructionAccount::readonly(self.bonding_curve.address()),
            InstructionAccount::readonly(self.sharing_config.address()),
            InstructionAccount::readonly(self.creator_vault.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 4] {
        [
            self.mint, self.bonding_curve, self.sharing_config, self.creator_vault
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

/// Accounts for `initialize`
pub struct InitializeAccounts<'a> {
    /// global
    pub global: &'a AccountView,
    /// user
    pub user: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
}

impl<'a> InitializeAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 3] {
        [
            InstructionAccount::writable(self.global.address()),
            InstructionAccount::writable_signer(self.user.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 3] {
        [
            self.global, self.user, self.system_program
        ]
    }
}

/// Accounts for `migrate`
pub struct MigrateAccounts<'a> {
    /// global
    pub global: &'a AccountView,
    /// withdraw_authority
    pub withdraw_authority: &'a AccountView,
    /// mint
    pub mint: &'a AccountView,
    /// bonding_curve
    pub bonding_curve: &'a AccountView,
    /// associated_bonding_curve
    pub associated_bonding_curve: &'a AccountView,
    /// user
    pub user: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
    /// token_program
    pub token_program: &'a AccountView,
    /// pump_amm
    pub pump_amm: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// pool_authority
    pub pool_authority: &'a AccountView,
    /// pool_authority_mint_account
    pub pool_authority_mint_account: &'a AccountView,
    /// pool_authority_wsol_account
    pub pool_authority_wsol_account: &'a AccountView,
    /// amm_global_config
    pub amm_global_config: &'a AccountView,
    /// wsol_mint
    pub wsol_mint: &'a AccountView,
    /// lp_mint
    pub lp_mint: &'a AccountView,
    /// user_pool_token_account
    pub user_pool_token_account: &'a AccountView,
    /// pool_base_token_account
    pub pool_base_token_account: &'a AccountView,
    /// pool_quote_token_account
    pub pool_quote_token_account: &'a AccountView,
    /// token_2022_program
    pub token_2022_program: &'a AccountView,
    /// associated_token_program
    pub associated_token_program: &'a AccountView,
    /// pump_amm_event_authority
    pub pump_amm_event_authority: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> MigrateAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 24] {
        [
            InstructionAccount::readonly(self.global.address()),
            InstructionAccount::writable(self.withdraw_authority.address()),
            InstructionAccount::readonly(self.mint.address()),
            InstructionAccount::writable(self.bonding_curve.address()),
            InstructionAccount::writable(self.associated_bonding_curve.address()),
            InstructionAccount::readonly_signer(self.user.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.pump_amm.address()),
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.pool_authority.address()),
            InstructionAccount::writable(self.pool_authority_mint_account.address()),
            InstructionAccount::writable(self.pool_authority_wsol_account.address()),
            InstructionAccount::readonly(self.amm_global_config.address()),
            InstructionAccount::readonly(self.wsol_mint.address()),
            InstructionAccount::writable(self.lp_mint.address()),
            InstructionAccount::writable(self.user_pool_token_account.address()),
            InstructionAccount::writable(self.pool_base_token_account.address()),
            InstructionAccount::writable(self.pool_quote_token_account.address()),
            InstructionAccount::readonly(self.token_2022_program.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
            InstructionAccount::readonly(self.pump_amm_event_authority.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 24] {
        [
            self.global, self.withdraw_authority, self.mint, self.bonding_curve, self.associated_bonding_curve, self.user, self.system_program, self.token_program, self.pump_amm, self.pool, self.pool_authority, self.pool_authority_mint_account, self.pool_authority_wsol_account, self.amm_global_config, self.wsol_mint, self.lp_mint, self.user_pool_token_account, self.pool_base_token_account, self.pool_quote_token_account, self.token_2022_program, self.associated_token_program, self.pump_amm_event_authority, self.event_authority, self.program
        ]
    }
}

/// Accounts for `migrate_bonding_curve_creator`
pub struct MigrateBondingCurveCreatorAccounts<'a> {
    /// mint
    pub mint: &'a AccountView,
    /// bonding_curve
    pub bonding_curve: &'a AccountView,
    /// sharing_config
    pub sharing_config: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> MigrateBondingCurveCreatorAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 5] {
        [
            InstructionAccount::readonly(self.mint.address()),
            InstructionAccount::writable(self.bonding_curve.address()),
            InstructionAccount::readonly(self.sharing_config.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 5] {
        [
            self.mint, self.bonding_curve, self.sharing_config, self.event_authority, self.program
        ]
    }
}

/// Accounts for `sell`
pub struct SellAccounts<'a> {
    /// global
    pub global: &'a AccountView,
    /// fee_recipient
    pub fee_recipient: &'a AccountView,
    /// mint
    pub mint: &'a AccountView,
    /// bonding_curve
    pub bonding_curve: &'a AccountView,
    /// associated_bonding_curve
    pub associated_bonding_curve: &'a AccountView,
    /// associated_user
    pub associated_user: &'a AccountView,
    /// user
    pub user: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
    /// creator_vault
    pub creator_vault: &'a AccountView,
    /// token_program
    pub token_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
    /// fee_config
    pub fee_config: &'a AccountView,
    /// fee_program
    pub fee_program: &'a AccountView,
}

impl<'a> SellAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 14] {
        [
            InstructionAccount::readonly(self.global.address()),
            InstructionAccount::writable(self.fee_recipient.address()),
            InstructionAccount::readonly(self.mint.address()),
            InstructionAccount::writable(self.bonding_curve.address()),
            InstructionAccount::writable(self.associated_bonding_curve.address()),
            InstructionAccount::writable(self.associated_user.address()),
            InstructionAccount::writable_signer(self.user.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::writable(self.creator_vault.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
            InstructionAccount::readonly(self.fee_config.address()),
            InstructionAccount::readonly(self.fee_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 14] {
        [
            self.global, self.fee_recipient, self.mint, self.bonding_curve, self.associated_bonding_curve, self.associated_user, self.user, self.system_program, self.creator_vault, self.token_program, self.event_authority, self.program, self.fee_config, self.fee_program
        ]
    }
}

/// Accounts for `set_creator`
pub struct SetCreatorAccounts<'a> {
    /// set_creator_authority
    pub set_creator_authority: &'a AccountView,
    /// global
    pub global: &'a AccountView,
    /// mint
    pub mint: &'a AccountView,
    /// metadata
    pub metadata: &'a AccountView,
    /// bonding_curve
    pub bonding_curve: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> SetCreatorAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 7] {
        [
            InstructionAccount::readonly_signer(self.set_creator_authority.address()),
            InstructionAccount::readonly(self.global.address()),
            InstructionAccount::readonly(self.mint.address()),
            InstructionAccount::readonly(self.metadata.address()),
            InstructionAccount::writable(self.bonding_curve.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 7] {
        [
            self.set_creator_authority, self.global, self.mint, self.metadata, self.bonding_curve, self.event_authority, self.program
        ]
    }
}

/// Accounts for `set_mayhem_virtual_params`
pub struct SetMayhemVirtualParamsAccounts<'a> {
    /// sol_vault_authority
    pub sol_vault_authority: &'a AccountView,
    /// mayhem_token_vault
    pub mayhem_token_vault: &'a AccountView,
    /// mint
    pub mint: &'a AccountView,
    /// global
    pub global: &'a AccountView,
    /// bonding_curve
    pub bonding_curve: &'a AccountView,
    /// token_program
    pub token_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> SetMayhemVirtualParamsAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 8] {
        [
            InstructionAccount::writable_signer(self.sol_vault_authority.address()),
            InstructionAccount::writable(self.mayhem_token_vault.address()),
            InstructionAccount::readonly(self.mint.address()),
            InstructionAccount::readonly(self.global.address()),
            InstructionAccount::writable(self.bonding_curve.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 8] {
        [
            self.sol_vault_authority, self.mayhem_token_vault, self.mint, self.global, self.bonding_curve, self.token_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `set_metaplex_creator`
pub struct SetMetaplexCreatorAccounts<'a> {
    /// mint
    pub mint: &'a AccountView,
    /// metadata
    pub metadata: &'a AccountView,
    /// bonding_curve
    pub bonding_curve: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> SetMetaplexCreatorAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 5] {
        [
            InstructionAccount::readonly(self.mint.address()),
            InstructionAccount::readonly(self.metadata.address()),
            InstructionAccount::writable(self.bonding_curve.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 5] {
        [
            self.mint, self.metadata, self.bonding_curve, self.event_authority, self.program
        ]
    }
}

/// Accounts for `set_params`
pub struct SetParamsAccounts<'a> {
    /// global
    pub global: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> SetParamsAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 4] {
        [
            InstructionAccount::writable(self.global.address()),
            InstructionAccount::writable_signer(self.authority.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 4] {
        [
            self.global, self.authority, self.event_authority, self.program
        ]
    }
}

/// Accounts for `set_reserved_fee_recipients`
pub struct SetReservedFeeRecipientsAccounts<'a> {
    /// global
    pub global: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
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
            InstructionAccount::writable(self.global.address()),
            InstructionAccount::readonly_signer(self.authority.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 4] {
        [
            self.global, self.authority, self.event_authority, self.program
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

/// Accounts for `toggle_create_v2`
pub struct ToggleCreateV2Accounts<'a> {
    /// global
    pub global: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> ToggleCreateV2Accounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 4] {
        [
            InstructionAccount::writable(self.global.address()),
            InstructionAccount::writable_signer(self.authority.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 4] {
        [
            self.global, self.authority, self.event_authority, self.program
        ]
    }
}

/// Accounts for `toggle_mayhem_mode`
pub struct ToggleMayhemModeAccounts<'a> {
    /// global
    pub global: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
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
            InstructionAccount::writable(self.global.address()),
            InstructionAccount::writable_signer(self.authority.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 4] {
        [
            self.global, self.authority, self.event_authority, self.program
        ]
    }
}

/// Accounts for `update_global_authority`
pub struct UpdateGlobalAuthorityAccounts<'a> {
    /// global
    pub global: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
    /// new_authority
    pub new_authority: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> UpdateGlobalAuthorityAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 5] {
        [
            InstructionAccount::writable(self.global.address()),
            InstructionAccount::readonly_signer(self.authority.address()),
            InstructionAccount::readonly(self.new_authority.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 5] {
        [
            self.global, self.authority, self.new_authority, self.event_authority, self.program
        ]
    }
}


// ============================================
// CPI Functions
// ============================================
/// CPI: admin_set_creator
#[inline(always)]
pub fn admin_set_creator<'a>(
    accounts: &AdminSetCreatorAccounts<'a>, args: &AdminSetCreatorArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<AdminSetCreatorArgs>()];
    data[0..8].copy_from_slice(&ADMIN_SET_CREATOR);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const AdminSetCreatorArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<AdminSetCreatorArgs>(),
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

/// CPI: admin_set_idl_authority
#[inline(always)]
pub fn admin_set_idl_authority<'a>(
    accounts: &AdminSetIdlAuthorityAccounts<'a>, args: &AdminSetIdlAuthorityArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<AdminSetIdlAuthorityArgs>()];
    data[0..8].copy_from_slice(&ADMIN_SET_IDL_AUTHORITY);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const AdminSetIdlAuthorityArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<AdminSetIdlAuthorityArgs>(),
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
    invoke_signed::<16>(&instruction, &account_views, signers)
}

/// CPI: buy_exact_sol_in
#[inline(always)]
pub fn buy_exact_sol_in<'a>(
    accounts: &BuyExactSolInAccounts<'a>, args: &BuyExactSolInArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<BuyExactSolInArgs>()];
    data[0..8].copy_from_slice(&BUY_EXACT_SOL_IN);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const BuyExactSolInArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<BuyExactSolInArgs>(),
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

/// CPI: collect_creator_fee
#[inline(always)]
pub fn collect_creator_fee<'a>(
    accounts: &CollectCreatorFeeAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&COLLECT_CREATOR_FEE);
    
    
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

/// CPI: create
#[inline(always)]
pub fn create<'a>(
    accounts: &CreateAccounts<'a>, args: &CreateArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<CreateArgs>()];
    data[0..8].copy_from_slice(&CREATE);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const CreateArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<CreateArgs>(),
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

/// CPI: create_v2
#[inline(always)]
pub fn create_v2<'a>(
    accounts: &CreateV2Accounts<'a>, args: &CreateV2Args,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<CreateV2Args>()];
    data[0..8].copy_from_slice(&CREATE_V2);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const CreateV2Args as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<CreateV2Args>(),
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

/// CPI: distribute_creator_fees
#[inline(always)]
pub fn distribute_creator_fees<'a>(
    accounts: &DistributeCreatorFeesAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&DISTRIBUTE_CREATOR_FEES);
    
    
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

/// CPI: get_minimum_distributable_fee
#[inline(always)]
pub fn get_minimum_distributable_fee<'a>(
    accounts: &GetMinimumDistributableFeeAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&GET_MINIMUM_DISTRIBUTABLE_FEE);
    
    
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

/// CPI: initialize
#[inline(always)]
pub fn initialize<'a>(
    accounts: &InitializeAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&INITIALIZE);
    
    
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

/// CPI: migrate
#[inline(always)]
pub fn migrate<'a>(
    accounts: &MigrateAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&MIGRATE);
    
    
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

/// CPI: migrate_bonding_curve_creator
#[inline(always)]
pub fn migrate_bonding_curve_creator<'a>(
    accounts: &MigrateBondingCurveCreatorAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&MIGRATE_BONDING_CURVE_CREATOR);
    
    
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
    invoke_signed::<14>(&instruction, &account_views, signers)
}

/// CPI: set_creator
#[inline(always)]
pub fn set_creator<'a>(
    accounts: &SetCreatorAccounts<'a>, args: &SetCreatorArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SetCreatorArgs>()];
    data[0..8].copy_from_slice(&SET_CREATOR);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SetCreatorArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SetCreatorArgs>(),
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

/// CPI: set_mayhem_virtual_params
#[inline(always)]
pub fn set_mayhem_virtual_params<'a>(
    accounts: &SetMayhemVirtualParamsAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&SET_MAYHEM_VIRTUAL_PARAMS);
    
    
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

/// CPI: set_metaplex_creator
#[inline(always)]
pub fn set_metaplex_creator<'a>(
    accounts: &SetMetaplexCreatorAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&SET_METAPLEX_CREATOR);
    
    
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

/// CPI: set_params
#[inline(always)]
pub fn set_params<'a>(
    accounts: &SetParamsAccounts<'a>, args: &SetParamsArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SetParamsArgs>()];
    data[0..8].copy_from_slice(&SET_PARAMS);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SetParamsArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SetParamsArgs>(),
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

/// CPI: toggle_create_v2
#[inline(always)]
pub fn toggle_create_v2<'a>(
    accounts: &ToggleCreateV2Accounts<'a>, args: &ToggleCreateV2Args,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<ToggleCreateV2Args>()];
    data[0..8].copy_from_slice(&TOGGLE_CREATE_V2);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const ToggleCreateV2Args as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<ToggleCreateV2Args>(),
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

/// CPI: update_global_authority
#[inline(always)]
pub fn update_global_authority<'a>(
    accounts: &UpdateGlobalAuthorityAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&UPDATE_GLOBAL_AUTHORITY);
    
    
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

