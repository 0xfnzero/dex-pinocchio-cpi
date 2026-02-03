//! CPI module for Vertigo
//!
//! Program: amm
//! Program ID: vrTGoBuy5rYSxAfV3jaRJWHH6nN9WK4NRExGxsk1bCJ
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
pub const ID: Address = Address::new_from_array(five8_const::decode_32_const("vrTGoBuy5rYSxAfV3jaRJWHH6nN9WK4NRExGxsk1bCJ"));

// ============================================
// Instruction Discriminators
// ============================================
pub const BUY: [u8; 8] = [102, 6, 61, 18, 1, 218, 235, 234];
pub const CLAIM: [u8; 8] = [62, 198, 214, 193, 213, 159, 108, 210];
pub const CREATE: [u8; 8] = [24, 30, 200, 40, 5, 28, 7, 119];
pub const QUOTE_BUY: [u8; 8] = [83, 9, 231, 110, 146, 31, 40, 12];
pub const QUOTE_SELL: [u8; 8] = [5, 178, 49, 206, 140, 231, 131, 145];
pub const SELL: [u8; 8] = [51, 230, 133, 164, 1, 127, 131, 173];

// ============================================
// Instruction Arguments
// ============================================
/// Arguments for `buy` / `sell` (SwapParams)
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SwapArgs {
    /// Amount to swap
    pub amount: u64,
    /// Minimum output amount (slippage protection)
    pub limit: u64,
}

/// Alias for buy
pub type BuyArgs = SwapArgs;

/// Arguments for `create`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct CreateArgs {
    pub params: [u8; 32],
}

/// Arguments for `quote_buy`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct QuoteBuyArgs {
    pub params: [u8; 32],
}

/// Arguments for `quote_sell`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct QuoteSellArgs {
    pub params: [u8; 32],
}

/// Alias for sell
pub type SellArgs = SwapArgs;


// ============================================
// Instruction Accounts
// ============================================
/// Accounts for `buy`
pub struct BuyAccounts<'a> {
    /// pool
    pub pool: &'a AccountView,
    /// user
    pub user: &'a AccountView,
    /// owner
    pub owner: &'a AccountView,
    /// mint_a
    pub mint_a: &'a AccountView,
    /// mint_b
    pub mint_b: &'a AccountView,
    /// Can be any token account owned by the user for this mint
    pub user_ta_a: &'a AccountView,
    /// Can be any token account owned by the user for this mint
    pub user_ta_b: &'a AccountView,
    /// vault_a
    pub vault_a: &'a AccountView,
    /// vault_b
    pub vault_b: &'a AccountView,
    /// token_program_a
    pub token_program_a: &'a AccountView,
    /// token_program_b
    pub token_program_b: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> BuyAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 13] {
        [
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::readonly_signer(self.user.address()),
            InstructionAccount::readonly(self.owner.address()),
            InstructionAccount::readonly(self.mint_a.address()),
            InstructionAccount::readonly(self.mint_b.address()),
            InstructionAccount::writable(self.user_ta_a.address()),
            InstructionAccount::writable(self.user_ta_b.address()),
            InstructionAccount::writable(self.vault_a.address()),
            InstructionAccount::writable(self.vault_b.address()),
            InstructionAccount::readonly(self.token_program_a.address()),
            InstructionAccount::readonly(self.token_program_b.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 13] {
        [
            self.pool, self.user, self.owner, self.mint_a, self.mint_b, self.user_ta_a, self.user_ta_b, self.vault_a, self.vault_b, self.token_program_a, self.token_program_b, self.system_program, self.program
        ]
    }
}

/// Accounts for `claim`
pub struct ClaimAccounts<'a> {
    /// pool
    pub pool: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
    /// claimer
    pub claimer: &'a AccountView,
    /// The token mint of the tokens used in the pool.
    pub mint_a: &'a AccountView,
    /// vault_a
    pub vault_a: &'a AccountView,
    /// Can be any token account owned by the user for this mint
    pub receiver_ta_a: &'a AccountView,
    /// token_program_a
    pub token_program_a: &'a AccountView,
}

impl<'a> ClaimAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 7] {
        [
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly_signer(self.claimer.address()),
            InstructionAccount::readonly(self.mint_a.address()),
            InstructionAccount::writable(self.vault_a.address()),
            InstructionAccount::writable(self.receiver_ta_a.address()),
            InstructionAccount::readonly(self.token_program_a.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 7] {
        [
            self.pool, self.system_program, self.claimer, self.mint_a, self.vault_a, self.receiver_ta_a, self.token_program_a
        ]
    }
}

/// Accounts for `create`
pub struct CreateAccounts<'a> {
    /// payer
    pub payer: &'a AccountView,
    /// owner
    pub owner: &'a AccountView,
    /// token_wallet_authority
    pub token_wallet_authority: &'a AccountView,
    /// mint_a
    pub mint_a: &'a AccountView,
    /// mint_b
    pub mint_b: &'a AccountView,
    /// Token wallet that funds the pool with token B.
    pub token_wallet_b: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// vault_a
    pub vault_a: &'a AccountView,
    /// vault_b
    pub vault_b: &'a AccountView,
    /// Required programs and sysvars
    pub token_program_a: &'a AccountView,
    /// token_program_b
    pub token_program_b: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
    /// rent
    pub rent: &'a AccountView,
}

