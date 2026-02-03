//! CPI module for Virtuals
//!
//! Program: virtuals_program
//! Program ID: 5U3EU2ubXtK84QcRjWVmYt9RaDyA8gKxdUrPFXmZyaki
//! Instructions: 8

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
pub const ID: Address = Address::new_from_array(five8_const::decode_32_const("5U3EU2ubXtK84QcRjWVmYt9RaDyA8gKxdUrPFXmZyaki"));

// ============================================
// Instruction Discriminators
// ============================================
pub const BUY: [u8; 8] = [102, 6, 61, 18, 1, 218, 235, 234];
pub const CLAIM_FEES: [u8; 8] = [82, 251, 233, 156, 12, 52, 184, 202];
pub const CREATE_METEORA_POOL: [u8; 8] = [246, 254, 33, 37, 225, 176, 41, 232];
pub const INITIALIZE: [u8; 8] = [175, 175, 109, 31, 13, 152, 155, 237];
pub const INITIALIZE_METEORA_ACCOUNTS: [u8; 8] = [53, 12, 118, 158, 253, 239, 185, 214];
pub const LAUNCH: [u8; 8] = [153, 241, 93, 225, 22, 69, 74, 61];
pub const SELL: [u8; 8] = [51, 230, 133, 164, 1, 127, 131, 173];
pub const UPDATE_POOL_CREATOR: [u8; 8] = [113, 225, 166, 185, 94, 231, 96, 28];

// ============================================
// Instruction Arguments
// ============================================
/// Arguments for `buy`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct BuyArgs {
    pub amount: u64,
    pub max_amount_out: u64,
}

/// Arguments for `launch`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct LaunchArgs {
    pub symbol: [u8; 32],
    pub name: [u8; 32],
    pub uri: [u8; 32],
}

/// Arguments for `sell`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SellArgs {
    pub amount: u64,
    pub min_amount_out: u64,
}


// ============================================
// Instruction Accounts
// ============================================
/// Accounts for `buy`
pub struct BuyAccounts<'a> {
    /// user
    pub user: &'a AccountView,
    /// vpool
    pub vpool: &'a AccountView,
    /// token_mint
    pub token_mint: &'a AccountView,
    /// user_virtuals_ata
    pub user_virtuals_ata: &'a AccountView,
    /// user_token_ata
    pub user_token_ata: &'a AccountView,
    /// vpool_token_ata
    pub vpool_token_ata: &'a AccountView,
    /// platform_prototype
    pub platform_prototype: &'a AccountView,
    /// platform_prototype_virtuals_ata
    pub platform_prototype_virtuals_ata: &'a AccountView,
    /// vpool_virtuals_ata
    pub vpool_virtuals_ata: &'a AccountView,
    /// token_program
    pub token_program: &'a AccountView,
}

impl<'a> BuyAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 10] {
        [
            InstructionAccount::readonly_signer(self.user.address()),
            InstructionAccount::writable(self.vpool.address()),
            InstructionAccount::readonly(self.token_mint.address()),
            InstructionAccount::writable(self.user_virtuals_ata.address()),
            InstructionAccount::writable(self.user_token_ata.address()),
            InstructionAccount::writable(self.vpool_token_ata.address()),
            InstructionAccount::writable(self.platform_prototype.address()),
            InstructionAccount::writable(self.platform_prototype_virtuals_ata.address()),
            InstructionAccount::writable(self.vpool_virtuals_ata.address()),
            InstructionAccount::readonly(self.token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 10] {
        [
            self.user, self.vpool, self.token_mint, self.user_virtuals_ata, self.user_token_ata, self.vpool_token_ata, self.platform_prototype, self.platform_prototype_virtuals_ata, self.vpool_virtuals_ata, self.token_program
        ]
    }
}

