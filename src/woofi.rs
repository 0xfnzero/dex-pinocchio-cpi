//! CPI module for Woofi
//!
//! Program: woofi
//! Program ID: WooFif76YGRNjk1pA8wCsN67aQsD9f9iLsz4NcJ1AVb
//! Instructions: 41

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
pub const ID: Address = Address::new_from_array(five8_const::decode_32_const("WooFif76YGRNjk1pA8wCsN67aQsD9f9iLsz4NcJ1AVb"));

// ============================================
// Instruction Discriminators
// ============================================
pub const CREATE_CONFIG: [u8; 8] = [201, 207, 243, 114, 75, 111, 47, 189];
pub const CREATE_WOORACLE: [u8; 8] = [73, 65, 167, 4, 144, 141, 147, 32];
pub const SET_ORACLE_MAXIMUM_AGE: [u8; 8] = [65, 10, 188, 33, 5, 143, 54, 161];
pub const SET_STALE_DURATION: [u8; 8] = [108, 101, 231, 96, 83, 110, 252, 46];
pub const SET_WOO_BOUND: [u8; 8] = [250, 175, 192, 204, 161, 134, 105, 137];
pub const SET_WOO_RANGE: [u8; 8] = [53, 126, 84, 226, 73, 140, 138, 19];
pub const SET_WOO_PRICE: [u8; 8] = [103, 138, 92, 57, 66, 194, 40, 23];
pub const SET_WOO_COEFF: [u8; 8] = [122, 0, 214, 59, 63, 158, 219, 0];
pub const SET_WOO_SPREAD: [u8; 8] = [235, 93, 181, 194, 129, 219, 119, 243];
pub const SET_WOO_ADMIN: [u8; 8] = [139, 94, 246, 31, 251, 133, 174, 104];
pub const SET_GUARDIAN_ADMIN: [u8; 8] = [143, 114, 173, 208, 10, 22, 52, 9];
pub const SET_LENDING_MANAGER: [u8; 8] = [174, 103, 65, 227, 230, 50, 204, 96];
pub const SET_SUPERCHARGER_VAULT_WHITELIST: [u8; 8] = [22, 236, 175, 219, 150, 205, 128, 228];
pub const SET_WOO_STATE: [u8; 8] = [123, 114, 129, 125, 65, 73, 165, 17];
pub const GET_PRICE: [u8; 8] = [238, 38, 193, 106, 228, 32, 210, 33];
pub const CREATE_POOL: [u8; 8] = [233, 146, 209, 142, 207, 104, 64, 188];
pub const CREATE_WOO_AMM_POOL: [u8; 8] = [10, 26, 29, 176, 163, 101, 80, 158];
pub const SET_POOL_ADMIN: [u8; 8] = [37, 9, 3, 197, 132, 224, 165, 21];
pub const SET_FEE_ADMIN: [u8; 8] = [45, 227, 180, 113, 69, 212, 7, 7];
pub const SET_PAUSE_ROLE: [u8; 8] = [251, 5, 169, 217, 244, 144, 160, 211];
pub const PAUSE: [u8; 8] = [211, 22, 221, 251, 74, 121, 193, 47];
pub const UNPAUSE: [u8; 8] = [169, 144, 4, 38, 10, 141, 188, 255];
pub const SET_POOL_FEE_RATE: [u8; 8] = [163, 215, 73, 116, 199, 173, 122, 165];
pub const SET_POOL_MAX_GAMMA: [u8; 8] = [17, 128, 160, 226, 144, 180, 74, 207];
pub const SET_POOL_MAX_NOTIONAL_SWAP: [u8; 8] = [130, 110, 160, 30, 77, 13, 141, 228];
pub const SET_POOL_CAP_BAL: [u8; 8] = [57, 148, 133, 68, 253, 224, 251, 93];
pub const SET_POOL_MIN_SWAP_AMOUNT: [u8; 8] = [137, 75, 238, 195, 163, 17, 73, 23];
pub const TRY_QUERY: [u8; 8] = [132, 54, 250, 71, 244, 207, 193, 163];
pub const QUERY: [u8; 8] = [39, 251, 130, 159, 46, 136, 164, 169];
pub const SWAP: [u8; 8] = [248, 198, 158, 145, 225, 117, 135, 200];
pub const DEPOSIT: [u8; 8] = [242, 35, 198, 137, 82, 225, 242, 182];
pub const WITHDRAW: [u8; 8] = [183, 18, 70, 156, 148, 109, 161, 34];
pub const CLAIM_FEE: [u8; 8] = [169, 32, 79, 137, 136, 232, 70, 137];
pub const CLAIM_FEE_AMOUNT: [u8; 8] = [50, 43, 157, 70, 143, 92, 217, 90];
pub const INCASE_TOKEN_GOT_STUCK: [u8; 8] = [117, 232, 146, 8, 250, 186, 60, 2];
pub const SET_WOOCONFIG_NEW_AUTHORITY: [u8; 8] = [114, 190, 175, 59, 207, 93, 25, 90];
pub const CLAIM_WOOCONFIG_AUTHORITY: [u8; 8] = [139, 138, 173, 105, 222, 232, 215, 115];
pub const CLAIM_WOORACLE_AUTHORITY: [u8; 8] = [225, 110, 65, 225, 138, 101, 142, 72];
pub const CLAIM_WOOPOOL_AUTHORITY: [u8; 8] = [173, 55, 188, 135, 230, 201, 96, 151];
pub const CLAIM_WOOAMMPOOL_AUTHORITY: [u8; 8] = [65, 224, 51, 150, 107, 227, 48, 59];
pub const REPAY_BY_LENDING_MANAGER: [u8; 8] = [158, 154, 161, 163, 96, 3, 0, 99];

// ============================================
// Instruction Arguments
// ============================================
/// Arguments for `createWooracle`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct CreatewooracleArgs {
    pub maximum_age: u64,
}

/// Arguments for `setOracleMaximumAge`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SetoraclemaximumageArgs {
    pub maximum_age: u64,
}

/// Arguments for `setStaleDuration`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SetstaledurationArgs {
    pub stale_duration: i64,
}

/// Arguments for `setWooBound`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SetwooboundArgs {
    pub bound: u64,
}

/// Arguments for `setWooRange`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SetwoorangeArgs {
    pub range_min: u128,
    pub range_max: u128,
}

/// Arguments for `setWooPrice`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SetwoopriceArgs {
    pub price: u128,
}

/// Arguments for `setWooCoeff`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SetwoocoeffArgs {
    pub coeff: u64,
}

