//! CPI module for Moonit
//!
//! Program: token_launchpad
//! Program ID: MoonCVVNZFSYkqNXP6bxHLPL6QQJiMagDL3qcqUQTrG
//! Instructions: 6

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
pub const ID: Address = Address::new_from_array(five8_const::decode_32_const("MoonCVVNZFSYkqNXP6bxHLPL6QQJiMagDL3qcqUQTrG"));

// ============================================
// Instruction Discriminators
// ============================================
pub const TOKEN_MINT: [u8; 8] = [3, 44, 164, 184, 123, 13, 245, 179];
pub const BUY: [u8; 8] = [102, 6, 61, 18, 1, 218, 235, 234];
pub const SELL: [u8; 8] = [51, 230, 133, 164, 1, 127, 131, 173];
pub const MIGRATE_FUNDS: [u8; 8] = [42, 229, 10, 231, 189, 62, 193, 174];
pub const CONFIG_INIT: [u8; 8] = [13, 236, 164, 173, 106, 253, 164, 185];
pub const CONFIG_UPDATE: [u8; 8] = [80, 37, 109, 136, 82, 135, 89, 241];

// ============================================
// Instruction Arguments
// ============================================
/// Arguments for `tokenMint`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct TokenmintArgs {
    pub mint_params: [u8; 32],
}

/// Arguments for `buy`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct BuyArgs {
    pub data: [u8; 32],
}

/// Arguments for `sell`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SellArgs {
    pub data: [u8; 32],
}

/// Arguments for `configInit`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct ConfiginitArgs {
    pub data: [u8; 32],
}

/// Arguments for `configUpdate`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct ConfigupdateArgs {
    pub data: [u8; 32],
}


// ============================================
// Instruction Accounts
// ============================================
/// Accounts for `tokenMint`
pub struct TokenmintAccounts<'a> {
    /// sender
    pub sender: &'a AccountView,
    /// backendAuthority
    pub backend_authority: &'a AccountView,
    /// curveAccount
    pub curve_account: &'a AccountView,
    /// mint
    pub mint: &'a AccountView,
    /// Type validating that the account is owned by the System Program = uninitialized
    pub mint_metadata: &'a AccountView,
    /// curveTokenAccount
    pub curve_token_account: &'a AccountView,
    /// configAccount
    pub config_account: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// associatedTokenProgram
    pub associated_token_program: &'a AccountView,
    /// mplTokenMetadata
    pub mpl_token_metadata: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
}

impl<'a> TokenmintAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 11] {
        [
            InstructionAccount::writable_signer(self.sender.address()),
            InstructionAccount::readonly_signer(self.backend_authority.address()),
            InstructionAccount::writable(self.curve_account.address()),
            InstructionAccount::writable_signer(self.mint.address()),
            InstructionAccount::writable(self.mint_metadata.address()),
            InstructionAccount::writable(self.curve_token_account.address()),
            InstructionAccount::readonly(self.config_account.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
            InstructionAccount::readonly(self.mpl_token_metadata.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 11] {
        [
            self.sender, self.backend_authority, self.curve_account, self.mint, self.mint_metadata, self.curve_token_account, self.config_account, self.token_program, self.associated_token_program, self.mpl_token_metadata, self.system_program
        ]
    }
}

/// Accounts for `buy`
pub struct BuyAccounts<'a> {
    /// sender
    pub sender: &'a AccountView,
    /// senderTokenAccount
    pub sender_token_account: &'a AccountView,
    /// curveAccount
    pub curve_account: &'a AccountView,
    /// curveTokenAccount
    pub curve_token_account: &'a AccountView,
    /// dexFee
    pub dex_fee: &'a AccountView,
    /// helioFee
    pub helio_fee: &'a AccountView,
    /// mint
    pub mint: &'a AccountView,
    /// configAccount
    pub config_account: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// associatedTokenProgram
    pub associated_token_program: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
}

impl<'a> BuyAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 11] {
        [
            InstructionAccount::writable_signer(self.sender.address()),
            InstructionAccount::writable(self.sender_token_account.address()),
            InstructionAccount::writable(self.curve_account.address()),
            InstructionAccount::writable(self.curve_token_account.address()),
            InstructionAccount::writable(self.dex_fee.address()),
            InstructionAccount::writable(self.helio_fee.address()),
            InstructionAccount::readonly(self.mint.address()),
            InstructionAccount::readonly(self.config_account.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 11] {
        [
            self.sender, self.sender_token_account, self.curve_account, self.curve_token_account, self.dex_fee, self.helio_fee, self.mint, self.config_account, self.token_program, self.associated_token_program, self.system_program
        ]
    }
}

/// Accounts for `sell`
pub struct SellAccounts<'a> {
    /// sender
    pub sender: &'a AccountView,
    /// senderTokenAccount
    pub sender_token_account: &'a AccountView,
    /// curveAccount
    pub curve_account: &'a AccountView,
    /// curveTokenAccount
    pub curve_token_account: &'a AccountView,
    /// dexFee
    pub dex_fee: &'a AccountView,
    /// helioFee
    pub helio_fee: &'a AccountView,
    /// mint
    pub mint: &'a AccountView,
    /// configAccount
    pub config_account: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// associatedTokenProgram
    pub associated_token_program: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
}