/// Accounts for `claim_fees`
pub struct ClaimFeesAccounts<'a> {
    /// payer
    pub payer: &'a AccountView,
    /// vpool
    pub vpool: &'a AccountView,
    /// virtuals_mint
    pub virtuals_mint: &'a AccountView,
    /// token_mint
    pub token_mint: &'a AccountView,
    /// vpool_virtuals_ata
    pub vpool_virtuals_ata: &'a AccountView,
    /// vpool_token_ata
    pub vpool_token_ata: &'a AccountView,
    /// platform
    pub platform: &'a AccountView,
    /// platform_virtuals_ata
    pub platform_virtuals_ata: &'a AccountView,
    /// platform_token_ata
    pub platform_token_ata: &'a AccountView,
    /// creator_virtuals_ata
    pub creator_virtuals_ata: &'a AccountView,
    /// creator_token_ata
    pub creator_token_ata: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// lp_mint
    pub lp_mint: &'a AccountView,
    /// lock_escrow
    pub lock_escrow: &'a AccountView,
    /// escrow_vault
    pub escrow_vault: &'a AccountView,
    /// token_program
    pub token_program: &'a AccountView,
    /// virtuals_vault
    pub virtuals_vault: &'a AccountView,
    /// token_vault
    pub token_vault: &'a AccountView,
    /// virtuals_token_vault
    pub virtuals_token_vault: &'a AccountView,
    /// token_token_vault
    pub token_token_vault: &'a AccountView,
    /// virtuals_vault_lp_mint
    pub virtuals_vault_lp_mint: &'a AccountView,
    /// token_vault_lp_mint
    pub token_vault_lp_mint: &'a AccountView,
    /// virtuals_vault_lp
    pub virtuals_vault_lp: &'a AccountView,
    /// token_vault_lp
    pub token_vault_lp: &'a AccountView,
    /// vault_program
    pub vault_program: &'a AccountView,
    /// associated_token_program
    pub associated_token_program: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
    /// dynamic_amm_program
    pub dynamic_amm_program: &'a AccountView,
}

impl<'a> ClaimFeesAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 28] {
        [
            InstructionAccount::writable_signer(self.payer.address()),
            InstructionAccount::writable(self.vpool.address()),
            InstructionAccount::readonly(self.virtuals_mint.address()),
            InstructionAccount::readonly(self.token_mint.address()),
            InstructionAccount::writable(self.vpool_virtuals_ata.address()),
            InstructionAccount::writable(self.vpool_token_ata.address()),
            InstructionAccount::writable(self.platform.address()),
            InstructionAccount::writable(self.platform_virtuals_ata.address()),
            InstructionAccount::writable(self.platform_token_ata.address()),
            InstructionAccount::writable(self.creator_virtuals_ata.address()),
            InstructionAccount::writable(self.creator_token_ata.address()),
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.lp_mint.address()),
            InstructionAccount::writable(self.lock_escrow.address()),
            InstructionAccount::writable(self.escrow_vault.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::writable(self.virtuals_vault.address()),
            InstructionAccount::writable(self.token_vault.address()),
            InstructionAccount::writable(self.virtuals_token_vault.address()),
            InstructionAccount::writable(self.token_token_vault.address()),
            InstructionAccount::writable(self.virtuals_vault_lp_mint.address()),
            InstructionAccount::writable(self.token_vault_lp_mint.address()),
            InstructionAccount::writable(self.virtuals_vault_lp.address()),
            InstructionAccount::writable(self.token_vault_lp.address()),
            InstructionAccount::readonly(self.vault_program.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.dynamic_amm_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 28] {
        [
            self.payer, self.vpool, self.virtuals_mint, self.token_mint, self.vpool_virtuals_ata, self.vpool_token_ata, self.platform, self.platform_virtuals_ata, self.platform_token_ata, self.creator_virtuals_ata, self.creator_token_ata, self.pool, self.lp_mint, self.lock_escrow, self.escrow_vault, self.token_program, self.virtuals_vault, self.token_vault, self.virtuals_token_vault, self.token_token_vault, self.virtuals_vault_lp_mint, self.token_vault_lp_mint, self.virtuals_vault_lp, self.token_vault_lp, self.vault_program, self.associated_token_program, self.system_program, self.dynamic_amm_program
        ]
    }
}

