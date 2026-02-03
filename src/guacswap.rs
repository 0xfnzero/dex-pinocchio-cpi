//! CPI module for Guacswap
//!
//! Program: guacswap
//! Program ID: Gswppe6ERWKpUTXvRPfXdzHhiCyJvLadVvXGfdpBqcE1
//! Instructions: 20

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
pub const ID: Address = Address::new_from_array(five8_const::decode_32_const("Gswppe6ERWKpUTXvRPfXdzHhiCyJvLadVvXGfdpBqcE1"));

// ============================================
// Instruction Discriminators
// ============================================
pub const CREATE_POOL: [u8; 8] = [233, 146, 209, 142, 207, 104, 64, 188];
pub const CREATE_PROVIDER: [u8; 8] = [74, 53, 211, 174, 38, 168, 227, 177];
pub const CREATE_STATE: [u8; 8] = [214, 211, 209, 79, 107, 105, 247, 222];
pub const ADD_TOKENS: [u8; 8] = [28, 218, 30, 209, 175, 155, 153, 240];
pub const WITHDRAW_BUYBACK: [u8; 8] = [188, 75, 30, 198, 99, 43, 12, 54];
pub const SWAP: [u8; 8] = [248, 198, 158, 145, 225, 117, 135, 200];
pub const WITHDRAW_SHARES: [u8; 8] = [176, 104, 154, 105, 250, 80, 68, 244];
pub const WITHDRAW_LP_FEE: [u8; 8] = [149, 161, 2, 213, 195, 147, 42, 65];
pub const WITHDRAW_PROJECT_FEE: [u8; 8] = [130, 201, 142, 156, 159, 207, 168, 22];
pub const CREATE_FARM: [u8; 8] = [74, 59, 128, 160, 87, 174, 153, 194];
pub const CREATE_DUAL_FARM: [u8; 8] = [42, 180, 103, 138, 206, 43, 208, 98];
pub const CREATE_TRIPLE_FARM: [u8; 8] = [154, 26, 180, 145, 18, 201, 135, 171];
pub const WITHDRAW_REWARDS: [u8; 8] = [10, 214, 219, 139, 205, 22, 251, 21];
pub const CLOSE_POOL: [u8; 8] = [140, 189, 209, 23, 239, 62, 239, 11];
pub const WITHDRAW_MERCANTI_FEE: [u8; 8] = [253, 229, 129, 37, 47, 72, 11, 240];
pub const ADD_SUPPLY: [u8; 8] = [80, 102, 70, 57, 235, 88, 239, 8];
pub const UPDATE_FEES: [u8; 8] = [225, 27, 13, 6, 69, 84, 172, 191];
pub const RESET_FARM: [u8; 8] = [47, 77, 233, 117, 118, 55, 61, 113];
pub const UPDATE_REWARD_TOKENS: [u8; 8] = [249, 236, 71, 74, 104, 58, 225, 28];
pub const FORCE_WITHDRAW_REWARDS: [u8; 8] = [214, 35, 143, 68, 97, 248, 101, 241];

// ============================================
// Instruction Arguments
// ============================================
/// Arguments for `createPool`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct CreatepoolArgs {
    pub lp_fee: [u8; 32],
    pub buyback_fee: [u8; 32],
    pub project_fee: [u8; 32],
    pub mercanti_fee: [u8; 32],
    pub initial_token_x: [u8; 32],
    pub initial_token_y: [u8; 32],
    pub bump: u8,
}

/// Arguments for `createProvider`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct CreateproviderArgs {
    pub token_x_amount: [u8; 32],
    pub token_y_amount: [u8; 32],
    pub bump: u8,
}

/// Arguments for `createState`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct CreatestateArgs {
    pub nonce: u8,
}

/// Arguments for `addTokens`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct AddtokensArgs {
    pub delta_x: [u8; 32],
    pub delta_y: [u8; 32],
}

/// Arguments for `swap`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SwapArgs {
    pub delta_in: [u8; 32],
    pub price_limit: [u8; 32],
    pub x_to_y: bool,
}

/// Arguments for `withdrawShares`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct WithdrawsharesArgs {
    pub shares: [u8; 32],
}

/// Arguments for `createFarm`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct CreatefarmArgs {
    pub supply: [u8; 32],
    pub duration: u64,
    pub bump: u8,
}

/// Arguments for `createDualFarm`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct CreatedualfarmArgs {
    pub supply_marco: [u8; 32],
    pub supply_project_first: [u8; 32],
    pub duration: u64,
    pub bump: u8,
}

/// Arguments for `createTripleFarm`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct CreatetriplefarmArgs {
    pub supply_marco: [u8; 32],
    pub supply_project_first: [u8; 32],
    pub supply_project_second: [u8; 32],
    pub duration: u64,
    pub bump: u8,
}

/// Arguments for `addSupply`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct AddsupplyArgs {
    pub supply_marco: [u8; 32],
    pub supply_project_first: [u8; 32],
    pub supply_project_second: [u8; 32],
    pub duration: u64,
}

/// Arguments for `updateFees`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct UpdatefeesArgs {
    pub new_buyback_fee: [u8; 32],
    pub new_project_fee: [u8; 32],
    pub new_provider_fee: [u8; 32],
    pub new_mercanti_fee: [u8; 32],
}


