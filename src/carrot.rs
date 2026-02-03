//! CPI module for Carrot
//!
//! Program: carrot
//! Program ID: CarrotwivhMpDnm27EHmRLeQ683Z1PufuqEmBZvD282s
//! Instructions: 42

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
pub const ID: Address = Address::new_from_array(five8_const::decode_32_const("CarrotwivhMpDnm27EHmRLeQ683Z1PufuqEmBZvD282s"));

// ============================================
// Instruction Discriminators
// ============================================
pub const INIT_VAULT: [u8; 8] = [77, 79, 85, 150, 33, 217, 52, 106];
pub const TRANSFER_VAULT_AUTHORITY: [u8; 8] = [139, 35, 83, 88, 52, 186, 162, 110];
pub const PAUSE_VAULT: [u8; 8] = [250, 6, 228, 57, 6, 104, 19, 210];
pub const MIGRATE_VAULT_ACCOUNT: [u8; 8] = [136, 92, 166, 245, 217, 170, 73, 108];
pub const DISTRIBUTE_VAULT_FEES: [u8; 8] = [158, 43, 226, 51, 80, 42, 40, 67];
pub const UPDATE_FEE: [u8; 8] = [232, 253, 195, 247, 148, 212, 73, 222];
pub const ISSUE: [u8; 8] = [190, 1, 98, 214, 81, 99, 222, 247];
pub const REDEEM: [u8; 8] = [184, 12, 86, 149, 70, 196, 97, 225];
pub const ADD_ASSET: [u8; 8] = [81, 53, 134, 142, 243, 73, 42, 179];
pub const UPDATE_ASSET: [u8; 8] = [56, 126, 238, 138, 192, 118, 228, 172];
pub const REMOVE_ASSET: [u8; 8] = [139, 243, 2, 142, 50, 197, 54, 181];
pub const REMOVE_STRATEGY: [u8; 8] = [185, 238, 33, 91, 134, 210, 97, 26];
pub const UPDATE_STRATEGY: [u8; 8] = [16, 76, 138, 179, 171, 112, 196, 21];
pub const MARGINFI_SUPPLY_STRATEGY_INIT: [u8; 8] = [177, 175, 81, 239, 57, 30, 251, 160];
pub const MARGINFI_SUPPLY_STRATEGY_DEPOSIT: [u8; 8] = [186, 12, 46, 101, 237, 152, 72, 252];
pub const MARGINFI_SUPPLY_STRATEGY_WITHDRAW: [u8; 8] = [217, 67, 169, 9, 14, 55, 71, 97];
pub const MARGINFI_SUPPLY_STRATEGY_CLAIM_EMISSIONS: [u8; 8] = [168, 249, 106, 188, 219, 36, 101, 132];
pub const KLEND_SUPPLY_STRATEGY_INIT: [u8; 8] = [158, 15, 27, 241, 53, 22, 117, 239];
pub const KLEND_SUPPLY_STRATEGY_DEPOSIT: [u8; 8] = [112, 30, 167, 72, 223, 248, 56, 192];
pub const KLEND_SUPPLY_STRATEGY_WITHDRAW: [u8; 8] = [198, 165, 26, 210, 209, 50, 56, 192];
pub const KLEND_SUPPLY_STRATEGY_CLAIM_EMISSIONS: [u8; 8] = [134, 218, 152, 220, 186, 129, 52, 175];
pub const SOLEND_SUPPLY_STRATEGY_INIT: [u8; 8] = [96, 112, 209, 132, 136, 45, 137, 33];
pub const SOLEND_SUPPLY_STRATEGY_DEPOSIT: [u8; 8] = [174, 98, 61, 32, 29, 132, 212, 159];
pub const SOLEND_SUPPLY_STRATEGY_WITHDRAW: [u8; 8] = [116, 25, 66, 127, 158, 245, 152, 244];
pub const MANGO_SUPPLY_STRATEGY_INIT: [u8; 8] = [171, 196, 79, 146, 67, 46, 23, 124];
pub const MANGO_SUPPLY_STRATEGY_DEPOSIT: [u8; 8] = [145, 134, 116, 16, 173, 216, 144, 179];
pub const MANGO_SUPPLY_STRATEGY_WITHDRAW: [u8; 8] = [246, 92, 37, 254, 180, 99, 171, 77];
pub const DRIFT_SUPPLY_STRATEGY_INIT: [u8; 8] = [113, 229, 159, 10, 39, 204, 102, 77];
pub const DRIFT_SUPPLY_STRATEGY_DEPOSIT: [u8; 8] = [223, 216, 94, 16, 168, 109, 107, 212];
pub const DRIFT_SUPPLY_STRATEGY_WITHDRAW: [u8; 8] = [227, 37, 134, 61, 133, 58, 72, 28];
pub const DRIFT_INSURANCE_FUND_STRATEGY_INIT: [u8; 8] = [134, 134, 2, 180, 247, 232, 205, 33];
pub const DRIFT_INSURANCE_FUND_STRATEGY_STAKE: [u8; 8] = [182, 76, 176, 27, 160, 47, 58, 179];
pub const DRIFT_INSURANCE_FUND_STRATEGY_UNSTAKE: [u8; 8] = [91, 150, 31, 34, 58, 35, 194, 57];
pub const DRIFT_INSURANCE_FUND_STRATEGY_WITHDRAW: [u8; 8] = [28, 209, 176, 53, 103, 64, 71, 173];
pub const CLEND_SUPPLY_STRATEGY_INIT: [u8; 8] = [223, 209, 52, 97, 27, 23, 98, 190];
pub const CLEND_SUPPLY_STRATEGY_DEPOSIT: [u8; 8] = [188, 224, 139, 151, 152, 156, 115, 243];
pub const CLEND_SUPPLY_STRATEGY_WITHDRAW: [u8; 8] = [219, 130, 60, 235, 151, 226, 46, 21];
pub const CHEST_STRATEGY_INIT: [u8; 8] = [47, 225, 103, 29, 28, 239, 246, 216];
pub const CHEST_STRATEGY_DEPOSIT: [u8; 8] = [67, 239, 123, 240, 182, 31, 21, 248];
pub const CHEST_STRATEGY_REQUEST_WITHDRAW: [u8; 8] = [3, 235, 227, 14, 76, 245, 237, 144];
pub const CHEST_STRATEGY_WITHDRAW: [u8; 8] = [225, 169, 142, 80, 186, 56, 87, 136];
pub const UPDATE_SWITCHBOARD_ORACLE_PRICE: [u8; 8] = [117, 128, 6, 136, 244, 23, 83, 10];

// ============================================
// Instruction Arguments
// ============================================
/// Arguments for `initVault`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct InitvaultArgs {
    pub args: [u8; 32],
}

/// Arguments for `pauseVault`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct PausevaultArgs {
    pub args: [u8; 32],
}

/// Arguments for `updateFee`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct UpdatefeeArgs {
    pub args: [u8; 32],
}

/// Arguments for `issue`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct IssueArgs {
    pub args: [u8; 32],
}

/// Arguments for `redeem`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct RedeemArgs {
    pub args: [u8; 32],
}

/// Arguments for `updateAsset`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct UpdateassetArgs {
    pub args: [u8; 32],
}

/// Arguments for `updateStrategy`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct UpdatestrategyArgs {
    pub args: [u8; 32],
}

/// Arguments for `marginfiSupplyStrategyInit`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct MarginfisupplystrategyinitArgs {
    pub args: [u8; 32],
}

/// Arguments for `marginfiSupplyStrategyDeposit`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct MarginfisupplystrategydepositArgs {
    pub args: [u8; 32],
}

/// Arguments for `marginfiSupplyStrategyWithdraw`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct MarginfisupplystrategywithdrawArgs {
    pub args: [u8; 32],
}

/// Arguments for `klendSupplyStrategyInit`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct KlendsupplystrategyinitArgs {
    pub args: [u8; 32],
}

/// Arguments for `klendSupplyStrategyDeposit`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct KlendsupplystrategydepositArgs {
    pub args: [u8; 32],
}

/// Arguments for `klendSupplyStrategyWithdraw`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct KlendsupplystrategywithdrawArgs {
    pub args: [u8; 32],
}

/// Arguments for `solendSupplyStrategyInit`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SolendsupplystrategyinitArgs {
    pub args: [u8; 32],
}

/// Arguments for `solendSupplyStrategyDeposit`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SolendsupplystrategydepositArgs {
    pub args: [u8; 32],
}

/// Arguments for `solendSupplyStrategyWithdraw`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SolendsupplystrategywithdrawArgs {
    pub args: [u8; 32],
}

/// Arguments for `mangoSupplyStrategyInit`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct MangosupplystrategyinitArgs {
    pub args: [u8; 32],
}

/// Arguments for `mangoSupplyStrategyDeposit`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct MangosupplystrategydepositArgs {
    pub args: [u8; 32],
}

/// Arguments for `mangoSupplyStrategyWithdraw`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct MangosupplystrategywithdrawArgs {
    pub args: [u8; 32],
}

/// Arguments for `driftSupplyStrategyInit`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct DriftsupplystrategyinitArgs {
    pub args: [u8; 32],
}

/// Arguments for `driftSupplyStrategyDeposit`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct DriftsupplystrategydepositArgs {
    pub args: [u8; 32],
}

/// Arguments for `driftSupplyStrategyWithdraw`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct DriftsupplystrategywithdrawArgs {
    pub args: [u8; 32],
}

/// Arguments for `driftInsuranceFundStrategyInit`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct DriftinsurancefundstrategyinitArgs {
    pub args: [u8; 32],
}

/// Arguments for `driftInsuranceFundStrategyStake`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct DriftinsurancefundstrategystakeArgs {
    pub args: [u8; 32],
}

/// Arguments for `driftInsuranceFundStrategyUnstake`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct DriftinsurancefundstrategyunstakeArgs {
    pub args: [u8; 32],
}

/// Arguments for `clendSupplyStrategyInit`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct ClendsupplystrategyinitArgs {
    pub args: [u8; 32],
}

/// Arguments for `clendSupplyStrategyDeposit`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct ClendsupplystrategydepositArgs {
    pub args: [u8; 32],
}

/// Arguments for `clendSupplyStrategyWithdraw`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct ClendsupplystrategywithdrawArgs {
    pub args: [u8; 32],
}

/// Arguments for `chestStrategyInit`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct CheststrategyinitArgs {
    pub args: [u8; 32],
}

/// Arguments for `chestStrategyDeposit`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct CheststrategydepositArgs {
    pub args: [u8; 32],
}

/// Arguments for `chestStrategyRequestWithdraw`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct CheststrategyrequestwithdrawArgs {
    pub args: [u8; 32],
}

/// Arguments for `chestStrategyWithdraw`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct CheststrategywithdrawArgs {
    pub args: [u8; 32],
}


// ============================================
// Instruction Accounts
// ============================================
/// Accounts for `initVault`
pub struct InitvaultAccounts<'a> {
    /// vault
    pub vault: &'a AccountView,
    /// shares
    pub shares: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
}

impl<'a> InitvaultAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 4] {
        [
            InstructionAccount::writable(self.vault.address()),
            InstructionAccount::readonly(self.shares.address()),
            InstructionAccount::writable_signer(self.authority.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 4] {
        [
            self.vault, self.shares, self.authority, self.system_program
        ]
    }
}

/// Accounts for `transferVaultAuthority`
pub struct TransfervaultauthorityAccounts<'a> {
    /// vault
    pub vault: &'a AccountView,
    /// newAuthority
    pub new_authority: &'a AccountView,
    /// oldAuthority
    pub old_authority: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
}

impl<'a> TransfervaultauthorityAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 4] {
        [
            InstructionAccount::writable(self.vault.address()),
            InstructionAccount::readonly(self.new_authority.address()),
            InstructionAccount::writable_signer(self.old_authority.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 4] {
        [
            self.vault, self.new_authority, self.old_authority, self.system_program
        ]
    }
}

/// Accounts for `pauseVault`
pub struct PausevaultAccounts<'a> {
    /// vault
    pub vault: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
}

impl<'a> PausevaultAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 2] {
        [
            InstructionAccount::writable(self.vault.address()),
            InstructionAccount::writable_signer(self.authority.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 2] {
        [
            self.vault, self.authority
        ]
    }
}

/// Accounts for `migrateVaultAccount`
pub struct MigratevaultaccountAccounts<'a> {
    /// vault
    pub vault: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
}

impl<'a> MigratevaultaccountAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 3] {
        [
            InstructionAccount::writable(self.vault.address()),
            InstructionAccount::writable_signer(self.authority.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 3] {
        [
            self.vault, self.authority, self.system_program
        ]
    }
}