/// Arguments for `setWooSpread`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SetwoospreadArgs {
    pub spread: u64,
}

/// Arguments for `setWooAdmin`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SetwooadminArgs {
    pub admin_authority: [u8; 32],
}

/// Arguments for `setGuardianAdmin`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SetguardianadminArgs {
    pub guardian_authority: [u8; 32],
}

/// Arguments for `setLendingManager`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SetlendingmanagerArgs {
    pub lending_managers: [u8; 32],
}

/// Arguments for `setSuperchargerVaultWhitelist`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SetsuperchargervaultwhitelistArgs {
    pub supercharger_vaults: [u8; 32],
}

/// Arguments for `setWooState`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SetwoostateArgs {
    pub price: u128,
    pub coeff: u64,
    pub spread: u64,
}

/// Arguments for `setPoolAdmin`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SetpooladminArgs {
    pub admin_authority: [u8; 32],
}

/// Arguments for `setFeeAdmin`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SetfeeadminArgs {
    pub fee_authority: [u8; 32],
}

/// Arguments for `setPauseRole`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SetpauseroleArgs {
    pub pause_authority: [u8; 32],
}

/// Arguments for `setPoolFeeRate`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SetpoolfeerateArgs {
    pub fee_rate: u16,
}

/// Arguments for `setPoolMaxGamma`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SetpoolmaxgammaArgs {
    pub max_gamma: u128,
}

/// Arguments for `setPoolMaxNotionalSwap`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SetpoolmaxnotionalswapArgs {
    pub max_notional_swap: u128,
}

/// Arguments for `setPoolCapBal`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SetpoolcapbalArgs {
    pub cap_bal: u128,
}

/// Arguments for `setPoolMinSwapAmount`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SetpoolminswapamountArgs {
    pub min_swap_amount: u128,
}

/// Arguments for `tryQuery`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct TryqueryArgs {
    pub from_amount: u128,
}

/// Arguments for `query`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct QueryArgs {
    pub from_amount: u128,
    pub min_to_amount: u128,
}

/// Arguments for `swap`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SwapArgs {
    pub from_amount: u128,
    pub min_to_amount: u128,
}

/// Arguments for `deposit`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct DepositArgs {
    pub amount: u128,
}

/// Arguments for `withdraw`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct WithdrawArgs {
    pub amount: u128,
}

/// Arguments for `claimFeeAmount`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct ClaimfeeamountArgs {
    pub claim_amount: u128,
}

/// Arguments for `incaseTokenGotStuck`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct IncasetokengotstuckArgs {
    pub amount: u128,
}

/// Arguments for `repayByLendingManager`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct RepaybylendingmanagerArgs {
    pub repay_amount: u128,
}


// ============================================
// Instruction Accounts
// ============================================
/// Accounts for `createConfig`
pub struct CreateconfigAccounts<'a> {
    /// authority
    pub authority: &'a AccountView,
    /// wooconfig
    pub wooconfig: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
}

impl<'a> CreateconfigAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 3] {
        [
            InstructionAccount::writable_signer(self.authority.address()),
            InstructionAccount::writable(self.wooconfig.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 3] {
        [
            self.authority, self.wooconfig, self.system_program
        ]
    }
}

/// Accounts for `createWooracle`
pub struct CreatewooracleAccounts<'a> {
    /// wooconfig
    pub wooconfig: &'a AccountView,
    /// tokenMint
    pub token_mint: &'a AccountView,
    /// wooracle
    pub wooracle: &'a AccountView,
    /// admin
    pub admin: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
    /// feedAccount
    pub feed_account: &'a AccountView,
    /// priceUpdate
    pub price_update: &'a AccountView,
    /// quoteTokenMint
    pub quote_token_mint: &'a AccountView,
    /// quoteFeedAccount
    pub quote_feed_account: &'a AccountView,
    /// quotePriceUpdate
    pub quote_price_update: &'a AccountView,
}

impl<'a> CreatewooracleAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 10] {
        [
            InstructionAccount::readonly(self.wooconfig.address()),
            InstructionAccount::readonly(self.token_mint.address()),
            InstructionAccount::writable(self.wooracle.address()),
            InstructionAccount::writable_signer(self.admin.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.feed_account.address()),
            InstructionAccount::readonly(self.price_update.address()),
            InstructionAccount::readonly(self.quote_token_mint.address()),
            InstructionAccount::readonly(self.quote_feed_account.address()),
            InstructionAccount::readonly(self.quote_price_update.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 10] {
        [
            self.wooconfig, self.token_mint, self.wooracle, self.admin, self.system_program, self.feed_account, self.price_update, self.quote_token_mint, self.quote_feed_account, self.quote_price_update
        ]
    }
}

/// Accounts for `setOracleMaximumAge`
pub struct SetoraclemaximumageAccounts<'a> {
    /// wooconfig
    pub wooconfig: &'a AccountView,
    /// wooracle
    pub wooracle: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
}

impl<'a> SetoraclemaximumageAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 3] {
        [
            InstructionAccount::readonly(self.wooconfig.address()),
            InstructionAccount::writable(self.wooracle.address()),
            InstructionAccount::readonly_signer(self.authority.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 3] {
        [
            self.wooconfig, self.wooracle, self.authority
        ]
    }
}

/// Accounts for `setStaleDuration`
pub struct SetstaledurationAccounts<'a> {
    /// wooconfig
    pub wooconfig: &'a AccountView,
    /// wooracle
    pub wooracle: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
}

impl<'a> SetstaledurationAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 3] {
        [
            InstructionAccount::readonly(self.wooconfig.address()),
            InstructionAccount::writable(self.wooracle.address()),
            InstructionAccount::readonly_signer(self.authority.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 3] {
        [
            self.wooconfig, self.wooracle, self.authority
        ]
    }
}

/// Accounts for `setWooBound`
pub struct SetwooboundAccounts<'a> {
    /// wooconfig
    pub wooconfig: &'a AccountView,
    /// wooracle
    pub wooracle: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
}

impl<'a> SetwooboundAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 3] {
        [
            InstructionAccount::readonly(self.wooconfig.address()),
            InstructionAccount::writable(self.wooracle.address()),
            InstructionAccount::readonly_signer(self.authority.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 3] {
        [
            self.wooconfig, self.wooracle, self.authority
        ]
    }
}

/// Accounts for `setWooRange`
pub struct SetwoorangeAccounts<'a> {
    /// wooconfig
    pub wooconfig: &'a AccountView,
    /// wooracle
    pub wooracle: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
}