// ============================================
// Instruction Accounts
// ============================================
/// Accounts for `createPool`
pub struct CreatepoolAccounts<'a> {
    /// state
    pub state: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// tokenX
    pub token_x: &'a AccountView,
    /// tokenY
    pub token_y: &'a AccountView,
    /// poolXAccount
    pub pool_x_account: &'a AccountView,
    /// poolYAccount
    pub pool_y_account: &'a AccountView,
    /// adminXAccount
    pub admin_x_account: &'a AccountView,
    /// adminYAccount
    pub admin_y_account: &'a AccountView,
    /// admin
    pub admin: &'a AccountView,
    /// projectOwner
    pub project_owner: &'a AccountView,
    /// programAuthority
    pub program_authority: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
}

impl<'a> CreatepoolAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 13] {
        [
            InstructionAccount::readonly(self.state.address()),
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::readonly(self.token_x.address()),
            InstructionAccount::readonly(self.token_y.address()),
            InstructionAccount::writable_signer(self.pool_x_account.address()),
            InstructionAccount::writable_signer(self.pool_y_account.address()),
            InstructionAccount::writable(self.admin_x_account.address()),
            InstructionAccount::writable(self.admin_y_account.address()),
            InstructionAccount::writable_signer(self.admin.address()),
            InstructionAccount::readonly(self.project_owner.address()),
            InstructionAccount::readonly(self.program_authority.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 13] {
        [
            self.state, self.pool, self.token_x, self.token_y, self.pool_x_account, self.pool_y_account, self.admin_x_account, self.admin_y_account, self.admin, self.project_owner, self.program_authority, self.system_program, self.token_program
        ]
    }
}

/// Accounts for `createProvider`
pub struct CreateproviderAccounts<'a> {
    /// pool
    pub pool: &'a AccountView,
    /// farm
    pub farm: &'a AccountView,
    /// provider
    pub provider: &'a AccountView,
    /// poolXAccount
    pub pool_x_account: &'a AccountView,
    /// poolYAccount
    pub pool_y_account: &'a AccountView,
    /// ownerXAccount
    pub owner_x_account: &'a AccountView,
    /// ownerYAccount
    pub owner_y_account: &'a AccountView,
    /// owner
    pub owner: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
}

impl<'a> CreateproviderAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 10] {
        [
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.farm.address()),
            InstructionAccount::writable(self.provider.address()),
            InstructionAccount::writable(self.pool_x_account.address()),
            InstructionAccount::writable(self.pool_y_account.address()),
            InstructionAccount::writable(self.owner_x_account.address()),
            InstructionAccount::writable(self.owner_y_account.address()),
            InstructionAccount::writable_signer(self.owner.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 10] {
        [
            self.pool, self.farm, self.provider, self.pool_x_account, self.pool_y_account, self.owner_x_account, self.owner_y_account, self.owner, self.system_program, self.token_program
        ]
    }
}

/// Accounts for `createState`
pub struct CreatestateAccounts<'a> {
    /// state
    pub state: &'a AccountView,
    /// admin
    pub admin: &'a AccountView,
    /// programAuthority
    pub program_authority: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
}

impl<'a> CreatestateAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 4] {
        [
            InstructionAccount::writable(self.state.address()),
            InstructionAccount::writable_signer(self.admin.address()),
            InstructionAccount::readonly(self.program_authority.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 4] {
        [
            self.state, self.admin, self.program_authority, self.system_program
        ]
    }
}

/// Accounts for `addTokens`
pub struct AddtokensAccounts<'a> {
    /// state
    pub state: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// farm
    pub farm: &'a AccountView,
    /// provider
    pub provider: &'a AccountView,
    /// ownerXAccount
    pub owner_x_account: &'a AccountView,
    /// ownerYAccount
    pub owner_y_account: &'a AccountView,
    /// poolXAccount
    pub pool_x_account: &'a AccountView,
    /// poolYAccount
    pub pool_y_account: &'a AccountView,
    /// ownerMarcoAccount
    pub owner_marco_account: &'a AccountView,
    /// ownerProjectFirstAccount
    pub owner_project_first_account: &'a AccountView,
    /// ownerProjectSecondAccount
    pub owner_project_second_account: &'a AccountView,
    /// tokenMarcoAccount
    pub token_marco_account: &'a AccountView,
    /// tokenProjectFirstAccount
    pub token_project_first_account: &'a AccountView,
    /// tokenProjectSecondAccount
    pub token_project_second_account: &'a AccountView,
    /// owner
    pub owner: &'a AccountView,
    /// programAuthority
    pub program_authority: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
}

impl<'a> AddtokensAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 17] {
        [
            InstructionAccount::readonly(self.state.address()),
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.farm.address()),
            InstructionAccount::writable(self.provider.address()),
            InstructionAccount::writable(self.owner_x_account.address()),
            InstructionAccount::writable(self.owner_y_account.address()),
            InstructionAccount::writable(self.pool_x_account.address()),
            InstructionAccount::writable(self.pool_y_account.address()),
            InstructionAccount::writable(self.owner_marco_account.address()),
            InstructionAccount::writable(self.owner_project_first_account.address()),
            InstructionAccount::writable(self.owner_project_second_account.address()),
            InstructionAccount::writable(self.token_marco_account.address()),
            InstructionAccount::writable(self.token_project_first_account.address()),
            InstructionAccount::writable(self.token_project_second_account.address()),
            InstructionAccount::writable_signer(self.owner.address()),
            InstructionAccount::readonly(self.program_authority.address()),
            InstructionAccount::readonly(self.token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 17] {
        [
            self.state, self.pool, self.farm, self.provider, self.owner_x_account, self.owner_y_account, self.pool_x_account, self.pool_y_account, self.owner_marco_account, self.owner_project_first_account, self.owner_project_second_account, self.token_marco_account, self.token_project_first_account, self.token_project_second_account, self.owner, self.program_authority, self.token_program
        ]
    }
}

