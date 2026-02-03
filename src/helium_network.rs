//! CPI module for Helium Network
//!
//! Program: treasury_management
//! Program ID: treaf4wWBBty3fHdyBpo35Mz84M8k3heKXmjmi9vFt5
//! Instructions: 3

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
pub const ID: Address = Address::new_from_array(five8_const::decode_32_const("treaf4wWBBty3fHdyBpo35Mz84M8k3heKXmjmi9vFt5"));

// ============================================
// Instruction Discriminators
// ============================================
pub const INITIALIZE_TREASURY_MANAGEMENT_V0: [u8; 8] = [149, 3, 201, 108, 130, 56, 56, 210];
pub const REDEEM_V0: [u8; 8] = [235, 127, 171, 139, 119, 77, 235, 118];
pub const UPDATE_TREASURY_MANAGEMENT_V0: [u8; 8] = [209, 139, 90, 226, 249, 149, 89, 217];

// ============================================
// Instruction Arguments
// ============================================
/// Arguments for `initialize_treasury_management_v0`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct InitializeTreasuryManagementV0Args {
    pub args: [u8; 32],
}

/// Arguments for `redeem_v0`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct RedeemV0Args {
    pub args: [u8; 32],
}

/// Arguments for `update_treasury_management_v0`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct UpdateTreasuryManagementV0Args {
    pub args: [u8; 32],
}


// ============================================
// Instruction Accounts
// ============================================
/// Accounts for `initialize_treasury_management_v0`
pub struct InitializeTreasuryManagementV0Accounts<'a> {
    /// payer
    pub payer: &'a AccountView,
    /// treasury_management
    pub treasury_management: &'a AccountView,
    /// treasury_mint
    pub treasury_mint: &'a AccountView,
    /// supply_mint
    pub supply_mint: &'a AccountView,
    /// mint_authority
    pub mint_authority: &'a AccountView,
    /// circuit_breaker
    pub circuit_breaker: &'a AccountView,
    /// treasury
    pub treasury: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
    /// circuit_breaker_program
    pub circuit_breaker_program: &'a AccountView,
    /// associated_token_program
    pub associated_token_program: &'a AccountView,
    /// token_program
    pub token_program: &'a AccountView,
}

impl<'a> InitializeTreasuryManagementV0Accounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 11] {
        [
            InstructionAccount::writable_signer(self.payer.address()),
            InstructionAccount::writable(self.treasury_management.address()),
            InstructionAccount::readonly(self.treasury_mint.address()),
            InstructionAccount::readonly(self.supply_mint.address()),
            InstructionAccount::readonly_signer(self.mint_authority.address()),
            InstructionAccount::writable(self.circuit_breaker.address()),
            InstructionAccount::writable(self.treasury.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.circuit_breaker_program.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 11] {
        [
            self.payer, self.treasury_management, self.treasury_mint, self.supply_mint, self.mint_authority, self.circuit_breaker, self.treasury, self.system_program, self.circuit_breaker_program, self.associated_token_program, self.token_program
        ]
    }
}

/// Accounts for `redeem_v0`
pub struct RedeemV0Accounts<'a> {
    /// treasury_management
    pub treasury_management: &'a AccountView,
    /// treasury_mint
    pub treasury_mint: &'a AccountView,
    /// supply_mint
    pub supply_mint: &'a AccountView,
    /// treasury
    pub treasury: &'a AccountView,
    /// circuit_breaker
    pub circuit_breaker: &'a AccountView,
    /// from
    pub from: &'a AccountView,
    /// to
    pub to: &'a AccountView,
    /// owner
    pub owner: &'a AccountView,
    /// circuit_breaker_program
    pub circuit_breaker_program: &'a AccountView,
    /// token_program
    pub token_program: &'a AccountView,
}

impl<'a> RedeemV0Accounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 10] {
        [
            InstructionAccount::readonly(self.treasury_management.address()),
            InstructionAccount::readonly(self.treasury_mint.address()),
            InstructionAccount::writable(self.supply_mint.address()),
            InstructionAccount::writable(self.treasury.address()),
            InstructionAccount::writable(self.circuit_breaker.address()),
            InstructionAccount::writable(self.from.address()),
            InstructionAccount::writable(self.to.address()),
            InstructionAccount::readonly_signer(self.owner.address()),
            InstructionAccount::readonly(self.circuit_breaker_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 10] {
        [
            self.treasury_management, self.treasury_mint, self.supply_mint, self.treasury, self.circuit_breaker, self.from, self.to, self.owner, self.circuit_breaker_program, self.token_program
        ]
    }
}

/// Accounts for `update_treasury_management_v0`
pub struct UpdateTreasuryManagementV0Accounts<'a> {
    /// treasury_management
    pub treasury_management: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
}

impl<'a> UpdateTreasuryManagementV0Accounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 2] {
        [
            InstructionAccount::writable(self.treasury_management.address()),
            InstructionAccount::readonly_signer(self.authority.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 2] {
        [
            self.treasury_management, self.authority
        ]
    }
}


// ============================================
// CPI Functions
// ============================================
/// CPI: initialize_treasury_management_v0
#[inline(always)]
pub fn initialize_treasury_management_v0<'a>(
    accounts: &InitializeTreasuryManagementV0Accounts<'a>, args: &InitializeTreasuryManagementV0Args,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<InitializeTreasuryManagementV0Args>()];
    data[0..8].copy_from_slice(&INITIALIZE_TREASURY_MANAGEMENT_V0);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const InitializeTreasuryManagementV0Args as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<InitializeTreasuryManagementV0Args>(),
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

/// CPI: redeem_v0
#[inline(always)]
pub fn redeem_v0<'a>(
    accounts: &RedeemV0Accounts<'a>, args: &RedeemV0Args,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<RedeemV0Args>()];
    data[0..8].copy_from_slice(&REDEEM_V0);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const RedeemV0Args as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<RedeemV0Args>(),
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

/// CPI: update_treasury_management_v0
#[inline(always)]
pub fn update_treasury_management_v0<'a>(
    accounts: &UpdateTreasuryManagementV0Accounts<'a>, args: &UpdateTreasuryManagementV0Args,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<UpdateTreasuryManagementV0Args>()];
    data[0..8].copy_from_slice(&UPDATE_TREASURY_MANAGEMENT_V0);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const UpdateTreasuryManagementV0Args as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<UpdateTreasuryManagementV0Args>(),
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