impl<'a> SetwoorangeAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 3] {
        [
            InstructionAccount::readonly(self.wooconfig.address()),
            InstructionAccount::writable(self.wooracle.address()),
            InstructionAccount::readonly_signer(self.authority.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 3] {
        [
            self.wooconfig, self.wooracle, self.authority
        ]
    }
}

/// Accounts for `setWooPrice`
pub struct SetwoopriceAccounts<'a> {
    /// wooconfig
    pub wooconfig: &'a AccountView,
    /// wooracle
    pub wooracle: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
}

impl<'a> SetwoopriceAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 3] {
        [
            InstructionAccount::readonly(self.wooconfig.address()),
            InstructionAccount::writable(self.wooracle.address()),
            InstructionAccount::readonly_signer(self.authority.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 3] {
        [
            self.wooconfig, self.wooracle, self.authority
        ]
    }
}

/// Accounts for `setWooCoeff`
pub struct SetwoocoeffAccounts<'a> {
    /// wooconfig
    pub wooconfig: &'a AccountView,
    /// wooracle
    pub wooracle: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
}

impl<'a> SetwoocoeffAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 3] {
        [
            InstructionAccount::readonly(self.wooconfig.address()),
            InstructionAccount::writable(self.wooracle.address()),
            InstructionAccount::readonly_signer(self.authority.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 3] {
        [
            self.wooconfig, self.wooracle, self.authority
        ]
    }
}

/// Accounts for `setWooSpread`
pub struct SetwoospreadAccounts<'a> {
    /// wooconfig
    pub wooconfig: &'a AccountView,
    /// wooracle
    pub wooracle: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
}

impl<'a> SetwoospreadAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 3] {
        [
            InstructionAccount::readonly(self.wooconfig.address()),
            InstructionAccount::writable(self.wooracle.address()),
            InstructionAccount::readonly_signer(self.authority.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 3] {
        [
            self.wooconfig, self.wooracle, self.authority
        ]
    }
}

/// Accounts for `setWooAdmin`
pub struct SetwooadminAccounts<'a> {
    /// wooconfig
    pub wooconfig: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
}

impl<'a> SetwooadminAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 2] {
        [
            InstructionAccount::writable(self.wooconfig.address()),
            InstructionAccount::readonly_signer(self.authority.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 2] {
        [
            self.wooconfig, self.authority
        ]
    }
}

/// Accounts for `setGuardianAdmin`
pub struct SetguardianadminAccounts<'a> {
    /// wooconfig
    pub wooconfig: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
}

impl<'a> SetguardianadminAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 2] {
        [
            InstructionAccount::writable(self.wooconfig.address()),
            InstructionAccount::readonly_signer(self.authority.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 2] {
        [
            self.wooconfig, self.authority
        ]
    }
}

/// Accounts for `setLendingManager`
pub struct SetlendingmanagerAccounts<'a> {
    /// wooconfig
    pub wooconfig: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
}

impl<'a> SetlendingmanagerAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 2] {
        [
            InstructionAccount::writable(self.wooconfig.address()),
            InstructionAccount::readonly_signer(self.authority.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 2] {
        [
            self.wooconfig, self.authority
        ]
    }
}

/// Accounts for `setSuperchargerVaultWhitelist`
pub struct SetsuperchargervaultwhitelistAccounts<'a> {
    /// wooconfig
    pub wooconfig: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
}

impl<'a> SetsuperchargervaultwhitelistAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 2] {
        [
            InstructionAccount::writable(self.wooconfig.address()),
            InstructionAccount::readonly_signer(self.authority.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 2] {
        [
            self.wooconfig, self.authority
        ]
    }
}

/// Accounts for `setWooState`
pub struct SetwoostateAccounts<'a> {
    /// wooconfig
    pub wooconfig: &'a AccountView,
    /// wooracle
    pub wooracle: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
}

impl<'a> SetwoostateAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 3] {
        [
            InstructionAccount::readonly(self.wooconfig.address()),
            InstructionAccount::writable(self.wooracle.address()),
            InstructionAccount::readonly_signer(self.authority.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 3] {
        [
            self.wooconfig, self.wooracle, self.authority
        ]
    }
}

/// Accounts for `getPrice`
pub struct GetpriceAccounts<'a> {
    /// wooconfig
    pub wooconfig: &'a AccountView,
    /// oracle
    pub oracle: &'a AccountView,
    /// priceUpdate
    pub price_update: &'a AccountView,
    /// quotePriceUpdate
    pub quote_price_update: &'a AccountView,
}

impl<'a> GetpriceAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 4] {
        [
            InstructionAccount::readonly(self.wooconfig.address()),
            InstructionAccount::readonly(self.oracle.address()),
            InstructionAccount::readonly(self.price_update.address()),
            InstructionAccount::readonly(self.quote_price_update.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 4] {
        [
            self.wooconfig, self.oracle, self.price_update, self.quote_price_update
        ]
    }
}

/// Accounts for `createPool`
pub struct CreatepoolAccounts<'a> {
    /// wooconfig
    pub wooconfig: &'a AccountView,
    /// tokenMint
    pub token_mint: &'a AccountView,
    /// quoteTokenMint
    pub quote_token_mint: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
    /// woopool
    pub woopool: &'a AccountView,
    /// tokenVault
    pub token_vault: &'a AccountView,
    /// wooracle
    pub wooracle: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
}

impl<'a> CreatepoolAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 9] {
        [
            InstructionAccount::readonly(self.wooconfig.address()),
            InstructionAccount::readonly(self.token_mint.address()),
            InstructionAccount::readonly(self.quote_token_mint.address()),
            InstructionAccount::writable_signer(self.authority.address()),
            InstructionAccount::writable(self.woopool.address()),
            InstructionAccount::writable_signer(self.token_vault.address()),
            InstructionAccount::readonly(self.wooracle.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 9] {
        [
            self.wooconfig, self.token_mint, self.quote_token_mint, self.authority, self.woopool, self.token_vault, self.wooracle, self.token_program, self.system_program
        ]
    }
}

/// Accounts for `createWooAmmPool`
pub struct CreatewooammpoolAccounts<'a> {
    /// wooconfig
    pub wooconfig: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
    /// wooammpool
    pub wooammpool: &'a AccountView,
    /// wooracleA
    pub wooracle_a: &'a AccountView,
    /// woopoolA
    pub woopool_a: &'a AccountView,
    /// wooracleB
    pub wooracle_b: &'a AccountView,
    /// woopoolB
    pub woopool_b: &'a AccountView,
    /// woopoolQuote
    pub woopool_quote: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
}