/// Accounts for `withdrawBuyback`
pub struct WithdrawbuybackAccounts<'a> {
    /// state
    pub state: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// buybackXAccount
    pub buyback_x_account: &'a AccountView,
    /// buybackYAccount
    pub buyback_y_account: &'a AccountView,
    /// poolXAccount
    pub pool_x_account: &'a AccountView,
    /// poolYAccount
    pub pool_y_account: &'a AccountView,
    /// admin
    pub admin: &'a AccountView,
    /// programAuthority
    pub program_authority: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
}

impl<'a> WithdrawbuybackAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 9] {
        [
            InstructionAccount::readonly(self.state.address()),
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.buyback_x_account.address()),
            InstructionAccount::writable(self.buyback_y_account.address()),
            InstructionAccount::writable(self.pool_x_account.address()),
            InstructionAccount::writable(self.pool_y_account.address()),
            InstructionAccount::writable_signer(self.admin.address()),
            InstructionAccount::readonly(self.program_authority.address()),
            InstructionAccount::readonly(self.token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 9] {
        [
            self.state, self.pool, self.buyback_x_account, self.buyback_y_account, self.pool_x_account, self.pool_y_account, self.admin, self.program_authority, self.token_program
        ]
    }
}

/// Accounts for `swap`
pub struct SwapAccounts<'a> {
    /// state
    pub state: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// tokenX
    pub token_x: &'a AccountView,
    /// tokenY
    pub token_y: &'a AccountView,
    /// poolXAccount
    pub pool_x_account: &'a AccountView,
    /// poolYAccount
    pub pool_y_account: &'a AccountView,
    /// swapperXAccount
    pub swapper_x_account: &'a AccountView,
    /// swapperYAccount
    pub swapper_y_account: &'a AccountView,
    /// swapper
    pub swapper: &'a AccountView,
    /// referrerXAccount
    pub referrer_x_account: &'a AccountView,
    /// referrerYAccount
    pub referrer_y_account: &'a AccountView,
    /// referrer
    pub referrer: &'a AccountView,
    /// programAuthority
    pub program_authority: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// associatedTokenProgram
    pub associated_token_program: &'a AccountView,
    /// rent
    pub rent: &'a AccountView,
}

impl<'a> SwapAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 17] {
        [
            InstructionAccount::readonly(self.state.address()),
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::readonly(self.token_x.address()),
            InstructionAccount::readonly(self.token_y.address()),
            InstructionAccount::writable(self.pool_x_account.address()),
            InstructionAccount::writable(self.pool_y_account.address()),
            InstructionAccount::writable(self.swapper_x_account.address()),
            InstructionAccount::writable(self.swapper_y_account.address()),
            InstructionAccount::writable_signer(self.swapper.address()),
            InstructionAccount::writable(self.referrer_x_account.address()),
            InstructionAccount::writable(self.referrer_y_account.address()),
            InstructionAccount::writable(self.referrer.address()),
            InstructionAccount::readonly(self.program_authority.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
            InstructionAccount::readonly(self.rent.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 17] {
        [
            self.state, self.pool, self.token_x, self.token_y, self.pool_x_account, self.pool_y_account, self.swapper_x_account, self.swapper_y_account, self.swapper, self.referrer_x_account, self.referrer_y_account, self.referrer, self.program_authority, self.system_program, self.token_program, self.associated_token_program, self.rent
        ]
    }
}

/// Accounts for `withdrawShares`
pub struct WithdrawsharesAccounts<'a> {
    /// state
    pub state: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// farm
    pub farm: &'a AccountView,
    /// provider
    pub provider: &'a AccountView,
    /// poolXAccount
    pub pool_x_account: &'a AccountView,
    /// poolYAccount
    pub pool_y_account: &'a AccountView,
    /// tokenMarcoAccount
    pub token_marco_account: &'a AccountView,
    /// tokenProjectFirstAccount
    pub token_project_first_account: &'a AccountView,
    /// tokenProjectSecondAccount
    pub token_project_second_account: &'a AccountView,
    /// ownerXAccount
    pub owner_x_account: &'a AccountView,
    /// ownerYAccount
    pub owner_y_account: &'a AccountView,
    /// ownerMarcoAccount
    pub owner_marco_account: &'a AccountView,
    /// ownerProjectFirstAccount
    pub owner_project_first_account: &'a AccountView,
    /// ownerProjectSecondAccount
    pub owner_project_second_account: &'a AccountView,
    /// owner
    pub owner: &'a AccountView,
    /// programAuthority
    pub program_authority: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
}