/// Accounts for `distributeVaultFees`
pub struct DistributevaultfeesAccounts<'a> {
    /// vault
    pub vault: &'a AccountView,
    /// shares
    pub shares: &'a AccountView,
    /// sharesDestination
    pub shares_destination: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// logProgram
    pub log_program: &'a AccountView,
}

impl<'a> DistributevaultfeesAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 7] {
        [
            InstructionAccount::writable(self.vault.address()),
            InstructionAccount::writable(self.shares.address()),
            InstructionAccount::writable(self.shares_destination.address()),
            InstructionAccount::writable_signer(self.authority.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.log_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 7] {
        [
            self.vault, self.shares, self.shares_destination, self.authority, self.system_program, self.token_program, self.log_program
        ]
    }
}

/// Accounts for `updateFee`
pub struct UpdatefeeAccounts<'a> {
    /// vault
    pub vault: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
}

impl<'a> UpdatefeeAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 2] {
        [
            InstructionAccount::writable(self.vault.address()),
            InstructionAccount::writable_signer(self.authority.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 2] {
        [
            self.vault, self.authority
        ]
    }
}

/// Accounts for `issue`
pub struct IssueAccounts<'a> {
    /// vault
    pub vault: &'a AccountView,
    /// shares
    pub shares: &'a AccountView,
    /// userSharesAta
    pub user_shares_ata: &'a AccountView,
    /// asset
    pub asset: &'a AccountView,
    /// vaultAssetAta
    pub vault_asset_ata: &'a AccountView,
    /// userAssetAta
    pub user_asset_ata: &'a AccountView,
    /// user
    pub user: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
    /// assetTokenProgram
    pub asset_token_program: &'a AccountView,
    /// sharesTokenProgram
    pub shares_token_program: &'a AccountView,
    /// logProgram
    pub log_program: &'a AccountView,
}

impl<'a> IssueAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 11] {
        [
            InstructionAccount::writable(self.vault.address()),
            InstructionAccount::writable(self.shares.address()),
            InstructionAccount::writable(self.user_shares_ata.address()),
            InstructionAccount::readonly(self.asset.address()),
            InstructionAccount::writable(self.vault_asset_ata.address()),
            InstructionAccount::writable(self.user_asset_ata.address()),
            InstructionAccount::writable_signer(self.user.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.asset_token_program.address()),
            InstructionAccount::readonly(self.shares_token_program.address()),
            InstructionAccount::readonly(self.log_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 11] {
        [
            self.vault, self.shares, self.user_shares_ata, self.asset, self.vault_asset_ata, self.user_asset_ata, self.user, self.system_program, self.asset_token_program, self.shares_token_program, self.log_program
        ]
    }
}

/// Accounts for `redeem`
pub struct RedeemAccounts<'a> {
    /// vault
    pub vault: &'a AccountView,
    /// shares
    pub shares: &'a AccountView,
    /// userSharesAta
    pub user_shares_ata: &'a AccountView,
    /// asset
    pub asset: &'a AccountView,
    /// vaultAssetAta
    pub vault_asset_ata: &'a AccountView,
    /// userAssetAta
    pub user_asset_ata: &'a AccountView,
    /// user
    pub user: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
    /// assetTokenProgram
    pub asset_token_program: &'a AccountView,
    /// sharesTokenProgram
    pub shares_token_program: &'a AccountView,
    /// logProgram
    pub log_program: &'a AccountView,
}

impl<'a> RedeemAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 11] {
        [
            InstructionAccount::writable(self.vault.address()),
            InstructionAccount::writable(self.shares.address()),
            InstructionAccount::writable(self.user_shares_ata.address()),
            InstructionAccount::readonly(self.asset.address()),
            InstructionAccount::writable(self.vault_asset_ata.address()),
            InstructionAccount::writable(self.user_asset_ata.address()),
            InstructionAccount::writable_signer(self.user.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.asset_token_program.address()),
            InstructionAccount::readonly(self.shares_token_program.address()),
            InstructionAccount::readonly(self.log_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 11] {
        [
            self.vault, self.shares, self.user_shares_ata, self.asset, self.vault_asset_ata, self.user_asset_ata, self.user, self.system_program, self.asset_token_program, self.shares_token_program, self.log_program
        ]
    }
}

/// Accounts for `addAsset`
pub struct AddassetAccounts<'a> {
    /// vault
    pub vault: &'a AccountView,
    /// vaultAssetAta
    pub vault_asset_ata: &'a AccountView,
    /// assetMint
    pub asset_mint: &'a AccountView,
    /// assetOracle
    pub asset_oracle: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
}

impl<'a> AddassetAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 6] {
        [
            InstructionAccount::writable(self.vault.address()),
            InstructionAccount::writable(self.vault_asset_ata.address()),
            InstructionAccount::readonly(self.asset_mint.address()),
            InstructionAccount::readonly(self.asset_oracle.address()),
            InstructionAccount::writable_signer(self.authority.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 6] {
        [
            self.vault, self.vault_asset_ata, self.asset_mint, self.asset_oracle, self.authority, self.system_program
        ]
    }
}

/// Accounts for `updateAsset`
pub struct UpdateassetAccounts<'a> {
    /// vault
    pub vault: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
}

impl<'a> UpdateassetAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 2] {
        [
            InstructionAccount::writable(self.vault.address()),
            InstructionAccount::writable_signer(self.authority.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 2] {
        [
            self.vault, self.authority
        ]
    }
}

/// Accounts for `removeAsset`
pub struct RemoveassetAccounts<'a> {
    /// vault
    pub vault: &'a AccountView,
    /// asset
    pub asset: &'a AccountView,
    /// vaultAssetAta
    pub vault_asset_ata: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
}

impl<'a> RemoveassetAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 6] {
        [
            InstructionAccount::writable(self.vault.address()),
            InstructionAccount::readonly(self.asset.address()),
            InstructionAccount::writable(self.vault_asset_ata.address()),
            InstructionAccount::writable_signer(self.authority.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 6] {
        [
            self.vault, self.asset, self.vault_asset_ata, self.authority, self.system_program, self.token_program
        ]
    }
}

/// Accounts for `removeStrategy`
pub struct RemovestrategyAccounts<'a> {
    /// vault
    pub vault: &'a AccountView,
    /// strategy
    pub strategy: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
}

impl<'a> RemovestrategyAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 4] {
        [
            InstructionAccount::writable(self.vault.address()),
            InstructionAccount::writable(self.strategy.address()),
            InstructionAccount::writable_signer(self.authority.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 4] {
        [
            self.vault, self.strategy, self.authority, self.system_program
        ]
    }
}

/// Accounts for `updateStrategy`
pub struct UpdatestrategyAccounts<'a> {
    /// vault
    pub vault: &'a AccountView,
    /// strategy
    pub strategy: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
}

impl<'a> UpdatestrategyAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 4] {
        [
            InstructionAccount::readonly(self.vault.address()),
            InstructionAccount::writable(self.strategy.address()),
            InstructionAccount::writable_signer(self.authority.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 4] {
        [
            self.vault, self.strategy, self.authority, self.system_program
        ]
    }
}

/// Accounts for `marginfiSupplyStrategyInit`
pub struct MarginfisupplystrategyinitAccounts<'a> {
    /// vault
    pub vault: &'a AccountView,
    /// strategy
    pub strategy: &'a AccountView,
    /// assetMint
    pub asset_mint: &'a AccountView,
    /// marginfiGroup
    pub marginfi_group: &'a AccountView,
    /// marginfiAccount
    pub marginfi_account: &'a AccountView,
    /// marginfiBank
    pub marginfi_bank: &'a AccountView,
    /// marginfiBankLiquidityVault
    pub marginfi_bank_liquidity_vault: &'a AccountView,
    /// marginfiBankLiquidityVaultAuthority
    pub marginfi_bank_liquidity_vault_authority: &'a AccountView,
    /// marginfiOracle
    pub marginfi_oracle: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
    /// marginfiProgram
    pub marginfi_program: &'a AccountView,
}

impl<'a> MarginfisupplystrategyinitAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 12] {
        [
            InstructionAccount::writable(self.vault.address()),
            InstructionAccount::writable(self.strategy.address()),
            InstructionAccount::readonly(self.asset_mint.address()),
            InstructionAccount::readonly(self.marginfi_group.address()),
            InstructionAccount::writable_signer(self.marginfi_account.address()),
            InstructionAccount::readonly(self.marginfi_bank.address()),
            InstructionAccount::readonly(self.marginfi_bank_liquidity_vault.address()),
            InstructionAccount::readonly(self.marginfi_bank_liquidity_vault_authority.address()),
            InstructionAccount::readonly(self.marginfi_oracle.address()),
            InstructionAccount::writable_signer(self.authority.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.marginfi_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 12] {
        [
            self.vault, self.strategy, self.asset_mint, self.marginfi_group, self.marginfi_account, self.marginfi_bank, self.marginfi_bank_liquidity_vault, self.marginfi_bank_liquidity_vault_authority, self.marginfi_oracle, self.authority, self.system_program, self.marginfi_program
        ]
    }
}

/// Accounts for `marginfiSupplyStrategyDeposit`
pub struct MarginfisupplystrategydepositAccounts<'a> {
    /// vault
    pub vault: &'a AccountView,
    /// assetMint
    pub asset_mint: &'a AccountView,
    /// vaultAssetAta
    pub vault_asset_ata: &'a AccountView,
    /// strategy
    pub strategy: &'a AccountView,
    /// marginfiGroup
    pub marginfi_group: &'a AccountView,
    /// marginfiAccount
    pub marginfi_account: &'a AccountView,
    /// marginfiBank
    pub marginfi_bank: &'a AccountView,
    /// marginfiBankLiquidityVault
    pub marginfi_bank_liquidity_vault: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// marginfiProgram
    pub marginfi_program: &'a AccountView,
    /// logProgram
    pub log_program: &'a AccountView,
}

impl<'a> MarginfisupplystrategydepositAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 13] {
        [
            InstructionAccount::writable(self.vault.address()),
            InstructionAccount::readonly(self.asset_mint.address()),
            InstructionAccount::writable(self.vault_asset_ata.address()),
            InstructionAccount::readonly(self.strategy.address()),
            InstructionAccount::readonly(self.marginfi_group.address()),
            InstructionAccount::writable(self.marginfi_account.address()),
            InstructionAccount::writable(self.marginfi_bank.address()),
            InstructionAccount::writable(self.marginfi_bank_liquidity_vault.address()),
            InstructionAccount::writable_signer(self.authority.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.marginfi_program.address()),
            InstructionAccount::readonly(self.log_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 13] {
        [
            self.vault, self.asset_mint, self.vault_asset_ata, self.strategy, self.marginfi_group, self.marginfi_account, self.marginfi_bank, self.marginfi_bank_liquidity_vault, self.authority, self.system_program, self.token_program, self.marginfi_program, self.log_program
        ]
    }
}

/// Accounts for `marginfiSupplyStrategyWithdraw`
pub struct MarginfisupplystrategywithdrawAccounts<'a> {
    /// vault
    pub vault: &'a AccountView,
    /// assetMint
    pub asset_mint: &'a AccountView,
    /// vaultAssetAta
    pub vault_asset_ata: &'a AccountView,
    /// strategy
    pub strategy: &'a AccountView,
    /// marginfiGroup
    pub marginfi_group: &'a AccountView,
    /// marginfiAccount
    pub marginfi_account: &'a AccountView,
    /// marginfiBank
    pub marginfi_bank: &'a AccountView,
    /// marginfiBankLiquidityVault
    pub marginfi_bank_liquidity_vault: &'a AccountView,
    /// marginfiBankLiquidityVaultAuthority
    pub marginfi_bank_liquidity_vault_authority: &'a AccountView,
    /// marginfiOracle
    pub marginfi_oracle: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// marginfiProgram
    pub marginfi_program: &'a AccountView,
    /// logProgram
    pub log_program: &'a AccountView,
}

impl<'a> MarginfisupplystrategywithdrawAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 15] {
        [
            InstructionAccount::writable(self.vault.address()),
            InstructionAccount::readonly(self.asset_mint.address()),
            InstructionAccount::writable(self.vault_asset_ata.address()),
            InstructionAccount::readonly(self.strategy.address()),
            InstructionAccount::readonly(self.marginfi_group.address()),
            InstructionAccount::writable(self.marginfi_account.address()),
            InstructionAccount::writable(self.marginfi_bank.address()),
            InstructionAccount::writable(self.marginfi_bank_liquidity_vault.address()),
            InstructionAccount::writable(self.marginfi_bank_liquidity_vault_authority.address()),
            InstructionAccount::readonly(self.marginfi_oracle.address()),
            InstructionAccount::writable_signer(self.authority.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.marginfi_program.address()),
            InstructionAccount::readonly(self.log_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 15] {
        [
            self.vault, self.asset_mint, self.vault_asset_ata, self.strategy, self.marginfi_group, self.marginfi_account, self.marginfi_bank, self.marginfi_bank_liquidity_vault, self.marginfi_bank_liquidity_vault_authority, self.marginfi_oracle, self.authority, self.system_program, self.token_program, self.marginfi_program, self.log_program
        ]
    }
}