impl<'a> CreatewooammpoolAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 9] {
        [
            InstructionAccount::readonly(self.wooconfig.address()),
            InstructionAccount::writable_signer(self.authority.address()),
            InstructionAccount::writable(self.wooammpool.address()),
            InstructionAccount::readonly(self.wooracle_a.address()),
            InstructionAccount::readonly(self.woopool_a.address()),
            InstructionAccount::readonly(self.wooracle_b.address()),
            InstructionAccount::readonly(self.woopool_b.address()),
            InstructionAccount::writable(self.woopool_quote.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 9] {
        [
            self.wooconfig, self.authority, self.wooammpool, self.wooracle_a, self.woopool_a, self.wooracle_b, self.woopool_b, self.woopool_quote, self.system_program
        ]
    }
}

/// Accounts for `setPoolAdmin`
pub struct SetpooladminAccounts<'a> {
    /// wooconfig
    pub wooconfig: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
}

impl<'a> SetpooladminAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 2] {
        [
            InstructionAccount::writable(self.wooconfig.address()),
            InstructionAccount::readonly_signer(self.authority.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 2] {
        [
            self.wooconfig, self.authority
        ]
    }
}

/// Accounts for `setFeeAdmin`
pub struct SetfeeadminAccounts<'a> {
    /// wooconfig
    pub wooconfig: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
}

impl<'a> SetfeeadminAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 2] {
        [
            InstructionAccount::writable(self.wooconfig.address()),
            InstructionAccount::readonly_signer(self.authority.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 2] {
        [
            self.wooconfig, self.authority
        ]
    }
}

/// Accounts for `setPauseRole`
pub struct SetpauseroleAccounts<'a> {
    /// wooconfig
    pub wooconfig: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
}

impl<'a> SetpauseroleAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 2] {
        [
            InstructionAccount::writable(self.wooconfig.address()),
            InstructionAccount::readonly_signer(self.authority.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 2] {
        [
            self.wooconfig, self.authority
        ]
    }
}

/// Accounts for `pause`
pub struct PauseAccounts<'a> {
    /// wooconfig
    pub wooconfig: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
}

impl<'a> PauseAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 2] {
        [
            InstructionAccount::writable(self.wooconfig.address()),
            InstructionAccount::readonly_signer(self.authority.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 2] {
        [
            self.wooconfig, self.authority
        ]
    }
}

/// Accounts for `unpause`
pub struct UnpauseAccounts<'a> {
    /// wooconfig
    pub wooconfig: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
}

impl<'a> UnpauseAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 2] {
        [
            InstructionAccount::writable(self.wooconfig.address()),
            InstructionAccount::readonly_signer(self.authority.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 2] {
        [
            self.wooconfig, self.authority
        ]
    }
}

/// Accounts for `setPoolFeeRate`
pub struct SetpoolfeerateAccounts<'a> {
    /// wooconfig
    pub wooconfig: &'a AccountView,
    /// woopool
    pub woopool: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
}

impl<'a> SetpoolfeerateAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 3] {
        [
            InstructionAccount::readonly(self.wooconfig.address()),
            InstructionAccount::writable(self.woopool.address()),
            InstructionAccount::readonly_signer(self.authority.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 3] {
        [
            self.wooconfig, self.woopool, self.authority
        ]
    }
}

/// Accounts for `setPoolMaxGamma`
pub struct SetpoolmaxgammaAccounts<'a> {
    /// wooconfig
    pub wooconfig: &'a AccountView,
    /// woopool
    pub woopool: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
}

impl<'a> SetpoolmaxgammaAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 3] {
        [
            InstructionAccount::readonly(self.wooconfig.address()),
            InstructionAccount::writable(self.woopool.address()),
            InstructionAccount::readonly_signer(self.authority.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 3] {
        [
            self.wooconfig, self.woopool, self.authority
        ]
    }
}

/// Accounts for `setPoolMaxNotionalSwap`
pub struct SetpoolmaxnotionalswapAccounts<'a> {
    /// wooconfig
    pub wooconfig: &'a AccountView,
    /// woopool
    pub woopool: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
}

impl<'a> SetpoolmaxnotionalswapAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 3] {
        [
            InstructionAccount::readonly(self.wooconfig.address()),
            InstructionAccount::writable(self.woopool.address()),
            InstructionAccount::readonly_signer(self.authority.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 3] {
        [
            self.wooconfig, self.woopool, self.authority
        ]
    }
}

/// Accounts for `setPoolCapBal`
pub struct SetpoolcapbalAccounts<'a> {
    /// wooconfig
    pub wooconfig: &'a AccountView,
    /// woopool
    pub woopool: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
}

impl<'a> SetpoolcapbalAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 3] {
        [
            InstructionAccount::readonly(self.wooconfig.address()),
            InstructionAccount::writable(self.woopool.address()),
            InstructionAccount::readonly_signer(self.authority.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 3] {
        [
            self.wooconfig, self.woopool, self.authority
        ]
    }
}

/// Accounts for `setPoolMinSwapAmount`
pub struct SetpoolminswapamountAccounts<'a> {
    /// wooconfig
    pub wooconfig: &'a AccountView,
    /// woopool
    pub woopool: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
}

impl<'a> SetpoolminswapamountAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 3] {
        [
            InstructionAccount::readonly(self.wooconfig.address()),
            InstructionAccount::writable(self.woopool.address()),
            InstructionAccount::readonly_signer(self.authority.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 3] {
        [
            self.wooconfig, self.woopool, self.authority
        ]
    }
}

/// Accounts for `tryQuery`
pub struct TryqueryAccounts<'a> {
    /// wooconfig
    pub wooconfig: &'a AccountView,
    /// wooracleFrom
    pub wooracle_from: &'a AccountView,
    /// woopoolFrom
    pub woopool_from: &'a AccountView,
    /// priceUpdateFrom
    pub price_update_from: &'a AccountView,
    /// wooracleTo
    pub wooracle_to: &'a AccountView,
    /// woopoolTo
    pub woopool_to: &'a AccountView,
    /// priceUpdateTo
    pub price_update_to: &'a AccountView,
    /// quotePriceUpdate
    pub quote_price_update: &'a AccountView,
}

impl<'a> TryqueryAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 8] {
        [
            InstructionAccount::readonly(self.wooconfig.address()),
            InstructionAccount::readonly(self.wooracle_from.address()),
            InstructionAccount::readonly(self.woopool_from.address()),
            InstructionAccount::writable(self.price_update_from.address()),
            InstructionAccount::readonly(self.wooracle_to.address()),
            InstructionAccount::readonly(self.woopool_to.address()),
            InstructionAccount::writable(self.price_update_to.address()),
            InstructionAccount::readonly(self.quote_price_update.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 8] {
        [
            self.wooconfig, self.wooracle_from, self.woopool_from, self.price_update_from, self.wooracle_to, self.woopool_to, self.price_update_to, self.quote_price_update
        ]
    }
}