impl<'a> WithdrawsharesAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 17] {
        [
            InstructionAccount::readonly(self.state.address()),
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.farm.address()),
            InstructionAccount::writable(self.provider.address()),
            InstructionAccount::writable(self.pool_x_account.address()),
            InstructionAccount::writable(self.pool_y_account.address()),
            InstructionAccount::writable(self.token_marco_account.address()),
            InstructionAccount::writable(self.token_project_first_account.address()),
            InstructionAccount::writable(self.token_project_second_account.address()),
            InstructionAccount::writable(self.owner_x_account.address()),
            InstructionAccount::writable(self.owner_y_account.address()),
            InstructionAccount::writable(self.owner_marco_account.address()),
            InstructionAccount::writable(self.owner_project_first_account.address()),
            InstructionAccount::writable(self.owner_project_second_account.address()),
            InstructionAccount::writable_signer(self.owner.address()),
            InstructionAccount::readonly(self.program_authority.address()),
            InstructionAccount::readonly(self.token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 17] {
        [
            self.state, self.pool, self.farm, self.provider, self.pool_x_account, self.pool_y_account, self.token_marco_account, self.token_project_first_account, self.token_project_second_account, self.owner_x_account, self.owner_y_account, self.owner_marco_account, self.owner_project_first_account, self.owner_project_second_account, self.owner, self.program_authority, self.token_program
        ]
    }
}

/// Accounts for `withdrawLpFee`
pub struct WithdrawlpfeeAccounts<'a> {
    /// state
    pub state: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// provider
    pub provider: &'a AccountView,
    /// ownerXAccount
    pub owner_x_account: &'a AccountView,
    /// ownerYAccount
    pub owner_y_account: &'a AccountView,
    /// poolXAccount
    pub pool_x_account: &'a AccountView,
    /// poolYAccount
    pub pool_y_account: &'a AccountView,
    /// owner
    pub owner: &'a AccountView,
    /// programAuthority
    pub program_authority: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
}

impl<'a> WithdrawlpfeeAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 10] {
        [
            InstructionAccount::readonly(self.state.address()),
            InstructionAccount::readonly(self.pool.address()),
            InstructionAccount::writable(self.provider.address()),
            InstructionAccount::writable(self.owner_x_account.address()),
            InstructionAccount::writable(self.owner_y_account.address()),
            InstructionAccount::writable(self.pool_x_account.address()),
            InstructionAccount::writable(self.pool_y_account.address()),
            InstructionAccount::writable_signer(self.owner.address()),
            InstructionAccount::readonly(self.program_authority.address()),
            InstructionAccount::readonly(self.token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 10] {
        [
            self.state, self.pool, self.provider, self.owner_x_account, self.owner_y_account, self.pool_x_account, self.pool_y_account, self.owner, self.program_authority, self.token_program
        ]
    }
}

/// Accounts for `withdrawProjectFee`
pub struct WithdrawprojectfeeAccounts<'a> {
    /// state
    pub state: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// projectOwnerXAccount
    pub project_owner_x_account: &'a AccountView,
    /// projectOwnerYAccount
    pub project_owner_y_account: &'a AccountView,
    /// poolXAccount
    pub pool_x_account: &'a AccountView,
    /// poolYAccount
    pub pool_y_account: &'a AccountView,
    /// projectOwner
    pub project_owner: &'a AccountView,
    /// programAuthority
    pub program_authority: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
}

impl<'a> WithdrawprojectfeeAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 9] {
        [
            InstructionAccount::readonly(self.state.address()),
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.project_owner_x_account.address()),
            InstructionAccount::writable(self.project_owner_y_account.address()),
            InstructionAccount::writable(self.pool_x_account.address()),
            InstructionAccount::writable(self.pool_y_account.address()),
            InstructionAccount::writable_signer(self.project_owner.address()),
            InstructionAccount::readonly(self.program_authority.address()),
            InstructionAccount::readonly(self.token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 9] {
        [
            self.state, self.pool, self.project_owner_x_account, self.project_owner_y_account, self.pool_x_account, self.pool_y_account, self.project_owner, self.program_authority, self.token_program
        ]
    }
}

/// Accounts for `createFarm`
pub struct CreatefarmAccounts<'a> {
    /// state
    pub state: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// farm
    pub farm: &'a AccountView,
    /// tokenMarco
    pub token_marco: &'a AccountView,
    /// tokenMarcoAccount
    pub token_marco_account: &'a AccountView,
    /// adminMarcoAccount
    pub admin_marco_account: &'a AccountView,
    /// admin
    pub admin: &'a AccountView,
    /// programAuthority
    pub program_authority: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
}

impl<'a> CreatefarmAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 10] {
        [
            InstructionAccount::readonly(self.state.address()),
            InstructionAccount::readonly(self.pool.address()),
            InstructionAccount::writable(self.farm.address()),
            InstructionAccount::readonly(self.token_marco.address()),
            InstructionAccount::writable_signer(self.token_marco_account.address()),
            InstructionAccount::writable(self.admin_marco_account.address()),
            InstructionAccount::writable_signer(self.admin.address()),
            InstructionAccount::readonly(self.program_authority.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 10] {
        [
            self.state, self.pool, self.farm, self.token_marco, self.token_marco_account, self.admin_marco_account, self.admin, self.program_authority, self.system_program, self.token_program
        ]
    }
}

/// Accounts for `createDualFarm`
pub struct CreatedualfarmAccounts<'a> {
    /// state
    pub state: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// farm
    pub farm: &'a AccountView,
    /// tokenMarco
    pub token_marco: &'a AccountView,
    /// tokenProjectFirst
    pub token_project_first: &'a AccountView,
    /// tokenMarcoAccount
    pub token_marco_account: &'a AccountView,
    /// tokenProjectFirstAccount
    pub token_project_first_account: &'a AccountView,
    /// adminMarcoAccount
    pub admin_marco_account: &'a AccountView,
    /// adminProjectFirstAccount
    pub admin_project_first_account: &'a AccountView,
    /// admin
    pub admin: &'a AccountView,
    /// programAuthority
    pub program_authority: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
}

