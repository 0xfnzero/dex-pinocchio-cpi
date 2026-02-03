//! CPI module for Heaven
//!
//! Program: heaven_dex
//! Program ID: HEAVENoP2qxoeuF8Dj2oT1GHEnu49U5mJYkdeC8BAX2o
//! Instructions: 2

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
pub const ID: Address = Address::new_from_array(five8_const::decode_32_const("HEAVENoP2qxoeuF8Dj2oT1GHEnu49U5mJYkdeC8BAX2o"));

// ============================================
// Instruction Discriminators
// ============================================
pub const BUY: [u8; 8] = [102, 6, 61, 18, 1, 218, 235, 234];
pub const SELL: [u8; 8] = [51, 230, 133, 164, 1, 127, 131, 173];

// ============================================
// Instruction Arguments
// ============================================
/// Arguments for `buy` / `sell` (SwapParams)
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SwapArgs {
    /// Exact input amount
    pub exact_in_amount: u64,
    /// Minimum output amount (slippage protection)
    pub minimum_out_amount: u64,
}

/// Alias for buy
pub type BuyArgs = SwapArgs;

/// Alias for sell
pub type SellArgs = SwapArgs;


// ============================================
// Instruction Accounts
// ============================================
/// Accounts for `buy`
pub struct BuyAccounts<'a> {
    /// Token program for token A
    pub token_a_program: &'a AccountView,
    /// Token program for token B
    pub token_b_program: &'a AccountView,
    /// associated_token_program
    pub associated_token_program: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
    /// Pool state account
    pub liquidity_pool_state: &'a AccountView,
    /// User performing the swap
    pub user: &'a AccountView,
    /// Token A mint
    pub token_a_mint: &'a AccountView,
    /// Token B mint
    pub token_b_mint: &'a AccountView,
    /// User's token A account
    pub user_token_a_vault: &'a AccountView,
    /// User's token B account
    pub user_token_b_vault: &'a AccountView,
    /// Pool's token A vault
    pub token_a_vault: &'a AccountView,
    /// Pool's token B vault
    pub token_b_vault: &'a AccountView,
    /// Protocol configuration
    pub protocol_config: &'a AccountView,
    /// Sysvar instructions
    pub instruction_sysvar_account_info: &'a AccountView,
}

impl<'a> BuyAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 14] {
        [
            InstructionAccount::readonly(self.token_a_program.address()),
            InstructionAccount::readonly(self.token_b_program.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::writable(self.liquidity_pool_state.address()),
            InstructionAccount::readonly_signer(self.user.address()),
            InstructionAccount::readonly(self.token_a_mint.address()),
            InstructionAccount::readonly(self.token_b_mint.address()),
            InstructionAccount::writable(self.user_token_a_vault.address()),
            InstructionAccount::writable(self.user_token_b_vault.address()),
            InstructionAccount::writable(self.token_a_vault.address()),
            InstructionAccount::writable(self.token_b_vault.address()),
            InstructionAccount::writable(self.protocol_config.address()),
            InstructionAccount::readonly(self.instruction_sysvar_account_info.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 14] {
        [
            self.token_a_program, self.token_b_program, self.associated_token_program, self.system_program, self.liquidity_pool_state, self.user, self.token_a_mint, self.token_b_mint, self.user_token_a_vault, self.user_token_b_vault, self.token_a_vault, self.token_b_vault, self.protocol_config, self.instruction_sysvar_account_info
        ]
    }
}

/// Accounts for `sell`
pub struct SellAccounts<'a> {
    /// Token program for token A
    pub token_a_program: &'a AccountView,
    /// Token program for token B
    pub token_b_program: &'a AccountView,
    /// associated_token_program
    pub associated_token_program: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
    /// Pool state account
    pub liquidity_pool_state: &'a AccountView,
    /// User performing the swap
    pub user: &'a AccountView,
    /// Token A mint
    pub token_a_mint: &'a AccountView,
    /// Token B mint
    pub token_b_mint: &'a AccountView,
    /// User's token A account
    pub user_token_a_vault: &'a AccountView,
    /// User's token B account
    pub user_token_b_vault: &'a AccountView,
    /// Pool's token A vault
    pub token_a_vault: &'a AccountView,
    /// Pool's token B vault
    pub token_b_vault: &'a AccountView,
    /// Protocol configuration
    pub protocol_config: &'a AccountView,
    /// Sysvar instructions
    pub instruction_sysvar_account_info: &'a AccountView,
}

impl<'a> SellAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 14] {
        [
            InstructionAccount::readonly(self.token_a_program.address()),
            InstructionAccount::readonly(self.token_b_program.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::writable(self.liquidity_pool_state.address()),
            InstructionAccount::readonly_signer(self.user.address()),
            InstructionAccount::readonly(self.token_a_mint.address()),
            InstructionAccount::readonly(self.token_b_mint.address()),
            InstructionAccount::writable(self.user_token_a_vault.address()),
            InstructionAccount::writable(self.user_token_b_vault.address()),
            InstructionAccount::writable(self.token_a_vault.address()),
            InstructionAccount::writable(self.token_b_vault.address()),
            InstructionAccount::writable(self.protocol_config.address()),
            InstructionAccount::readonly(self.instruction_sysvar_account_info.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 14] {
        [
            self.token_a_program, self.token_b_program, self.associated_token_program, self.system_program, self.liquidity_pool_state, self.user, self.token_a_mint, self.token_b_mint, self.user_token_a_vault, self.user_token_b_vault, self.token_a_vault, self.token_b_vault, self.protocol_config, self.instruction_sysvar_account_info
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
    invoke_signed::<14>(&instruction, &account_views, signers)
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