/// Accounts for `query`
pub struct QueryAccounts<'a> {
    /// wooconfig
    pub wooconfig: &'a AccountView,
    /// wooracleFrom
    pub wooracle_from: &'a AccountView,
    /// woopoolFrom
    pub woopool_from: &'a AccountView,
    /// tokenVaultFrom
    pub token_vault_from: &'a AccountView,
    /// priceUpdateFrom
    pub price_update_from: &'a AccountView,
    /// wooracleTo
    pub wooracle_to: &'a AccountView,
    /// woopoolTo
    pub woopool_to: &'a AccountView,
    /// tokenVaultTo
    pub token_vault_to: &'a AccountView,
    /// priceUpdateTo
    pub price_update_to: &'a AccountView,
    /// woopoolQuote
    pub woopool_quote: &'a AccountView,
    /// quotePriceUpdate
    pub quote_price_update: &'a AccountView,
    /// quoteTokenVault
    pub quote_token_vault: &'a AccountView,
}

impl<'a> QueryAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 12] {
        [
            InstructionAccount::readonly(self.wooconfig.address()),
            InstructionAccount::readonly(self.wooracle_from.address()),
            InstructionAccount::readonly(self.woopool_from.address()),
            InstructionAccount::writable(self.token_vault_from.address()),
            InstructionAccount::writable(self.price_update_from.address()),
            InstructionAccount::readonly(self.wooracle_to.address()),
            InstructionAccount::readonly(self.woopool_to.address()),
            InstructionAccount::readonly(self.token_vault_to.address()),
            InstructionAccount::writable(self.price_update_to.address()),
            InstructionAccount::readonly(self.woopool_quote.address()),
            InstructionAccount::readonly(self.quote_price_update.address()),
            InstructionAccount::readonly(self.quote_token_vault.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 12] {
        [
            self.wooconfig, self.wooracle_from, self.woopool_from, self.token_vault_from, self.price_update_from, self.wooracle_to, self.woopool_to, self.token_vault_to, self.price_update_to, self.woopool_quote, self.quote_price_update, self.quote_token_vault
        ]
    }
}

/// Accounts for `swap`
pub struct SwapAccounts<'a> {
    /// wooconfig
    pub wooconfig: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// payer
    pub payer: &'a AccountView,
    /// wooracleFrom
    pub wooracle_from: &'a AccountView,
    /// woopoolFrom
    pub woopool_from: &'a AccountView,
    /// tokenOwnerAccountFrom
    pub token_owner_account_from: &'a AccountView,
    /// tokenVaultFrom
    pub token_vault_from: &'a AccountView,
    /// priceUpdateFrom
    pub price_update_from: &'a AccountView,
    /// wooracleTo
    pub wooracle_to: &'a AccountView,
    /// woopoolTo
    pub woopool_to: &'a AccountView,
    /// tokenOwnerAccountTo
    pub token_owner_account_to: &'a AccountView,
    /// tokenVaultTo
    pub token_vault_to: &'a AccountView,
    /// priceUpdateTo
    pub price_update_to: &'a AccountView,
    /// woopoolQuote
    pub woopool_quote: &'a AccountView,
    /// quotePriceUpdate
    pub quote_price_update: &'a AccountView,
    /// quoteTokenVault
    pub quote_token_vault: &'a AccountView,
    /// rebateTo
    pub rebate_to: &'a AccountView,
}

impl<'a> SwapAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 17] {
        [
            InstructionAccount::readonly(self.wooconfig.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly_signer(self.payer.address()),
            InstructionAccount::writable(self.wooracle_from.address()),
            InstructionAccount::writable(self.woopool_from.address()),
            InstructionAccount::writable(self.token_owner_account_from.address()),
            InstructionAccount::writable(self.token_vault_from.address()),
            InstructionAccount::writable(self.price_update_from.address()),
            InstructionAccount::writable(self.wooracle_to.address()),
            InstructionAccount::writable(self.woopool_to.address()),
            InstructionAccount::writable(self.token_owner_account_to.address()),
            InstructionAccount::writable(self.token_vault_to.address()),
            InstructionAccount::writable(self.price_update_to.address()),
            InstructionAccount::writable(self.woopool_quote.address()),
            InstructionAccount::readonly(self.quote_price_update.address()),
            InstructionAccount::writable(self.quote_token_vault.address()),
            InstructionAccount::readonly(self.rebate_to.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 17] {
        [
            self.wooconfig, self.token_program, self.payer, self.wooracle_from, self.woopool_from, self.token_owner_account_from, self.token_vault_from, self.price_update_from, self.wooracle_to, self.woopool_to, self.token_owner_account_to, self.token_vault_to, self.price_update_to, self.woopool_quote, self.quote_price_update, self.quote_token_vault, self.rebate_to
        ]
    }
}

/// Accounts for `deposit`
pub struct DepositAccounts<'a> {
    /// wooconfig
    pub wooconfig: &'a AccountView,
    /// tokenMint
    pub token_mint: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
    /// tokenOwnerAccount
    pub token_owner_account: &'a AccountView,
    /// woopool
    pub woopool: &'a AccountView,
    /// tokenVault
    pub token_vault: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
}

impl<'a> DepositAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 7] {
        [
            InstructionAccount::readonly(self.wooconfig.address()),
            InstructionAccount::readonly(self.token_mint.address()),
            InstructionAccount::readonly_signer(self.authority.address()),
            InstructionAccount::writable(self.token_owner_account.address()),
            InstructionAccount::writable(self.woopool.address()),
            InstructionAccount::writable(self.token_vault.address()),
            InstructionAccount::readonly(self.token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 7] {
        [
            self.wooconfig, self.token_mint, self.authority, self.token_owner_account, self.woopool, self.token_vault, self.token_program
        ]
    }
}

/// Accounts for `withdraw`
pub struct WithdrawAccounts<'a> {
    /// wooconfig
    pub wooconfig: &'a AccountView,
    /// tokenMint
    pub token_mint: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
    /// toTokenAccount
    pub to_token_account: &'a AccountView,
    /// woopool
    pub woopool: &'a AccountView,
    /// tokenVault
    pub token_vault: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
}