impl<'a> CreatedualfarmAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 13] {
        [
            InstructionAccount::readonly(self.state.address()),
            InstructionAccount::readonly(self.pool.address()),
            InstructionAccount::writable(self.farm.address()),
            InstructionAccount::readonly(self.token_marco.address()),
            InstructionAccount::readonly(self.token_project_first.address()),
            InstructionAccount::writable_signer(self.token_marco_account.address()),
            InstructionAccount::writable_signer(self.token_project_first_account.address()),
            InstructionAccount::writable(self.admin_marco_account.address()),
            InstructionAccount::writable(self.admin_project_first_account.address()),
            InstructionAccount::writable_signer(self.admin.address()),
            InstructionAccount::readonly(self.program_authority.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 13] {
        [
            self.state, self.pool, self.farm, self.token_marco, self.token_project_first, self.token_marco_account, self.token_project_first_account, self.admin_marco_account, self.admin_project_first_account, self.admin, self.program_authority, self.system_program, self.token_program
        ]
    }
}

/// Accounts for `createTripleFarm`
pub struct CreatetriplefarmAccounts<'a> {
    /// state
    pub state: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// farm
    pub farm: &'a AccountView,
    /// tokenX
    pub token_x: &'a AccountView,
    /// tokenY
    pub token_y: &'a AccountView,
    /// tokenMarco
    pub token_marco: &'a AccountView,
    /// tokenProjectFirst
    pub token_project_first: &'a AccountView,
    /// tokenProjectSecond
    pub token_project_second: &'a AccountView,
    /// tokenMarcoAccount
    pub token_marco_account: &'a AccountView,
    /// tokenProjectFirstAccount
    pub token_project_first_account: &'a AccountView,
    /// tokenProjectSecondAccount
    pub token_project_second_account: &'a AccountView,
    /// adminMarcoAccount
    pub admin_marco_account: &'a AccountView,
    /// adminProjectFirstAccount
    pub admin_project_first_account: &'a AccountView,
    /// adminProjectSecondAccount
    pub admin_project_second_account: &'a AccountView,
    /// admin
    pub admin: &'a AccountView,
    /// programAuthority
    pub program_authority: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
}

impl<'a> CreatetriplefarmAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 18] {
        [
            InstructionAccount::readonly(self.state.address()),
            InstructionAccount::readonly(self.pool.address()),
            InstructionAccount::writable(self.farm.address()),
            InstructionAccount::readonly(self.token_x.address()),
            InstructionAccount::readonly(self.token_y.address()),
            InstructionAccount::readonly(self.token_marco.address()),
            InstructionAccount::readonly(self.token_project_first.address()),
            InstructionAccount::readonly(self.token_project_second.address()),
            InstructionAccount::writable_signer(self.token_marco_account.address()),
            InstructionAccount::writable_signer(self.token_project_first_account.address()),
            InstructionAccount::writable_signer(self.token_project_second_account.address()),
            InstructionAccount::writable(self.admin_marco_account.address()),
            InstructionAccount::writable(self.admin_project_first_account.address()),
            InstructionAccount::writable(self.admin_project_second_account.address()),
            InstructionAccount::writable_signer(self.admin.address()),
            InstructionAccount::readonly(self.program_authority.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 18] {
        [
            self.state, self.pool, self.farm, self.token_x, self.token_y, self.token_marco, self.token_project_first, self.token_project_second, self.token_marco_account, self.token_project_first_account, self.token_project_second_account, self.admin_marco_account, self.admin_project_first_account, self.admin_project_second_account, self.admin, self.program_authority, self.system_program, self.token_program
        ]
    }
}

/// Accounts for `withdrawRewards`
pub struct WithdrawrewardsAccounts<'a> {
    /// state
    pub state: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// farm
    pub farm: &'a AccountView,
    /// provider
    pub provider: &'a AccountView,
    /// tokenMarcoAccount
    pub token_marco_account: &'a AccountView,
    /// tokenProjectFirstAccount
    pub token_project_first_account: &'a AccountView,
    /// tokenProjectSecondAccount
    pub token_project_second_account: &'a AccountView,
    /// ownerMarcoAccount
    pub owner_marco_account: &'a AccountView,
    /// ownerProjectFirstAccount
    pub owner_project_first_account: &'a AccountView,
    /// ownerProjectSecondAccount
    pub owner_project_second_account: &'a AccountView,
    /// owner
    pub owner: &'a AccountView,
    /// programAuthority
    pub program_authority: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
}

impl<'a> WithdrawrewardsAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 13] {
        [
            InstructionAccount::readonly(self.state.address()),
            InstructionAccount::readonly(self.pool.address()),
            InstructionAccount::writable(self.farm.address()),
            InstructionAccount::writable(self.provider.address()),
            InstructionAccount::writable(self.token_marco_account.address()),
            InstructionAccount::writable(self.token_project_first_account.address()),
            InstructionAccount::writable(self.token_project_second_account.address()),
            InstructionAccount::writable(self.owner_marco_account.address()),
            InstructionAccount::writable(self.owner_project_first_account.address()),
            InstructionAccount::writable(self.owner_project_second_account.address()),
            InstructionAccount::writable_signer(self.owner.address()),
            InstructionAccount::readonly(self.program_authority.address()),
            InstructionAccount::readonly(self.token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 13] {
        [
            self.state, self.pool, self.farm, self.provider, self.token_marco_account, self.token_project_first_account, self.token_project_second_account, self.owner_marco_account, self.owner_project_first_account, self.owner_project_second_account, self.owner, self.program_authority, self.token_program
        ]
    }
}

