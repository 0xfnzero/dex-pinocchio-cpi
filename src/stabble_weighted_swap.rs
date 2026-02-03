//! CPI module for Stabble Weighted Swap
//!
//! Program: weighted_swap
//! Program ID: swapFpHZwjELNnjvThjajtiVmkz3yPQEHjLtka2fwHW
//! Instructions: 13

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
pub const ID: Address = Address::new_from_array(five8_const::decode_32_const("swapFpHZwjELNnjvThjajtiVmkz3yPQEHjLtka2fwHW"));

// ============================================
// Instruction Discriminators
// ============================================
pub const ACCEPT_OWNER: [u8; 8] = [176, 23, 41, 28, 23, 111, 8, 4];
pub const CHANGE_MAX_SUPPLY: [u8; 8] = [93, 176, 0, 205, 69, 63, 87, 80];
pub const CHANGE_SWAP_FEE: [u8; 8] = [231, 15, 132, 51, 132, 165, 64, 170];
pub const DEPOSIT: [u8; 8] = [242, 35, 198, 137, 82, 225, 242, 182];
pub const INITIALIZE: [u8; 8] = [175, 175, 109, 31, 13, 152, 155, 237];
pub const PAUSE: [u8; 8] = [211, 22, 221, 251, 74, 121, 193, 47];
pub const REJECT_OWNER: [u8; 8] = [238, 206, 198, 215, 51, 178, 133, 228];
pub const SHUTDOWN: [u8; 8] = [146, 204, 241, 213, 86, 21, 253, 211];
pub const SWAP: [u8; 8] = [248, 198, 158, 145, 225, 117, 135, 200];
pub const SWAP_V2: [u8; 8] = [43, 4, 237, 11, 26, 201, 30, 98];
pub const TRANSFER_OWNER: [u8; 8] = [245, 25, 221, 175, 106, 229, 225, 45];
pub const UNPAUSE: [u8; 8] = [169, 144, 4, 38, 10, 141, 188, 255];
pub const WITHDRAW: [u8; 8] = [183, 18, 70, 156, 148, 109, 161, 34];

// ============================================
// Instruction Arguments
// ============================================
/// Arguments for `change_max_supply`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct ChangeMaxSupplyArgs {
    pub new_max_supply: u64,
}

/// Arguments for `change_swap_fee`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct ChangeSwapFeeArgs {
    pub new_swap_fee: u64,
}

/// Arguments for `deposit`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct DepositArgs {
    pub amounts: [u8; 32],
    pub minimum_amount_out: u64,
}

/// Arguments for `initialize`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct InitializeArgs {
    pub swap_fee: u64,
    pub weights: [u8; 32],
    pub max_caps: [u8; 32],
}

/// Arguments for `swap`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SwapArgs {
    pub amount_in: Option<u64>,
    pub minimum_amount_out: u64,
}

/// Arguments for `swap_v2`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SwapV2Args {
    pub amount_in: Option<u64>,
    pub minimum_amount_out: u64,
}

/// Arguments for `transfer_owner`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct TransferOwnerArgs {
    pub new_owner: [u8; 32],
}

/// Arguments for `withdraw`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct WithdrawArgs {
    pub amount: u64,
    pub minimum_amounts_out: [u8; 32],
}


// ============================================
// Instruction Accounts
// ============================================
/// Accounts for `accept_owner`
pub struct AcceptOwnerAccounts<'a> {
    /// pending_owner
    pub pending_owner: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
}

impl<'a> AcceptOwnerAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 2] {
        [
            InstructionAccount::readonly_signer(self.pending_owner.address()),
            InstructionAccount::writable(self.pool.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 2] {
        [
            self.pending_owner, self.pool
        ]
    }
}

/// Accounts for `change_max_supply`
pub struct ChangeMaxSupplyAccounts<'a> {
    /// owner
    pub owner: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
}

impl<'a> ChangeMaxSupplyAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 2] {
        [
            InstructionAccount::readonly_signer(self.owner.address()),
            InstructionAccount::writable(self.pool.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 2] {
        [
            self.owner, self.pool
        ]
    }
}