impl<'a> WithdrawAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 7] {
        [
            InstructionAccount::readonly(self.wooconfig.address()),
            InstructionAccount::readonly(self.token_mint.address()),
            InstructionAccount::readonly_signer(self.authority.address()),
            InstructionAccount::writable(self.to_token_account.address()),
            InstructionAccount::writable(self.woopool.address()),
            InstructionAccount::writable(self.token_vault.address()),
            InstructionAccount::readonly(self.token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 7] {
        [
            self.wooconfig, self.token_mint, self.authority, self.to_token_account, self.woopool, self.token_vault, self.token_program
        ]
    }
}

/// Accounts for `claimFee`
pub struct ClaimfeeAccounts<'a> {
    /// wooconfig
    pub wooconfig: &'a AccountView,
    /// tokenMint
    pub token_mint: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
    /// woopool
    pub woopool: &'a AccountView,
    /// tokenVault
    pub token_vault: &'a AccountView,
    /// claimFeeToAccount
    pub claim_fee_to_account: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
}

impl<'a> ClaimfeeAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 7] {
        [
            InstructionAccount::readonly(self.wooconfig.address()),
            InstructionAccount::readonly(self.token_mint.address()),
            InstructionAccount::readonly_signer(self.authority.address()),
            InstructionAccount::writable(self.woopool.address()),
            InstructionAccount::writable(self.token_vault.address()),
            InstructionAccount::writable(self.claim_fee_to_account.address()),
            InstructionAccount::readonly(self.token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 7] {
        [
            self.wooconfig, self.token_mint, self.authority, self.woopool, self.token_vault, self.claim_fee_to_account, self.token_program
        ]
    }
}

/// Accounts for `claimFeeAmount`
pub struct ClaimfeeamountAccounts<'a> {
    /// wooconfig
    pub wooconfig: &'a AccountView,
    /// tokenMint
    pub token_mint: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
    /// woopool
    pub woopool: &'a AccountView,
    /// tokenVault
    pub token_vault: &'a AccountView,
    /// claimFeeToAccount
    pub claim_fee_to_account: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
}

impl<'a> ClaimfeeamountAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 7] {
        [
            InstructionAccount::readonly(self.wooconfig.address()),
            InstructionAccount::readonly(self.token_mint.address()),
            InstructionAccount::readonly_signer(self.authority.address()),
            InstructionAccount::writable(self.woopool.address()),
            InstructionAccount::writable(self.token_vault.address()),
            InstructionAccount::writable(self.claim_fee_to_account.address()),
            InstructionAccount::readonly(self.token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 7] {
        [
            self.wooconfig, self.token_mint, self.authority, self.woopool, self.token_vault, self.claim_fee_to_account, self.token_program
        ]
    }
}

/// Accounts for `incaseTokenGotStuck`
pub struct IncasetokengotstuckAccounts<'a> {
    /// tokenMint
    pub token_mint: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
    /// toTokenAccount
    pub to_token_account: &'a AccountView,
    /// woopool
    pub woopool: &'a AccountView,
    /// tokenVault
    pub token_vault: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
}

impl<'a> IncasetokengotstuckAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 6] {
        [
            InstructionAccount::readonly(self.token_mint.address()),
            InstructionAccount::readonly_signer(self.authority.address()),
            InstructionAccount::writable(self.to_token_account.address()),
            InstructionAccount::writable(self.woopool.address()),
            InstructionAccount::writable(self.token_vault.address()),
            InstructionAccount::readonly(self.token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 6] {
        [
            self.token_mint, self.authority, self.to_token_account, self.woopool, self.token_vault, self.token_program
        ]
    }
}

/// Accounts for `setWooconfigNewAuthority`
pub struct SetwooconfignewauthorityAccounts<'a> {
    /// authority
    pub authority: &'a AccountView,
    /// wooconfig
    pub wooconfig: &'a AccountView,
    /// newAuthority
    pub new_authority: &'a AccountView,
}

impl<'a> SetwooconfignewauthorityAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 3] {
        [
            InstructionAccount::readonly_signer(self.authority.address()),
            InstructionAccount::writable(self.wooconfig.address()),
            InstructionAccount::readonly(self.new_authority.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 3] {
        [
            self.authority, self.wooconfig, self.new_authority
        ]
    }
}

/// Accounts for `claimWooconfigAuthority`
pub struct ClaimwooconfigauthorityAccounts<'a> {
    /// newAuthority
    pub new_authority: &'a AccountView,
    /// wooconfig
    pub wooconfig: &'a AccountView,
}

impl<'a> ClaimwooconfigauthorityAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 2] {
        [
            InstructionAccount::readonly_signer(self.new_authority.address()),
            InstructionAccount::writable(self.wooconfig.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 2] {
        [
            self.new_authority, self.wooconfig
        ]
    }
}

/// Accounts for `claimWooracleAuthority`
pub struct ClaimwooracleauthorityAccounts<'a> {
    /// newAuthority
    pub new_authority: &'a AccountView,
    /// wooconfig
    pub wooconfig: &'a AccountView,
    /// wooracle
    pub wooracle: &'a AccountView,
}

impl<'a> ClaimwooracleauthorityAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 3] {
        [
            InstructionAccount::readonly_signer(self.new_authority.address()),
            InstructionAccount::readonly(self.wooconfig.address()),
            InstructionAccount::writable(self.wooracle.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 3] {
        [
            self.new_authority, self.wooconfig, self.wooracle
        ]
    }
}

/// Accounts for `claimWoopoolAuthority`
pub struct ClaimwoopoolauthorityAccounts<'a> {
    /// newAuthority
    pub new_authority: &'a AccountView,
    /// wooconfig
    pub wooconfig: &'a AccountView,
    /// woopool
    pub woopool: &'a AccountView,
}

impl<'a> ClaimwoopoolauthorityAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 3] {
        [
            InstructionAccount::readonly_signer(self.new_authority.address()),
            InstructionAccount::readonly(self.wooconfig.address()),
            InstructionAccount::writable(self.woopool.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 3] {
        [
            self.new_authority, self.wooconfig, self.woopool
        ]
    }
}

/// Accounts for `claimWooammpoolAuthority`
pub struct ClaimwooammpoolauthorityAccounts<'a> {
    /// newAuthority
    pub new_authority: &'a AccountView,
    /// wooconfig
    pub wooconfig: &'a AccountView,
    /// wooammpool
    pub wooammpool: &'a AccountView,
}

impl<'a> ClaimwooammpoolauthorityAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 3] {
        [
            InstructionAccount::readonly_signer(self.new_authority.address()),
            InstructionAccount::readonly(self.wooconfig.address()),
            InstructionAccount::writable(self.wooammpool.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 3] {
        [
            self.new_authority, self.wooconfig, self.wooammpool
        ]
    }
}