/// Accounts for `closePool`
pub struct ClosepoolAccounts<'a> {
    /// state
    pub state: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// farm
    pub farm: &'a AccountView,
    /// tokenMarcoAccount
    pub token_marco_account: &'a AccountView,
    /// tokenProjectFirstAccount
    pub token_project_first_account: &'a AccountView,
    /// tokenProjectSecondAccount
    pub token_project_second_account: &'a AccountView,
    /// poolXAccount
    pub pool_x_account: &'a AccountView,
    /// poolYAccount
    pub pool_y_account: &'a AccountView,
    /// buybackXAccount
    pub buyback_x_account: &'a AccountView,
    /// buybackYAccount
    pub buyback_y_account: &'a AccountView,
    /// admin
    pub admin: &'a AccountView,
    /// programAuthority
    pub program_authority: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
}

impl<'a> ClosepoolAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 13] {
        [
            InstructionAccount::readonly(self.state.address()),
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.farm.address()),
            InstructionAccount::writable(self.token_marco_account.address()),
            InstructionAccount::writable(self.token_project_first_account.address()),
            InstructionAccount::writable(self.token_project_second_account.address()),
            InstructionAccount::writable(self.pool_x_account.address()),
            InstructionAccount::writable(self.pool_y_account.address()),
            InstructionAccount::writable(self.buyback_x_account.address()),
            InstructionAccount::writable(self.buyback_y_account.address()),
            InstructionAccount::writable_signer(self.admin.address()),
            InstructionAccount::readonly(self.program_authority.address()),
            InstructionAccount::readonly(self.token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 13] {
        [
            self.state, self.pool, self.farm, self.token_marco_account, self.token_project_first_account, self.token_project_second_account, self.pool_x_account, self.pool_y_account, self.buyback_x_account, self.buyback_y_account, self.admin, self.program_authority, self.token_program
        ]
    }
}

/// Accounts for `withdrawMercantiFee`
pub struct WithdrawmercantifeeAccounts<'a> {
    /// state
    pub state: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// mercantiXAccount
    pub mercanti_x_account: &'a AccountView,
    /// mercantiYAccount
    pub mercanti_y_account: &'a AccountView,
    /// poolXAccount
    pub pool_x_account: &'a AccountView,
    /// poolYAccount
    pub pool_y_account: &'a AccountView,
    /// admin
    pub admin: &'a AccountView,
    /// programAuthority
    pub program_authority: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
}

impl<'a> WithdrawmercantifeeAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 9] {
        [
            InstructionAccount::readonly(self.state.address()),
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.mercanti_x_account.address()),
            InstructionAccount::writable(self.mercanti_y_account.address()),
            InstructionAccount::writable(self.pool_x_account.address()),
            InstructionAccount::writable(self.pool_y_account.address()),
            InstructionAccount::writable_signer(self.admin.address()),
            InstructionAccount::readonly(self.program_authority.address()),
            InstructionAccount::readonly(self.token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 9] {
        [
            self.state, self.pool, self.mercanti_x_account, self.mercanti_y_account, self.pool_x_account, self.pool_y_account, self.admin, self.program_authority, self.token_program
        ]
    }
}

/// Accounts for `addSupply`
pub struct AddsupplyAccounts<'a> {
    /// state
    pub state: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// farm
    pub farm: &'a AccountView,
    /// tokenMarcoAccount
    pub token_marco_account: &'a AccountView,
    /// tokenProjectFirstAccount
    pub token_project_first_account: &'a AccountView,
    /// tokenProjectSecondAccount
    pub token_project_second_account: &'a AccountView,
    /// adminMarcoAccount
    pub admin_marco_account: &'a AccountView,
    /// adminProjectFirstAccount
    pub admin_project_first_account: &'a AccountView,
    /// adminProjectSecondAccount
    pub admin_project_second_account: &'a AccountView,
    /// admin
    pub admin: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
}

impl<'a> AddsupplyAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 11] {
        [
            InstructionAccount::readonly(self.state.address()),
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.farm.address()),
            InstructionAccount::writable(self.token_marco_account.address()),
            InstructionAccount::writable(self.token_project_first_account.address()),
            InstructionAccount::writable(self.token_project_second_account.address()),
            InstructionAccount::writable(self.admin_marco_account.address()),
            InstructionAccount::writable(self.admin_project_first_account.address()),
            InstructionAccount::writable(self.admin_project_second_account.address()),
            InstructionAccount::readonly_signer(self.admin.address()),
            InstructionAccount::readonly(self.token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 11] {
        [
            self.state, self.pool, self.farm, self.token_marco_account, self.token_project_first_account, self.token_project_second_account, self.admin_marco_account, self.admin_project_first_account, self.admin_project_second_account, self.admin, self.token_program
        ]
    }
}

/// Accounts for `updateFees`
pub struct UpdatefeesAccounts<'a> {
    /// state
    pub state: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// admin
    pub admin: &'a AccountView,
    /// programAuthority
    pub program_authority: &'a AccountView,
}

impl<'a> UpdatefeesAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 4] {
        [
            InstructionAccount::readonly(self.state.address()),
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable_signer(self.admin.address()),
            InstructionAccount::readonly(self.program_authority.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 4] {
        [
            self.state, self.pool, self.admin, self.program_authority
        ]
    }
}

