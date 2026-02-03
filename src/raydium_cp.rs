//! CPI module for Raydium CP
//!
//! Program: raydium_cp_swap
//! Program ID: CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C
//! Instructions: 14

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
pub const ID: Address = Address::new_from_array(five8_const::decode_32_const("CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C"));

// ============================================
// Instruction Discriminators
// ============================================
pub const CLOSE_PERMISSION_PDA: [u8; 8] = [156, 84, 32, 118, 69, 135, 70, 123];
pub const COLLECT_CREATOR_FEE: [u8; 8] = [20, 22, 86, 123, 198, 28, 219, 132];
pub const COLLECT_FUND_FEE: [u8; 8] = [167, 138, 78, 149, 223, 194, 6, 126];
pub const COLLECT_PROTOCOL_FEE: [u8; 8] = [136, 136, 252, 221, 194, 66, 126, 89];
pub const CREATE_AMM_CONFIG: [u8; 8] = [137, 52, 237, 212, 215, 117, 108, 104];
pub const CREATE_PERMISSION_PDA: [u8; 8] = [135, 136, 2, 216, 137, 169, 181, 202];
pub const DEPOSIT: [u8; 8] = [242, 35, 198, 137, 82, 225, 242, 182];
pub const INITIALIZE: [u8; 8] = [175, 175, 109, 31, 13, 152, 155, 237];
pub const INITIALIZE_WITH_PERMISSION: [u8; 8] = [63, 55, 254, 65, 49, 178, 89, 121];
pub const SWAP_BASE_INPUT: [u8; 8] = [143, 190, 90, 218, 196, 30, 51, 222];
pub const SWAP_BASE_OUTPUT: [u8; 8] = [55, 217, 98, 86, 163, 74, 180, 173];
pub const UPDATE_AMM_CONFIG: [u8; 8] = [49, 60, 174, 136, 154, 28, 116, 200];
pub const UPDATE_POOL_STATUS: [u8; 8] = [130, 87, 108, 6, 46, 224, 117, 123];
pub const WITHDRAW: [u8; 8] = [183, 18, 70, 156, 148, 109, 161, 34];

// ============================================
// Instruction Arguments
// ============================================
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
    pub creator_fee_rate: u64,
}

/// Arguments for `deposit`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct DepositArgs {
    pub lp_token_amount: u64,
    pub maximum_token_0_amount: u64,
    pub maximum_token_1_amount: u64,
}

/// Arguments for `initialize`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct InitializeArgs {
    pub init_amount_0: u64,
    pub init_amount_1: u64,
    pub open_time: u64,
}

/// Arguments for `initialize_with_permission`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct InitializeWithPermissionArgs {
    pub init_amount_0: u64,
    pub init_amount_1: u64,
    pub open_time: u64,
    pub creator_fee_on: [u8; 32],
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
    pub param: u8,
    pub value: u64,
}

/// Arguments for `update_pool_status`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct UpdatePoolStatusArgs {
    pub status: u8,
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
/// Accounts for `close_permission_pda`
pub struct ClosePermissionPdaAccounts<'a> {
    /// owner
    pub owner: &'a AccountView,
    /// permission_authority
    pub permission_authority: &'a AccountView,
    /// Initialize config state account to store protocol owner address and fee rates.
    pub permission: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
}

impl<'a> ClosePermissionPdaAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 4] {
        [
            InstructionAccount::writable_signer(self.owner.address()),
            InstructionAccount::readonly(self.permission_authority.address()),
            InstructionAccount::writable(self.permission.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 4] {
        [
            self.owner, self.permission_authority, self.permission, self.system_program
        ]
    }
}

/// Accounts for `collect_creator_fee`
pub struct CollectCreatorFeeAccounts<'a> {
    /// Only pool creator can collect fee
    pub creator: &'a AccountView,
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
    pub creator_token_0: &'a AccountView,
    /// The address that receives the collected token_1 fund fees
    pub creator_token_1: &'a AccountView,
    /// Spl token program or token program 2022
    pub token_0_program: &'a AccountView,
    /// Spl token program or token program 2022
    pub token_1_program: &'a AccountView,
    /// Program to create an ATA for receiving position NFT
    pub associated_token_program: &'a AccountView,
    /// To create a new program account
    pub system_program: &'a AccountView,
}