/// Accounts for `create_meteora_pool`
pub struct CreateMeteoraPoolAccounts<'a> {
    /// vpool
    pub vpool: &'a AccountView,
    /// meteora_deployer
    pub meteora_deployer: &'a AccountView,
    /// meteora_deployer_virtuals_ata
    pub meteora_deployer_virtuals_ata: &'a AccountView,
    /// meteora_deployer_token_ata
    pub meteora_deployer_token_ata: &'a AccountView,
    /// vpool_virtuals_ata
    pub vpool_virtuals_ata: &'a AccountView,
    /// vpool_token_ata
    pub vpool_token_ata: &'a AccountView,
    /// lock_escrow
    pub lock_escrow: &'a AccountView,
    /// escrow_vault
    pub escrow_vault: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// config
    pub config: &'a AccountView,
    /// lp_mint
    pub lp_mint: &'a AccountView,
    /// virtuals_mint
    pub virtuals_mint: &'a AccountView,
    /// token_mint
    pub token_mint: &'a AccountView,
    /// virtuals_vault
    pub virtuals_vault: &'a AccountView,
    /// token_vault
    pub token_vault: &'a AccountView,
    /// virtuals_token_vault
    pub virtuals_token_vault: &'a AccountView,
    /// token_token_vault
    pub token_token_vault: &'a AccountView,
    /// virtuals_vault_lp_mint
    pub virtuals_vault_lp_mint: &'a AccountView,
    /// token_vault_lp_mint
    pub token_vault_lp_mint: &'a AccountView,
    /// virtuals_vault_lp
    pub virtuals_vault_lp: &'a AccountView,
    /// token_vault_lp
    pub token_vault_lp: &'a AccountView,
    /// pool_virtuals_ata
    pub pool_virtuals_ata: &'a AccountView,
    /// pool_token_ata
    pub pool_token_ata: &'a AccountView,
    /// meteora_deployer_pool_lp
    pub meteora_deployer_pool_lp: &'a AccountView,
    /// protocol_virtuals_fee
    pub protocol_virtuals_fee: &'a AccountView,
    /// protocol_token_fee
    pub protocol_token_fee: &'a AccountView,
    /// payer
    pub payer: &'a AccountView,
    /// token_metadata
    pub token_metadata: &'a AccountView,
    /// Rent account.
    pub rent: &'a AccountView,
    /// mint_metadata
    pub mint_metadata: &'a AccountView,
    /// metadata_program
    pub metadata_program: &'a AccountView,
    /// vault_program
    pub vault_program: &'a AccountView,
    /// Token program.
    pub token_program: &'a AccountView,
    /// Associated token program.
    pub associated_token_program: &'a AccountView,
    /// System program.
    pub system_program: &'a AccountView,
    /// dynamic_amm_program
    pub dynamic_amm_program: &'a AccountView,
}

impl<'a> CreateMeteoraPoolAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 36] {
        [
            InstructionAccount::writable(self.vpool.address()),
            InstructionAccount::writable(self.meteora_deployer.address()),
            InstructionAccount::writable(self.meteora_deployer_virtuals_ata.address()),
            InstructionAccount::writable(self.meteora_deployer_token_ata.address()),
            InstructionAccount::writable(self.vpool_virtuals_ata.address()),
            InstructionAccount::writable(self.vpool_token_ata.address()),
            InstructionAccount::writable(self.lock_escrow.address()),
            InstructionAccount::writable(self.escrow_vault.address()),
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::readonly(self.config.address()),
            InstructionAccount::writable(self.lp_mint.address()),
            InstructionAccount::readonly(self.virtuals_mint.address()),
            InstructionAccount::readonly(self.token_mint.address()),
            InstructionAccount::writable(self.virtuals_vault.address()),
            InstructionAccount::writable(self.token_vault.address()),
            InstructionAccount::writable(self.virtuals_token_vault.address()),
            InstructionAccount::writable(self.token_token_vault.address()),
            InstructionAccount::writable(self.virtuals_vault_lp_mint.address()),
            InstructionAccount::writable(self.token_vault_lp_mint.address()),
            InstructionAccount::writable(self.virtuals_vault_lp.address()),
            InstructionAccount::writable(self.token_vault_lp.address()),
            InstructionAccount::writable(self.pool_virtuals_ata.address()),
            InstructionAccount::writable(self.pool_token_ata.address()),
            InstructionAccount::writable(self.meteora_deployer_pool_lp.address()),
            InstructionAccount::writable(self.protocol_virtuals_fee.address()),
            InstructionAccount::writable(self.protocol_token_fee.address()),
            InstructionAccount::writable_signer(self.payer.address()),
            InstructionAccount::writable(self.token_metadata.address()),
            InstructionAccount::readonly(self.rent.address()),
            InstructionAccount::writable(self.mint_metadata.address()),
            InstructionAccount::readonly(self.metadata_program.address()),
            InstructionAccount::readonly(self.vault_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.dynamic_amm_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 36] {
        [
            self.vpool, self.meteora_deployer, self.meteora_deployer_virtuals_ata, self.meteora_deployer_token_ata, self.vpool_virtuals_ata, self.vpool_token_ata, self.lock_escrow, self.escrow_vault, self.pool, self.config, self.lp_mint, self.virtuals_mint, self.token_mint, self.virtuals_vault, self.token_vault, self.virtuals_token_vault, self.token_token_vault, self.virtuals_vault_lp_mint, self.token_vault_lp_mint, self.virtuals_vault_lp, self.token_vault_lp, self.pool_virtuals_ata, self.pool_token_ata, self.meteora_deployer_pool_lp, self.protocol_virtuals_fee, self.protocol_token_fee, self.payer, self.token_metadata, self.rent, self.mint_metadata, self.metadata_program, self.vault_program, self.token_program, self.associated_token_program, self.system_program, self.dynamic_amm_program
        ]
    }
}

/// Accounts for `initialize`
pub struct InitializeAccounts<'a> {
    /// payer
    pub payer: &'a AccountView,
    /// virtuals_mint
    pub virtuals_mint: &'a AccountView,
    /// token_mint
    pub token_mint: &'a AccountView,
    /// vpool_virtuals_ata
    pub vpool_virtuals_ata: &'a AccountView,
    /// vpool_token_ata
    pub vpool_token_ata: &'a AccountView,
    /// vpool
    pub vpool: &'a AccountView,
    /// token_program
    pub token_program: &'a AccountView,
    /// associated_token_program
    pub associated_token_program: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
}

impl<'a> InitializeAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 9] {
        [
            InstructionAccount::writable_signer(self.payer.address()),
            InstructionAccount::readonly(self.virtuals_mint.address()),
            InstructionAccount::writable(self.token_mint.address()),
            InstructionAccount::writable(self.vpool_virtuals_ata.address()),
            InstructionAccount::writable(self.vpool_token_ata.address()),
            InstructionAccount::writable(self.vpool.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 9] {
        [
            self.payer, self.virtuals_mint, self.token_mint, self.vpool_virtuals_ata, self.vpool_token_ata, self.vpool, self.token_program, self.associated_token_program, self.system_program
        ]
    }
}