/// Accounts for `change_swap_fee`
pub struct ChangeSwapFeeAccounts<'a> {
    /// owner
    pub owner: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
}

impl<'a> ChangeSwapFeeAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 2] {
        [
            InstructionAccount::readonly_signer(self.owner.address()),
            InstructionAccount::writable(self.pool.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 2] {
        [
            self.owner, self.pool
        ]
    }
}

/// Accounts for `deposit`
pub struct DepositAccounts<'a> {
    /// user
    pub user: &'a AccountView,
    /// user_pool_token
    pub user_pool_token: &'a AccountView,
    /// mint
    pub mint: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// pool_authority
    pub pool_authority: &'a AccountView,
    /// vault
    pub vault: &'a AccountView,
    /// vault_authority
    pub vault_authority: &'a AccountView,
    /// token_program
    pub token_program: &'a AccountView,
    /// token_program_2022
    pub token_program_2022: &'a AccountView,
}

impl<'a> DepositAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 9] {
        [
            InstructionAccount::readonly_signer(self.user.address()),
            InstructionAccount::writable(self.user_pool_token.address()),
            InstructionAccount::writable(self.mint.address()),
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::readonly(self.pool_authority.address()),
            InstructionAccount::readonly(self.vault.address()),
            InstructionAccount::readonly(self.vault_authority.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.token_program_2022.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 9] {
        [
            self.user, self.user_pool_token, self.mint, self.pool, self.pool_authority, self.vault, self.vault_authority, self.token_program, self.token_program_2022
        ]
    }
}

/// Accounts for `initialize`
pub struct InitializeAccounts<'a> {
    /// owner
    pub owner: &'a AccountView,
    /// mint
    pub mint: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// pool_authority
    pub pool_authority: &'a AccountView,
    /// withdraw_authority
    pub withdraw_authority: &'a AccountView,
    /// vault
    pub vault: &'a AccountView,
}

impl<'a> InitializeAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 6] {
        [
            InstructionAccount::readonly_signer(self.owner.address()),
            InstructionAccount::readonly(self.mint.address()),
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::readonly(self.pool_authority.address()),
            InstructionAccount::readonly(self.withdraw_authority.address()),
            InstructionAccount::readonly(self.vault.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 6] {
        [
            self.owner, self.mint, self.pool, self.pool_authority, self.withdraw_authority, self.vault
        ]
    }
}

/// Accounts for `pause`
pub struct PauseAccounts<'a> {
    /// owner
    pub owner: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
}

impl<'a> PauseAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 2] {
        [
            InstructionAccount::readonly_signer(self.owner.address()),
            InstructionAccount::writable(self.pool.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 2] {
        [
            self.owner, self.pool
        ]
    }
}

/// Accounts for `reject_owner`
pub struct RejectOwnerAccounts<'a> {
    /// pending_owner
    pub pending_owner: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
}

impl<'a> RejectOwnerAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 2] {
        [
            InstructionAccount::readonly_signer(self.pending_owner.address()),
            InstructionAccount::writable(self.pool.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 2] {
        [
            self.pending_owner, self.pool
        ]
    }
}

/// Accounts for `shutdown`
pub struct ShutdownAccounts<'a> {
    /// owner
    pub owner: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
}

impl<'a> ShutdownAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 2] {
        [
            InstructionAccount::writable(self.owner.address()),
            InstructionAccount::writable(self.pool.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 2] {
        [
            self.owner, self.pool
        ]
    }
}

/// Accounts for `swap`
pub struct SwapAccounts<'a> {
    /// user
    pub user: &'a AccountView,
    /// user_token_in
    pub user_token_in: &'a AccountView,
    /// user_token_out
    pub user_token_out: &'a AccountView,
    /// vault_token_in
    pub vault_token_in: &'a AccountView,
    /// vault_token_out
    pub vault_token_out: &'a AccountView,
    /// beneficiary_token_out
    pub beneficiary_token_out: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// withdraw_authority
    pub withdraw_authority: &'a AccountView,
    /// vault
    pub vault: &'a AccountView,
    /// vault_authority
    pub vault_authority: &'a AccountView,
    /// vault_program
    pub vault_program: &'a AccountView,
    /// token_program
    pub token_program: &'a AccountView,
}