/// Accounts for `resetFarm`
pub struct ResetfarmAccounts<'a> {
    /// state
    pub state: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// farm
    pub farm: &'a AccountView,
    /// tokenMarcoAccount
    pub token_marco_account: &'a AccountView,
    /// tokenProjectFirstAccount
    pub token_project_first_account: &'a AccountView,
    /// tokenProjectSecondAccount
    pub token_project_second_account: &'a AccountView,
    /// adminMarcoAccount
    pub admin_marco_account: &'a AccountView,
    /// adminProjectFirstAccount
    pub admin_project_first_account: &'a AccountView,
    /// adminProjectSecondAccount
    pub admin_project_second_account: &'a AccountView,
    /// admin
    pub admin: &'a AccountView,
    /// programAuthority
    pub program_authority: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
}

impl<'a> ResetfarmAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 12] {
        [
            InstructionAccount::readonly(self.state.address()),
            InstructionAccount::readonly(self.pool.address()),
            InstructionAccount::writable(self.farm.address()),
            InstructionAccount::writable(self.token_marco_account.address()),
            InstructionAccount::writable(self.token_project_first_account.address()),
            InstructionAccount::writable(self.token_project_second_account.address()),
            InstructionAccount::writable(self.admin_marco_account.address()),
            InstructionAccount::writable(self.admin_project_first_account.address()),
            InstructionAccount::writable(self.admin_project_second_account.address()),
            InstructionAccount::writable_signer(self.admin.address()),
            InstructionAccount::readonly(self.program_authority.address()),
            InstructionAccount::readonly(self.token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 12] {
        [
            self.state, self.pool, self.farm, self.token_marco_account, self.token_project_first_account, self.token_project_second_account, self.admin_marco_account, self.admin_project_first_account, self.admin_project_second_account, self.admin, self.program_authority, self.token_program
        ]
    }
}

/// Accounts for `updateRewardTokens`
pub struct UpdaterewardtokensAccounts<'a> {
    /// state
    pub state: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// farm
    pub farm: &'a AccountView,
    /// tokenMarcoAccount
    pub token_marco_account: &'a AccountView,
    /// tokenProjectFirstAccount
    pub token_project_first_account: &'a AccountView,
    /// tokenProjectSecondAccount
    pub token_project_second_account: &'a AccountView,
    /// tokenMarco
    pub token_marco: &'a AccountView,
    /// newTokenMarcoAccount
    pub new_token_marco_account: &'a AccountView,
    /// admin
    pub admin: &'a AccountView,
    /// programAuthority
    pub program_authority: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
}

impl<'a> UpdaterewardtokensAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 12] {
        [
            InstructionAccount::readonly(self.state.address()),
            InstructionAccount::readonly(self.pool.address()),
            InstructionAccount::writable(self.farm.address()),
            InstructionAccount::writable(self.token_marco_account.address()),
            InstructionAccount::writable(self.token_project_first_account.address()),
            InstructionAccount::writable(self.token_project_second_account.address()),
            InstructionAccount::readonly(self.token_marco.address()),
            InstructionAccount::writable_signer(self.new_token_marco_account.address()),
            InstructionAccount::writable_signer(self.admin.address()),
            InstructionAccount::readonly(self.program_authority.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 12] {
        [
            self.state, self.pool, self.farm, self.token_marco_account, self.token_project_first_account, self.token_project_second_account, self.token_marco, self.new_token_marco_account, self.admin, self.program_authority, self.system_program, self.token_program
        ]
    }
}

/// Accounts for `forceWithdrawRewards`
pub struct ForcewithdrawrewardsAccounts<'a> {
    /// state
    pub state: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// farm
    pub farm: &'a AccountView,
    /// provider
    pub provider: &'a AccountView,
    /// tokenMarcoAccount
    pub token_marco_account: &'a AccountView,
    /// tokenProjectFirstAccount
    pub token_project_first_account: &'a AccountView,
    /// tokenProjectSecondAccount
    pub token_project_second_account: &'a AccountView,
    /// ownerMarcoAccount
    pub owner_marco_account: &'a AccountView,
    /// ownerProjectFirstAccount
    pub owner_project_first_account: &'a AccountView,
    /// ownerProjectSecondAccount
    pub owner_project_second_account: &'a AccountView,
    /// admin
    pub admin: &'a AccountView,
    /// programAuthority
    pub program_authority: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
}

impl<'a> ForcewithdrawrewardsAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 13] {
        [
            InstructionAccount::readonly(self.state.address()),
            InstructionAccount::readonly(self.pool.address()),
            InstructionAccount::writable(self.farm.address()),
            InstructionAccount::writable(self.provider.address()),
            InstructionAccount::writable(self.token_marco_account.address()),
            InstructionAccount::writable(self.token_project_first_account.address()),
            InstructionAccount::writable(self.token_project_second_account.address()),
            InstructionAccount::writable(self.owner_marco_account.address()),
            InstructionAccount::writable(self.owner_project_first_account.address()),
            InstructionAccount::writable(self.owner_project_second_account.address()),
            InstructionAccount::writable_signer(self.admin.address()),
            InstructionAccount::readonly(self.program_authority.address()),
            InstructionAccount::readonly(self.token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 13] {
        [
            self.state, self.pool, self.farm, self.provider, self.token_marco_account, self.token_project_first_account, self.token_project_second_account, self.owner_marco_account, self.owner_project_first_account, self.owner_project_second_account, self.admin, self.program_authority, self.token_program
        ]
    }
}