impl<'a> CollectCreatorFeeAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 14] {
        [
            InstructionAccount::writable_signer(self.creator.address()),
            InstructionAccount::readonly(self.authority.address()),
            InstructionAccount::writable(self.pool_state.address()),
            InstructionAccount::readonly(self.amm_config.address()),
            InstructionAccount::writable(self.token_0_vault.address()),
            InstructionAccount::writable(self.token_1_vault.address()),
            InstructionAccount::readonly(self.vault_0_mint.address()),
            InstructionAccount::readonly(self.vault_1_mint.address()),
            InstructionAccount::writable(self.creator_token_0.address()),
            InstructionAccount::writable(self.creator_token_1.address()),
            InstructionAccount::readonly(self.token_0_program.address()),
            InstructionAccount::readonly(self.token_1_program.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 14] {
        [
            self.creator, self.authority, self.pool_state, self.amm_config, self.token_0_vault, self.token_1_vault, self.vault_0_mint, self.vault_1_mint, self.creator_token_0, self.creator_token_1, self.token_0_program, self.token_1_program, self.associated_token_program, self.system_program
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
    /// Initialize config state account to store protocol owner address and fee rates.
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

/// Accounts for `create_permission_pda`
pub struct CreatePermissionPdaAccounts<'a> {
    /// owner
    pub owner: &'a AccountView,
    /// permission_authority
    pub permission_authority: &'a AccountView,
    /// Initialize config state account to store protocol owner address and fee rates.
    pub permission: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
}

impl<'a> CreatePermissionPdaAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 4] {
        [
            InstructionAccount::writable_signer(self.owner.address()),
            InstructionAccount::readonly(self.permission_authority.address()),
            InstructionAccount::writable(self.permission.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 4] {
        [
            self.owner, self.permission_authority, self.permission, self.system_program
        ]
    }
}

/// Accounts for `deposit`
pub struct DepositAccounts<'a> {
    /// Pays to mint the position
    pub owner: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
    /// pool_state
    pub pool_state: &'a AccountView,
    /// Owner lp token account
    pub owner_lp_token: &'a AccountView,
    /// The payer's token account for token_0
    pub token_0_account: &'a AccountView,
    /// The payer's token account for token_1
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
    /// Lp token mint
    pub lp_mint: &'a AccountView,
}

impl<'a> DepositAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 13] {
        [
            InstructionAccount::readonly_signer(self.owner.address()),
            InstructionAccount::readonly(self.authority.address()),
            InstructionAccount::writable(self.pool_state.address()),
            InstructionAccount::writable(self.owner_lp_token.address()),
            InstructionAccount::writable(self.token_0_account.address()),
            InstructionAccount::writable(self.token_1_account.address()),
            InstructionAccount::writable(self.token_0_vault.address()),
            InstructionAccount::writable(self.token_1_vault.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.token_program_2022.address()),
            InstructionAccount::readonly(self.vault_0_mint.address()),
            InstructionAccount::readonly(self.vault_1_mint.address()),
            InstructionAccount::writable(self.lp_mint.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 13] {
        [
            self.owner, self.authority, self.pool_state, self.owner_lp_token, self.token_0_account, self.token_1_account, self.token_0_vault, self.token_1_vault, self.token_program, self.token_program_2022, self.vault_0_mint, self.vault_1_mint, self.lp_mint
        ]
    }
}

/// Accounts for `initialize`
pub struct InitializeAccounts<'a> {
    /// Address paying to create the pool. Can be anyone
    pub creator: &'a AccountView,
    /// Which config the pool belongs to.
    pub amm_config: &'a AccountView,
    /// pool vault and lp mint authority
    pub authority: &'a AccountView,
    /// PDA account:
    pub pool_state: &'a AccountView,
    /// Token_0 mint, the key must smaller than token_1 mint.
    pub token_0_mint: &'a AccountView,
    /// Token_1 mint, the key must grater then token_0 mint.
    pub token_1_mint: &'a AccountView,
    /// pool lp mint
    pub lp_mint: &'a AccountView,
    /// payer token0 account
    pub creator_token_0: &'a AccountView,
    /// creator token1 account
    pub creator_token_1: &'a AccountView,
    /// creator lp token account
    pub creator_lp_token: &'a AccountView,
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
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 20] {
        [
            InstructionAccount::writable_signer(self.creator.address()),
            InstructionAccount::readonly(self.amm_config.address()),
            InstructionAccount::readonly(self.authority.address()),
            InstructionAccount::writable(self.pool_state.address()),
            InstructionAccount::readonly(self.token_0_mint.address()),
            InstructionAccount::readonly(self.token_1_mint.address()),
            InstructionAccount::writable(self.lp_mint.address()),
            InstructionAccount::writable(self.creator_token_0.address()),
            InstructionAccount::writable(self.creator_token_1.address()),
            InstructionAccount::writable(self.creator_lp_token.address()),
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
    pub fn to_views(&self) -> [&'a AccountView; 20] {
        [
            self.creator, self.amm_config, self.authority, self.pool_state, self.token_0_mint, self.token_1_mint, self.lp_mint, self.creator_token_0, self.creator_token_1, self.creator_lp_token, self.token_0_vault, self.token_1_vault, self.create_pool_fee, self.observation_state, self.token_program, self.token_0_program, self.token_1_program, self.associated_token_program, self.system_program, self.rent
        ]
    }
}

/// Accounts for `initialize_with_permission`
pub struct InitializeWithPermissionAccounts<'a> {
    /// Address paying to create the pool. Can be anyone
    pub payer: &'a AccountView,
    /// creator
    pub creator: &'a AccountView,
    /// Which config the pool belongs to.
    pub amm_config: &'a AccountView,
    /// pool vault and lp mint authority
    pub authority: &'a AccountView,
    /// PDA account:
    pub pool_state: &'a AccountView,
    /// Token_0 mint, the key must smaller than token_1 mint.
    pub token_0_mint: &'a AccountView,
    /// Token_1 mint, the key must grater then token_0 mint.
    pub token_1_mint: &'a AccountView,
    /// pool lp mint
    pub lp_mint: &'a AccountView,
    /// payer token0 account
    pub payer_token_0: &'a AccountView,
    /// payer token1 account
    pub payer_token_1: &'a AccountView,
    /// payer lp token account
    pub payer_lp_token: &'a AccountView,
    /// token_0_vault
    pub token_0_vault: &'a AccountView,
    /// token_1_vault
    pub token_1_vault: &'a AccountView,
    /// create pool fee account
    pub create_pool_fee: &'a AccountView,
    /// an account to store oracle observations
    pub observation_state: &'a AccountView,
    /// permission
    pub permission: &'a AccountView,
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
}