/// Accounts for `initialize_meteora_accounts`
pub struct InitializeMeteoraAccountsAccounts<'a> {
    /// vpool
    pub vpool: &'a AccountView,
    /// meteora_deployer
    pub meteora_deployer: &'a AccountView,
    /// meteora_deployer_virtuals_ata
    pub meteora_deployer_virtuals_ata: &'a AccountView,
    /// meteora_deployer_token_ata
    pub meteora_deployer_token_ata: &'a AccountView,
    /// vpool_virtuals_ata
    pub vpool_virtuals_ata: &'a AccountView,
    /// vpool_token_ata
    pub vpool_token_ata: &'a AccountView,
    /// lock_escrow
    pub lock_escrow: &'a AccountView,
    /// escrow_vault
    pub escrow_vault: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// config
    pub config: &'a AccountView,
    /// lp_mint
    pub lp_mint: &'a AccountView,
    /// virtuals_mint
    pub virtuals_mint: &'a AccountView,
    /// token_mint
    pub token_mint: &'a AccountView,
    /// virtuals_vault
    pub virtuals_vault: &'a AccountView,
    /// token_vault
    pub token_vault: &'a AccountView,
    /// virtuals_token_vault
    pub virtuals_token_vault: &'a AccountView,
    /// token_token_vault
    pub token_token_vault: &'a AccountView,
    /// virtuals_vault_lp_mint
    pub virtuals_vault_lp_mint: &'a AccountView,
    /// token_vault_lp_mint
    pub token_vault_lp_mint: &'a AccountView,
    /// virtuals_vault_lp
    pub virtuals_vault_lp: &'a AccountView,
    /// token_vault_lp
    pub token_vault_lp: &'a AccountView,
    /// pool_virtuals_ata
    pub pool_virtuals_ata: &'a AccountView,
    /// pool_token_ata
    pub pool_token_ata: &'a AccountView,
    /// meteora_deployer_pool_lp
    pub meteora_deployer_pool_lp: &'a AccountView,
    /// protocol_virtuals_fee
    pub protocol_virtuals_fee: &'a AccountView,
    /// protocol_token_fee
    pub protocol_token_fee: &'a AccountView,
    /// payer
    pub payer: &'a AccountView,
    /// token_metadata
    pub token_metadata: &'a AccountView,
    /// Rent account.
    pub rent: &'a AccountView,
    /// mint_metadata
    pub mint_metadata: &'a AccountView,
    /// metadata_program
    pub metadata_program: &'a AccountView,
    /// vault_program
    pub vault_program: &'a AccountView,
    /// Token program.
    pub token_program: &'a AccountView,
    /// Associated token program.
    pub associated_token_program: &'a AccountView,
    /// System program.
    pub system_program: &'a AccountView,
    /// dynamic_amm_program
    pub dynamic_amm_program: &'a AccountView,
}

impl<'a> InitializeMeteoraAccountsAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 36] {
        [
            InstructionAccount::writable(self.vpool.address()),
            InstructionAccount::writable(self.meteora_deployer.address()),
            InstructionAccount::writable(self.meteora_deployer_virtuals_ata.address()),
            InstructionAccount::writable(self.meteora_deployer_token_ata.address()),
            InstructionAccount::writable(self.vpool_virtuals_ata.address()),
            InstructionAccount::writable(self.vpool_token_ata.address()),
            InstructionAccount::writable(self.lock_escrow.address()),
            InstructionAccount::writable(self.escrow_vault.address()),
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::readonly(self.config.address()),
            InstructionAccount::writable(self.lp_mint.address()),
            InstructionAccount::readonly(self.virtuals_mint.address()),
            InstructionAccount::readonly(self.token_mint.address()),
            InstructionAccount::writable(self.virtuals_vault.address()),
            InstructionAccount::writable(self.token_vault.address()),
            InstructionAccount::writable(self.virtuals_token_vault.address()),
            InstructionAccount::writable(self.token_token_vault.address()),
            InstructionAccount::writable(self.virtuals_vault_lp_mint.address()),
            InstructionAccount::writable(self.token_vault_lp_mint.address()),
            InstructionAccount::writable(self.virtuals_vault_lp.address()),
            InstructionAccount::writable(self.token_vault_lp.address()),
            InstructionAccount::writable(self.pool_virtuals_ata.address()),
            InstructionAccount::writable(self.pool_token_ata.address()),
            InstructionAccount::writable(self.meteora_deployer_pool_lp.address()),
            InstructionAccount::writable(self.protocol_virtuals_fee.address()),
            InstructionAccount::writable(self.protocol_token_fee.address()),
            InstructionAccount::writable_signer(self.payer.address()),
            InstructionAccount::writable(self.token_metadata.address()),
            InstructionAccount::readonly(self.rent.address()),
            InstructionAccount::writable(self.mint_metadata.address()),
            InstructionAccount::readonly(self.metadata_program.address()),
            InstructionAccount::readonly(self.vault_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.dynamic_amm_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 36] {
        [
            self.vpool, self.meteora_deployer, self.meteora_deployer_virtuals_ata, self.meteora_deployer_token_ata, self.vpool_virtuals_ata, self.vpool_token_ata, self.lock_escrow, self.escrow_vault, self.pool, self.config, self.lp_mint, self.virtuals_mint, self.token_mint, self.virtuals_vault, self.token_vault, self.virtuals_token_vault, self.token_token_vault, self.virtuals_vault_lp_mint, self.token_vault_lp_mint, self.virtuals_vault_lp, self.token_vault_lp, self.pool_virtuals_ata, self.pool_token_ata, self.meteora_deployer_pool_lp, self.protocol_virtuals_fee, self.protocol_token_fee, self.payer, self.token_metadata, self.rent, self.mint_metadata, self.metadata_program, self.vault_program, self.token_program, self.associated_token_program, self.system_program, self.dynamic_amm_program
        ]
    }
}

