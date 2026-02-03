//! CPI module for Saber Decimals
//!
//! Program: add_decimals
//! Program ID: DecZY86MU5Gj7kppfUCEmd4LbXXuyZH1yHaP2NTqdiZB
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
pub const ID: Address = Address::new_from_array(five8_const::decode_32_const("DecZY86MU5Gj7kppfUCEmd4LbXXuyZH1yHaP2NTqdiZB"));

// ============================================
// Instruction Discriminators
// ============================================
pub const INITIALIZE_WRAPPER: [u8; 8] = [143, 211, 228, 247, 131, 67, 40, 30];
pub const DEPOSIT: [u8; 8] = [242, 35, 198, 137, 82, 225, 242, 182];
pub const WITHDRAW: [u8; 8] = [183, 18, 70, 156, 148, 109, 161, 34];

// ============================================
// Instruction Arguments
// ============================================
/// Arguments for `initializeWrapper`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct InitializewrapperArgs {
    pub nonce: u8,
}

/// Arguments for `deposit`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct DepositArgs {
    pub deposit_amount: u64,
}

/// Arguments for `withdraw`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct WithdrawArgs {
    pub max_burn_amount: u64,
}


// ============================================
// Instruction Accounts
// ============================================
/// Accounts for `initializeWrapper`
pub struct InitializewrapperAccounts<'a> {
    /// wrapper
    pub wrapper: &'a AccountView,
    /// wrapperUnderlyingTokens
    pub wrapper_underlying_tokens: &'a AccountView,
    /// underlyingMint
    pub underlying_mint: &'a AccountView,
    /// wrapperMint
    pub wrapper_mint: &'a AccountView,
    /// payer
    pub payer: &'a AccountView,
    /// rent
    pub rent: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
}

impl<'a> InitializewrapperAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 7] {
        [
            InstructionAccount::writable(self.wrapper.address()),
            InstructionAccount::readonly(self.wrapper_underlying_tokens.address()),
            InstructionAccount::readonly(self.underlying_mint.address()),
            InstructionAccount::readonly(self.wrapper_mint.address()),
            InstructionAccount::readonly_signer(self.payer.address()),
            InstructionAccount::readonly(self.rent.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 7] {
        [
            self.wrapper, self.wrapper_underlying_tokens, self.underlying_mint, self.wrapper_mint, self.payer, self.rent, self.system_program
        ]
    }
}

/// Accounts for `deposit`
pub struct DepositAccounts<'a> {
    /// wrapper
    pub wrapper: &'a AccountView,
    /// wrapperMint
    pub wrapper_mint: &'a AccountView,
    /// wrapperUnderlyingTokens
    pub wrapper_underlying_tokens: &'a AccountView,
    /// owner
    pub owner: &'a AccountView,
    /// userUnderlyingTokens
    pub user_underlying_tokens: &'a AccountView,
    /// userWrappedTokens
    pub user_wrapped_tokens: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
}

impl<'a> DepositAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 7] {
        [
            InstructionAccount::readonly(self.wrapper.address()),
            InstructionAccount::writable(self.wrapper_mint.address()),
            InstructionAccount::writable(self.wrapper_underlying_tokens.address()),
            InstructionAccount::readonly_signer(self.owner.address()),
            InstructionAccount::writable(self.user_underlying_tokens.address()),
            InstructionAccount::writable(self.user_wrapped_tokens.address()),
            InstructionAccount::readonly(self.token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 7] {
        [
            self.wrapper, self.wrapper_mint, self.wrapper_underlying_tokens, self.owner, self.user_underlying_tokens, self.user_wrapped_tokens, self.token_program
        ]
    }
}

/// Accounts for `withdraw`
pub struct WithdrawAccounts<'a> {
    /// wrapper
    pub wrapper: &'a AccountView,
    /// wrapperMint
    pub wrapper_mint: &'a AccountView,
    /// wrapperUnderlyingTokens
    pub wrapper_underlying_tokens: &'a AccountView,
    /// owner
    pub owner: &'a AccountView,
    /// userUnderlyingTokens
    pub user_underlying_tokens: &'a AccountView,
    /// userWrappedTokens
    pub user_wrapped_tokens: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
}

impl<'a> WithdrawAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 7] {
        [
            InstructionAccount::readonly(self.wrapper.address()),
            InstructionAccount::writable(self.wrapper_mint.address()),
            InstructionAccount::writable(self.wrapper_underlying_tokens.address()),
            InstructionAccount::readonly_signer(self.owner.address()),
            InstructionAccount::writable(self.user_underlying_tokens.address()),
            InstructionAccount::writable(self.user_wrapped_tokens.address()),
            InstructionAccount::readonly(self.token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 7] {
        [
            self.wrapper, self.wrapper_mint, self.wrapper_underlying_tokens, self.owner, self.user_underlying_tokens, self.user_wrapped_tokens, self.token_program
        ]
    }
}


// ============================================
// CPI Functions
// ============================================
/// CPI: initializeWrapper
#[inline(always)]
pub fn initialize_wrapper<'a>(
    accounts: &InitializewrapperAccounts<'a>, args: &InitializewrapperArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<InitializewrapperArgs>()];
    data[0..8].copy_from_slice(&INITIALIZE_WRAPPER);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const InitializewrapperArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<InitializewrapperArgs>(),
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
    invoke_signed::<7>(&instruction, &account_views, signers)
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
    invoke_signed::<7>(&instruction, &account_views, signers)
}

