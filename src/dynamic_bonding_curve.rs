//! CPI module for Dynamic Bonding Curve
//!
//! Program: dynamic_bonding_curve
//! Program ID: dbcij3LWUppWqq96dh6gJWwBifmcGfLSB5D4DuSMaqN
//! Instructions: 28

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
pub const ID: Address = Address::new_from_array(five8_const::decode_32_const("dbcij3LWUppWqq96dh6gJWwBifmcGfLSB5D4DuSMaqN"));

// ============================================
// Instruction Discriminators
// ============================================
pub const CLAIM_CREATOR_TRADING_FEE: [u8; 8] = [82, 220, 250, 189, 3, 85, 107, 45];
pub const CLAIM_LEGACY_POOL_CREATION_FEE: [u8; 8] = [96, 11, 187, 225, 54, 117, 161, 134];
pub const CLAIM_PARTNER_POOL_CREATION_FEE: [u8; 8] = [250, 238, 26, 4, 139, 10, 101, 248];
pub const CLAIM_PROTOCOL_FEE: [u8; 8] = [165, 228, 133, 48, 99, 249, 255, 33];
pub const CLAIM_PROTOCOL_POOL_CREATION_FEE: [u8; 8] = [114, 205, 83, 188, 240, 153, 25, 54];
pub const CLAIM_TRADING_FEE: [u8; 8] = [8, 236, 89, 49, 152, 125, 177, 81];
pub const CLOSE_CLAIM_PROTOCOL_FEE_OPERATOR: [u8; 8] = [8, 41, 87, 35, 80, 48, 121, 26];
pub const CREATE_CLAIM_PROTOCOL_FEE_OPERATOR: [u8; 8] = [51, 19, 150, 252, 105, 157, 48, 91];
pub const CREATE_CONFIG: [u8; 8] = [201, 207, 243, 114, 75, 111, 47, 189];
pub const CREATE_LOCKER: [u8; 8] = [167, 90, 137, 154, 75, 47, 17, 84];
pub const CREATE_PARTNER_METADATA: [u8; 8] = [192, 168, 234, 191, 188, 226, 227, 255];
pub const CREATE_VIRTUAL_POOL_METADATA: [u8; 8] = [45, 97, 187, 103, 254, 109, 124, 134];
pub const CREATOR_WITHDRAW_SURPLUS: [u8; 8] = [165, 3, 137, 7, 28, 134, 76, 80];
pub const INITIALIZE_VIRTUAL_POOL_WITH_SPL_TOKEN: [u8; 8] = [140, 85, 215, 176, 102, 54, 104, 79];
pub const INITIALIZE_VIRTUAL_POOL_WITH_TOKEN2022: [u8; 8] = [169, 118, 51, 78, 145, 110, 220, 155];
pub const MIGRATE_METEORA_DAMM: [u8; 8] = [27, 1, 48, 22, 180, 63, 118, 217];
pub const MIGRATE_METEORA_DAMM_CLAIM_LP_TOKEN: [u8; 8] = [139, 133, 2, 30, 91, 145, 127, 154];
pub const MIGRATE_METEORA_DAMM_LOCK_LP_TOKEN: [u8; 8] = [177, 55, 238, 157, 251, 88, 165, 42];
pub const MIGRATION_DAMM_V2: [u8; 8] = [156, 169, 230, 103, 53, 228, 80, 64];
pub const MIGRATION_DAMM_V2_CREATE_METADATA: [u8; 8] = [109, 189, 19, 36, 195, 183, 222, 82];
pub const MIGRATION_METEORA_DAMM_CREATE_METADATA: [u8; 8] = [47, 94, 126, 115, 221, 226, 194, 133];
pub const PARTNER_WITHDRAW_SURPLUS: [u8; 8] = [168, 173, 72, 100, 201, 98, 38, 92];
pub const PROTOCOL_WITHDRAW_SURPLUS: [u8; 8] = [54, 136, 225, 138, 172, 182, 214, 167];
pub const SWAP: [u8; 8] = [248, 198, 158, 145, 225, 117, 135, 200];
pub const SWAP2: [u8; 8] = [65, 75, 63, 76, 235, 91, 91, 136];
pub const TRANSFER_POOL_CREATOR: [u8; 8] = [20, 7, 169, 33, 58, 147, 166, 33];
pub const WITHDRAW_LEFTOVER: [u8; 8] = [20, 198, 202, 237, 235, 243, 183, 66];
pub const WITHDRAW_MIGRATION_FEE: [u8; 8] = [237, 142, 45, 23, 129, 6, 222, 162];

// ============================================
// Instruction Arguments
// ============================================
/// Arguments for `claim_creator_trading_fee`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct ClaimCreatorTradingFeeArgs {
    pub max_base_amount: u64,
    pub max_quote_amount: u64,
}

/// Arguments for `claim_trading_fee`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct ClaimTradingFeeArgs {
    pub max_amount_a: u64,
    pub max_amount_b: u64,
}

/// Arguments for `create_config`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct CreateConfigArgs {
    pub config_parameters: [u8; 32],
}

/// Arguments for `create_partner_metadata`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct CreatePartnerMetadataArgs {
    pub metadata: [u8; 32],
}

/// Arguments for `create_virtual_pool_metadata`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct CreateVirtualPoolMetadataArgs {
    pub metadata: [u8; 32],
}

/// Arguments for `initialize_virtual_pool_with_spl_token`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct InitializeVirtualPoolWithSplTokenArgs {
    pub params: [u8; 32],
}

/// Arguments for `initialize_virtual_pool_with_token2022`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct InitializeVirtualPoolWithToken2022Args {
    pub params: [u8; 32],
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

/// Arguments for `withdraw_migration_fee`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct WithdrawMigrationFeeArgs {
    pub flag: u8,
}