/// Accounts for `marginfiSupplyStrategyClaimEmissions`
pub struct MarginfisupplystrategyclaimemissionsAccounts<'a> {
    /// vault
    pub vault: &'a AccountView,
    /// assetMint
    pub asset_mint: &'a AccountView,
    /// vaultAssetAta
    pub vault_asset_ata: &'a AccountView,
    /// strategy
    pub strategy: &'a AccountView,
    /// marginfiGroup
    pub marginfi_group: &'a AccountView,
    /// marginfiAccount
    pub marginfi_account: &'a AccountView,
    /// marginfiBank
    pub marginfi_bank: &'a AccountView,
    /// marginfiEmissionsVault
    pub marginfi_emissions_vault: &'a AccountView,
    /// marginfiEmissionsAuth
    pub marginfi_emissions_auth: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// marginfiProgram
    pub marginfi_program: &'a AccountView,
    /// logProgram
    pub log_program: &'a AccountView,
}

impl<'a> MarginfisupplystrategyclaimemissionsAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 13] {
        [
            InstructionAccount::readonly(self.vault.address()),
            InstructionAccount::readonly(self.asset_mint.address()),
            InstructionAccount::writable(self.vault_asset_ata.address()),
            InstructionAccount::readonly(self.strategy.address()),
            InstructionAccount::readonly(self.marginfi_group.address()),
            InstructionAccount::writable(self.marginfi_account.address()),
            InstructionAccount::writable(self.marginfi_bank.address()),
            InstructionAccount::writable(self.marginfi_emissions_vault.address()),
            InstructionAccount::readonly(self.marginfi_emissions_auth.address()),
            InstructionAccount::writable_signer(self.authority.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.marginfi_program.address()),
            InstructionAccount::readonly(self.log_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 13] {
        [
            self.vault, self.asset_mint, self.vault_asset_ata, self.strategy, self.marginfi_group, self.marginfi_account, self.marginfi_bank, self.marginfi_emissions_vault, self.marginfi_emissions_auth, self.authority, self.token_program, self.marginfi_program, self.log_program
        ]
    }
}

/// Accounts for `klendSupplyStrategyInit`
pub struct KlendsupplystrategyinitAccounts<'a> {
    /// vault
    pub vault: &'a AccountView,
    /// strategy
    pub strategy: &'a AccountView,
    /// assetMint
    pub asset_mint: &'a AccountView,
    /// assetPythOracle
    pub asset_pyth_oracle: &'a AccountView,
    /// scopePrices
    pub scope_prices: &'a AccountView,
    /// userMetadata
    pub user_metadata: &'a AccountView,
    /// obligation
    pub obligation: &'a AccountView,
    /// obligationFarm
    pub obligation_farm: &'a AccountView,
    /// lendingMarket
    pub lending_market: &'a AccountView,
    /// lendingMarketAuthority
    pub lending_market_authority: &'a AccountView,
    /// reserve
    pub reserve: &'a AccountView,
    /// reserveCollateralMint
    pub reserve_collateral_mint: &'a AccountView,
    /// reserveLiquiditySupply
    pub reserve_liquidity_supply: &'a AccountView,
    /// reserveDestinationDepositCollateral
    pub reserve_destination_deposit_collateral: &'a AccountView,
    /// reserveFarmState
    pub reserve_farm_state: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
    /// rent
    pub rent: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// klendProgram
    pub klend_program: &'a AccountView,
    /// kfarmsProgram
    pub kfarms_program: &'a AccountView,
}

impl<'a> KlendsupplystrategyinitAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 21] {
        [
            InstructionAccount::writable(self.vault.address()),
            InstructionAccount::writable(self.strategy.address()),
            InstructionAccount::readonly(self.asset_mint.address()),
            InstructionAccount::writable(self.asset_pyth_oracle.address()),
            InstructionAccount::writable(self.scope_prices.address()),
            InstructionAccount::writable(self.user_metadata.address()),
            InstructionAccount::writable(self.obligation.address()),
            InstructionAccount::writable(self.obligation_farm.address()),
            InstructionAccount::writable(self.lending_market.address()),
            InstructionAccount::writable(self.lending_market_authority.address()),
            InstructionAccount::writable(self.reserve.address()),
            InstructionAccount::readonly(self.reserve_collateral_mint.address()),
            InstructionAccount::readonly(self.reserve_liquidity_supply.address()),
            InstructionAccount::readonly(self.reserve_destination_deposit_collateral.address()),
            InstructionAccount::writable(self.reserve_farm_state.address()),
            InstructionAccount::writable_signer(self.authority.address()),
            InstructionAccount::readonly(self.rent.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.klend_program.address()),
            InstructionAccount::readonly(self.kfarms_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 21] {
        [
            self.vault, self.strategy, self.asset_mint, self.asset_pyth_oracle, self.scope_prices, self.user_metadata, self.obligation, self.obligation_farm, self.lending_market, self.lending_market_authority, self.reserve, self.reserve_collateral_mint, self.reserve_liquidity_supply, self.reserve_destination_deposit_collateral, self.reserve_farm_state, self.authority, self.rent, self.system_program, self.token_program, self.klend_program, self.kfarms_program
        ]
    }
}

/// Accounts for `klendSupplyStrategyDeposit`
pub struct KlendsupplystrategydepositAccounts<'a> {
    /// vault
    pub vault: &'a AccountView,
    /// assetMint
    pub asset_mint: &'a AccountView,
    /// vaultAssetAta
    pub vault_asset_ata: &'a AccountView,
    /// strategy
    pub strategy: &'a AccountView,
    /// reserve
    pub reserve: &'a AccountView,
    /// reserveLiquiditySupply
    pub reserve_liquidity_supply: &'a AccountView,
    /// reserveCollateralMint
    pub reserve_collateral_mint: &'a AccountView,
    /// reserveDestinationDepositCollateral
    pub reserve_destination_deposit_collateral: &'a AccountView,
    /// vaultCollateralAta
    pub vault_collateral_ata: &'a AccountView,
    /// lendingMarket
    pub lending_market: &'a AccountView,
    /// lendingMarketAuthority
    pub lending_market_authority: &'a AccountView,
    /// oracle
    pub oracle: &'a AccountView,
    /// scopePrices
    pub scope_prices: &'a AccountView,
    /// obligation
    pub obligation: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
    /// rent
    pub rent: &'a AccountView,
    /// instructionsSysvar
    pub instructions_sysvar: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
    /// collateralTokenProgram
    pub collateral_token_program: &'a AccountView,
    /// assetTokenProgram
    pub asset_token_program: &'a AccountView,
    /// klendProgram
    pub klend_program: &'a AccountView,
    /// kfarmsProgram
    pub kfarms_program: &'a AccountView,
    /// logProgram
    pub log_program: &'a AccountView,
}

impl<'a> KlendsupplystrategydepositAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 23] {
        [
            InstructionAccount::writable(self.vault.address()),
            InstructionAccount::writable(self.asset_mint.address()),
            InstructionAccount::writable(self.vault_asset_ata.address()),
            InstructionAccount::readonly(self.strategy.address()),
            InstructionAccount::writable(self.reserve.address()),
            InstructionAccount::writable(self.reserve_liquidity_supply.address()),
            InstructionAccount::writable(self.reserve_collateral_mint.address()),
            InstructionAccount::writable(self.reserve_destination_deposit_collateral.address()),
            InstructionAccount::writable(self.vault_collateral_ata.address()),
            InstructionAccount::readonly(self.lending_market.address()),
            InstructionAccount::writable(self.lending_market_authority.address()),
            InstructionAccount::readonly(self.oracle.address()),
            InstructionAccount::readonly(self.scope_prices.address()),
            InstructionAccount::writable(self.obligation.address()),
            InstructionAccount::writable_signer(self.authority.address()),
            InstructionAccount::readonly(self.rent.address()),
            InstructionAccount::readonly(self.instructions_sysvar.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.collateral_token_program.address()),
            InstructionAccount::readonly(self.asset_token_program.address()),
            InstructionAccount::readonly(self.klend_program.address()),
            InstructionAccount::readonly(self.kfarms_program.address()),
            InstructionAccount::readonly(self.log_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 23] {
        [
            self.vault, self.asset_mint, self.vault_asset_ata, self.strategy, self.reserve, self.reserve_liquidity_supply, self.reserve_collateral_mint, self.reserve_destination_deposit_collateral, self.vault_collateral_ata, self.lending_market, self.lending_market_authority, self.oracle, self.scope_prices, self.obligation, self.authority, self.rent, self.instructions_sysvar, self.system_program, self.collateral_token_program, self.asset_token_program, self.klend_program, self.kfarms_program, self.log_program
        ]
    }
}

/// Accounts for `klendSupplyStrategyWithdraw`
pub struct KlendsupplystrategywithdrawAccounts<'a> {
    /// vault
    pub vault: &'a AccountView,
    /// assetMint
    pub asset_mint: &'a AccountView,
    /// vaultAssetAta
    pub vault_asset_ata: &'a AccountView,
    /// strategy
    pub strategy: &'a AccountView,
    /// reserve
    pub reserve: &'a AccountView,
    /// reserveLiquiditySupply
    pub reserve_liquidity_supply: &'a AccountView,
    /// reserveCollateralMint
    pub reserve_collateral_mint: &'a AccountView,
    /// reserveDestinationDepositCollateral
    pub reserve_destination_deposit_collateral: &'a AccountView,
    /// vaultCollateralAta
    pub vault_collateral_ata: &'a AccountView,
    /// lendingMarket
    pub lending_market: &'a AccountView,
    /// lendingMarketAuthority
    pub lending_market_authority: &'a AccountView,
    /// oracle
    pub oracle: &'a AccountView,
    /// scopePrices
    pub scope_prices: &'a AccountView,
    /// obligation
    pub obligation: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
    /// rent
    pub rent: &'a AccountView,
    /// instructionsSysvar
    pub instructions_sysvar: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
    /// collateralTokenProgram
    pub collateral_token_program: &'a AccountView,
    /// assetTokenProgram
    pub asset_token_program: &'a AccountView,
    /// klendProgram
    pub klend_program: &'a AccountView,
    /// kfarmsProgram
    pub kfarms_program: &'a AccountView,
    /// logProgram
    pub log_program: &'a AccountView,
}

impl<'a> KlendsupplystrategywithdrawAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 23] {
        [
            InstructionAccount::writable(self.vault.address()),
            InstructionAccount::writable(self.asset_mint.address()),
            InstructionAccount::writable(self.vault_asset_ata.address()),
            InstructionAccount::readonly(self.strategy.address()),
            InstructionAccount::writable(self.reserve.address()),
            InstructionAccount::writable(self.reserve_liquidity_supply.address()),
            InstructionAccount::writable(self.reserve_collateral_mint.address()),
            InstructionAccount::writable(self.reserve_destination_deposit_collateral.address()),
            InstructionAccount::writable(self.vault_collateral_ata.address()),
            InstructionAccount::readonly(self.lending_market.address()),
            InstructionAccount::writable(self.lending_market_authority.address()),
            InstructionAccount::readonly(self.oracle.address()),
            InstructionAccount::readonly(self.scope_prices.address()),
            InstructionAccount::writable(self.obligation.address()),
            InstructionAccount::writable_signer(self.authority.address()),
            InstructionAccount::readonly(self.rent.address()),
            InstructionAccount::readonly(self.instructions_sysvar.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.collateral_token_program.address()),
            InstructionAccount::readonly(self.asset_token_program.address()),
            InstructionAccount::readonly(self.klend_program.address()),
            InstructionAccount::readonly(self.kfarms_program.address()),
            InstructionAccount::readonly(self.log_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 23] {
        [
            self.vault, self.asset_mint, self.vault_asset_ata, self.strategy, self.reserve, self.reserve_liquidity_supply, self.reserve_collateral_mint, self.reserve_destination_deposit_collateral, self.vault_collateral_ata, self.lending_market, self.lending_market_authority, self.oracle, self.scope_prices, self.obligation, self.authority, self.rent, self.instructions_sysvar, self.system_program, self.collateral_token_program, self.asset_token_program, self.klend_program, self.kfarms_program, self.log_program
        ]
    }
}