/// Accounts for `repayByLendingManager`
pub struct RepaybylendingmanagerAccounts<'a> {
    /// wooconfig
    pub wooconfig: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
    /// woopool
    pub woopool: &'a AccountView,
    /// tokenVault
    pub token_vault: &'a AccountView,
    /// superChargerVault
    pub super_charger_vault: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
}

impl<'a> RepaybylendingmanagerAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 6] {
        [
            InstructionAccount::readonly(self.wooconfig.address()),
            InstructionAccount::readonly_signer(self.authority.address()),
            InstructionAccount::writable(self.woopool.address()),
            InstructionAccount::writable(self.token_vault.address()),
            InstructionAccount::writable(self.super_charger_vault.address()),
            InstructionAccount::readonly(self.token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 6] {
        [
            self.wooconfig, self.authority, self.woopool, self.token_vault, self.super_charger_vault, self.token_program
        ]
    }
}


// ============================================
// CPI Functions
// ============================================
/// CPI: createConfig
#[inline(always)]
pub fn create_config<'a>(
    accounts: &CreateconfigAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&CREATE_CONFIG);
    
    
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

/// CPI: createWooracle
#[inline(always)]
pub fn create_wooracle<'a>(
    accounts: &CreatewooracleAccounts<'a>, args: &CreatewooracleArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<CreatewooracleArgs>()];
    data[0..8].copy_from_slice(&CREATE_WOORACLE);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const CreatewooracleArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<CreatewooracleArgs>(),
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

/// CPI: setOracleMaximumAge
#[inline(always)]
pub fn set_oracle_maximum_age<'a>(
    accounts: &SetoraclemaximumageAccounts<'a>, args: &SetoraclemaximumageArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SetoraclemaximumageArgs>()];
    data[0..8].copy_from_slice(&SET_ORACLE_MAXIMUM_AGE);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SetoraclemaximumageArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SetoraclemaximumageArgs>(),
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

/// CPI: setStaleDuration
#[inline(always)]
pub fn set_stale_duration<'a>(
    accounts: &SetstaledurationAccounts<'a>, args: &SetstaledurationArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SetstaledurationArgs>()];
    data[0..8].copy_from_slice(&SET_STALE_DURATION);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SetstaledurationArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SetstaledurationArgs>(),
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

/// CPI: setWooBound
#[inline(always)]
pub fn set_woo_bound<'a>(
    accounts: &SetwooboundAccounts<'a>, args: &SetwooboundArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SetwooboundArgs>()];
    data[0..8].copy_from_slice(&SET_WOO_BOUND);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SetwooboundArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SetwooboundArgs>(),
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

/// CPI: setWooRange
#[inline(always)]
pub fn set_woo_range<'a>(
    accounts: &SetwoorangeAccounts<'a>, args: &SetwoorangeArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SetwoorangeArgs>()];
    data[0..8].copy_from_slice(&SET_WOO_RANGE);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SetwoorangeArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SetwoorangeArgs>(),
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

/// CPI: setWooPrice
#[inline(always)]
pub fn set_woo_price<'a>(
    accounts: &SetwoopriceAccounts<'a>, args: &SetwoopriceArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SetwoopriceArgs>()];
    data[0..8].copy_from_slice(&SET_WOO_PRICE);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SetwoopriceArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SetwoopriceArgs>(),
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

/// CPI: setWooCoeff
#[inline(always)]
pub fn set_woo_coeff<'a>(
    accounts: &SetwoocoeffAccounts<'a>, args: &SetwoocoeffArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SetwoocoeffArgs>()];
    data[0..8].copy_from_slice(&SET_WOO_COEFF);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SetwoocoeffArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SetwoocoeffArgs>(),
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

/// CPI: setWooSpread
#[inline(always)]
pub fn set_woo_spread<'a>(
    accounts: &SetwoospreadAccounts<'a>, args: &SetwoospreadArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SetwoospreadArgs>()];
    data[0..8].copy_from_slice(&SET_WOO_SPREAD);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SetwoospreadArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SetwoospreadArgs>(),
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

/// CPI: setWooAdmin
#[inline(always)]
pub fn set_woo_admin<'a>(
    accounts: &SetwooadminAccounts<'a>, args: &SetwooadminArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SetwooadminArgs>()];
    data[0..8].copy_from_slice(&SET_WOO_ADMIN);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SetwooadminArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SetwooadminArgs>(),
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

/// CPI: setGuardianAdmin
#[inline(always)]
pub fn set_guardian_admin<'a>(
    accounts: &SetguardianadminAccounts<'a>, args: &SetguardianadminArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SetguardianadminArgs>()];
    data[0..8].copy_from_slice(&SET_GUARDIAN_ADMIN);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SetguardianadminArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SetguardianadminArgs>(),
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

/// CPI: setLendingManager
#[inline(always)]
pub fn set_lending_manager<'a>(
    accounts: &SetlendingmanagerAccounts<'a>, args: &SetlendingmanagerArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SetlendingmanagerArgs>()];
    data[0..8].copy_from_slice(&SET_LENDING_MANAGER);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SetlendingmanagerArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SetlendingmanagerArgs>(),
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

/// CPI: setSuperchargerVaultWhitelist
#[inline(always)]
pub fn set_supercharger_vault_whitelist<'a>(
    accounts: &SetsuperchargervaultwhitelistAccounts<'a>, args: &SetsuperchargervaultwhitelistArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SetsuperchargervaultwhitelistArgs>()];
    data[0..8].copy_from_slice(&SET_SUPERCHARGER_VAULT_WHITELIST);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SetsuperchargervaultwhitelistArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SetsuperchargervaultwhitelistArgs>(),
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

/// CPI: setWooState
#[inline(always)]
pub fn set_woo_state<'a>(
    accounts: &SetwoostateAccounts<'a>, args: &SetwoostateArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SetwoostateArgs>()];
    data[0..8].copy_from_slice(&SET_WOO_STATE);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SetwoostateArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SetwoostateArgs>(),
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

/// CPI: getPrice
#[inline(always)]
pub fn get_price<'a>(
    accounts: &GetpriceAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&GET_PRICE);
    
    
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

/// CPI: createPool
#[inline(always)]
pub fn create_pool<'a>(
    accounts: &CreatepoolAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&CREATE_POOL);
    
    
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

/// CPI: createWooAmmPool
#[inline(always)]
pub fn create_woo_amm_pool<'a>(
    accounts: &CreatewooammpoolAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&CREATE_WOO_AMM_POOL);
    
    
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