impl<'a> InitializeWithPermissionAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 21] {
        [
            InstructionAccount::writable_signer(self.payer.address()),
            InstructionAccount::readonly(self.creator.address()),
            InstructionAccount::readonly(self.amm_config.address()),
            InstructionAccount::readonly(self.authority.address()),
            InstructionAccount::writable(self.pool_state.address()),
            InstructionAccount::readonly(self.token_0_mint.address()),
            InstructionAccount::readonly(self.token_1_mint.address()),
            InstructionAccount::writable(self.lp_mint.address()),
            InstructionAccount::writable(self.payer_token_0.address()),
            InstructionAccount::writable(self.payer_token_1.address()),
            InstructionAccount::writable(self.payer_lp_token.address()),
            InstructionAccount::writable(self.token_0_vault.address()),
            InstructionAccount::writable(self.token_1_vault.address()),
            InstructionAccount::writable(self.create_pool_fee.address()),
            InstructionAccount::writable(self.observation_state.address()),
            InstructionAccount::readonly(self.permission.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.token_0_program.address()),
            InstructionAccount::readonly(self.token_1_program.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 21] {
        [
            self.payer, self.creator, self.amm_config, self.authority, self.pool_state, self.token_0_mint, self.token_1_mint, self.lp_mint, self.payer_token_0, self.payer_token_1, self.payer_lp_token, self.token_0_vault, self.token_1_vault, self.create_pool_fee, self.observation_state, self.permission, self.token_program, self.token_0_program, self.token_1_program, self.associated_token_program, self.system_program
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
    /// Amm config account to be changed
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

/// Accounts for `update_pool_status`
pub struct UpdatePoolStatusAccounts<'a> {
    /// authority
    pub authority: &'a AccountView,
    /// pool_state
    pub pool_state: &'a AccountView,
}

impl<'a> UpdatePoolStatusAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 2] {
        [
            InstructionAccount::readonly_signer(self.authority.address()),
            InstructionAccount::writable(self.pool_state.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 2] {
        [
            self.authority, self.pool_state
        ]
    }
}

/// Accounts for `withdraw`
pub struct WithdrawAccounts<'a> {
    /// Pays to mint the position
    pub owner: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
    /// Pool state account
    pub pool_state: &'a AccountView,
    /// Owner lp token account
    pub owner_lp_token: &'a AccountView,
    /// The token account for receive token_0,
    pub token_0_account: &'a AccountView,
    /// The token account for receive token_1
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
    /// Pool lp token mint
    pub lp_mint: &'a AccountView,
    /// memo program
    pub memo_program: &'a AccountView,
}

impl<'a> WithdrawAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 14] {
        [
            InstructionAccount::readonly_signer(self.owner.address()),
            InstructionAccount::readonly(self.authority.address()),
            InstructionAccount::writable(self.pool_state.address()),
            InstructionAccount::writable(self.owner_lp_token.address()),
            InstructionAccount::writable(self.token_0_account.address()),
            InstructionAccount::writable(self.token_1_account.address()),
            InstructionAccount::writable(self.token_0_vault.address()),
            InstructionAccount::writable(self.token_1_vault.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.token_program_2022.address()),
            InstructionAccount::readonly(self.vault_0_mint.address()),
            InstructionAccount::readonly(self.vault_1_mint.address()),
            InstructionAccount::writable(self.lp_mint.address()),
            InstructionAccount::readonly(self.memo_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 14] {
        [
            self.owner, self.authority, self.pool_state, self.owner_lp_token, self.token_0_account, self.token_1_account, self.token_0_vault, self.token_1_vault, self.token_program, self.token_program_2022, self.vault_0_mint, self.vault_1_mint, self.lp_mint, self.memo_program
        ]
    }
}


// ============================================
// CPI Functions
// ============================================
/// CPI: close_permission_pda
#[inline(always)]
pub fn close_permission_pda<'a>(
    accounts: &ClosePermissionPdaAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&CLOSE_PERMISSION_PDA);
    
    
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
    invoke_signed::<14>(&instruction, &account_views, signers)
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

/// CPI: create_permission_pda
#[inline(always)]
pub fn create_permission_pda<'a>(
    accounts: &CreatePermissionPdaAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&CREATE_PERMISSION_PDA);
    
    
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
    invoke_signed::<20>(&instruction, &account_views, signers)
}

/// CPI: initialize_with_permission
#[inline(always)]
pub fn initialize_with_permission<'a>(
    accounts: &InitializeWithPermissionAccounts<'a>, args: &InitializeWithPermissionArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<InitializeWithPermissionArgs>()];
    data[0..8].copy_from_slice(&INITIALIZE_WITH_PERMISSION);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const InitializeWithPermissionArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<InitializeWithPermissionArgs>(),
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

/// CPI: update_pool_status
#[inline(always)]
pub fn update_pool_status<'a>(
    accounts: &UpdatePoolStatusAccounts<'a>, args: &UpdatePoolStatusArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<UpdatePoolStatusArgs>()];
    data[0..8].copy_from_slice(&UPDATE_POOL_STATUS);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const UpdatePoolStatusArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<UpdatePoolStatusArgs>(),
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
    invoke_signed::<14>(&instruction, &account_views, signers)
}