/// Accounts for `klendSupplyStrategyClaimEmissions`
pub struct KlendsupplystrategyclaimemissionsAccounts<'a> {
    /// vault
    pub vault: &'a AccountView,
    /// assetMint
    pub asset_mint: &'a AccountView,
    /// vaultAssetAta
    pub vault_asset_ata: &'a AccountView,
    /// strategy
    pub strategy: &'a AccountView,
    /// scopePrices
    pub scope_prices: &'a AccountView,
    /// userState
    pub user_state: &'a AccountView,
    /// farmState
    pub farm_state: &'a AccountView,
    /// kfarmsGlobalConfig
    pub kfarms_global_config: &'a AccountView,
    /// kfarmsVaultsAuthority
    pub kfarms_vaults_authority: &'a AccountView,
    /// kfarmsRewardsVault
    pub kfarms_rewards_vault: &'a AccountView,
    /// kfarmsRewardsTreasuryVault
    pub kfarms_rewards_treasury_vault: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// kfarmsProgram
    pub kfarms_program: &'a AccountView,
    /// logProgram
    pub log_program: &'a AccountView,
}

impl<'a> KlendsupplystrategyclaimemissionsAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 15] {
        [
            InstructionAccount::writable(self.vault.address()),
            InstructionAccount::readonly(self.asset_mint.address()),
            InstructionAccount::writable(self.vault_asset_ata.address()),
            InstructionAccount::readonly(self.strategy.address()),
            InstructionAccount::readonly(self.scope_prices.address()),
            InstructionAccount::writable(self.user_state.address()),
            InstructionAccount::writable(self.farm_state.address()),
            InstructionAccount::readonly(self.kfarms_global_config.address()),
            InstructionAccount::readonly(self.kfarms_vaults_authority.address()),
            InstructionAccount::writable(self.kfarms_rewards_vault.address()),
            InstructionAccount::writable(self.kfarms_rewards_treasury_vault.address()),
            InstructionAccount::writable_signer(self.authority.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.kfarms_program.address()),
            InstructionAccount::readonly(self.log_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 15] {
        [
            self.vault, self.asset_mint, self.vault_asset_ata, self.strategy, self.scope_prices, self.user_state, self.farm_state, self.kfarms_global_config, self.kfarms_vaults_authority, self.kfarms_rewards_vault, self.kfarms_rewards_treasury_vault, self.authority, self.token_program, self.kfarms_program, self.log_program
        ]
    }
}

/// Accounts for `solendSupplyStrategyInit`
pub struct SolendsupplystrategyinitAccounts<'a> {
    /// vault
    pub vault: &'a AccountView,
    /// strategy
    pub strategy: &'a AccountView,
    /// assetMint
    pub asset_mint: &'a AccountView,
    /// assetPythOracle
    pub asset_pyth_oracle: &'a AccountView,
    /// assetSwitchboardOracle
    pub asset_switchboard_oracle: &'a AccountView,
    /// obligation
    pub obligation: &'a AccountView,
    /// lendingMarket
    pub lending_market: &'a AccountView,
    /// lendingMarketAuthority
    pub lending_market_authority: &'a AccountView,
    /// reserve
    pub reserve: &'a AccountView,
    /// reserveCollateralMint
    pub reserve_collateral_mint: &'a AccountView,
    /// reserveLiquiditySupply
    pub reserve_liquidity_supply: &'a AccountView,
    /// reserveDepositCollateralAta
    pub reserve_deposit_collateral_ata: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
    /// rent
    pub rent: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// solendProgram
    pub solend_program: &'a AccountView,
}

impl<'a> SolendsupplystrategyinitAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 17] {
        [
            InstructionAccount::writable(self.vault.address()),
            InstructionAccount::writable(self.strategy.address()),
            InstructionAccount::readonly(self.asset_mint.address()),
            InstructionAccount::readonly(self.asset_pyth_oracle.address()),
            InstructionAccount::readonly(self.asset_switchboard_oracle.address()),
            InstructionAccount::writable(self.obligation.address()),
            InstructionAccount::readonly(self.lending_market.address()),
            InstructionAccount::readonly(self.lending_market_authority.address()),
            InstructionAccount::readonly(self.reserve.address()),
            InstructionAccount::readonly(self.reserve_collateral_mint.address()),
            InstructionAccount::readonly(self.reserve_liquidity_supply.address()),
            InstructionAccount::readonly(self.reserve_deposit_collateral_ata.address()),
            InstructionAccount::writable_signer(self.authority.address()),
            InstructionAccount::readonly(self.rent.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.solend_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 17] {
        [
            self.vault, self.strategy, self.asset_mint, self.asset_pyth_oracle, self.asset_switchboard_oracle, self.obligation, self.lending_market, self.lending_market_authority, self.reserve, self.reserve_collateral_mint, self.reserve_liquidity_supply, self.reserve_deposit_collateral_ata, self.authority, self.rent, self.system_program, self.token_program, self.solend_program
        ]
    }
}

/// Accounts for `solendSupplyStrategyDeposit`
pub struct SolendsupplystrategydepositAccounts<'a> {
    /// vault
    pub vault: &'a AccountView,
    /// assetMint
    pub asset_mint: &'a AccountView,
    /// vaultAssetAta
    pub vault_asset_ata: &'a AccountView,
    /// strategy
    pub strategy: &'a AccountView,
    /// assetPythOracle
    pub asset_pyth_oracle: &'a AccountView,
    /// assetSwitchboardOracle
    pub asset_switchboard_oracle: &'a AccountView,
    /// obligation
    pub obligation: &'a AccountView,
    /// lendingMarket
    pub lending_market: &'a AccountView,
    /// lendingMarketAuthority
    pub lending_market_authority: &'a AccountView,
    /// reserve
    pub reserve: &'a AccountView,
    /// reserveLiquiditySupply
    pub reserve_liquidity_supply: &'a AccountView,
    /// reserveCollateralMint
    pub reserve_collateral_mint: &'a AccountView,
    /// vaultCollateralAta
    pub vault_collateral_ata: &'a AccountView,
    /// depositCollateralAta
    pub deposit_collateral_ata: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
    /// rent
    pub rent: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// solendProgram
    pub solend_program: &'a AccountView,
    /// logProgram
    pub log_program: &'a AccountView,
}

impl<'a> SolendsupplystrategydepositAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 20] {
        [
            InstructionAccount::writable(self.vault.address()),
            InstructionAccount::readonly(self.asset_mint.address()),
            InstructionAccount::writable(self.vault_asset_ata.address()),
            InstructionAccount::readonly(self.strategy.address()),
            InstructionAccount::readonly(self.asset_pyth_oracle.address()),
            InstructionAccount::readonly(self.asset_switchboard_oracle.address()),
            InstructionAccount::writable(self.obligation.address()),
            InstructionAccount::writable(self.lending_market.address()),
            InstructionAccount::readonly(self.lending_market_authority.address()),
            InstructionAccount::writable(self.reserve.address()),
            InstructionAccount::writable(self.reserve_liquidity_supply.address()),
            InstructionAccount::writable(self.reserve_collateral_mint.address()),
            InstructionAccount::writable(self.vault_collateral_ata.address()),
            InstructionAccount::writable(self.deposit_collateral_ata.address()),
            InstructionAccount::writable_signer(self.authority.address()),
            InstructionAccount::readonly(self.rent.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.solend_program.address()),
            InstructionAccount::readonly(self.log_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 20] {
        [
            self.vault, self.asset_mint, self.vault_asset_ata, self.strategy, self.asset_pyth_oracle, self.asset_switchboard_oracle, self.obligation, self.lending_market, self.lending_market_authority, self.reserve, self.reserve_liquidity_supply, self.reserve_collateral_mint, self.vault_collateral_ata, self.deposit_collateral_ata, self.authority, self.rent, self.system_program, self.token_program, self.solend_program, self.log_program
        ]
    }
}

/// Accounts for `solendSupplyStrategyWithdraw`
pub struct SolendsupplystrategywithdrawAccounts<'a> {
    /// vault
    pub vault: &'a AccountView,
    /// assetMint
    pub asset_mint: &'a AccountView,
    /// vaultAssetAta
    pub vault_asset_ata: &'a AccountView,
    /// strategy
    pub strategy: &'a AccountView,
    /// assetPythOracle
    pub asset_pyth_oracle: &'a AccountView,
    /// assetSwitchboardOracle
    pub asset_switchboard_oracle: &'a AccountView,
    /// obligation
    pub obligation: &'a AccountView,
    /// lendingMarket
    pub lending_market: &'a AccountView,
    /// lendingMarketAuthority
    pub lending_market_authority: &'a AccountView,
    /// reserve
    pub reserve: &'a AccountView,
    /// reserveLiquiditySupply
    pub reserve_liquidity_supply: &'a AccountView,
    /// reserveCollateralMint
    pub reserve_collateral_mint: &'a AccountView,
    /// vaultCollateralAta
    pub vault_collateral_ata: &'a AccountView,
    /// withdrawCollateralAta
    pub withdraw_collateral_ata: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
    /// rent
    pub rent: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// solendProgram
    pub solend_program: &'a AccountView,
    /// logProgram
    pub log_program: &'a AccountView,
}

impl<'a> SolendsupplystrategywithdrawAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 20] {
        [
            InstructionAccount::writable(self.vault.address()),
            InstructionAccount::readonly(self.asset_mint.address()),
            InstructionAccount::writable(self.vault_asset_ata.address()),
            InstructionAccount::readonly(self.strategy.address()),
            InstructionAccount::readonly(self.asset_pyth_oracle.address()),
            InstructionAccount::readonly(self.asset_switchboard_oracle.address()),
            InstructionAccount::writable(self.obligation.address()),
            InstructionAccount::writable(self.lending_market.address()),
            InstructionAccount::readonly(self.lending_market_authority.address()),
            InstructionAccount::writable(self.reserve.address()),
            InstructionAccount::writable(self.reserve_liquidity_supply.address()),
            InstructionAccount::writable(self.reserve_collateral_mint.address()),
            InstructionAccount::writable(self.vault_collateral_ata.address()),
            InstructionAccount::writable(self.withdraw_collateral_ata.address()),
            InstructionAccount::writable_signer(self.authority.address()),
            InstructionAccount::readonly(self.rent.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.solend_program.address()),
            InstructionAccount::readonly(self.log_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 20] {
        [
            self.vault, self.asset_mint, self.vault_asset_ata, self.strategy, self.asset_pyth_oracle, self.asset_switchboard_oracle, self.obligation, self.lending_market, self.lending_market_authority, self.reserve, self.reserve_liquidity_supply, self.reserve_collateral_mint, self.vault_collateral_ata, self.withdraw_collateral_ata, self.authority, self.rent, self.system_program, self.token_program, self.solend_program, self.log_program
        ]
    }
}

/// Accounts for `mangoSupplyStrategyInit`
pub struct MangosupplystrategyinitAccounts<'a> {
    /// vault
    pub vault: &'a AccountView,
    /// strategy
    pub strategy: &'a AccountView,
    /// assetMint
    pub asset_mint: &'a AccountView,
    /// mangoGroup
    pub mango_group: &'a AccountView,
    /// mangoAccount
    pub mango_account: &'a AccountView,
    /// mangoBank
    pub mango_bank: &'a AccountView,
    /// mangoVault
    pub mango_vault: &'a AccountView,
    /// pythOracle
    pub pyth_oracle: &'a AccountView,
    /// switchboardOracle
    pub switchboard_oracle: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
    /// mangoProgram
    pub mango_program: &'a AccountView,
}

impl<'a> MangosupplystrategyinitAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 12] {
        [
            InstructionAccount::writable(self.vault.address()),
            InstructionAccount::writable(self.strategy.address()),
            InstructionAccount::readonly(self.asset_mint.address()),
            InstructionAccount::readonly(self.mango_group.address()),
            InstructionAccount::writable(self.mango_account.address()),
            InstructionAccount::readonly(self.mango_bank.address()),
            InstructionAccount::readonly(self.mango_vault.address()),
            InstructionAccount::readonly(self.pyth_oracle.address()),
            InstructionAccount::readonly(self.switchboard_oracle.address()),
            InstructionAccount::writable_signer(self.authority.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.mango_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 12] {
        [
            self.vault, self.strategy, self.asset_mint, self.mango_group, self.mango_account, self.mango_bank, self.mango_vault, self.pyth_oracle, self.switchboard_oracle, self.authority, self.system_program, self.mango_program
        ]
    }
}

/// Accounts for `mangoSupplyStrategyDeposit`
pub struct MangosupplystrategydepositAccounts<'a> {
    /// vault
    pub vault: &'a AccountView,
    /// assetMint
    pub asset_mint: &'a AccountView,
    /// vaultAssetAta
    pub vault_asset_ata: &'a AccountView,
    /// strategy
    pub strategy: &'a AccountView,
    /// mangoGroup
    pub mango_group: &'a AccountView,
    /// mangoAccount
    pub mango_account: &'a AccountView,
    /// mangoBank
    pub mango_bank: &'a AccountView,
    /// mangoVault
    pub mango_vault: &'a AccountView,
    /// pythOracle
    pub pyth_oracle: &'a AccountView,
    /// switchboardOracle
    pub switchboard_oracle: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// mangoProgram
    pub mango_program: &'a AccountView,
    /// logProgram
    pub log_program: &'a AccountView,
}

impl<'a> MangosupplystrategydepositAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 15] {
        [
            InstructionAccount::writable(self.vault.address()),
            InstructionAccount::readonly(self.asset_mint.address()),
            InstructionAccount::writable(self.vault_asset_ata.address()),
            InstructionAccount::readonly(self.strategy.address()),
            InstructionAccount::readonly(self.mango_group.address()),
            InstructionAccount::writable(self.mango_account.address()),
            InstructionAccount::writable(self.mango_bank.address()),
            InstructionAccount::writable(self.mango_vault.address()),
            InstructionAccount::readonly(self.pyth_oracle.address()),
            InstructionAccount::readonly(self.switchboard_oracle.address()),
            InstructionAccount::writable_signer(self.authority.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.mango_program.address()),
            InstructionAccount::readonly(self.log_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 15] {
        [
            self.vault, self.asset_mint, self.vault_asset_ata, self.strategy, self.mango_group, self.mango_account, self.mango_bank, self.mango_vault, self.pyth_oracle, self.switchboard_oracle, self.authority, self.system_program, self.token_program, self.mango_program, self.log_program
        ]
    }
}

/// Accounts for `mangoSupplyStrategyWithdraw`
pub struct MangosupplystrategywithdrawAccounts<'a> {
    /// vault
    pub vault: &'a AccountView,
    /// assetMint
    pub asset_mint: &'a AccountView,
    /// vaultAssetAta
    pub vault_asset_ata: &'a AccountView,
    /// strategy
    pub strategy: &'a AccountView,
    /// mangoGroup
    pub mango_group: &'a AccountView,
    /// mangoAccount
    pub mango_account: &'a AccountView,
    /// mangoBank
    pub mango_bank: &'a AccountView,
    /// mangoVault
    pub mango_vault: &'a AccountView,
    /// pythOracle
    pub pyth_oracle: &'a AccountView,
    /// switchboardOracle
    pub switchboard_oracle: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// mangoProgram
    pub mango_program: &'a AccountView,
    /// logProgram
    pub log_program: &'a AccountView,
}

impl<'a> MangosupplystrategywithdrawAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 15] {
        [
            InstructionAccount::writable(self.vault.address()),
            InstructionAccount::readonly(self.asset_mint.address()),
            InstructionAccount::writable(self.vault_asset_ata.address()),
            InstructionAccount::readonly(self.strategy.address()),
            InstructionAccount::readonly(self.mango_group.address()),
            InstructionAccount::writable(self.mango_account.address()),
            InstructionAccount::writable(self.mango_bank.address()),
            InstructionAccount::writable(self.mango_vault.address()),
            InstructionAccount::readonly(self.pyth_oracle.address()),
            InstructionAccount::readonly(self.switchboard_oracle.address()),
            InstructionAccount::writable_signer(self.authority.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.mango_program.address()),
            InstructionAccount::readonly(self.log_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 15] {
        [
            self.vault, self.asset_mint, self.vault_asset_ata, self.strategy, self.mango_group, self.mango_account, self.mango_bank, self.mango_vault, self.pyth_oracle, self.switchboard_oracle, self.authority, self.system_program, self.token_program, self.mango_program, self.log_program
        ]
    }
}

/// Accounts for `driftSupplyStrategyInit`
pub struct DriftsupplystrategyinitAccounts<'a> {
    /// vault
    pub vault: &'a AccountView,
    /// strategy
    pub strategy: &'a AccountView,
    /// assetMint
    pub asset_mint: &'a AccountView,
    /// driftUser
    pub drift_user: &'a AccountView,
    /// driftUserStats
    pub drift_user_stats: &'a AccountView,
    /// driftState
    pub drift_state: &'a AccountView,
    /// driftSigner
    pub drift_signer: &'a AccountView,
    /// driftSpotMarket
    pub drift_spot_market: &'a AccountView,
    /// driftPerpMarket
    pub drift_perp_market: &'a AccountView,
    /// driftSpotMarketVault
    pub drift_spot_market_vault: &'a AccountView,
    /// spotPythOracle
    pub spot_pyth_oracle: &'a AccountView,
    /// perpPythOracle
    pub perp_pyth_oracle: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
    /// rent
    pub rent: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
    /// driftProgram
    pub drift_program: &'a AccountView,
}

impl<'a> DriftsupplystrategyinitAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 16] {
        [
            InstructionAccount::writable(self.vault.address()),
            InstructionAccount::writable(self.strategy.address()),
            InstructionAccount::readonly(self.asset_mint.address()),
            InstructionAccount::writable(self.drift_user.address()),
            InstructionAccount::writable(self.drift_user_stats.address()),
            InstructionAccount::writable(self.drift_state.address()),
            InstructionAccount::readonly(self.drift_signer.address()),
            InstructionAccount::readonly(self.drift_spot_market.address()),
            InstructionAccount::readonly(self.drift_perp_market.address()),
            InstructionAccount::readonly(self.drift_spot_market_vault.address()),
            InstructionAccount::readonly(self.spot_pyth_oracle.address()),
            InstructionAccount::readonly(self.perp_pyth_oracle.address()),
            InstructionAccount::writable_signer(self.authority.address()),
            InstructionAccount::readonly(self.rent.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.drift_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 16] {
        [
            self.vault, self.strategy, self.asset_mint, self.drift_user, self.drift_user_stats, self.drift_state, self.drift_signer, self.drift_spot_market, self.drift_perp_market, self.drift_spot_market_vault, self.spot_pyth_oracle, self.perp_pyth_oracle, self.authority, self.rent, self.system_program, self.drift_program
        ]
    }
}

/// Accounts for `driftSupplyStrategyDeposit`
pub struct DriftsupplystrategydepositAccounts<'a> {
    /// vault
    pub vault: &'a AccountView,
    /// assetMint
    pub asset_mint: &'a AccountView,
    /// vaultAssetAta
    pub vault_asset_ata: &'a AccountView,
    /// strategy
    pub strategy: &'a AccountView,
    /// driftUser
    pub drift_user: &'a AccountView,
    /// driftUserStats
    pub drift_user_stats: &'a AccountView,
    /// driftState
    pub drift_state: &'a AccountView,
    /// driftSpotMarket
    pub drift_spot_market: &'a AccountView,
    /// driftSpotMarketVault
    pub drift_spot_market_vault: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
    /// driftProgram
    pub drift_program: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// logProgram
    pub log_program: &'a AccountView,
}

impl<'a> DriftsupplystrategydepositAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 14] {
        [
            InstructionAccount::writable(self.vault.address()),
            InstructionAccount::readonly(self.asset_mint.address()),
            InstructionAccount::writable(self.vault_asset_ata.address()),
            InstructionAccount::readonly(self.strategy.address()),
            InstructionAccount::writable(self.drift_user.address()),
            InstructionAccount::writable(self.drift_user_stats.address()),
            InstructionAccount::writable(self.drift_state.address()),
            InstructionAccount::writable(self.drift_spot_market.address()),
            InstructionAccount::writable(self.drift_spot_market_vault.address()),
            InstructionAccount::writable_signer(self.authority.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.drift_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.log_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 14] {
        [
            self.vault, self.asset_mint, self.vault_asset_ata, self.strategy, self.drift_user, self.drift_user_stats, self.drift_state, self.drift_spot_market, self.drift_spot_market_vault, self.authority, self.system_program, self.drift_program, self.token_program, self.log_program
        ]
    }
}

/// Accounts for `driftSupplyStrategyWithdraw`
pub struct DriftsupplystrategywithdrawAccounts<'a> {
    /// vault
    pub vault: &'a AccountView,
    /// assetMint
    pub asset_mint: &'a AccountView,
    /// vaultAssetAta
    pub vault_asset_ata: &'a AccountView,
    /// strategy
    pub strategy: &'a AccountView,
    /// driftUser
    pub drift_user: &'a AccountView,
    /// driftUserStats
    pub drift_user_stats: &'a AccountView,
    /// driftState
    pub drift_state: &'a AccountView,
    /// driftSpotMarketVault
    pub drift_spot_market_vault: &'a AccountView,
    /// driftSpotMarket
    pub drift_spot_market: &'a AccountView,
    /// driftSigner
    pub drift_signer: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
    /// driftProgram
    pub drift_program: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// logProgram
    pub log_program: &'a AccountView,
}

impl<'a> DriftsupplystrategywithdrawAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 15] {
        [
            InstructionAccount::writable(self.vault.address()),
            InstructionAccount::readonly(self.asset_mint.address()),
            InstructionAccount::writable(self.vault_asset_ata.address()),
            InstructionAccount::readonly(self.strategy.address()),
            InstructionAccount::writable(self.drift_user.address()),
            InstructionAccount::writable(self.drift_user_stats.address()),
            InstructionAccount::writable(self.drift_state.address()),
            InstructionAccount::writable(self.drift_spot_market_vault.address()),
            InstructionAccount::writable(self.drift_spot_market.address()),
            InstructionAccount::readonly(self.drift_signer.address()),
            InstructionAccount::writable_signer(self.authority.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.drift_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.log_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 15] {
        [
            self.vault, self.asset_mint, self.vault_asset_ata, self.strategy, self.drift_user, self.drift_user_stats, self.drift_state, self.drift_spot_market_vault, self.drift_spot_market, self.drift_signer, self.authority, self.system_program, self.drift_program, self.token_program, self.log_program
        ]
    }
}

/// Accounts for `driftInsuranceFundStrategyInit`
pub struct DriftinsurancefundstrategyinitAccounts<'a> {
    /// vault
    pub vault: &'a AccountView,
    /// strategy
    pub strategy: &'a AccountView,
    /// assetMint
    pub asset_mint: &'a AccountView,
    /// driftUserStats
    pub drift_user_stats: &'a AccountView,
    /// driftSpotMarket
    pub drift_spot_market: &'a AccountView,
    /// driftSpotMarketVault
    pub drift_spot_market_vault: &'a AccountView,
    /// driftState
    pub drift_state: &'a AccountView,
    /// driftInsuranceFundStake
    pub drift_insurance_fund_stake: &'a AccountView,
    /// driftInsuranceFundVault
    pub drift_insurance_fund_vault: &'a AccountView,
    /// driftSigner
    pub drift_signer: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
    /// rent
    pub rent: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// driftProgram
    pub drift_program: &'a AccountView,
}

impl<'a> DriftinsurancefundstrategyinitAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 15] {
        [
            InstructionAccount::writable(self.vault.address()),
            InstructionAccount::writable(self.strategy.address()),
            InstructionAccount::readonly(self.asset_mint.address()),
            InstructionAccount::writable(self.drift_user_stats.address()),
            InstructionAccount::writable(self.drift_spot_market.address()),
            InstructionAccount::readonly(self.drift_spot_market_vault.address()),
            InstructionAccount::writable(self.drift_state.address()),
            InstructionAccount::writable(self.drift_insurance_fund_stake.address()),
            InstructionAccount::readonly(self.drift_insurance_fund_vault.address()),
            InstructionAccount::writable(self.drift_signer.address()),
            InstructionAccount::writable_signer(self.authority.address()),
            InstructionAccount::readonly(self.rent.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.drift_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 15] {
        [
            self.vault, self.strategy, self.asset_mint, self.drift_user_stats, self.drift_spot_market, self.drift_spot_market_vault, self.drift_state, self.drift_insurance_fund_stake, self.drift_insurance_fund_vault, self.drift_signer, self.authority, self.rent, self.system_program, self.token_program, self.drift_program
        ]
    }
}

/// Accounts for `driftInsuranceFundStrategyStake`
pub struct DriftinsurancefundstrategystakeAccounts<'a> {
    /// vault
    pub vault: &'a AccountView,
    /// strategy
    pub strategy: &'a AccountView,
    /// assetMint
    pub asset_mint: &'a AccountView,
    /// vaultAssetAta
    pub vault_asset_ata: &'a AccountView,
    /// driftUserStats
    pub drift_user_stats: &'a AccountView,
    /// driftSpotMarket
    pub drift_spot_market: &'a AccountView,
    /// driftSpotMarketVault
    pub drift_spot_market_vault: &'a AccountView,
    /// driftState
    pub drift_state: &'a AccountView,
    /// driftInsuranceFundStake
    pub drift_insurance_fund_stake: &'a AccountView,
    /// driftInsuranceFundVault
    pub drift_insurance_fund_vault: &'a AccountView,
    /// driftSigner
    pub drift_signer: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// driftProgram
    pub drift_program: &'a AccountView,
    /// logProgram
    pub log_program: &'a AccountView,
}

impl<'a> DriftinsurancefundstrategystakeAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 15] {
        [
            InstructionAccount::writable(self.vault.address()),
            InstructionAccount::writable(self.strategy.address()),
            InstructionAccount::readonly(self.asset_mint.address()),
            InstructionAccount::writable(self.vault_asset_ata.address()),
            InstructionAccount::writable(self.drift_user_stats.address()),
            InstructionAccount::writable(self.drift_spot_market.address()),
            InstructionAccount::writable(self.drift_spot_market_vault.address()),
            InstructionAccount::readonly(self.drift_state.address()),
            InstructionAccount::writable(self.drift_insurance_fund_stake.address()),
            InstructionAccount::writable(self.drift_insurance_fund_vault.address()),
            InstructionAccount::writable(self.drift_signer.address()),
            InstructionAccount::writable_signer(self.authority.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.drift_program.address()),
            InstructionAccount::readonly(self.log_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 15] {
        [
            self.vault, self.strategy, self.asset_mint, self.vault_asset_ata, self.drift_user_stats, self.drift_spot_market, self.drift_spot_market_vault, self.drift_state, self.drift_insurance_fund_stake, self.drift_insurance_fund_vault, self.drift_signer, self.authority, self.token_program, self.drift_program, self.log_program
        ]
    }
}

/// Accounts for `driftInsuranceFundStrategyUnstake`
pub struct DriftinsurancefundstrategyunstakeAccounts<'a> {
    /// vault
    pub vault: &'a AccountView,
    /// strategy
    pub strategy: &'a AccountView,
    /// driftUserStats
    pub drift_user_stats: &'a AccountView,
    /// driftSpotMarket
    pub drift_spot_market: &'a AccountView,
    /// driftInsuranceFundStake
    pub drift_insurance_fund_stake: &'a AccountView,
    /// driftInsuranceFundVault
    pub drift_insurance_fund_vault: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
    /// driftProgram
    pub drift_program: &'a AccountView,
}

impl<'a> DriftinsurancefundstrategyunstakeAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 8] {
        [
            InstructionAccount::writable(self.vault.address()),
            InstructionAccount::readonly(self.strategy.address()),
            InstructionAccount::writable(self.drift_user_stats.address()),
            InstructionAccount::writable(self.drift_spot_market.address()),
            InstructionAccount::writable(self.drift_insurance_fund_stake.address()),
            InstructionAccount::writable(self.drift_insurance_fund_vault.address()),
            InstructionAccount::writable_signer(self.authority.address()),
            InstructionAccount::readonly(self.drift_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 8] {
        [
            self.vault, self.strategy, self.drift_user_stats, self.drift_spot_market, self.drift_insurance_fund_stake, self.drift_insurance_fund_vault, self.authority, self.drift_program
        ]
    }
}

/// Accounts for `driftInsuranceFundStrategyWithdraw`
pub struct DriftinsurancefundstrategywithdrawAccounts<'a> {
    /// vault
    pub vault: &'a AccountView,
    /// strategy
    pub strategy: &'a AccountView,
    /// assetMint
    pub asset_mint: &'a AccountView,
    /// vaultAssetAta
    pub vault_asset_ata: &'a AccountView,
    /// driftUserStats
    pub drift_user_stats: &'a AccountView,
    /// driftSpotMarket
    pub drift_spot_market: &'a AccountView,
    /// driftState
    pub drift_state: &'a AccountView,
    /// driftInsuranceFundStake
    pub drift_insurance_fund_stake: &'a AccountView,
    /// driftInsuranceFundVault
    pub drift_insurance_fund_vault: &'a AccountView,
    /// driftSigner
    pub drift_signer: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// driftProgram
    pub drift_program: &'a AccountView,
    /// logProgram
    pub log_program: &'a AccountView,
}

impl<'a> DriftinsurancefundstrategywithdrawAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 14] {
        [
            InstructionAccount::writable(self.vault.address()),
            InstructionAccount::writable(self.strategy.address()),
            InstructionAccount::readonly(self.asset_mint.address()),
            InstructionAccount::writable(self.vault_asset_ata.address()),
            InstructionAccount::writable(self.drift_user_stats.address()),
            InstructionAccount::writable(self.drift_spot_market.address()),
            InstructionAccount::readonly(self.drift_state.address()),
            InstructionAccount::writable(self.drift_insurance_fund_stake.address()),
            InstructionAccount::writable(self.drift_insurance_fund_vault.address()),
            InstructionAccount::writable(self.drift_signer.address()),
            InstructionAccount::writable_signer(self.authority.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.drift_program.address()),
            InstructionAccount::readonly(self.log_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 14] {
        [
            self.vault, self.strategy, self.asset_mint, self.vault_asset_ata, self.drift_user_stats, self.drift_spot_market, self.drift_state, self.drift_insurance_fund_stake, self.drift_insurance_fund_vault, self.drift_signer, self.authority, self.token_program, self.drift_program, self.log_program
        ]
    }
}

/// Accounts for `clendSupplyStrategyInit`
pub struct ClendsupplystrategyinitAccounts<'a> {
    /// vault
    pub vault: &'a AccountView,
    /// strategy
    pub strategy: &'a AccountView,
    /// assetMint
    pub asset_mint: &'a AccountView,
    /// clendGroup
    pub clend_group: &'a AccountView,
    /// clendAccount
    pub clend_account: &'a AccountView,
    /// clendBank
    pub clend_bank: &'a AccountView,
    /// clendBankLiquidityVault
    pub clend_bank_liquidity_vault: &'a AccountView,
    /// clendBankLiquidityVaultAuthority
    pub clend_bank_liquidity_vault_authority: &'a AccountView,
    /// clendOracle
    pub clend_oracle: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
    /// clendProgram
    pub clend_program: &'a AccountView,
}

impl<'a> ClendsupplystrategyinitAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 12] {
        [
            InstructionAccount::writable(self.vault.address()),
            InstructionAccount::writable(self.strategy.address()),
            InstructionAccount::readonly(self.asset_mint.address()),
            InstructionAccount::readonly(self.clend_group.address()),
            InstructionAccount::writable_signer(self.clend_account.address()),
            InstructionAccount::readonly(self.clend_bank.address()),
            InstructionAccount::readonly(self.clend_bank_liquidity_vault.address()),
            InstructionAccount::readonly(self.clend_bank_liquidity_vault_authority.address()),
            InstructionAccount::readonly(self.clend_oracle.address()),
            InstructionAccount::writable_signer(self.authority.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.clend_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 12] {
        [
            self.vault, self.strategy, self.asset_mint, self.clend_group, self.clend_account, self.clend_bank, self.clend_bank_liquidity_vault, self.clend_bank_liquidity_vault_authority, self.clend_oracle, self.authority, self.system_program, self.clend_program
        ]
    }
}

/// Accounts for `clendSupplyStrategyDeposit`
pub struct ClendsupplystrategydepositAccounts<'a> {
    /// vault
    pub vault: &'a AccountView,
    /// assetMint
    pub asset_mint: &'a AccountView,
    /// vaultAssetAta
    pub vault_asset_ata: &'a AccountView,
    /// strategy
    pub strategy: &'a AccountView,
    /// clendGroup
    pub clend_group: &'a AccountView,
    /// clendAccount
    pub clend_account: &'a AccountView,
    /// clendBank
    pub clend_bank: &'a AccountView,
    /// clendBankLiquidityVault
    pub clend_bank_liquidity_vault: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// clendProgram
    pub clend_program: &'a AccountView,
    /// logProgram
    pub log_program: &'a AccountView,
}

impl<'a> ClendsupplystrategydepositAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 13] {
        [
            InstructionAccount::writable(self.vault.address()),
            InstructionAccount::readonly(self.asset_mint.address()),
            InstructionAccount::writable(self.vault_asset_ata.address()),
            InstructionAccount::readonly(self.strategy.address()),
            InstructionAccount::readonly(self.clend_group.address()),
            InstructionAccount::writable(self.clend_account.address()),
            InstructionAccount::writable(self.clend_bank.address()),
            InstructionAccount::writable(self.clend_bank_liquidity_vault.address()),
            InstructionAccount::writable_signer(self.authority.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.clend_program.address()),
            InstructionAccount::readonly(self.log_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 13] {
        [
            self.vault, self.asset_mint, self.vault_asset_ata, self.strategy, self.clend_group, self.clend_account, self.clend_bank, self.clend_bank_liquidity_vault, self.authority, self.system_program, self.token_program, self.clend_program, self.log_program
        ]
    }
}

/// Accounts for `clendSupplyStrategyWithdraw`
pub struct ClendsupplystrategywithdrawAccounts<'a> {
    /// vault
    pub vault: &'a AccountView,
    /// assetMint
    pub asset_mint: &'a AccountView,
    /// vaultAssetAta
    pub vault_asset_ata: &'a AccountView,
    /// strategy
    pub strategy: &'a AccountView,
    /// clendGroup
    pub clend_group: &'a AccountView,
    /// clendAccount
    pub clend_account: &'a AccountView,
    /// clendBank
    pub clend_bank: &'a AccountView,
    /// clendBankLiquidityVault
    pub clend_bank_liquidity_vault: &'a AccountView,
    /// clendBankLiquidityVaultAuthority
    pub clend_bank_liquidity_vault_authority: &'a AccountView,
    /// clendOracle
    pub clend_oracle: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// clendProgram
    pub clend_program: &'a AccountView,
    /// logProgram
    pub log_program: &'a AccountView,
}

impl<'a> ClendsupplystrategywithdrawAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 15] {
        [
            InstructionAccount::writable(self.vault.address()),
            InstructionAccount::readonly(self.asset_mint.address()),
            InstructionAccount::writable(self.vault_asset_ata.address()),
            InstructionAccount::readonly(self.strategy.address()),
            InstructionAccount::readonly(self.clend_group.address()),
            InstructionAccount::writable(self.clend_account.address()),
            InstructionAccount::writable(self.clend_bank.address()),
            InstructionAccount::writable(self.clend_bank_liquidity_vault.address()),
            InstructionAccount::writable(self.clend_bank_liquidity_vault_authority.address()),
            InstructionAccount::readonly(self.clend_oracle.address()),
            InstructionAccount::writable_signer(self.authority.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.clend_program.address()),
            InstructionAccount::readonly(self.log_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 15] {
        [
            self.vault, self.asset_mint, self.vault_asset_ata, self.strategy, self.clend_group, self.clend_account, self.clend_bank, self.clend_bank_liquidity_vault, self.clend_bank_liquidity_vault_authority, self.clend_oracle, self.authority, self.system_program, self.token_program, self.clend_program, self.log_program
        ]
    }
}

/// Accounts for `chestStrategyInit`
pub struct CheststrategyinitAccounts<'a> {
    /// vault
    pub vault: &'a AccountView,
    /// strategy
    pub strategy: &'a AccountView,
    /// assetMint
    pub asset_mint: &'a AccountView,
    /// assetTokenAccount
    pub asset_token_account: &'a AccountView,
    /// chest
    pub chest: &'a AccountView,
    /// coinTokenAccount
    pub coin_token_account: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
}

impl<'a> CheststrategyinitAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 8] {
        [
            InstructionAccount::writable(self.vault.address()),
            InstructionAccount::writable(self.strategy.address()),
            InstructionAccount::readonly(self.asset_mint.address()),
            InstructionAccount::readonly(self.asset_token_account.address()),
            InstructionAccount::readonly(self.chest.address()),
            InstructionAccount::readonly(self.coin_token_account.address()),
            InstructionAccount::writable_signer(self.authority.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 8] {
        [
            self.vault, self.strategy, self.asset_mint, self.asset_token_account, self.chest, self.coin_token_account, self.authority, self.system_program
        ]
    }
}

/// Accounts for `chestStrategyDeposit`
pub struct CheststrategydepositAccounts<'a> {
    /// vault
    pub vault: &'a AccountView,
    /// strategy
    pub strategy: &'a AccountView,
    /// assetMint
    pub asset_mint: &'a AccountView,
    /// chest
    pub chest: &'a AccountView,
    /// coin
    pub coin: &'a AccountView,
    /// chestAssetReserve
    pub chest_asset_reserve: &'a AccountView,
    /// driftVault
    pub drift_vault: &'a AccountView,
    /// driftVaultDepositor
    pub drift_vault_depositor: &'a AccountView,
    /// driftVaultTokenAccount
    pub drift_vault_token_account: &'a AccountView,
    /// driftUser
    pub drift_user: &'a AccountView,
    /// driftUserStats
    pub drift_user_stats: &'a AccountView,
    /// driftSpotMarket
    pub drift_spot_market: &'a AccountView,
    /// driftSpotMarketVault
    pub drift_spot_market_vault: &'a AccountView,
    /// driftState
    pub drift_state: &'a AccountView,
    /// vaultAssetTokenAccount
    pub vault_asset_token_account: &'a AccountView,
    /// vaultCoinTokenAccount
    pub vault_coin_token_account: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// chestProgram
    pub chest_program: &'a AccountView,
    /// driftProgram
    pub drift_program: &'a AccountView,
    /// driftVaultsProgram
    pub drift_vaults_program: &'a AccountView,
    /// logProgram
    pub log_program: &'a AccountView,
}