impl<'a> SellAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 11] {
        [
            InstructionAccount::writable_signer(self.sender.address()),
            InstructionAccount::writable(self.sender_token_account.address()),
            InstructionAccount::writable(self.curve_account.address()),
            InstructionAccount::writable(self.curve_token_account.address()),
            InstructionAccount::writable(self.dex_fee.address()),
            InstructionAccount::writable(self.helio_fee.address()),
            InstructionAccount::readonly(self.mint.address()),
            InstructionAccount::readonly(self.config_account.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 11] {
        [
            self.sender, self.sender_token_account, self.curve_account, self.curve_token_account, self.dex_fee, self.helio_fee, self.mint, self.config_account, self.token_program, self.associated_token_program, self.system_program
        ]
    }
}

/// Accounts for `migrateFunds`
pub struct MigratefundsAccounts<'a> {
    /// BE Authority
    pub backend_authority: &'a AccountView,
    /// Migration Authority
    pub migration_authority: &'a AccountView,
    /// Curve Account
    pub curve_account: &'a AccountView,
    /// Curve Token Account
    pub curve_token_account: &'a AccountView,
    /// Authority token Account
    pub migration_authority_token_account: &'a AccountView,
    /// InterfaceAccount: checks program ownership + deserialize into Mint
    pub mint: &'a AccountView,
    /// dexFeeAccount
    pub dex_fee_account: &'a AccountView,
    /// helioFeeAccount
    pub helio_fee_account: &'a AccountView,
    /// configAccount
    pub config_account: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// associatedTokenProgram
    pub associated_token_program: &'a AccountView,
}

impl<'a> MigratefundsAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 12] {
        [
            InstructionAccount::readonly_signer(self.backend_authority.address()),
            InstructionAccount::writable_signer(self.migration_authority.address()),
            InstructionAccount::writable(self.curve_account.address()),
            InstructionAccount::writable(self.curve_token_account.address()),
            InstructionAccount::writable(self.migration_authority_token_account.address()),
            InstructionAccount::writable(self.mint.address()),
            InstructionAccount::writable(self.dex_fee_account.address()),
            InstructionAccount::writable(self.helio_fee_account.address()),
            InstructionAccount::readonly(self.config_account.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 12] {
        [
            self.backend_authority, self.migration_authority, self.curve_account, self.curve_token_account, self.migration_authority_token_account, self.mint, self.dex_fee_account, self.helio_fee_account, self.config_account, self.system_program, self.token_program, self.associated_token_program
        ]
    }
}

/// Accounts for `configInit`
pub struct ConfiginitAccounts<'a> {
    /// configAuthority
    pub config_authority: &'a AccountView,
    /// configAccount
    pub config_account: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
}

impl<'a> ConfiginitAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 3] {
        [
            InstructionAccount::writable_signer(self.config_authority.address()),
            InstructionAccount::writable(self.config_account.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 3] {
        [
            self.config_authority, self.config_account, self.system_program
        ]
    }
}

/// Accounts for `configUpdate`
pub struct ConfigupdateAccounts<'a> {
    /// configAuthority
    pub config_authority: &'a AccountView,
    /// configAccount
    pub config_account: &'a AccountView,
}

impl<'a> ConfigupdateAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 2] {
        [
            InstructionAccount::readonly_signer(self.config_authority.address()),
            InstructionAccount::writable(self.config_account.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 2] {
        [
            self.config_authority, self.config_account
        ]
    }
}


// ============================================
// CPI Functions
// ============================================
/// CPI: tokenMint
#[inline(always)]
pub fn token_mint<'a>(
    accounts: &TokenmintAccounts<'a>, args: &TokenmintArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<TokenmintArgs>()];
    data[0..8].copy_from_slice(&TOKEN_MINT);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const TokenmintArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<TokenmintArgs>(),
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
    invoke_signed::<11>(&instruction, &account_views, signers)
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
    invoke_signed::<11>(&instruction, &account_views, signers)
}

/// CPI: migrateFunds
#[inline(always)]
pub fn migrate_funds<'a>(
    accounts: &MigratefundsAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&MIGRATE_FUNDS);
    
    
    // Build InstructionAccount array
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

/// CPI: configInit
#[inline(always)]
pub fn config_init<'a>(
    accounts: &ConfiginitAccounts<'a>, args: &ConfiginitArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<ConfiginitArgs>()];
    data[0..8].copy_from_slice(&CONFIG_INIT);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const ConfiginitArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<ConfiginitArgs>(),
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

/// CPI: configUpdate
#[inline(always)]
pub fn config_update<'a>(
    accounts: &ConfigupdateAccounts<'a>, args: &ConfigupdateArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<ConfigupdateArgs>()];
    data[0..8].copy_from_slice(&CONFIG_UPDATE);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const ConfigupdateArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<ConfigupdateArgs>(),
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