/// Accounts for `launch`
pub struct LaunchAccounts<'a> {
    /// creator
    pub creator: &'a AccountView,
    /// creator_virtuals_ata
    pub creator_virtuals_ata: &'a AccountView,
    /// token_mint
    pub token_mint: &'a AccountView,
    /// platform_prototype
    pub platform_prototype: &'a AccountView,
    /// platform_prototype_virtuals_ata
    pub platform_prototype_virtuals_ata: &'a AccountView,
    /// vpool
    pub vpool: &'a AccountView,
    /// token_metadata
    pub token_metadata: &'a AccountView,
    /// metadata_program
    pub metadata_program: &'a AccountView,
    /// token_program
    pub token_program: &'a AccountView,
    /// associated_token_program
    pub associated_token_program: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
    /// rent
    pub rent: &'a AccountView,
}

impl<'a> LaunchAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 12] {
        [
            InstructionAccount::writable_signer(self.creator.address()),
            InstructionAccount::writable(self.creator_virtuals_ata.address()),
            InstructionAccount::writable(self.token_mint.address()),
            InstructionAccount::writable(self.platform_prototype.address()),
            InstructionAccount::writable(self.platform_prototype_virtuals_ata.address()),
            InstructionAccount::writable(self.vpool.address()),
            InstructionAccount::writable(self.token_metadata.address()),
            InstructionAccount::readonly(self.metadata_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.rent.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 12] {
        [
            self.creator, self.creator_virtuals_ata, self.token_mint, self.platform_prototype, self.platform_prototype_virtuals_ata, self.vpool, self.token_metadata, self.metadata_program, self.token_program, self.associated_token_program, self.system_program, self.rent
        ]
    }
}

/// Accounts for `sell`
pub struct SellAccounts<'a> {
    /// user
    pub user: &'a AccountView,
    /// vpool
    pub vpool: &'a AccountView,
    /// token_mint
    pub token_mint: &'a AccountView,
    /// user_virtuals_ata
    pub user_virtuals_ata: &'a AccountView,
    /// user_token_ata
    pub user_token_ata: &'a AccountView,
    /// vpool_token_ata
    pub vpool_token_ata: &'a AccountView,
    /// platform_prototype
    pub platform_prototype: &'a AccountView,
    /// platform_prototype_virtuals_ata
    pub platform_prototype_virtuals_ata: &'a AccountView,
    /// vpool_virtuals_ata
    pub vpool_virtuals_ata: &'a AccountView,
    /// token_program
    pub token_program: &'a AccountView,
}

impl<'a> SellAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 10] {
        [
            InstructionAccount::readonly_signer(self.user.address()),
            InstructionAccount::writable(self.vpool.address()),
            InstructionAccount::readonly(self.token_mint.address()),
            InstructionAccount::writable(self.user_virtuals_ata.address()),
            InstructionAccount::writable(self.user_token_ata.address()),
            InstructionAccount::writable(self.vpool_token_ata.address()),
            InstructionAccount::writable(self.platform_prototype.address()),
            InstructionAccount::writable(self.platform_prototype_virtuals_ata.address()),
            InstructionAccount::writable(self.vpool_virtuals_ata.address()),
            InstructionAccount::readonly(self.token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 10] {
        [
            self.user, self.vpool, self.token_mint, self.user_virtuals_ata, self.user_token_ata, self.vpool_token_ata, self.platform_prototype, self.platform_prototype_virtuals_ata, self.vpool_virtuals_ata, self.token_program
        ]
    }
}