impl<'a> SwapAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 12] {
        [
            InstructionAccount::readonly_signer(self.user.address()),
            InstructionAccount::writable(self.user_token_in.address()),
            InstructionAccount::writable(self.user_token_out.address()),
            InstructionAccount::writable(self.vault_token_in.address()),
            InstructionAccount::writable(self.vault_token_out.address()),
            InstructionAccount::writable(self.beneficiary_token_out.address()),
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::readonly(self.withdraw_authority.address()),
            InstructionAccount::readonly(self.vault.address()),
            InstructionAccount::readonly(self.vault_authority.address()),
            InstructionAccount::readonly(self.vault_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 12] {
        [
            self.user, self.user_token_in, self.user_token_out, self.vault_token_in, self.vault_token_out, self.beneficiary_token_out, self.pool, self.withdraw_authority, self.vault, self.vault_authority, self.vault_program, self.token_program
        ]
    }
}

/// Accounts for `swap_v2`
pub struct SwapV2Accounts<'a> {
    /// user
    pub user: &'a AccountView,
    /// mint_in
    pub mint_in: &'a AccountView,
    /// mint_out
    pub mint_out: &'a AccountView,
    /// user_token_in
    pub user_token_in: &'a AccountView,
    /// user_token_out
    pub user_token_out: &'a AccountView,
    /// vault_token_in
    pub vault_token_in: &'a AccountView,
    /// vault_token_out
    pub vault_token_out: &'a AccountView,
    /// beneficiary_token_out
    pub beneficiary_token_out: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// withdraw_authority
    pub withdraw_authority: &'a AccountView,
    /// vault
    pub vault: &'a AccountView,
    /// vault_authority
    pub vault_authority: &'a AccountView,
    /// vault_program
    pub vault_program: &'a AccountView,
    /// token_program
    pub token_program: &'a AccountView,
    /// token_2022_program
    pub token_2022_program: &'a AccountView,
}

impl<'a> SwapV2Accounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 15] {
        [
            InstructionAccount::readonly_signer(self.user.address()),
            InstructionAccount::readonly(self.mint_in.address()),
            InstructionAccount::readonly(self.mint_out.address()),
            InstructionAccount::writable(self.user_token_in.address()),
            InstructionAccount::writable(self.user_token_out.address()),
            InstructionAccount::writable(self.vault_token_in.address()),
            InstructionAccount::writable(self.vault_token_out.address()),
            InstructionAccount::writable(self.beneficiary_token_out.address()),
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::readonly(self.withdraw_authority.address()),
            InstructionAccount::readonly(self.vault.address()),
            InstructionAccount::readonly(self.vault_authority.address()),
            InstructionAccount::readonly(self.vault_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.token_2022_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 15] {
        [
            self.user, self.mint_in, self.mint_out, self.user_token_in, self.user_token_out, self.vault_token_in, self.vault_token_out, self.beneficiary_token_out, self.pool, self.withdraw_authority, self.vault, self.vault_authority, self.vault_program, self.token_program, self.token_2022_program
        ]
    }
}

/// Accounts for `transfer_owner`
pub struct TransferOwnerAccounts<'a> {
    /// owner
    pub owner: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
}

impl<'a> TransferOwnerAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 2] {
        [
            InstructionAccount::readonly_signer(self.owner.address()),
            InstructionAccount::writable(self.pool.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 2] {
        [
            self.owner, self.pool
        ]
    }
}

/// Accounts for `unpause`
pub struct UnpauseAccounts<'a> {
    /// owner
    pub owner: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
}

impl<'a> UnpauseAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 2] {
        [
            InstructionAccount::readonly_signer(self.owner.address()),
            InstructionAccount::writable(self.pool.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 2] {
        [
            self.owner, self.pool
        ]
    }
}