impl<'a> CreateAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 13] {
        [
            InstructionAccount::writable_signer(self.payer.address()),
            InstructionAccount::readonly_signer(self.owner.address()),
            InstructionAccount::readonly_signer(self.token_wallet_authority.address()),
            InstructionAccount::readonly(self.mint_a.address()),
            InstructionAccount::readonly(self.mint_b.address()),
            InstructionAccount::writable(self.token_wallet_b.address()),
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.vault_a.address()),
            InstructionAccount::writable(self.vault_b.address()),
            InstructionAccount::readonly(self.token_program_a.address()),
            InstructionAccount::readonly(self.token_program_b.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.rent.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 13] {
        [
            self.payer, self.owner, self.token_wallet_authority, self.mint_a, self.mint_b, self.token_wallet_b, self.pool, self.vault_a, self.vault_b, self.token_program_a, self.token_program_b, self.system_program, self.rent
        ]
    }
}

/// Accounts for `quote_buy`
pub struct QuoteBuyAccounts<'a> {
    /// pool
    pub pool: &'a AccountView,
    /// owner
    pub owner: &'a AccountView,
    /// user
    pub user: &'a AccountView,
    /// mint_a
    pub mint_a: &'a AccountView,
    /// mint_b
    pub mint_b: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> QuoteBuyAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 6] {
        [
            InstructionAccount::readonly(self.pool.address()),
            InstructionAccount::readonly(self.owner.address()),
            InstructionAccount::readonly(self.user.address()),
            InstructionAccount::readonly(self.mint_a.address()),
            InstructionAccount::readonly(self.mint_b.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 6] {
        [
            self.pool, self.owner, self.user, self.mint_a, self.mint_b, self.program
        ]
    }
}

/// Accounts for `quote_sell`
pub struct QuoteSellAccounts<'a> {
    /// pool
    pub pool: &'a AccountView,
    /// owner
    pub owner: &'a AccountView,
    /// user
    pub user: &'a AccountView,
    /// mint_a
    pub mint_a: &'a AccountView,
    /// mint_b
    pub mint_b: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> QuoteSellAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 6] {
        [
            InstructionAccount::readonly(self.pool.address()),
            InstructionAccount::readonly(self.owner.address()),
            InstructionAccount::readonly(self.user.address()),
            InstructionAccount::readonly(self.mint_a.address()),
            InstructionAccount::readonly(self.mint_b.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 6] {
        [
            self.pool, self.owner, self.user, self.mint_a, self.mint_b, self.program
        ]
    }
}

/// Accounts for `sell`
pub struct SellAccounts<'a> {
    /// pool
    pub pool: &'a AccountView,
    /// user
    pub user: &'a AccountView,
    /// owner
    pub owner: &'a AccountView,
    /// mint_a
    pub mint_a: &'a AccountView,
    /// mint_b
    pub mint_b: &'a AccountView,
    /// Can be any token account owned by the user for this mint
    pub user_ta_a: &'a AccountView,
    /// Can be any token account owned by the user for this mint
    pub user_ta_b: &'a AccountView,
    /// vault_a
    pub vault_a: &'a AccountView,
    /// vault_b
    pub vault_b: &'a AccountView,
    /// token_program_a
    pub token_program_a: &'a AccountView,
    /// token_program_b
    pub token_program_b: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> SellAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 13] {
        [
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::readonly_signer(self.user.address()),
            InstructionAccount::readonly(self.owner.address()),
            InstructionAccount::readonly(self.mint_a.address()),
            InstructionAccount::readonly(self.mint_b.address()),
            InstructionAccount::writable(self.user_ta_a.address()),
            InstructionAccount::writable(self.user_ta_b.address()),
            InstructionAccount::writable(self.vault_a.address()),
            InstructionAccount::writable(self.vault_b.address()),
            InstructionAccount::readonly(self.token_program_a.address()),
            InstructionAccount::readonly(self.token_program_b.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 13] {
        [
            self.pool, self.user, self.owner, self.mint_a, self.mint_b, self.user_ta_a, self.user_ta_b, self.vault_a, self.vault_b, self.token_program_a, self.token_program_b, self.system_program, self.program
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
    invoke_signed::<13>(&instruction, &account_views, signers)
}

/// CPI: claim
#[inline(always)]
pub fn claim<'a>(
    accounts: &ClaimAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&CLAIM);
    
    
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
    invoke_signed::<13>(&instruction, &account_views, signers)
}

/// CPI: quote_buy
#[inline(always)]
pub fn quote_buy<'a>(
    accounts: &QuoteBuyAccounts<'a>, args: &QuoteBuyArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<QuoteBuyArgs>()];
    data[0..8].copy_from_slice(&QUOTE_BUY);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const QuoteBuyArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<QuoteBuyArgs>(),
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

/// CPI: quote_sell
#[inline(always)]
pub fn quote_sell<'a>(
    accounts: &QuoteSellAccounts<'a>, args: &QuoteSellArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<QuoteSellArgs>()];
    data[0..8].copy_from_slice(&QUOTE_SELL);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const QuoteSellArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<QuoteSellArgs>(),
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
    invoke_signed::<13>(&instruction, &account_views, signers)
}