impl<'a> CheststrategydepositAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 22] {
        [
            InstructionAccount::writable(self.vault.address()),
            InstructionAccount::writable(self.strategy.address()),
            InstructionAccount::readonly(self.asset_mint.address()),
            InstructionAccount::writable(self.chest.address()),
            InstructionAccount::writable(self.coin.address()),
            InstructionAccount::writable(self.chest_asset_reserve.address()),
            InstructionAccount::writable(self.drift_vault.address()),
            InstructionAccount::writable(self.drift_vault_depositor.address()),
            InstructionAccount::writable(self.drift_vault_token_account.address()),
            InstructionAccount::writable(self.drift_user.address()),
            InstructionAccount::writable(self.drift_user_stats.address()),
            InstructionAccount::writable(self.drift_spot_market.address()),
            InstructionAccount::writable(self.drift_spot_market_vault.address()),
            InstructionAccount::writable(self.drift_state.address()),
            InstructionAccount::writable(self.vault_asset_token_account.address()),
            InstructionAccount::writable(self.vault_coin_token_account.address()),
            InstructionAccount::writable_signer(self.authority.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.chest_program.address()),
            InstructionAccount::readonly(self.drift_program.address()),
            InstructionAccount::readonly(self.drift_vaults_program.address()),
            InstructionAccount::readonly(self.log_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 22] {
        [
            self.vault, self.strategy, self.asset_mint, self.chest, self.coin, self.chest_asset_reserve, self.drift_vault, self.drift_vault_depositor, self.drift_vault_token_account, self.drift_user, self.drift_user_stats, self.drift_spot_market, self.drift_spot_market_vault, self.drift_state, self.vault_asset_token_account, self.vault_coin_token_account, self.authority, self.token_program, self.chest_program, self.drift_program, self.drift_vaults_program, self.log_program
        ]
    }
}

/// Accounts for `chestStrategyRequestWithdraw`
pub struct CheststrategyrequestwithdrawAccounts<'a> {
    /// vault
    pub vault: &'a AccountView,
    /// strategy
    pub strategy: &'a AccountView,
    /// assetMint
    pub asset_mint: &'a AccountView,
    /// chest
    pub chest: &'a AccountView,
    /// coin
    pub coin: &'a AccountView,
    /// chestCoinReserve
    pub chest_coin_reserve: &'a AccountView,
    /// redemptionRequest
    pub redemption_request: &'a AccountView,
    /// driftVault
    pub drift_vault: &'a AccountView,
    /// driftVaultDepositor
    pub drift_vault_depositor: &'a AccountView,
    /// driftUser
    pub drift_user: &'a AccountView,
    /// assetDestination
    pub asset_destination: &'a AccountView,
    /// vaultCoinTokenAccount
    pub vault_coin_token_account: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// chestProgram
    pub chest_program: &'a AccountView,
    /// logProgram
    pub log_program: &'a AccountView,
}

impl<'a> CheststrategyrequestwithdrawAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 17] {
        [
            InstructionAccount::writable(self.vault.address()),
            InstructionAccount::writable(self.strategy.address()),
            InstructionAccount::readonly(self.asset_mint.address()),
            InstructionAccount::writable(self.chest.address()),
            InstructionAccount::writable(self.coin.address()),
            InstructionAccount::writable(self.chest_coin_reserve.address()),
            InstructionAccount::writable(self.redemption_request.address()),
            InstructionAccount::writable(self.drift_vault.address()),
            InstructionAccount::writable(self.drift_vault_depositor.address()),
            InstructionAccount::writable(self.drift_user.address()),
            InstructionAccount::readonly(self.asset_destination.address()),
            InstructionAccount::writable(self.vault_coin_token_account.address()),
            InstructionAccount::writable_signer(self.authority.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.chest_program.address()),
            InstructionAccount::readonly(self.log_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 17] {
        [
            self.vault, self.strategy, self.asset_mint, self.chest, self.coin, self.chest_coin_reserve, self.redemption_request, self.drift_vault, self.drift_vault_depositor, self.drift_user, self.asset_destination, self.vault_coin_token_account, self.authority, self.system_program, self.token_program, self.chest_program, self.log_program
        ]
    }
}

/// Accounts for `chestStrategyWithdraw`
pub struct CheststrategywithdrawAccounts<'a> {
    /// vault
    pub vault: &'a AccountView,
    /// strategy
    pub strategy: &'a AccountView,
    /// assetMint
    pub asset_mint: &'a AccountView,
    /// vaultAssetAta
    pub vault_asset_ata: &'a AccountView,
    /// strategyAssetTokenAccount
    pub strategy_asset_token_account: &'a AccountView,
    /// chest
    pub chest: &'a AccountView,
    /// chestRedemptionRequest
    pub chest_redemption_request: &'a AccountView,
    /// coin
    pub coin: &'a AccountView,
    /// vaultCoinTokenAccount
    pub vault_coin_token_account: &'a AccountView,
    /// driftVault
    pub drift_vault: &'a AccountView,
    /// driftVaultDepositor
    pub drift_vault_depositor: &'a AccountView,
    /// driftUser
    pub drift_user: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// chestProgram
    pub chest_program: &'a AccountView,
    /// logProgram
    pub log_program: &'a AccountView,
}

impl<'a> CheststrategywithdrawAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 17] {
        [
            InstructionAccount::writable(self.vault.address()),
            InstructionAccount::writable(self.strategy.address()),
            InstructionAccount::readonly(self.asset_mint.address()),
            InstructionAccount::writable(self.vault_asset_ata.address()),
            InstructionAccount::writable(self.strategy_asset_token_account.address()),
            InstructionAccount::writable(self.chest.address()),
            InstructionAccount::readonly(self.chest_redemption_request.address()),
            InstructionAccount::writable(self.coin.address()),
            InstructionAccount::writable(self.vault_coin_token_account.address()),
            InstructionAccount::writable(self.drift_vault.address()),
            InstructionAccount::writable(self.drift_vault_depositor.address()),
            InstructionAccount::writable(self.drift_user.address()),
            InstructionAccount::writable_signer(self.authority.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.chest_program.address()),
            InstructionAccount::readonly(self.log_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 17] {
        [
            self.vault, self.strategy, self.asset_mint, self.vault_asset_ata, self.strategy_asset_token_account, self.chest, self.chest_redemption_request, self.coin, self.vault_coin_token_account, self.drift_vault, self.drift_vault_depositor, self.drift_user, self.authority, self.system_program, self.token_program, self.chest_program, self.log_program
        ]
    }
}

/// Accounts for `updateSwitchboardOraclePrice`
pub struct UpdateswitchboardoraclepriceAccounts<'a> {
    /// vault
    pub vault: &'a AccountView,
    /// shares
    pub shares: &'a AccountView,
    /// feed
    pub feed: &'a AccountView,
    /// queue
    pub queue: &'a AccountView,
    /// programState
    pub program_state: &'a AccountView,
    /// recentSlothashes
    pub recent_slothashes: &'a AccountView,
    /// rewardVault
    pub reward_vault: &'a AccountView,
    /// tokenMint
    pub token_mint: &'a AccountView,
    /// switchboardOnDemandProgram
    pub switchboard_on_demand_program: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
    /// logProgram
    pub log_program: &'a AccountView,
}

impl<'a> UpdateswitchboardoraclepriceAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 12] {
        [
            InstructionAccount::writable(self.vault.address()),
            InstructionAccount::readonly(self.shares.address()),
            InstructionAccount::writable(self.feed.address()),
            InstructionAccount::readonly(self.queue.address()),
            InstructionAccount::readonly(self.program_state.address()),
            InstructionAccount::readonly(self.recent_slothashes.address()),
            InstructionAccount::writable(self.reward_vault.address()),
            InstructionAccount::readonly(self.token_mint.address()),
            InstructionAccount::readonly(self.switchboard_on_demand_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.log_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 12] {
        [
            self.vault, self.shares, self.feed, self.queue, self.program_state, self.recent_slothashes, self.reward_vault, self.token_mint, self.switchboard_on_demand_program, self.token_program, self.system_program, self.log_program
        ]
    }
}


// ============================================
// CPI Functions
// ============================================
/// CPI: initVault
#[inline(always)]
pub fn init_vault<'a>(
    accounts: &InitvaultAccounts<'a>, args: &InitvaultArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<InitvaultArgs>()];
    data[0..8].copy_from_slice(&INIT_VAULT);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const InitvaultArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<InitvaultArgs>(),
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

/// CPI: transferVaultAuthority
#[inline(always)]
pub fn transfer_vault_authority<'a>(
    accounts: &TransfervaultauthorityAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&TRANSFER_VAULT_AUTHORITY);
    
    
    // Build InstructionAccount array
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

/// CPI: pauseVault
#[inline(always)]
pub fn pause_vault<'a>(
    accounts: &PausevaultAccounts<'a>, args: &PausevaultArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<PausevaultArgs>()];
    data[0..8].copy_from_slice(&PAUSE_VAULT);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const PausevaultArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<PausevaultArgs>(),
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

/// CPI: migrateVaultAccount
#[inline(always)]
pub fn migrate_vault_account<'a>(
    accounts: &MigratevaultaccountAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&MIGRATE_VAULT_ACCOUNT);
    
    
    // Build InstructionAccount array
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

/// CPI: distributeVaultFees
#[inline(always)]
pub fn distribute_vault_fees<'a>(
    accounts: &DistributevaultfeesAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&DISTRIBUTE_VAULT_FEES);
    
    
    // Build InstructionAccount array
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

/// CPI: updateFee
#[inline(always)]
pub fn update_fee<'a>(
    accounts: &UpdatefeeAccounts<'a>, args: &UpdatefeeArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<UpdatefeeArgs>()];
    data[0..8].copy_from_slice(&UPDATE_FEE);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const UpdatefeeArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<UpdatefeeArgs>(),
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

/// CPI: issue
#[inline(always)]
pub fn issue<'a>(
    accounts: &IssueAccounts<'a>, args: &IssueArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<IssueArgs>()];
    data[0..8].copy_from_slice(&ISSUE);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const IssueArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<IssueArgs>(),
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

/// CPI: redeem
#[inline(always)]
pub fn redeem<'a>(
    accounts: &RedeemAccounts<'a>, args: &RedeemArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<RedeemArgs>()];
    data[0..8].copy_from_slice(&REDEEM);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const RedeemArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<RedeemArgs>(),
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

/// CPI: addAsset
#[inline(always)]
pub fn add_asset<'a>(
    accounts: &AddassetAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&ADD_ASSET);
    
    
    // Build InstructionAccount array
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

/// CPI: updateAsset
#[inline(always)]
pub fn update_asset<'a>(
    accounts: &UpdateassetAccounts<'a>, args: &UpdateassetArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<UpdateassetArgs>()];
    data[0..8].copy_from_slice(&UPDATE_ASSET);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const UpdateassetArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<UpdateassetArgs>(),
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

/// CPI: removeAsset
#[inline(always)]
pub fn remove_asset<'a>(
    accounts: &RemoveassetAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&REMOVE_ASSET);
    
    
    // Build InstructionAccount array
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

/// CPI: removeStrategy
#[inline(always)]
pub fn remove_strategy<'a>(
    accounts: &RemovestrategyAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&REMOVE_STRATEGY);
    
    
    // Build InstructionAccount array
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

/// CPI: updateStrategy
#[inline(always)]
pub fn update_strategy<'a>(
    accounts: &UpdatestrategyAccounts<'a>, args: &UpdatestrategyArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<UpdatestrategyArgs>()];
    data[0..8].copy_from_slice(&UPDATE_STRATEGY);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const UpdatestrategyArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<UpdatestrategyArgs>(),
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

/// CPI: marginfiSupplyStrategyInit
#[inline(always)]
pub fn marginfi_supply_strategy_init<'a>(
    accounts: &MarginfisupplystrategyinitAccounts<'a>, args: &MarginfisupplystrategyinitArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<MarginfisupplystrategyinitArgs>()];
    data[0..8].copy_from_slice(&MARGINFI_SUPPLY_STRATEGY_INIT);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const MarginfisupplystrategyinitArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<MarginfisupplystrategyinitArgs>(),
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

/// CPI: marginfiSupplyStrategyDeposit
#[inline(always)]
pub fn marginfi_supply_strategy_deposit<'a>(
    accounts: &MarginfisupplystrategydepositAccounts<'a>, args: &MarginfisupplystrategydepositArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<MarginfisupplystrategydepositArgs>()];
    data[0..8].copy_from_slice(&MARGINFI_SUPPLY_STRATEGY_DEPOSIT);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const MarginfisupplystrategydepositArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<MarginfisupplystrategydepositArgs>(),
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