/// Accounts for `withdraw`
pub struct WithdrawAccounts<'a> {
    /// user
    pub user: &'a AccountView,
    /// user_pool_token
    pub user_pool_token: &'a AccountView,
    /// mint
    pub mint: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// withdraw_authority
    pub withdraw_authority: &'a AccountView,
    /// vault
    pub vault: &'a AccountView,
    /// vault_authority
    pub vault_authority: &'a AccountView,
    /// vault_program
    pub vault_program: &'a AccountView,
    /// token_program
    pub token_program: &'a AccountView,
    /// token_program_2022
    pub token_program_2022: &'a AccountView,
}

impl<'a> WithdrawAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 10] {
        [
            InstructionAccount::readonly_signer(self.user.address()),
            InstructionAccount::writable(self.user_pool_token.address()),
            InstructionAccount::writable(self.mint.address()),
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::readonly(self.withdraw_authority.address()),
            InstructionAccount::readonly(self.vault.address()),
            InstructionAccount::readonly(self.vault_authority.address()),
            InstructionAccount::readonly(self.vault_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.token_program_2022.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 10] {
        [
            self.user, self.user_pool_token, self.mint, self.pool, self.withdraw_authority, self.vault, self.vault_authority, self.vault_program, self.token_program, self.token_program_2022
        ]
    }
}


// ============================================
// CPI Functions
// ============================================
/// CPI: accept_owner
#[inline(always)]
pub fn accept_owner<'a>(
    accounts: &AcceptOwnerAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&ACCEPT_OWNER);
    
    
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

/// CPI: change_max_supply
#[inline(always)]
pub fn change_max_supply<'a>(
    accounts: &ChangeMaxSupplyAccounts<'a>, args: &ChangeMaxSupplyArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<ChangeMaxSupplyArgs>()];
    data[0..8].copy_from_slice(&CHANGE_MAX_SUPPLY);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const ChangeMaxSupplyArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<ChangeMaxSupplyArgs>(),
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

/// CPI: change_swap_fee
#[inline(always)]
pub fn change_swap_fee<'a>(
    accounts: &ChangeSwapFeeAccounts<'a>, args: &ChangeSwapFeeArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<ChangeSwapFeeArgs>()];
    data[0..8].copy_from_slice(&CHANGE_SWAP_FEE);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const ChangeSwapFeeArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<ChangeSwapFeeArgs>(),
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
    invoke_signed::<9>(&instruction, &account_views, signers)
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
    invoke_signed::<6>(&instruction, &account_views, signers)
}

/// CPI: pause
#[inline(always)]
pub fn pause<'a>(
    accounts: &PauseAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&PAUSE);
    
    
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

/// CPI: reject_owner
#[inline(always)]
pub fn reject_owner<'a>(
    accounts: &RejectOwnerAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&REJECT_OWNER);
    
    
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

/// CPI: shutdown
#[inline(always)]
pub fn shutdown<'a>(
    accounts: &ShutdownAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&SHUTDOWN);
    
    
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
    invoke_signed::<12>(&instruction, &account_views, signers)
}

/// CPI: swap_v2
#[inline(always)]
pub fn swap_v2<'a>(
    accounts: &SwapV2Accounts<'a>, args: &SwapV2Args,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SwapV2Args>()];
    data[0..8].copy_from_slice(&SWAP_V2);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SwapV2Args as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SwapV2Args>(),
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

/// CPI: transfer_owner
#[inline(always)]
pub fn transfer_owner<'a>(
    accounts: &TransferOwnerAccounts<'a>, args: &TransferOwnerArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<TransferOwnerArgs>()];
    data[0..8].copy_from_slice(&TRANSFER_OWNER);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const TransferOwnerArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<TransferOwnerArgs>(),
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

/// CPI: unpause
#[inline(always)]
pub fn unpause<'a>(
    accounts: &UnpauseAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&UNPAUSE);
    
    
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
    invoke_signed::<10>(&instruction, &account_views, signers)
}