/// CPI: setPoolAdmin
#[inline(always)]
pub fn set_pool_admin<'a>(
    accounts: &SetpooladminAccounts<'a>, args: &SetpooladminArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SetpooladminArgs>()];
    data[0..8].copy_from_slice(&SET_POOL_ADMIN);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SetpooladminArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SetpooladminArgs>(),
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

/// CPI: setFeeAdmin
#[inline(always)]
pub fn set_fee_admin<'a>(
    accounts: &SetfeeadminAccounts<'a>, args: &SetfeeadminArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SetfeeadminArgs>()];
    data[0..8].copy_from_slice(&SET_FEE_ADMIN);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SetfeeadminArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SetfeeadminArgs>(),
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

/// CPI: setPauseRole
#[inline(always)]
pub fn set_pause_role<'a>(
    accounts: &SetpauseroleAccounts<'a>, args: &SetpauseroleArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SetpauseroleArgs>()];
    data[0..8].copy_from_slice(&SET_PAUSE_ROLE);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SetpauseroleArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SetpauseroleArgs>(),
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

/// CPI: setPoolFeeRate
#[inline(always)]
pub fn set_pool_fee_rate<'a>(
    accounts: &SetpoolfeerateAccounts<'a>, args: &SetpoolfeerateArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SetpoolfeerateArgs>()];
    data[0..8].copy_from_slice(&SET_POOL_FEE_RATE);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SetpoolfeerateArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SetpoolfeerateArgs>(),
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

/// CPI: setPoolMaxGamma
#[inline(always)]
pub fn set_pool_max_gamma<'a>(
    accounts: &SetpoolmaxgammaAccounts<'a>, args: &SetpoolmaxgammaArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SetpoolmaxgammaArgs>()];
    data[0..8].copy_from_slice(&SET_POOL_MAX_GAMMA);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SetpoolmaxgammaArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SetpoolmaxgammaArgs>(),
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

/// CPI: setPoolMaxNotionalSwap
#[inline(always)]
pub fn set_pool_max_notional_swap<'a>(
    accounts: &SetpoolmaxnotionalswapAccounts<'a>, args: &SetpoolmaxnotionalswapArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SetpoolmaxnotionalswapArgs>()];
    data[0..8].copy_from_slice(&SET_POOL_MAX_NOTIONAL_SWAP);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SetpoolmaxnotionalswapArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SetpoolmaxnotionalswapArgs>(),
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

/// CPI: setPoolCapBal
#[inline(always)]
pub fn set_pool_cap_bal<'a>(
    accounts: &SetpoolcapbalAccounts<'a>, args: &SetpoolcapbalArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SetpoolcapbalArgs>()];
    data[0..8].copy_from_slice(&SET_POOL_CAP_BAL);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SetpoolcapbalArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SetpoolcapbalArgs>(),
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

/// CPI: setPoolMinSwapAmount
#[inline(always)]
pub fn set_pool_min_swap_amount<'a>(
    accounts: &SetpoolminswapamountAccounts<'a>, args: &SetpoolminswapamountArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SetpoolminswapamountArgs>()];
    data[0..8].copy_from_slice(&SET_POOL_MIN_SWAP_AMOUNT);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SetpoolminswapamountArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SetpoolminswapamountArgs>(),
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

/// CPI: tryQuery
#[inline(always)]
pub fn try_query<'a>(
    accounts: &TryqueryAccounts<'a>, args: &TryqueryArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<TryqueryArgs>()];
    data[0..8].copy_from_slice(&TRY_QUERY);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const TryqueryArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<TryqueryArgs>(),
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

/// CPI: query
#[inline(always)]
pub fn query<'a>(
    accounts: &QueryAccounts<'a>, args: &QueryArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<QueryArgs>()];
    data[0..8].copy_from_slice(&QUERY);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const QueryArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<QueryArgs>(),
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

/// CPI: claimFee
#[inline(always)]
pub fn claim_fee<'a>(
    accounts: &ClaimfeeAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&CLAIM_FEE);
    
    
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

/// CPI: claimFeeAmount
#[inline(always)]
pub fn claim_fee_amount<'a>(
    accounts: &ClaimfeeamountAccounts<'a>, args: &ClaimfeeamountArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<ClaimfeeamountArgs>()];
    data[0..8].copy_from_slice(&CLAIM_FEE_AMOUNT);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const ClaimfeeamountArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<ClaimfeeamountArgs>(),
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

/// CPI: incaseTokenGotStuck
#[inline(always)]
pub fn incase_token_got_stuck<'a>(
    accounts: &IncasetokengotstuckAccounts<'a>, args: &IncasetokengotstuckArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<IncasetokengotstuckArgs>()];
    data[0..8].copy_from_slice(&INCASE_TOKEN_GOT_STUCK);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const IncasetokengotstuckArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<IncasetokengotstuckArgs>(),
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

/// CPI: setWooconfigNewAuthority
#[inline(always)]
pub fn set_wooconfig_new_authority<'a>(
    accounts: &SetwooconfignewauthorityAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&SET_WOOCONFIG_NEW_AUTHORITY);
    
    
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

/// CPI: claimWooconfigAuthority
#[inline(always)]
pub fn claim_wooconfig_authority<'a>(
    accounts: &ClaimwooconfigauthorityAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&CLAIM_WOOCONFIG_AUTHORITY);
    
    
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

/// CPI: claimWooracleAuthority
#[inline(always)]
pub fn claim_wooracle_authority<'a>(
    accounts: &ClaimwooracleauthorityAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&CLAIM_WOORACLE_AUTHORITY);
    
    
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

/// CPI: claimWoopoolAuthority
#[inline(always)]
pub fn claim_woopool_authority<'a>(
    accounts: &ClaimwoopoolauthorityAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&CLAIM_WOOPOOL_AUTHORITY);
    
    
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

/// CPI: claimWooammpoolAuthority
#[inline(always)]
pub fn claim_wooammpool_authority<'a>(
    accounts: &ClaimwooammpoolauthorityAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&CLAIM_WOOAMMPOOL_AUTHORITY);
    
    
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

/// CPI: repayByLendingManager
#[inline(always)]
pub fn repay_by_lending_manager<'a>(
    accounts: &RepaybylendingmanagerAccounts<'a>, args: &RepaybylendingmanagerArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<RepaybylendingmanagerArgs>()];
    data[0..8].copy_from_slice(&REPAY_BY_LENDING_MANAGER);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const RepaybylendingmanagerArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<RepaybylendingmanagerArgs>(),
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