/// CPI: marginfiSupplyStrategyWithdraw
#[inline(always)]
pub fn marginfi_supply_strategy_withdraw<'a>(
    accounts: &MarginfisupplystrategywithdrawAccounts<'a>, args: &MarginfisupplystrategywithdrawArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<MarginfisupplystrategywithdrawArgs>()];
    data[0..8].copy_from_slice(&MARGINFI_SUPPLY_STRATEGY_WITHDRAW);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const MarginfisupplystrategywithdrawArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<MarginfisupplystrategywithdrawArgs>(),
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

/// CPI: marginfiSupplyStrategyClaimEmissions
#[inline(always)]
pub fn marginfi_supply_strategy_claim_emissions<'a>(
    accounts: &MarginfisupplystrategyclaimemissionsAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&MARGINFI_SUPPLY_STRATEGY_CLAIM_EMISSIONS);
    
    
    // Build InstructionAccount array
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

/// CPI: klendSupplyStrategyInit
#[inline(always)]
pub fn klend_supply_strategy_init<'a>(
    accounts: &KlendsupplystrategyinitAccounts<'a>, args: &KlendsupplystrategyinitArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<KlendsupplystrategyinitArgs>()];
    data[0..8].copy_from_slice(&KLEND_SUPPLY_STRATEGY_INIT);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const KlendsupplystrategyinitArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<KlendsupplystrategyinitArgs>(),
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

/// CPI: klendSupplyStrategyDeposit
#[inline(always)]
pub fn klend_supply_strategy_deposit<'a>(
    accounts: &KlendsupplystrategydepositAccounts<'a>, args: &KlendsupplystrategydepositArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<KlendsupplystrategydepositArgs>()];
    data[0..8].copy_from_slice(&KLEND_SUPPLY_STRATEGY_DEPOSIT);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const KlendsupplystrategydepositArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<KlendsupplystrategydepositArgs>(),
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

/// CPI: klendSupplyStrategyWithdraw
#[inline(always)]
pub fn klend_supply_strategy_withdraw<'a>(
    accounts: &KlendsupplystrategywithdrawAccounts<'a>, args: &KlendsupplystrategywithdrawArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<KlendsupplystrategywithdrawArgs>()];
    data[0..8].copy_from_slice(&KLEND_SUPPLY_STRATEGY_WITHDRAW);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const KlendsupplystrategywithdrawArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<KlendsupplystrategywithdrawArgs>(),
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

/// CPI: klendSupplyStrategyClaimEmissions
#[inline(always)]
pub fn klend_supply_strategy_claim_emissions<'a>(
    accounts: &KlendsupplystrategyclaimemissionsAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&KLEND_SUPPLY_STRATEGY_CLAIM_EMISSIONS);
    
    
    // Build InstructionAccount array
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

/// CPI: solendSupplyStrategyInit
#[inline(always)]
pub fn solend_supply_strategy_init<'a>(
    accounts: &SolendsupplystrategyinitAccounts<'a>, args: &SolendsupplystrategyinitArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SolendsupplystrategyinitArgs>()];
    data[0..8].copy_from_slice(&SOLEND_SUPPLY_STRATEGY_INIT);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SolendsupplystrategyinitArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SolendsupplystrategyinitArgs>(),
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
    invoke_signed::<17>(&instruction, &account_views, signers)
}

/// CPI: solendSupplyStrategyDeposit
#[inline(always)]
pub fn solend_supply_strategy_deposit<'a>(
    accounts: &SolendsupplystrategydepositAccounts<'a>, args: &SolendsupplystrategydepositArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SolendsupplystrategydepositArgs>()];
    data[0..8].copy_from_slice(&SOLEND_SUPPLY_STRATEGY_DEPOSIT);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SolendsupplystrategydepositArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SolendsupplystrategydepositArgs>(),
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

/// CPI: solendSupplyStrategyWithdraw
#[inline(always)]
pub fn solend_supply_strategy_withdraw<'a>(
    accounts: &SolendsupplystrategywithdrawAccounts<'a>, args: &SolendsupplystrategywithdrawArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SolendsupplystrategywithdrawArgs>()];
    data[0..8].copy_from_slice(&SOLEND_SUPPLY_STRATEGY_WITHDRAW);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SolendsupplystrategywithdrawArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SolendsupplystrategywithdrawArgs>(),
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

/// CPI: mangoSupplyStrategyInit
#[inline(always)]
pub fn mango_supply_strategy_init<'a>(
    accounts: &MangosupplystrategyinitAccounts<'a>, args: &MangosupplystrategyinitArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<MangosupplystrategyinitArgs>()];
    data[0..8].copy_from_slice(&MANGO_SUPPLY_STRATEGY_INIT);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const MangosupplystrategyinitArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<MangosupplystrategyinitArgs>(),
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

/// CPI: mangoSupplyStrategyDeposit
#[inline(always)]
pub fn mango_supply_strategy_deposit<'a>(
    accounts: &MangosupplystrategydepositAccounts<'a>, args: &MangosupplystrategydepositArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<MangosupplystrategydepositArgs>()];
    data[0..8].copy_from_slice(&MANGO_SUPPLY_STRATEGY_DEPOSIT);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const MangosupplystrategydepositArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<MangosupplystrategydepositArgs>(),
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

/// CPI: mangoSupplyStrategyWithdraw
#[inline(always)]
pub fn mango_supply_strategy_withdraw<'a>(
    accounts: &MangosupplystrategywithdrawAccounts<'a>, args: &MangosupplystrategywithdrawArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<MangosupplystrategywithdrawArgs>()];
    data[0..8].copy_from_slice(&MANGO_SUPPLY_STRATEGY_WITHDRAW);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const MangosupplystrategywithdrawArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<MangosupplystrategywithdrawArgs>(),
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

/// CPI: driftSupplyStrategyInit
#[inline(always)]
pub fn drift_supply_strategy_init<'a>(
    accounts: &DriftsupplystrategyinitAccounts<'a>, args: &DriftsupplystrategyinitArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<DriftsupplystrategyinitArgs>()];
    data[0..8].copy_from_slice(&DRIFT_SUPPLY_STRATEGY_INIT);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const DriftsupplystrategyinitArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<DriftsupplystrategyinitArgs>(),
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

/// CPI: driftSupplyStrategyDeposit
#[inline(always)]
pub fn drift_supply_strategy_deposit<'a>(
    accounts: &DriftsupplystrategydepositAccounts<'a>, args: &DriftsupplystrategydepositArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<DriftsupplystrategydepositArgs>()];
    data[0..8].copy_from_slice(&DRIFT_SUPPLY_STRATEGY_DEPOSIT);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const DriftsupplystrategydepositArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<DriftsupplystrategydepositArgs>(),
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

/// CPI: driftSupplyStrategyWithdraw
#[inline(always)]
pub fn drift_supply_strategy_withdraw<'a>(
    accounts: &DriftsupplystrategywithdrawAccounts<'a>, args: &DriftsupplystrategywithdrawArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<DriftsupplystrategywithdrawArgs>()];
    data[0..8].copy_from_slice(&DRIFT_SUPPLY_STRATEGY_WITHDRAW);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const DriftsupplystrategywithdrawArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<DriftsupplystrategywithdrawArgs>(),
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

/// CPI: driftInsuranceFundStrategyInit
#[inline(always)]
pub fn drift_insurance_fund_strategy_init<'a>(
    accounts: &DriftinsurancefundstrategyinitAccounts<'a>, args: &DriftinsurancefundstrategyinitArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<DriftinsurancefundstrategyinitArgs>()];
    data[0..8].copy_from_slice(&DRIFT_INSURANCE_FUND_STRATEGY_INIT);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const DriftinsurancefundstrategyinitArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<DriftinsurancefundstrategyinitArgs>(),
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

/// CPI: driftInsuranceFundStrategyStake
#[inline(always)]
pub fn drift_insurance_fund_strategy_stake<'a>(
    accounts: &DriftinsurancefundstrategystakeAccounts<'a>, args: &DriftinsurancefundstrategystakeArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<DriftinsurancefundstrategystakeArgs>()];
    data[0..8].copy_from_slice(&DRIFT_INSURANCE_FUND_STRATEGY_STAKE);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const DriftinsurancefundstrategystakeArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<DriftinsurancefundstrategystakeArgs>(),
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

/// CPI: driftInsuranceFundStrategyUnstake
#[inline(always)]
pub fn drift_insurance_fund_strategy_unstake<'a>(
    accounts: &DriftinsurancefundstrategyunstakeAccounts<'a>, args: &DriftinsurancefundstrategyunstakeArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<DriftinsurancefundstrategyunstakeArgs>()];
    data[0..8].copy_from_slice(&DRIFT_INSURANCE_FUND_STRATEGY_UNSTAKE);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const DriftinsurancefundstrategyunstakeArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<DriftinsurancefundstrategyunstakeArgs>(),
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

/// CPI: driftInsuranceFundStrategyWithdraw
#[inline(always)]
pub fn drift_insurance_fund_strategy_withdraw<'a>(
    accounts: &DriftinsurancefundstrategywithdrawAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&DRIFT_INSURANCE_FUND_STRATEGY_WITHDRAW);
    
    
    // Build InstructionAccount array
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

/// CPI: clendSupplyStrategyInit
#[inline(always)]
pub fn clend_supply_strategy_init<'a>(
    accounts: &ClendsupplystrategyinitAccounts<'a>, args: &ClendsupplystrategyinitArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<ClendsupplystrategyinitArgs>()];
    data[0..8].copy_from_slice(&CLEND_SUPPLY_STRATEGY_INIT);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const ClendsupplystrategyinitArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<ClendsupplystrategyinitArgs>(),
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

/// CPI: clendSupplyStrategyDeposit
#[inline(always)]
pub fn clend_supply_strategy_deposit<'a>(
    accounts: &ClendsupplystrategydepositAccounts<'a>, args: &ClendsupplystrategydepositArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<ClendsupplystrategydepositArgs>()];
    data[0..8].copy_from_slice(&CLEND_SUPPLY_STRATEGY_DEPOSIT);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const ClendsupplystrategydepositArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<ClendsupplystrategydepositArgs>(),
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

/// CPI: clendSupplyStrategyWithdraw
#[inline(always)]
pub fn clend_supply_strategy_withdraw<'a>(
    accounts: &ClendsupplystrategywithdrawAccounts<'a>, args: &ClendsupplystrategywithdrawArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<ClendsupplystrategywithdrawArgs>()];
    data[0..8].copy_from_slice(&CLEND_SUPPLY_STRATEGY_WITHDRAW);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const ClendsupplystrategywithdrawArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<ClendsupplystrategywithdrawArgs>(),
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

/// CPI: chestStrategyInit
#[inline(always)]
pub fn chest_strategy_init<'a>(
    accounts: &CheststrategyinitAccounts<'a>, args: &CheststrategyinitArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<CheststrategyinitArgs>()];
    data[0..8].copy_from_slice(&CHEST_STRATEGY_INIT);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const CheststrategyinitArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<CheststrategyinitArgs>(),
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

/// CPI: chestStrategyDeposit
#[inline(always)]
pub fn chest_strategy_deposit<'a>(
    accounts: &CheststrategydepositAccounts<'a>, args: &CheststrategydepositArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<CheststrategydepositArgs>()];
    data[0..8].copy_from_slice(&CHEST_STRATEGY_DEPOSIT);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const CheststrategydepositArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<CheststrategydepositArgs>(),
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

/// CPI: chestStrategyRequestWithdraw
#[inline(always)]
pub fn chest_strategy_request_withdraw<'a>(
    accounts: &CheststrategyrequestwithdrawAccounts<'a>, args: &CheststrategyrequestwithdrawArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<CheststrategyrequestwithdrawArgs>()];
    data[0..8].copy_from_slice(&CHEST_STRATEGY_REQUEST_WITHDRAW);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const CheststrategyrequestwithdrawArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<CheststrategyrequestwithdrawArgs>(),
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
    invoke_signed::<17>(&instruction, &account_views, signers)
}

/// CPI: chestStrategyWithdraw
#[inline(always)]
pub fn chest_strategy_withdraw<'a>(
    accounts: &CheststrategywithdrawAccounts<'a>, args: &CheststrategywithdrawArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<CheststrategywithdrawArgs>()];
    data[0..8].copy_from_slice(&CHEST_STRATEGY_WITHDRAW);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const CheststrategywithdrawArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<CheststrategywithdrawArgs>(),
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
    invoke_signed::<17>(&instruction, &account_views, signers)
}

/// CPI: updateSwitchboardOraclePrice
#[inline(always)]
pub fn update_switchboard_oracle_price<'a>(
    accounts: &UpdateswitchboardoraclepriceAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&UPDATE_SWITCHBOARD_ORACLE_PRICE);
    
    
    // Build InstructionAccount array
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