// ============================================
// Instruction Accounts
// ============================================
/// Accounts for `claim_creator_trading_fee`
pub struct ClaimCreatorTradingFeeAccounts<'a> {
    /// pool_authority
    pub pool_authority: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// The treasury token a account
    pub token_a_account: &'a AccountView,
    /// The treasury token b account
    pub token_b_account: &'a AccountView,
    /// The vault token account for input token
    pub base_vault: &'a AccountView,
    /// The vault token account for output token
    pub quote_vault: &'a AccountView,
    /// The mint of token a
    pub base_mint: &'a AccountView,
    /// The mint of token b
    pub quote_mint: &'a AccountView,
    /// creator
    pub creator: &'a AccountView,
    /// Token a program
    pub token_base_program: &'a AccountView,
    /// Token b program
    pub token_quote_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> ClaimCreatorTradingFeeAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 13] {
        [
            InstructionAccount::readonly(self.pool_authority.address()),
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.token_a_account.address()),
            InstructionAccount::writable(self.token_b_account.address()),
            InstructionAccount::writable(self.base_vault.address()),
            InstructionAccount::writable(self.quote_vault.address()),
            InstructionAccount::readonly(self.base_mint.address()),
            InstructionAccount::readonly(self.quote_mint.address()),
            InstructionAccount::readonly_signer(self.creator.address()),
            InstructionAccount::readonly(self.token_base_program.address()),
            InstructionAccount::readonly(self.token_quote_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 13] {
        [
            self.pool_authority, self.pool, self.token_a_account, self.token_b_account, self.base_vault, self.quote_vault, self.base_mint, self.quote_mint, self.creator, self.token_base_program, self.token_quote_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `claim_legacy_pool_creation_fee`
pub struct ClaimLegacyPoolCreationFeeAccounts<'a> {
    /// pool
    pub pool: &'a AccountView,
    /// Claim fee operator
    pub claim_fee_operator: &'a AccountView,
    /// Operator
    pub signer: &'a AccountView,
    /// treasury
    pub treasury: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> ClaimLegacyPoolCreationFeeAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 7] {
        [
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::readonly(self.claim_fee_operator.address()),
            InstructionAccount::readonly_signer(self.signer.address()),
            InstructionAccount::writable(self.treasury.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 7] {
        [
            self.pool, self.claim_fee_operator, self.signer, self.treasury, self.system_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `claim_partner_pool_creation_fee`
pub struct ClaimPartnerPoolCreationFeeAccounts<'a> {
    /// config
    pub config: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// fee_claimer
    pub fee_claimer: &'a AccountView,
    /// fee_receiver
    pub fee_receiver: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> ClaimPartnerPoolCreationFeeAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 6] {
        [
            InstructionAccount::readonly(self.config.address()),
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::readonly_signer(self.fee_claimer.address()),
            InstructionAccount::writable(self.fee_receiver.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 6] {
        [
            self.config, self.pool, self.fee_claimer, self.fee_receiver, self.event_authority, self.program
        ]
    }
}

/// Accounts for `claim_protocol_fee`
pub struct ClaimProtocolFeeAccounts<'a> {
    /// pool_authority
    pub pool_authority: &'a AccountView,
    /// config
    pub config: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// The vault token account for input token
    pub base_vault: &'a AccountView,
    /// The vault token account for output token
    pub quote_vault: &'a AccountView,
    /// The mint of token a
    pub base_mint: &'a AccountView,
    /// The mint of token b
    pub quote_mint: &'a AccountView,
    /// The treasury token a account
    pub token_base_account: &'a AccountView,
    /// The treasury token b account
    pub token_quote_account: &'a AccountView,
    /// Claim fee operator
    pub claim_fee_operator: &'a AccountView,
    /// Signer
    pub signer: &'a AccountView,
    /// Token a program
    pub token_base_program: &'a AccountView,
    /// Token b program
    pub token_quote_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> ClaimProtocolFeeAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 15] {
        [
            InstructionAccount::readonly(self.pool_authority.address()),
            InstructionAccount::readonly(self.config.address()),
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.base_vault.address()),
            InstructionAccount::writable(self.quote_vault.address()),
            InstructionAccount::readonly(self.base_mint.address()),
            InstructionAccount::readonly(self.quote_mint.address()),
            InstructionAccount::writable(self.token_base_account.address()),
            InstructionAccount::writable(self.token_quote_account.address()),
            InstructionAccount::readonly(self.claim_fee_operator.address()),
            InstructionAccount::readonly_signer(self.signer.address()),
            InstructionAccount::readonly(self.token_base_program.address()),
            InstructionAccount::readonly(self.token_quote_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 15] {
        [
            self.pool_authority, self.config, self.pool, self.base_vault, self.quote_vault, self.base_mint, self.quote_mint, self.token_base_account, self.token_quote_account, self.claim_fee_operator, self.signer, self.token_base_program, self.token_quote_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `claim_protocol_pool_creation_fee`
pub struct ClaimProtocolPoolCreationFeeAccounts<'a> {
    /// config
    pub config: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// Claim fee operator
    pub claim_fee_operator: &'a AccountView,
    /// Operator
    pub signer: &'a AccountView,
    /// treasury
    pub treasury: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> ClaimProtocolPoolCreationFeeAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 7] {
        [
            InstructionAccount::readonly(self.config.address()),
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::readonly(self.claim_fee_operator.address()),
            InstructionAccount::readonly_signer(self.signer.address()),
            InstructionAccount::writable(self.treasury.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 7] {
        [
            self.config, self.pool, self.claim_fee_operator, self.signer, self.treasury, self.event_authority, self.program
        ]
    }
}

/// Accounts for `claim_trading_fee`
pub struct ClaimTradingFeeAccounts<'a> {
    /// pool_authority
    pub pool_authority: &'a AccountView,
    /// config
    pub config: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// The treasury token a account
    pub token_a_account: &'a AccountView,
    /// The treasury token b account
    pub token_b_account: &'a AccountView,
    /// The vault token account for input token
    pub base_vault: &'a AccountView,
    /// The vault token account for output token
    pub quote_vault: &'a AccountView,
    /// The mint of token a
    pub base_mint: &'a AccountView,
    /// The mint of token b
    pub quote_mint: &'a AccountView,
    /// fee_claimer
    pub fee_claimer: &'a AccountView,
    /// Token a program
    pub token_base_program: &'a AccountView,
    /// Token b program
    pub token_quote_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> ClaimTradingFeeAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 14] {
        [
            InstructionAccount::readonly(self.pool_authority.address()),
            InstructionAccount::readonly(self.config.address()),
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.token_a_account.address()),
            InstructionAccount::writable(self.token_b_account.address()),
            InstructionAccount::writable(self.base_vault.address()),
            InstructionAccount::writable(self.quote_vault.address()),
            InstructionAccount::readonly(self.base_mint.address()),
            InstructionAccount::readonly(self.quote_mint.address()),
            InstructionAccount::readonly_signer(self.fee_claimer.address()),
            InstructionAccount::readonly(self.token_base_program.address()),
            InstructionAccount::readonly(self.token_quote_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 14] {
        [
            self.pool_authority, self.config, self.pool, self.token_a_account, self.token_b_account, self.base_vault, self.quote_vault, self.base_mint, self.quote_mint, self.fee_claimer, self.token_base_program, self.token_quote_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `close_claim_protocol_fee_operator`
pub struct CloseClaimProtocolFeeOperatorAccounts<'a> {
    /// claim_fee_operator
    pub claim_fee_operator: &'a AccountView,
    /// rent_receiver
    pub rent_receiver: &'a AccountView,
    /// signer
    pub signer: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> CloseClaimProtocolFeeOperatorAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 5] {
        [
            InstructionAccount::writable(self.claim_fee_operator.address()),
            InstructionAccount::writable(self.rent_receiver.address()),
            InstructionAccount::readonly_signer(self.signer.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 5] {
        [
            self.claim_fee_operator, self.rent_receiver, self.signer, self.event_authority, self.program
        ]
    }
}

/// Accounts for `create_claim_protocol_fee_operator`
pub struct CreateClaimProtocolFeeOperatorAccounts<'a> {
    /// claim_fee_operator
    pub claim_fee_operator: &'a AccountView,
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

impl<'a> CreateClaimProtocolFeeOperatorAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 7] {
        [
            InstructionAccount::writable(self.claim_fee_operator.address()),
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
            self.claim_fee_operator, self.operator, self.signer, self.payer, self.system_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `create_config`
pub struct CreateConfigAccounts<'a> {
    /// config
    pub config: &'a AccountView,
    /// fee_claimer
    pub fee_claimer: &'a AccountView,
    /// leftover_receiver
    pub leftover_receiver: &'a AccountView,
    /// quote mint
    pub quote_mint: &'a AccountView,
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
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 8] {
        [
            InstructionAccount::writable_signer(self.config.address()),
            InstructionAccount::readonly(self.fee_claimer.address()),
            InstructionAccount::readonly(self.leftover_receiver.address()),
            InstructionAccount::readonly(self.quote_mint.address()),
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
            self.config, self.fee_claimer, self.leftover_receiver, self.quote_mint, self.payer, self.system_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `create_locker`
pub struct CreateLockerAccounts<'a> {
    /// Virtual pool
    pub virtual_pool: &'a AccountView,
    /// Config
    pub config: &'a AccountView,
    /// pool_authority
    pub pool_authority: &'a AccountView,
    /// base_vault
    pub base_vault: &'a AccountView,
    /// base_mint
    pub base_mint: &'a AccountView,
    /// base
    pub base: &'a AccountView,
    /// creator
    pub creator: &'a AccountView,
    /// escrow
    pub escrow: &'a AccountView,
    /// escrow_token
    pub escrow_token: &'a AccountView,
    /// payer
    pub payer: &'a AccountView,
    /// token_program
    pub token_program: &'a AccountView,
    /// locker_program
    pub locker_program: &'a AccountView,
    /// locker_event_authority
    pub locker_event_authority: &'a AccountView,
    /// System program.
    pub system_program: &'a AccountView,
}

impl<'a> CreateLockerAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 14] {
        [
            InstructionAccount::writable(self.virtual_pool.address()),
            InstructionAccount::readonly(self.config.address()),
            InstructionAccount::writable(self.pool_authority.address()),
            InstructionAccount::writable(self.base_vault.address()),
            InstructionAccount::writable(self.base_mint.address()),
            InstructionAccount::writable(self.base.address()),
            InstructionAccount::readonly(self.creator.address()),
            InstructionAccount::writable(self.escrow.address()),
            InstructionAccount::writable(self.escrow_token.address()),
            InstructionAccount::writable_signer(self.payer.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.locker_program.address()),
            InstructionAccount::readonly(self.locker_event_authority.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 14] {
        [
            self.virtual_pool, self.config, self.pool_authority, self.base_vault, self.base_mint, self.base, self.creator, self.escrow, self.escrow_token, self.payer, self.token_program, self.locker_program, self.locker_event_authority, self.system_program
        ]
    }
}

/// Accounts for `create_partner_metadata`
pub struct CreatePartnerMetadataAccounts<'a> {
    /// Partner metadata
    pub partner_metadata: &'a AccountView,
    /// Payer of the partner metadata.
    pub payer: &'a AccountView,
    /// Fee claimer for partner
    pub fee_claimer: &'a AccountView,
    /// System program.
    pub system_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> CreatePartnerMetadataAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 6] {
        [
            InstructionAccount::writable(self.partner_metadata.address()),
            InstructionAccount::writable_signer(self.payer.address()),
            InstructionAccount::readonly_signer(self.fee_claimer.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 6] {
        [
            self.partner_metadata, self.payer, self.fee_claimer, self.system_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `create_virtual_pool_metadata`
pub struct CreateVirtualPoolMetadataAccounts<'a> {
    /// virtual_pool
    pub virtual_pool: &'a AccountView,
    /// Virtual pool metadata
    pub virtual_pool_metadata: &'a AccountView,
    /// creator
    pub creator: &'a AccountView,
    /// Payer of the virtual pool metadata.
    pub payer: &'a AccountView,
    /// System program.
    pub system_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> CreateVirtualPoolMetadataAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 7] {
        [
            InstructionAccount::writable(self.virtual_pool.address()),
            InstructionAccount::writable(self.virtual_pool_metadata.address()),
            InstructionAccount::readonly_signer(self.creator.address()),
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
            self.virtual_pool, self.virtual_pool_metadata, self.creator, self.payer, self.system_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `creator_withdraw_surplus`
pub struct CreatorWithdrawSurplusAccounts<'a> {
    /// pool_authority
    pub pool_authority: &'a AccountView,
    /// config
    pub config: &'a AccountView,
    /// virtual_pool
    pub virtual_pool: &'a AccountView,
    /// The receiver token account
    pub token_quote_account: &'a AccountView,
    /// The vault token account for output token
    pub quote_vault: &'a AccountView,
    /// The mint of quote token
    pub quote_mint: &'a AccountView,
    /// creator
    pub creator: &'a AccountView,
    /// Token b program
    pub token_quote_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> CreatorWithdrawSurplusAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 10] {
        [
            InstructionAccount::readonly(self.pool_authority.address()),
            InstructionAccount::readonly(self.config.address()),
            InstructionAccount::writable(self.virtual_pool.address()),
            InstructionAccount::writable(self.token_quote_account.address()),
            InstructionAccount::writable(self.quote_vault.address()),
            InstructionAccount::readonly(self.quote_mint.address()),
            InstructionAccount::readonly_signer(self.creator.address()),
            InstructionAccount::readonly(self.token_quote_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 10] {
        [
            self.pool_authority, self.config, self.virtual_pool, self.token_quote_account, self.quote_vault, self.quote_mint, self.creator, self.token_quote_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `initialize_virtual_pool_with_spl_token`
pub struct InitializeVirtualPoolWithSplTokenAccounts<'a> {
    /// Which config the pool belongs to.
    pub config: &'a AccountView,
    /// pool_authority
    pub pool_authority: &'a AccountView,
    /// creator
    pub creator: &'a AccountView,
    /// base_mint
    pub base_mint: &'a AccountView,
    /// quote_mint
    pub quote_mint: &'a AccountView,
    /// Initialize an account to store the pool state
    pub pool: &'a AccountView,
    /// Token a vault for the pool
    pub base_vault: &'a AccountView,
    /// Token b vault for the pool
    pub quote_vault: &'a AccountView,
    /// mint_metadata
    pub mint_metadata: &'a AccountView,
    /// metadata_program
    pub metadata_program: &'a AccountView,
    /// Address paying to create the pool. Can be anyone
    pub payer: &'a AccountView,
    /// Program to create mint account and mint tokens
    pub token_quote_program: &'a AccountView,
    /// token_program
    pub token_program: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> InitializeVirtualPoolWithSplTokenAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 16] {
        [
            InstructionAccount::readonly(self.config.address()),
            InstructionAccount::readonly(self.pool_authority.address()),
            InstructionAccount::readonly_signer(self.creator.address()),
            InstructionAccount::writable_signer(self.base_mint.address()),
            InstructionAccount::readonly(self.quote_mint.address()),
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.base_vault.address()),
            InstructionAccount::writable(self.quote_vault.address()),
            InstructionAccount::writable(self.mint_metadata.address()),
            InstructionAccount::readonly(self.metadata_program.address()),
            InstructionAccount::writable_signer(self.payer.address()),
            InstructionAccount::readonly(self.token_quote_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 16] {
        [
            self.config, self.pool_authority, self.creator, self.base_mint, self.quote_mint, self.pool, self.base_vault, self.quote_vault, self.mint_metadata, self.metadata_program, self.payer, self.token_quote_program, self.token_program, self.system_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `initialize_virtual_pool_with_token2022`
pub struct InitializeVirtualPoolWithToken2022Accounts<'a> {
    /// Which config the pool belongs to.
    pub config: &'a AccountView,
    /// pool_authority
    pub pool_authority: &'a AccountView,
    /// creator
    pub creator: &'a AccountView,
    /// Unique token mint address, initialize in contract
    pub base_mint: &'a AccountView,
    /// quote_mint
    pub quote_mint: &'a AccountView,
    /// Initialize an account to store the pool state
    pub pool: &'a AccountView,
    /// base_vault
    pub base_vault: &'a AccountView,
    /// Token quote vault for the pool
    pub quote_vault: &'a AccountView,
    /// Address paying to create the pool. Can be anyone
    pub payer: &'a AccountView,
    /// Program to create mint account and mint tokens
    pub token_quote_program: &'a AccountView,
    /// token program for base mint
    pub token_program: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> InitializeVirtualPoolWithToken2022Accounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 14] {
        [
            InstructionAccount::readonly(self.config.address()),
            InstructionAccount::readonly(self.pool_authority.address()),
            InstructionAccount::readonly_signer(self.creator.address()),
            InstructionAccount::writable_signer(self.base_mint.address()),
            InstructionAccount::readonly(self.quote_mint.address()),
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.base_vault.address()),
            InstructionAccount::writable(self.quote_vault.address()),
            InstructionAccount::writable_signer(self.payer.address()),
            InstructionAccount::readonly(self.token_quote_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 14] {
        [
            self.config, self.pool_authority, self.creator, self.base_mint, self.quote_mint, self.pool, self.base_vault, self.quote_vault, self.payer, self.token_quote_program, self.token_program, self.system_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `migrate_meteora_damm`
pub struct MigrateMeteoraDammAccounts<'a> {
    /// virtual pool
    pub virtual_pool: &'a AccountView,
    /// migration_metadata
    pub migration_metadata: &'a AccountView,
    /// config
    pub config: &'a AccountView,
    /// pool_authority
    pub pool_authority: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// pool config
    pub damm_config: &'a AccountView,
    /// lp_mint
    pub lp_mint: &'a AccountView,
    /// token_a_mint
    pub token_a_mint: &'a AccountView,
    /// token_b_mint
    pub token_b_mint: &'a AccountView,
    /// a_vault
    pub a_vault: &'a AccountView,
    /// b_vault
    pub b_vault: &'a AccountView,
    /// a_token_vault
    pub a_token_vault: &'a AccountView,
    /// b_token_vault
    pub b_token_vault: &'a AccountView,
    /// a_vault_lp_mint
    pub a_vault_lp_mint: &'a AccountView,
    /// b_vault_lp_mint
    pub b_vault_lp_mint: &'a AccountView,
    /// a_vault_lp
    pub a_vault_lp: &'a AccountView,
    /// b_vault_lp
    pub b_vault_lp: &'a AccountView,
    /// base_vault
    pub base_vault: &'a AccountView,
    /// quote_vault
    pub quote_vault: &'a AccountView,
    /// virtual_pool_lp
    pub virtual_pool_lp: &'a AccountView,
    /// protocol_token_a_fee
    pub protocol_token_a_fee: &'a AccountView,
    /// protocol_token_b_fee
    pub protocol_token_b_fee: &'a AccountView,
    /// payer
    pub payer: &'a AccountView,
    /// rent
    pub rent: &'a AccountView,
    /// mint_metadata
    pub mint_metadata: &'a AccountView,
    /// metadata_program
    pub metadata_program: &'a AccountView,
    /// amm_program
    pub amm_program: &'a AccountView,
    /// vault_program
    pub vault_program: &'a AccountView,
    /// token_program
    pub token_program: &'a AccountView,
    /// associated_token_program
    pub associated_token_program: &'a AccountView,
    /// System program.
    pub system_program: &'a AccountView,
}

impl<'a> MigrateMeteoraDammAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 31] {
        [
            InstructionAccount::writable(self.virtual_pool.address()),
            InstructionAccount::writable(self.migration_metadata.address()),
            InstructionAccount::readonly(self.config.address()),
            InstructionAccount::writable(self.pool_authority.address()),
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::readonly(self.damm_config.address()),
            InstructionAccount::writable(self.lp_mint.address()),
            InstructionAccount::writable(self.token_a_mint.address()),
            InstructionAccount::readonly(self.token_b_mint.address()),
            InstructionAccount::writable(self.a_vault.address()),
            InstructionAccount::writable(self.b_vault.address()),
            InstructionAccount::writable(self.a_token_vault.address()),
            InstructionAccount::writable(self.b_token_vault.address()),
            InstructionAccount::writable(self.a_vault_lp_mint.address()),
            InstructionAccount::writable(self.b_vault_lp_mint.address()),
            InstructionAccount::writable(self.a_vault_lp.address()),
            InstructionAccount::writable(self.b_vault_lp.address()),
            InstructionAccount::writable(self.base_vault.address()),
            InstructionAccount::writable(self.quote_vault.address()),
            InstructionAccount::writable(self.virtual_pool_lp.address()),
            InstructionAccount::writable(self.protocol_token_a_fee.address()),
            InstructionAccount::writable(self.protocol_token_b_fee.address()),
            InstructionAccount::writable_signer(self.payer.address()),
            InstructionAccount::readonly(self.rent.address()),
            InstructionAccount::writable(self.mint_metadata.address()),
            InstructionAccount::readonly(self.metadata_program.address()),
            InstructionAccount::readonly(self.amm_program.address()),
            InstructionAccount::readonly(self.vault_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 31] {
        [
            self.virtual_pool, self.migration_metadata, self.config, self.pool_authority, self.pool, self.damm_config, self.lp_mint, self.token_a_mint, self.token_b_mint, self.a_vault, self.b_vault, self.a_token_vault, self.b_token_vault, self.a_vault_lp_mint, self.b_vault_lp_mint, self.a_vault_lp, self.b_vault_lp, self.base_vault, self.quote_vault, self.virtual_pool_lp, self.protocol_token_a_fee, self.protocol_token_b_fee, self.payer, self.rent, self.mint_metadata, self.metadata_program, self.amm_program, self.vault_program, self.token_program, self.associated_token_program, self.system_program
        ]
    }
}

/// Accounts for `migrate_meteora_damm_claim_lp_token`
pub struct MigrateMeteoraDammClaimLpTokenAccounts<'a> {
    /// virtual_pool
    pub virtual_pool: &'a AccountView,
    /// migration metadata
    pub migration_metadata: &'a AccountView,
    /// pool_authority
    pub pool_authority: &'a AccountView,
    /// lp_mint
    pub lp_mint: &'a AccountView,
    /// source_token
    pub source_token: &'a AccountView,
    /// destination_token
    pub destination_token: &'a AccountView,
    /// owner
    pub owner: &'a AccountView,
    /// sender
    pub sender: &'a AccountView,
    /// token_program
    pub token_program: &'a AccountView,
}

impl<'a> MigrateMeteoraDammClaimLpTokenAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 9] {
        [
            InstructionAccount::readonly(self.virtual_pool.address()),
            InstructionAccount::writable(self.migration_metadata.address()),
            InstructionAccount::writable(self.pool_authority.address()),
            InstructionAccount::readonly(self.lp_mint.address()),
            InstructionAccount::writable(self.source_token.address()),
            InstructionAccount::writable(self.destination_token.address()),
            InstructionAccount::readonly(self.owner.address()),
            InstructionAccount::readonly_signer(self.sender.address()),
            InstructionAccount::readonly(self.token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 9] {
        [
            self.virtual_pool, self.migration_metadata, self.pool_authority, self.lp_mint, self.source_token, self.destination_token, self.owner, self.sender, self.token_program
        ]
    }
}

/// Accounts for `migrate_meteora_damm_lock_lp_token`
pub struct MigrateMeteoraDammLockLpTokenAccounts<'a> {
    /// virtual_pool
    pub virtual_pool: &'a AccountView,
    /// migration_metadata
    pub migration_metadata: &'a AccountView,
    /// pool_authority
    pub pool_authority: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// lp_mint
    pub lp_mint: &'a AccountView,
    /// lock_escrow
    pub lock_escrow: &'a AccountView,
    /// owner
    pub owner: &'a AccountView,
    /// source_tokens
    pub source_tokens: &'a AccountView,
    /// escrow_vault
    pub escrow_vault: &'a AccountView,
    /// amm_program
    pub amm_program: &'a AccountView,
    /// a_vault
    pub a_vault: &'a AccountView,
    /// b_vault
    pub b_vault: &'a AccountView,
    /// a_vault_lp
    pub a_vault_lp: &'a AccountView,
    /// b_vault_lp
    pub b_vault_lp: &'a AccountView,
    /// a_vault_lp_mint
    pub a_vault_lp_mint: &'a AccountView,
    /// b_vault_lp_mint
    pub b_vault_lp_mint: &'a AccountView,
    /// token_program
    pub token_program: &'a AccountView,
}

impl<'a> MigrateMeteoraDammLockLpTokenAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 17] {
        [
            InstructionAccount::readonly(self.virtual_pool.address()),
            InstructionAccount::writable(self.migration_metadata.address()),
            InstructionAccount::writable(self.pool_authority.address()),
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::readonly(self.lp_mint.address()),
            InstructionAccount::writable(self.lock_escrow.address()),
            InstructionAccount::readonly(self.owner.address()),
            InstructionAccount::writable(self.source_tokens.address()),
            InstructionAccount::writable(self.escrow_vault.address()),
            InstructionAccount::readonly(self.amm_program.address()),
            InstructionAccount::readonly(self.a_vault.address()),
            InstructionAccount::readonly(self.b_vault.address()),
            InstructionAccount::readonly(self.a_vault_lp.address()),
            InstructionAccount::readonly(self.b_vault_lp.address()),
            InstructionAccount::readonly(self.a_vault_lp_mint.address()),
            InstructionAccount::readonly(self.b_vault_lp_mint.address()),
            InstructionAccount::readonly(self.token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 17] {
        [
            self.virtual_pool, self.migration_metadata, self.pool_authority, self.pool, self.lp_mint, self.lock_escrow, self.owner, self.source_tokens, self.escrow_vault, self.amm_program, self.a_vault, self.b_vault, self.a_vault_lp, self.b_vault_lp, self.a_vault_lp_mint, self.b_vault_lp_mint, self.token_program
        ]
    }
}

/// Accounts for `migration_damm_v2`
pub struct MigrationDammV2Accounts<'a> {
    /// virtual pool
    pub virtual_pool: &'a AccountView,
    /// migration_metadata
    pub migration_metadata: &'a AccountView,
    /// virtual pool config key
    pub config: &'a AccountView,
    /// pool_authority
    pub pool_authority: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// first_position_nft_mint
    pub first_position_nft_mint: &'a AccountView,
    /// first_position_nft_account
    pub first_position_nft_account: &'a AccountView,
    /// first_position
    pub first_position: &'a AccountView,
    /// second_position_nft_mint
    pub second_position_nft_mint: &'a AccountView,
    /// second_position_nft_account
    pub second_position_nft_account: &'a AccountView,
    /// second_position
    pub second_position: &'a AccountView,
    /// damm_pool_authority
    pub damm_pool_authority: &'a AccountView,
    /// amm_program
    pub amm_program: &'a AccountView,
    /// base_mint
    pub base_mint: &'a AccountView,
    /// quote_mint
    pub quote_mint: &'a AccountView,
    /// token_a_vault
    pub token_a_vault: &'a AccountView,
    /// token_b_vault
    pub token_b_vault: &'a AccountView,
    /// base_vault
    pub base_vault: &'a AccountView,
    /// quote_vault
    pub quote_vault: &'a AccountView,
    /// payer
    pub payer: &'a AccountView,
    /// token_base_program
    pub token_base_program: &'a AccountView,
    /// token_quote_program
    pub token_quote_program: &'a AccountView,
    /// token_2022_program
    pub token_2022_program: &'a AccountView,
    /// damm_event_authority
    pub damm_event_authority: &'a AccountView,
    /// System program.
    pub system_program: &'a AccountView,
}

impl<'a> MigrationDammV2Accounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 25] {
        [
            InstructionAccount::writable(self.virtual_pool.address()),
            InstructionAccount::readonly(self.migration_metadata.address()),
            InstructionAccount::readonly(self.config.address()),
            InstructionAccount::writable(self.pool_authority.address()),
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable_signer(self.first_position_nft_mint.address()),
            InstructionAccount::writable(self.first_position_nft_account.address()),
            InstructionAccount::writable(self.first_position.address()),
            InstructionAccount::writable_signer(self.second_position_nft_mint.address()),
            InstructionAccount::writable(self.second_position_nft_account.address()),
            InstructionAccount::writable(self.second_position.address()),
            InstructionAccount::readonly(self.damm_pool_authority.address()),
            InstructionAccount::readonly(self.amm_program.address()),
            InstructionAccount::writable(self.base_mint.address()),
            InstructionAccount::writable(self.quote_mint.address()),
            InstructionAccount::writable(self.token_a_vault.address()),
            InstructionAccount::writable(self.token_b_vault.address()),
            InstructionAccount::writable(self.base_vault.address()),
            InstructionAccount::writable(self.quote_vault.address()),
            InstructionAccount::writable_signer(self.payer.address()),
            InstructionAccount::readonly(self.token_base_program.address()),
            InstructionAccount::readonly(self.token_quote_program.address()),
            InstructionAccount::readonly(self.token_2022_program.address()),
            InstructionAccount::readonly(self.damm_event_authority.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 25] {
        [
            self.virtual_pool, self.migration_metadata, self.config, self.pool_authority, self.pool, self.first_position_nft_mint, self.first_position_nft_account, self.first_position, self.second_position_nft_mint, self.second_position_nft_account, self.second_position, self.damm_pool_authority, self.amm_program, self.base_mint, self.quote_mint, self.token_a_vault, self.token_b_vault, self.base_vault, self.quote_vault, self.payer, self.token_base_program, self.token_quote_program, self.token_2022_program, self.damm_event_authority, self.system_program
        ]
    }
}

/// Accounts for `migration_damm_v2_create_metadata`
pub struct MigrationDammV2CreateMetadataAccounts<'a> {
    /// virtual_pool
    pub virtual_pool: &'a AccountView,
    /// config
    pub config: &'a AccountView,
    /// migration_metadata
    pub migration_metadata: &'a AccountView,
    /// payer
    pub payer: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> MigrationDammV2CreateMetadataAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 7] {
        [
            InstructionAccount::readonly(self.virtual_pool.address()),
            InstructionAccount::readonly(self.config.address()),
            InstructionAccount::readonly(self.migration_metadata.address()),
            InstructionAccount::readonly(self.payer.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 7] {
        [
            self.virtual_pool, self.config, self.migration_metadata, self.payer, self.system_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `migration_meteora_damm_create_metadata`
pub struct MigrationMeteoraDammCreateMetadataAccounts<'a> {
    /// virtual_pool
    pub virtual_pool: &'a AccountView,
    /// config
    pub config: &'a AccountView,
    /// migration_metadata
    pub migration_metadata: &'a AccountView,
    /// payer
    pub payer: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> MigrationMeteoraDammCreateMetadataAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 7] {
        [
            InstructionAccount::readonly(self.virtual_pool.address()),
            InstructionAccount::readonly(self.config.address()),
            InstructionAccount::writable(self.migration_metadata.address()),
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
            self.virtual_pool, self.config, self.migration_metadata, self.payer, self.system_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `partner_withdraw_surplus`
pub struct PartnerWithdrawSurplusAccounts<'a> {
    /// pool_authority
    pub pool_authority: &'a AccountView,
    /// config
    pub config: &'a AccountView,
    /// virtual_pool
    pub virtual_pool: &'a AccountView,
    /// The receiver token account
    pub token_quote_account: &'a AccountView,
    /// The vault token account for output token
    pub quote_vault: &'a AccountView,
    /// The mint of quote token
    pub quote_mint: &'a AccountView,
    /// fee_claimer
    pub fee_claimer: &'a AccountView,
    /// Token b program
    pub token_quote_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> PartnerWithdrawSurplusAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 10] {
        [
            InstructionAccount::readonly(self.pool_authority.address()),
            InstructionAccount::readonly(self.config.address()),
            InstructionAccount::writable(self.virtual_pool.address()),
            InstructionAccount::writable(self.token_quote_account.address()),
            InstructionAccount::writable(self.quote_vault.address()),
            InstructionAccount::readonly(self.quote_mint.address()),
            InstructionAccount::readonly_signer(self.fee_claimer.address()),
            InstructionAccount::readonly(self.token_quote_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 10] {
        [
            self.pool_authority, self.config, self.virtual_pool, self.token_quote_account, self.quote_vault, self.quote_mint, self.fee_claimer, self.token_quote_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `protocol_withdraw_surplus`
pub struct ProtocolWithdrawSurplusAccounts<'a> {
    /// pool_authority
    pub pool_authority: &'a AccountView,
    /// config
    pub config: &'a AccountView,
    /// virtual_pool
    pub virtual_pool: &'a AccountView,
    /// The treasury quote token account
    pub token_quote_account: &'a AccountView,
    /// The vault token account for quote token
    pub quote_vault: &'a AccountView,
    /// The mint of token
    pub quote_mint: &'a AccountView,
    /// Claim fee operator
    pub claim_fee_operator: &'a AccountView,
    /// Signer
    pub signer: &'a AccountView,
    /// Token b program
    pub token_quote_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> ProtocolWithdrawSurplusAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 11] {
        [
            InstructionAccount::readonly(self.pool_authority.address()),
            InstructionAccount::readonly(self.config.address()),
            InstructionAccount::writable(self.virtual_pool.address()),
            InstructionAccount::writable(self.token_quote_account.address()),
            InstructionAccount::writable(self.quote_vault.address()),
            InstructionAccount::readonly(self.quote_mint.address()),
            InstructionAccount::readonly(self.claim_fee_operator.address()),
            InstructionAccount::readonly_signer(self.signer.address()),
            InstructionAccount::readonly(self.token_quote_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 11] {
        [
            self.pool_authority, self.config, self.virtual_pool, self.token_quote_account, self.quote_vault, self.quote_mint, self.claim_fee_operator, self.signer, self.token_quote_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `swap`
pub struct SwapAccounts<'a> {
    /// pool_authority
    pub pool_authority: &'a AccountView,
    /// config key
    pub config: &'a AccountView,
    /// Pool account
    pub pool: &'a AccountView,
    /// The user token account for input token
    pub input_token_account: &'a AccountView,
    /// The user token account for output token
    pub output_token_account: &'a AccountView,
    /// The vault token account for base token
    pub base_vault: &'a AccountView,
    /// The vault token account for quote token
    pub quote_vault: &'a AccountView,
    /// The mint of base token
    pub base_mint: &'a AccountView,
    /// The mint of quote token
    pub quote_mint: &'a AccountView,
    /// The user performing the swap
    pub payer: &'a AccountView,
    /// Token base program
    pub token_base_program: &'a AccountView,
    /// Token quote program
    pub token_quote_program: &'a AccountView,
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
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 15] {
        [
            InstructionAccount::readonly(self.pool_authority.address()),
            InstructionAccount::readonly(self.config.address()),
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.input_token_account.address()),
            InstructionAccount::writable(self.output_token_account.address()),
            InstructionAccount::writable(self.base_vault.address()),
            InstructionAccount::writable(self.quote_vault.address()),
            InstructionAccount::readonly(self.base_mint.address()),
            InstructionAccount::readonly(self.quote_mint.address()),
            InstructionAccount::readonly_signer(self.payer.address()),
            InstructionAccount::readonly(self.token_base_program.address()),
            InstructionAccount::readonly(self.token_quote_program.address()),
            InstructionAccount::writable(self.referral_token_account.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 15] {
        [
            self.pool_authority, self.config, self.pool, self.input_token_account, self.output_token_account, self.base_vault, self.quote_vault, self.base_mint, self.quote_mint, self.payer, self.token_base_program, self.token_quote_program, self.referral_token_account, self.event_authority, self.program
        ]
    }
}

/// Accounts for `swap2`
pub struct Swap2Accounts<'a> {
    /// pool_authority
    pub pool_authority: &'a AccountView,
    /// config key
    pub config: &'a AccountView,
    /// Pool account
    pub pool: &'a AccountView,
    /// The user token account for input token
    pub input_token_account: &'a AccountView,
    /// The user token account for output token
    pub output_token_account: &'a AccountView,
    /// The vault token account for base token
    pub base_vault: &'a AccountView,
    /// The vault token account for quote token
    pub quote_vault: &'a AccountView,
    /// The mint of base token
    pub base_mint: &'a AccountView,
    /// The mint of quote token
    pub quote_mint: &'a AccountView,
    /// The user performing the swap
    pub payer: &'a AccountView,
    /// Token base program
    pub token_base_program: &'a AccountView,
    /// Token quote program
    pub token_quote_program: &'a AccountView,
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
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 15] {
        [
            InstructionAccount::readonly(self.pool_authority.address()),
            InstructionAccount::readonly(self.config.address()),
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.input_token_account.address()),
            InstructionAccount::writable(self.output_token_account.address()),
            InstructionAccount::writable(self.base_vault.address()),
            InstructionAccount::writable(self.quote_vault.address()),
            InstructionAccount::readonly(self.base_mint.address()),
            InstructionAccount::readonly(self.quote_mint.address()),
            InstructionAccount::readonly_signer(self.payer.address()),
            InstructionAccount::readonly(self.token_base_program.address()),
            InstructionAccount::readonly(self.token_quote_program.address()),
            InstructionAccount::writable(self.referral_token_account.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 15] {
        [
            self.pool_authority, self.config, self.pool, self.input_token_account, self.output_token_account, self.base_vault, self.quote_vault, self.base_mint, self.quote_mint, self.payer, self.token_base_program, self.token_quote_program, self.referral_token_account, self.event_authority, self.program
        ]
    }
}

/// Accounts for `transfer_pool_creator`
pub struct TransferPoolCreatorAccounts<'a> {
    /// virtual_pool
    pub virtual_pool: &'a AccountView,
    /// config
    pub config: &'a AccountView,
    /// creator
    pub creator: &'a AccountView,
    /// new_creator
    pub new_creator: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> TransferPoolCreatorAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 6] {
        [
            InstructionAccount::writable(self.virtual_pool.address()),
            InstructionAccount::readonly(self.config.address()),
            InstructionAccount::readonly_signer(self.creator.address()),
            InstructionAccount::readonly(self.new_creator.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 6] {
        [
            self.virtual_pool, self.config, self.creator, self.new_creator, self.event_authority, self.program
        ]
    }
}

/// Accounts for `withdraw_leftover`
pub struct WithdrawLeftoverAccounts<'a> {
    /// pool_authority
    pub pool_authority: &'a AccountView,
    /// config
    pub config: &'a AccountView,
    /// virtual_pool
    pub virtual_pool: &'a AccountView,
    /// The receiver token account, withdraw to ATA
    pub token_base_account: &'a AccountView,
    /// The vault token account for output token
    pub base_vault: &'a AccountView,
    /// The mint of quote token
    pub base_mint: &'a AccountView,
    /// leftover_receiver
    pub leftover_receiver: &'a AccountView,
    /// Token base program
    pub token_base_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> WithdrawLeftoverAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 10] {
        [
            InstructionAccount::readonly(self.pool_authority.address()),
            InstructionAccount::readonly(self.config.address()),
            InstructionAccount::writable(self.virtual_pool.address()),
            InstructionAccount::writable(self.token_base_account.address()),
            InstructionAccount::writable(self.base_vault.address()),
            InstructionAccount::readonly(self.base_mint.address()),
            InstructionAccount::readonly(self.leftover_receiver.address()),
            InstructionAccount::readonly(self.token_base_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 10] {
        [
            self.pool_authority, self.config, self.virtual_pool, self.token_base_account, self.base_vault, self.base_mint, self.leftover_receiver, self.token_base_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `withdraw_migration_fee`
pub struct WithdrawMigrationFeeAccounts<'a> {
    /// pool_authority
    pub pool_authority: &'a AccountView,
    /// config
    pub config: &'a AccountView,
    /// virtual_pool
    pub virtual_pool: &'a AccountView,
    /// The receiver token account
    pub token_quote_account: &'a AccountView,
    /// The vault token account for output token
    pub quote_vault: &'a AccountView,
    /// The mint of quote token
    pub quote_mint: &'a AccountView,
    /// sender
    pub sender: &'a AccountView,
    /// Token b program
    pub token_quote_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> WithdrawMigrationFeeAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 10] {
        [
            InstructionAccount::readonly(self.pool_authority.address()),
            InstructionAccount::readonly(self.config.address()),
            InstructionAccount::writable(self.virtual_pool.address()),
            InstructionAccount::writable(self.token_quote_account.address()),
            InstructionAccount::writable(self.quote_vault.address()),
            InstructionAccount::readonly(self.quote_mint.address()),
            InstructionAccount::readonly_signer(self.sender.address()),
            InstructionAccount::readonly(self.token_quote_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 10] {
        [
            self.pool_authority, self.config, self.virtual_pool, self.token_quote_account, self.quote_vault, self.quote_mint, self.sender, self.token_quote_program, self.event_authority, self.program
        ]
    }
}


// ============================================
// CPI Functions
// ============================================
/// CPI: claim_creator_trading_fee
#[inline(always)]
pub fn claim_creator_trading_fee<'a>(
    accounts: &ClaimCreatorTradingFeeAccounts<'a>, args: &ClaimCreatorTradingFeeArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<ClaimCreatorTradingFeeArgs>()];
    data[0..8].copy_from_slice(&CLAIM_CREATOR_TRADING_FEE);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const ClaimCreatorTradingFeeArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<ClaimCreatorTradingFeeArgs>(),
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

/// CPI: claim_legacy_pool_creation_fee
#[inline(always)]
pub fn claim_legacy_pool_creation_fee<'a>(
    accounts: &ClaimLegacyPoolCreationFeeAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&CLAIM_LEGACY_POOL_CREATION_FEE);
    
    
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

/// CPI: claim_partner_pool_creation_fee
#[inline(always)]
pub fn claim_partner_pool_creation_fee<'a>(
    accounts: &ClaimPartnerPoolCreationFeeAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&CLAIM_PARTNER_POOL_CREATION_FEE);
    
    
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

/// CPI: claim_protocol_fee
#[inline(always)]
pub fn claim_protocol_fee<'a>(
    accounts: &ClaimProtocolFeeAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&CLAIM_PROTOCOL_FEE);
    
    
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

/// CPI: claim_protocol_pool_creation_fee
#[inline(always)]
pub fn claim_protocol_pool_creation_fee<'a>(
    accounts: &ClaimProtocolPoolCreationFeeAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&CLAIM_PROTOCOL_POOL_CREATION_FEE);
    
    
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

/// CPI: claim_trading_fee
#[inline(always)]
pub fn claim_trading_fee<'a>(
    accounts: &ClaimTradingFeeAccounts<'a>, args: &ClaimTradingFeeArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<ClaimTradingFeeArgs>()];
    data[0..8].copy_from_slice(&CLAIM_TRADING_FEE);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const ClaimTradingFeeArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<ClaimTradingFeeArgs>(),
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

/// CPI: close_claim_protocol_fee_operator
#[inline(always)]
pub fn close_claim_protocol_fee_operator<'a>(
    accounts: &CloseClaimProtocolFeeOperatorAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&CLOSE_CLAIM_PROTOCOL_FEE_OPERATOR);
    
    
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

/// CPI: create_claim_protocol_fee_operator
#[inline(always)]
pub fn create_claim_protocol_fee_operator<'a>(
    accounts: &CreateClaimProtocolFeeOperatorAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&CREATE_CLAIM_PROTOCOL_FEE_OPERATOR);
    
    
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

/// CPI: create_locker
#[inline(always)]
pub fn create_locker<'a>(
    accounts: &CreateLockerAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&CREATE_LOCKER);
    
    
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

/// CPI: create_partner_metadata
#[inline(always)]
pub fn create_partner_metadata<'a>(
    accounts: &CreatePartnerMetadataAccounts<'a>, args: &CreatePartnerMetadataArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<CreatePartnerMetadataArgs>()];
    data[0..8].copy_from_slice(&CREATE_PARTNER_METADATA);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const CreatePartnerMetadataArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<CreatePartnerMetadataArgs>(),
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

/// CPI: create_virtual_pool_metadata
#[inline(always)]
pub fn create_virtual_pool_metadata<'a>(
    accounts: &CreateVirtualPoolMetadataAccounts<'a>, args: &CreateVirtualPoolMetadataArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<CreateVirtualPoolMetadataArgs>()];
    data[0..8].copy_from_slice(&CREATE_VIRTUAL_POOL_METADATA);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const CreateVirtualPoolMetadataArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<CreateVirtualPoolMetadataArgs>(),
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

/// CPI: creator_withdraw_surplus
#[inline(always)]
pub fn creator_withdraw_surplus<'a>(
    accounts: &CreatorWithdrawSurplusAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&CREATOR_WITHDRAW_SURPLUS);
    
    
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

/// CPI: initialize_virtual_pool_with_spl_token
#[inline(always)]
pub fn initialize_virtual_pool_with_spl_token<'a>(
    accounts: &InitializeVirtualPoolWithSplTokenAccounts<'a>, args: &InitializeVirtualPoolWithSplTokenArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<InitializeVirtualPoolWithSplTokenArgs>()];
    data[0..8].copy_from_slice(&INITIALIZE_VIRTUAL_POOL_WITH_SPL_TOKEN);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const InitializeVirtualPoolWithSplTokenArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<InitializeVirtualPoolWithSplTokenArgs>(),
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

/// CPI: initialize_virtual_pool_with_token2022
#[inline(always)]
pub fn initialize_virtual_pool_with_token2022<'a>(
    accounts: &InitializeVirtualPoolWithToken2022Accounts<'a>, args: &InitializeVirtualPoolWithToken2022Args,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<InitializeVirtualPoolWithToken2022Args>()];
    data[0..8].copy_from_slice(&INITIALIZE_VIRTUAL_POOL_WITH_TOKEN2022);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const InitializeVirtualPoolWithToken2022Args as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<InitializeVirtualPoolWithToken2022Args>(),
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

/// CPI: migrate_meteora_damm
#[inline(always)]
pub fn migrate_meteora_damm<'a>(
    accounts: &MigrateMeteoraDammAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&MIGRATE_METEORA_DAMM);
    
    
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
    invoke_signed::<31>(&instruction, &account_views, signers)
}

/// CPI: migrate_meteora_damm_claim_lp_token
#[inline(always)]
pub fn migrate_meteora_damm_claim_lp_token<'a>(
    accounts: &MigrateMeteoraDammClaimLpTokenAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&MIGRATE_METEORA_DAMM_CLAIM_LP_TOKEN);
    
    
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

/// CPI: migrate_meteora_damm_lock_lp_token
#[inline(always)]
pub fn migrate_meteora_damm_lock_lp_token<'a>(
    accounts: &MigrateMeteoraDammLockLpTokenAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&MIGRATE_METEORA_DAMM_LOCK_LP_TOKEN);
    
    
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
    invoke_signed::<17>(&instruction, &account_views, signers)
}

/// CPI: migration_damm_v2
#[inline(always)]
pub fn migration_damm_v2<'a>(
    accounts: &MigrationDammV2Accounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&MIGRATION_DAMM_V2);
    
    
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
    invoke_signed::<25>(&instruction, &account_views, signers)
}

/// CPI: migration_damm_v2_create_metadata
#[inline(always)]
pub fn migration_damm_v2_create_metadata<'a>(
    accounts: &MigrationDammV2CreateMetadataAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&MIGRATION_DAMM_V2_CREATE_METADATA);
    
    
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

/// CPI: migration_meteora_damm_create_metadata
#[inline(always)]
pub fn migration_meteora_damm_create_metadata<'a>(
    accounts: &MigrationMeteoraDammCreateMetadataAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&MIGRATION_METEORA_DAMM_CREATE_METADATA);
    
    
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

/// CPI: partner_withdraw_surplus
#[inline(always)]
pub fn partner_withdraw_surplus<'a>(
    accounts: &PartnerWithdrawSurplusAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&PARTNER_WITHDRAW_SURPLUS);
    
    
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

/// CPI: protocol_withdraw_surplus
#[inline(always)]
pub fn protocol_withdraw_surplus<'a>(
    accounts: &ProtocolWithdrawSurplusAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&PROTOCOL_WITHDRAW_SURPLUS);
    
    
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
    invoke_signed::<15>(&instruction, &account_views, signers)
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
    invoke_signed::<15>(&instruction, &account_views, signers)
}

/// CPI: transfer_pool_creator
#[inline(always)]
pub fn transfer_pool_creator<'a>(
    accounts: &TransferPoolCreatorAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&TRANSFER_POOL_CREATOR);
    
    
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

/// CPI: withdraw_leftover
#[inline(always)]
pub fn withdraw_leftover<'a>(
    accounts: &WithdrawLeftoverAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&WITHDRAW_LEFTOVER);
    
    
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

/// CPI: withdraw_migration_fee
#[inline(always)]
pub fn withdraw_migration_fee<'a>(
    accounts: &WithdrawMigrationFeeAccounts<'a>, args: &WithdrawMigrationFeeArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<WithdrawMigrationFeeArgs>()];
    data[0..8].copy_from_slice(&WITHDRAW_MIGRATION_FEE);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const WithdrawMigrationFeeArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<WithdrawMigrationFeeArgs>(),
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