// ============================================
// CPI Functions
// ============================================
/// CPI: createPool
#[inline(always)]
pub fn create_pool<'a>(
    accounts: &CreatepoolAccounts<'a>, args: &CreatepoolArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<CreatepoolArgs>()];
    data[0..8].copy_from_slice(&CREATE_POOL);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const CreatepoolArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<CreatepoolArgs>(),
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

/// CPI: createProvider
#[inline(always)]
pub fn create_provider<'a>(
    accounts: &CreateproviderAccounts<'a>, args: &CreateproviderArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<CreateproviderArgs>()];
    data[0..8].copy_from_slice(&CREATE_PROVIDER);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const CreateproviderArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<CreateproviderArgs>(),
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

/// CPI: createState
#[inline(always)]
pub fn create_state<'a>(
    accounts: &CreatestateAccounts<'a>, args: &CreatestateArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<CreatestateArgs>()];
    data[0..8].copy_from_slice(&CREATE_STATE);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const CreatestateArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<CreatestateArgs>(),
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

/// CPI: addTokens
#[inline(always)]
pub fn add_tokens<'a>(
    accounts: &AddtokensAccounts<'a>, args: &AddtokensArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<AddtokensArgs>()];
    data[0..8].copy_from_slice(&ADD_TOKENS);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const AddtokensArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<AddtokensArgs>(),
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

/// CPI: withdrawBuyback
#[inline(always)]
pub fn withdraw_buyback<'a>(
    accounts: &WithdrawbuybackAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&WITHDRAW_BUYBACK);
    
    
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
    invoke_signed::<17>(&instruction, &account_views, signers)
}

/// CPI: withdrawShares
#[inline(always)]
pub fn withdraw_shares<'a>(
    accounts: &WithdrawsharesAccounts<'a>, args: &WithdrawsharesArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<WithdrawsharesArgs>()];
    data[0..8].copy_from_slice(&WITHDRAW_SHARES);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const WithdrawsharesArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<WithdrawsharesArgs>(),
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

/// CPI: withdrawLpFee
#[inline(always)]
pub fn withdraw_lp_fee<'a>(
    accounts: &WithdrawlpfeeAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&WITHDRAW_LP_FEE);
    
    
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

/// CPI: withdrawProjectFee
#[inline(always)]
pub fn withdraw_project_fee<'a>(
    accounts: &WithdrawprojectfeeAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&WITHDRAW_PROJECT_FEE);
    
    
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

/// CPI: createFarm
#[inline(always)]
pub fn create_farm<'a>(
    accounts: &CreatefarmAccounts<'a>, args: &CreatefarmArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<CreatefarmArgs>()];
    data[0..8].copy_from_slice(&CREATE_FARM);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const CreatefarmArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<CreatefarmArgs>(),
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

/// CPI: createDualFarm
#[inline(always)]
pub fn create_dual_farm<'a>(
    accounts: &CreatedualfarmAccounts<'a>, args: &CreatedualfarmArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<CreatedualfarmArgs>()];
    data[0..8].copy_from_slice(&CREATE_DUAL_FARM);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const CreatedualfarmArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<CreatedualfarmArgs>(),
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

/// CPI: createTripleFarm
#[inline(always)]
pub fn create_triple_farm<'a>(
    accounts: &CreatetriplefarmAccounts<'a>, args: &CreatetriplefarmArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<CreatetriplefarmArgs>()];
    data[0..8].copy_from_slice(&CREATE_TRIPLE_FARM);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const CreatetriplefarmArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<CreatetriplefarmArgs>(),
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
    invoke_signed::<18>(&instruction, &account_views, signers)
}

/// CPI: withdrawRewards
#[inline(always)]
pub fn withdraw_rewards<'a>(
    accounts: &WithdrawrewardsAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&WITHDRAW_REWARDS);
    
    
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

/// CPI: closePool
#[inline(always)]
pub fn close_pool<'a>(
    accounts: &ClosepoolAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&CLOSE_POOL);
    
    
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

/// CPI: withdrawMercantiFee
#[inline(always)]
pub fn withdraw_mercanti_fee<'a>(
    accounts: &WithdrawmercantifeeAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&WITHDRAW_MERCANTI_FEE);
    
    
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

/// CPI: addSupply
#[inline(always)]
pub fn add_supply<'a>(
    accounts: &AddsupplyAccounts<'a>, args: &AddsupplyArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<AddsupplyArgs>()];
    data[0..8].copy_from_slice(&ADD_SUPPLY);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const AddsupplyArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<AddsupplyArgs>(),
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

/// CPI: updateFees
#[inline(always)]
pub fn update_fees<'a>(
    accounts: &UpdatefeesAccounts<'a>, args: &UpdatefeesArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<UpdatefeesArgs>()];
    data[0..8].copy_from_slice(&UPDATE_FEES);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const UpdatefeesArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<UpdatefeesArgs>(),
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

/// CPI: resetFarm
#[inline(always)]
pub fn reset_farm<'a>(
    accounts: &ResetfarmAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&RESET_FARM);
    
    
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

/// CPI: updateRewardTokens
#[inline(always)]
pub fn update_reward_tokens<'a>(
    accounts: &UpdaterewardtokensAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&UPDATE_REWARD_TOKENS);
    
    
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

/// CPI: forceWithdrawRewards
#[inline(always)]
pub fn force_withdraw_rewards<'a>(
    accounts: &ForcewithdrawrewardsAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&FORCE_WITHDRAW_REWARDS);
    
    
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