/// Accounts for `update_pool_creator`
pub struct UpdatePoolCreatorAccounts<'a> {
    /// creator
    pub creator: &'a AccountView,
    /// new_creator
    pub new_creator: &'a AccountView,
    /// virtuals_mint
    pub virtuals_mint: &'a AccountView,
    /// token_mint
    pub token_mint: &'a AccountView,
    /// new_creator_virtuals_ata
    pub new_creator_virtuals_ata: &'a AccountView,
    /// new_creator_token_ata
    pub new_creator_token_ata: &'a AccountView,
    /// vpool
    pub vpool: &'a AccountView,
    /// token_program
    pub token_program: &'a AccountView,
    /// associated_token_program
    pub associated_token_program: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
}

impl<'a> UpdatePoolCreatorAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 10] {
        [
            InstructionAccount::writable_signer(self.creator.address()),
            InstructionAccount::readonly(self.new_creator.address()),
            InstructionAccount::readonly(self.virtuals_mint.address()),
            InstructionAccount::readonly(self.token_mint.address()),
            InstructionAccount::writable(self.new_creator_virtuals_ata.address()),
            InstructionAccount::writable(self.new_creator_token_ata.address()),
            InstructionAccount::readonly(self.vpool.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 10] {
        [
            self.creator, self.new_creator, self.virtuals_mint, self.token_mint, self.new_creator_virtuals_ata, self.new_creator_token_ata, self.vpool, self.token_program, self.associated_token_program, self.system_program
        ]
    }
}


// ============================================
// CPI Functions
// ============================================
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
    invoke_signed::<10>(&instruction, &account_views, signers)
}

/// CPI: claim_fees
#[inline(always)]
pub fn claim_fees<'a>(
    accounts: &ClaimFeesAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&CLAIM_FEES);
    
    
    // Build InstructionAccount array
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

/// CPI: create_meteora_pool
#[inline(always)]
pub fn create_meteora_pool<'a>(
    accounts: &CreateMeteoraPoolAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&CREATE_METEORA_POOL);
    
    
    // Build InstructionAccount array
    let instruction_accounts = accounts.to_instruction_accounts();
    
    // Build InstructionView
    let instruction = InstructionView {
        program_id: &ID,
        accounts: &instruction_accounts,
        data: &data,
    };
    
    // Execute CPI
    let account_views = accounts.to_views();
    invoke_signed::<36>(&instruction, &account_views, signers)
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
    invoke_signed::<9>(&instruction, &account_views, signers)
}

/// CPI: initialize_meteora_accounts
#[inline(always)]
pub fn initialize_meteora_accounts<'a>(
    accounts: &InitializeMeteoraAccountsAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&INITIALIZE_METEORA_ACCOUNTS);
    
    
    // Build InstructionAccount array
    let instruction_accounts = accounts.to_instruction_accounts();
    
    // Build InstructionView
    let instruction = InstructionView {
        program_id: &ID,
        accounts: &instruction_accounts,
        data: &data,
    };
    
    // Execute CPI
    let account_views = accounts.to_views();
    invoke_signed::<36>(&instruction, &account_views, signers)
}

/// CPI: launch
#[inline(always)]
pub fn launch<'a>(
    accounts: &LaunchAccounts<'a>, args: &LaunchArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<LaunchArgs>()];
    data[0..8].copy_from_slice(&LAUNCH);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const LaunchArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<LaunchArgs>(),
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
    invoke_signed::<10>(&instruction, &account_views, signers)
}

/// CPI: update_pool_creator
#[inline(always)]
pub fn update_pool_creator<'a>(
    accounts: &UpdatePoolCreatorAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&UPDATE_POOL_CREATOR);
    
    
    // Build InstructionAccount array
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

