//! CPI module for Boop fun
//!
//! Program: boop
//! Program ID: boop8hVGQGqehUK2iVEMEnMrL5RbjywRzHKBmBE7ry4
//! Instructions: 29

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
pub const ID: Address = Address::new_from_array(five8_const::decode_32_const("boop8hVGQGqehUK2iVEMEnMrL5RbjywRzHKBmBE7ry4"));

// ============================================
// Instruction Discriminators
// ============================================
pub const ADD_OPERATORS: [u8; 8] = [165, 199, 62, 214, 81, 54, 4, 150];
pub const BUY_TOKEN: [u8; 8] = [138, 127, 14, 91, 38, 87, 115, 105];
pub const CANCEL_AUTHORITY_TRANSFER: [u8; 8] = [94, 131, 125, 184, 183, 24, 125, 229];
pub const CLOSE_BONDING_CURVE_VAULT: [u8; 8] = [189, 71, 189, 239, 113, 66, 59, 189];
pub const COLLECT_METEORA_TRADING_FEES: [u8; 8] = [249, 95, 126, 91, 81, 162, 83, 250];
pub const COLLECT_METEORA_TRADING_FEES_V2: [u8; 8] = [96, 39, 109, 46, 5, 161, 15, 57];
pub const COLLECT_TRADING_FEES: [u8; 8] = [189, 38, 205, 234, 81, 77, 25, 1];
pub const COLLECT_TRADING_FEES_V2: [u8; 8] = [180, 138, 160, 155, 243, 88, 168, 7];
pub const COMPLETE_AUTHORITY_TRANSFER: [u8; 8] = [81, 233, 91, 132, 175, 31, 151, 141];
pub const CREATE_METEORA_POOL: [u8; 8] = [246, 254, 33, 37, 225, 176, 41, 232];
pub const CREATE_RAYDIUM_POOL: [u8; 8] = [65, 45, 119, 77, 204, 178, 84, 2];
pub const CREATE_RAYDIUM_RANDOM_POOL: [u8; 8] = [78, 44, 173, 29, 132, 180, 4, 172];
pub const CREATE_TOKEN: [u8; 8] = [84, 52, 204, 228, 24, 140, 234, 75];
pub const CREATE_TOKEN_FALLBACK: [u8; 8] = [253, 184, 126, 199, 235, 232, 172, 162];
pub const DEPLOY_BONDING_CURVE: [u8; 8] = [180, 89, 199, 76, 168, 236, 217, 138];
pub const DEPLOY_BONDING_CURVE_FALLBACK: [u8; 8] = [53, 230, 172, 84, 77, 174, 22, 61];
pub const DEPOSIT_INTO_RAYDIUM: [u8; 8] = [168, 89, 99, 30, 117, 49, 88, 224];
pub const GRADUATE: [u8; 8] = [45, 235, 225, 181, 17, 218, 64, 130];
pub const INITIALIZE: [u8; 8] = [175, 175, 109, 31, 13, 152, 155, 237];
pub const INITIATE_AUTHORITY_TRANSFER: [u8; 8] = [210, 43, 101, 215, 119, 140, 106, 218];
pub const LOCK_RAYDIUM_LIQUIDITY: [u8; 8] = [173, 255, 148, 6, 122, 99, 140, 22];
pub const REMOVE_OPERATORS: [u8; 8] = [42, 20, 89, 83, 222, 37, 4, 109];
pub const SELL_TOKEN: [u8; 8] = [109, 61, 40, 187, 230, 176, 135, 174];
pub const SPLIT_POST_GRADUATION_TRADING_FEES: [u8; 8] = [241, 178, 177, 69, 38, 187, 58, 176];
pub const SPLIT_TRADING_FEES: [u8; 8] = [96, 126, 225, 47, 185, 213, 50, 58];
pub const SWAP_SOL_FOR_TOKENS_ON_RAYDIUM: [u8; 8] = [107, 248, 131, 239, 152, 234, 54, 35];
pub const SWAP_TOKENS_FOR_SOL_ON_RAYDIUM: [u8; 8] = [216, 172, 130, 148, 34, 98, 215, 163];
pub const TOGGLE_PAUSED: [u8; 8] = [54, 83, 147, 198, 123, 97, 218, 72];
pub const UPDATE_CONFIG: [u8; 8] = [29, 158, 252, 191, 10, 83, 219, 99];

// ============================================
// Instruction Arguments
// ============================================
/// Arguments for `add_operators`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct AddOperatorsArgs {
    pub operators: [u8; 32],
}

/// Arguments for `buy_token`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct BuyTokenArgs {
    pub buy_amount: u64,
    pub amount_out_min: u64,
}

/// Arguments for `create_token`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct CreateTokenArgs {
    pub salt: u64,
    pub name: [u8; 32],
    pub symbol: [u8; 32],
    pub uri: [u8; 32],
}

/// Arguments for `create_token_fallback`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct CreateTokenFallbackArgs {
    pub salt: u64,
    pub name: [u8; 32],
    pub symbol: [u8; 32],
    pub uri: [u8; 32],
}

/// Arguments for `deploy_bonding_curve`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct DeployBondingCurveArgs {
    pub creator: [u8; 32],
    pub salt: u64,
}

/// Arguments for `deploy_bonding_curve_fallback`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct DeployBondingCurveFallbackArgs {
    pub creator: [u8; 32],
    pub salt: u64,
}

/// Arguments for `deposit_into_raydium`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct DepositIntoRaydiumArgs {
    pub lp_token_amount: u64,
    pub maximum_token_0_amount: u64,
    pub maximum_token_1_amount: u64,
}

/// Arguments for `initialize`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct InitializeArgs {
    pub protocol_fee_recipient: [u8; 32],
    pub token_distributor: [u8; 32],
}

/// Arguments for `initiate_authority_transfer`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct InitiateAuthorityTransferArgs {
    pub new_authority: [u8; 32],
}

/// Arguments for `remove_operators`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct RemoveOperatorsArgs {
    pub operators: [u8; 32],
}

/// Arguments for `sell_token`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SellTokenArgs {
    pub sell_amount: u64,
    pub amount_out_min: u64,
}

/// Arguments for `swap_sol_for_tokens_on_raydium`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SwapSolForTokensOnRaydiumArgs {
    pub amount_in: u64,
    pub minimum_amount_out: u64,
}

/// Arguments for `swap_tokens_for_sol_on_raydium`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SwapTokensForSolOnRaydiumArgs {
    pub amount_in: u64,
    pub minimum_amount_out: u64,
}

/// Arguments for `update_config`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct UpdateConfigArgs {
    pub new_protocol_fee_recipient: [u8; 32],
    pub new_virtual_sol_reserves: u64,
    pub new_virtual_token_reserves: u64,
    pub new_graduation_target: u64,
    pub new_graduation_fee: u64,
    pub new_damping_term: u8,
    pub new_swap_fee_basis_points: u8,
    pub new_token_for_stakers_basis_points: u16,
    pub new_token_amount_for_raydium_liquidity: u64,
    pub new_max_graduation_price_deviation_basis_points: u16,
    pub new_max_swap_amount_for_pool_price_correction_basis_points: u16,
}


// ============================================
// Instruction Accounts
// ============================================
/// Accounts for `add_operators`
pub struct AddOperatorsAccounts<'a> {
    /// config
    pub config: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
}

impl<'a> AddOperatorsAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 3] {
        [
            InstructionAccount::writable(self.config.address()),
            InstructionAccount::writable_signer(self.authority.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 3] {
        [
            self.config, self.authority, self.system_program
        ]
    }
}

/// Accounts for `buy_token`
pub struct BuyTokenAccounts<'a> {
    /// mint
    pub mint: &'a AccountView,
    /// bonding_curve
    pub bonding_curve: &'a AccountView,
    /// trading_fees_vault
    pub trading_fees_vault: &'a AccountView,
    /// bonding_curve_vault
    pub bonding_curve_vault: &'a AccountView,
    /// bonding_curve_sol_vault
    pub bonding_curve_sol_vault: &'a AccountView,
    /// recipient_token_account
    pub recipient_token_account: &'a AccountView,
    /// buyer
    pub buyer: &'a AccountView,
    /// config
    pub config: &'a AccountView,
    /// vault_authority
    pub vault_authority: &'a AccountView,
    /// wsol
    pub wsol: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
    /// token_program
    pub token_program: &'a AccountView,
    /// associated_token_program
    pub associated_token_program: &'a AccountView,
}

impl<'a> BuyTokenAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 13] {
        [
            InstructionAccount::readonly(self.mint.address()),
            InstructionAccount::writable(self.bonding_curve.address()),
            InstructionAccount::writable(self.trading_fees_vault.address()),
            InstructionAccount::writable(self.bonding_curve_vault.address()),
            InstructionAccount::writable(self.bonding_curve_sol_vault.address()),
            InstructionAccount::readonly(self.recipient_token_account.address()),
            InstructionAccount::writable_signer(self.buyer.address()),
            InstructionAccount::readonly(self.config.address()),
            InstructionAccount::readonly(self.vault_authority.address()),
            InstructionAccount::readonly(self.wsol.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 13] {
        [
            self.mint, self.bonding_curve, self.trading_fees_vault, self.bonding_curve_vault, self.bonding_curve_sol_vault, self.recipient_token_account, self.buyer, self.config, self.vault_authority, self.wsol, self.system_program, self.token_program, self.associated_token_program
        ]
    }
}

/// Accounts for `cancel_authority_transfer`
pub struct CancelAuthorityTransferAccounts<'a> {
    /// authority
    pub authority: &'a AccountView,
    /// config
    pub config: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
}

impl<'a> CancelAuthorityTransferAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 3] {
        [
            InstructionAccount::readonly_signer(self.authority.address()),
            InstructionAccount::writable(self.config.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 3] {
        [
            self.authority, self.config, self.system_program
        ]
    }
}

/// Accounts for `close_bonding_curve_vault`
pub struct CloseBondingCurveVaultAccounts<'a> {
    /// config
    pub config: &'a AccountView,
    /// operator
    pub operator: &'a AccountView,
    /// vault_authority
    pub vault_authority: &'a AccountView,
    /// bonding_curve
    pub bonding_curve: &'a AccountView,
    /// bonding_curve_vault
    pub bonding_curve_vault: &'a AccountView,
    /// mint
    pub mint: &'a AccountView,
    /// recipient_token_account
    pub recipient_token_account: &'a AccountView,
    /// recipient
    pub recipient: &'a AccountView,
    /// token_program
    pub token_program: &'a AccountView,
    /// System program for creating associated token if needed
    pub system_program: &'a AccountView,
    /// associated_token_program
    pub associated_token_program: &'a AccountView,
    /// rent
    pub rent: &'a AccountView,
}

impl<'a> CloseBondingCurveVaultAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 12] {
        [
            InstructionAccount::readonly(self.config.address()),
            InstructionAccount::writable_signer(self.operator.address()),
            InstructionAccount::readonly(self.vault_authority.address()),
            InstructionAccount::writable(self.bonding_curve.address()),
            InstructionAccount::writable(self.bonding_curve_vault.address()),
            InstructionAccount::readonly(self.mint.address()),
            InstructionAccount::writable(self.recipient_token_account.address()),
            InstructionAccount::writable(self.recipient.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
            InstructionAccount::readonly(self.rent.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 12] {
        [
            self.config, self.operator, self.vault_authority, self.bonding_curve, self.bonding_curve_vault, self.mint, self.recipient_token_account, self.recipient, self.token_program, self.system_program, self.associated_token_program, self.rent
        ]
    }
}

/// Accounts for `collect_meteora_trading_fees`
pub struct CollectMeteoraTradingFeesAccounts<'a> {
    /// operator
    pub operator: &'a AccountView,
    /// protocol_fee_recipient
    pub protocol_fee_recipient: &'a AccountView,
    /// config
    pub config: &'a AccountView,
    /// pool_authority
    pub pool_authority: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// position
    pub position: &'a AccountView,
    /// The token account to receive token a
    pub token_a_account: &'a AccountView,
    /// The token account to receive token b
    pub token_b_account: &'a AccountView,
    /// token_a_vault
    pub token_a_vault: &'a AccountView,
    /// token_b_vault
    pub token_b_vault: &'a AccountView,
    /// token_a_mint
    pub token_a_mint: &'a AccountView,
    /// token_b_mint
    pub token_b_mint: &'a AccountView,
    /// position_nft_account
    pub position_nft_account: &'a AccountView,
    /// vault_authority
    pub vault_authority: &'a AccountView,
    /// token_program
    pub token_program: &'a AccountView,
    /// associated_token_program
    pub associated_token_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// cp_amm_program
    pub cp_amm_program: &'a AccountView,
}

impl<'a> CollectMeteoraTradingFeesAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 18] {
        [
            InstructionAccount::writable_signer(self.operator.address()),
            InstructionAccount::readonly(self.protocol_fee_recipient.address()),
            InstructionAccount::readonly(self.config.address()),
            InstructionAccount::readonly(self.pool_authority.address()),
            InstructionAccount::readonly(self.pool.address()),
            InstructionAccount::writable(self.position.address()),
            InstructionAccount::writable(self.token_a_account.address()),
            InstructionAccount::writable(self.token_b_account.address()),
            InstructionAccount::writable(self.token_a_vault.address()),
            InstructionAccount::writable(self.token_b_vault.address()),
            InstructionAccount::readonly(self.token_a_mint.address()),
            InstructionAccount::readonly(self.token_b_mint.address()),
            InstructionAccount::writable(self.position_nft_account.address()),
            InstructionAccount::readonly(self.vault_authority.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.cp_amm_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 18] {
        [
            self.operator, self.protocol_fee_recipient, self.config, self.pool_authority, self.pool, self.position, self.token_a_account, self.token_b_account, self.token_a_vault, self.token_b_vault, self.token_a_mint, self.token_b_mint, self.position_nft_account, self.vault_authority, self.token_program, self.associated_token_program, self.event_authority, self.cp_amm_program
        ]
    }
}

/// Accounts for `collect_meteora_trading_fees_v2`
pub struct CollectMeteoraTradingFeesV2Accounts<'a> {
    /// operator
    pub operator: &'a AccountView,
    /// protocol_fee_recipient
    pub protocol_fee_recipient: &'a AccountView,
    /// config
    pub config: &'a AccountView,
    /// pool_authority
    pub pool_authority: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// position
    pub position: &'a AccountView,
    /// The token account to receive token a
    pub token_a_account: &'a AccountView,
    /// The token account to receive token b
    pub token_b_account: &'a AccountView,
    /// token_a_vault
    pub token_a_vault: &'a AccountView,
    /// token_b_vault
    pub token_b_vault: &'a AccountView,
    /// token_a_mint
    pub token_a_mint: &'a AccountView,
    /// token_b_mint
    pub token_b_mint: &'a AccountView,
    /// position_nft_account
    pub position_nft_account: &'a AccountView,
    /// vault_authority
    pub vault_authority: &'a AccountView,
    /// token_program
    pub token_program: &'a AccountView,
    /// associated_token_program
    pub associated_token_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// cp_amm_program
    pub cp_amm_program: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
}

impl<'a> CollectMeteoraTradingFeesV2Accounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 19] {
        [
            InstructionAccount::writable_signer(self.operator.address()),
            InstructionAccount::readonly(self.protocol_fee_recipient.address()),
            InstructionAccount::readonly(self.config.address()),
            InstructionAccount::readonly(self.pool_authority.address()),
            InstructionAccount::readonly(self.pool.address()),
            InstructionAccount::writable(self.position.address()),
            InstructionAccount::writable(self.token_a_account.address()),
            InstructionAccount::writable(self.token_b_account.address()),
            InstructionAccount::writable(self.token_a_vault.address()),
            InstructionAccount::writable(self.token_b_vault.address()),
            InstructionAccount::readonly(self.token_a_mint.address()),
            InstructionAccount::readonly(self.token_b_mint.address()),
            InstructionAccount::writable(self.position_nft_account.address()),
            InstructionAccount::readonly(self.vault_authority.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.cp_amm_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 19] {
        [
            self.operator, self.protocol_fee_recipient, self.config, self.pool_authority, self.pool, self.position, self.token_a_account, self.token_b_account, self.token_a_vault, self.token_b_vault, self.token_a_mint, self.token_b_mint, self.position_nft_account, self.vault_authority, self.token_program, self.associated_token_program, self.event_authority, self.cp_amm_program, self.system_program
        ]
    }
}

/// Accounts for `collect_trading_fees`
pub struct CollectTradingFeesAccounts<'a> {
    /// operator
    pub operator: &'a AccountView,
    /// protocol_fee_recipient
    pub protocol_fee_recipient: &'a AccountView,
    /// config
    pub config: &'a AccountView,
    /// lock_program
    pub lock_program: &'a AccountView,
    /// vault_authority
    pub vault_authority: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
    /// Fee token account
    pub fee_nft_account: &'a AccountView,
    /// Store the locked the information of liquidity
    pub locked_liquidity: &'a AccountView,
    /// cpmm_program
    pub cpmm_program: &'a AccountView,
    /// cp_authority
    pub cp_authority: &'a AccountView,
    /// pool_state
    pub pool_state: &'a AccountView,
    /// lp mint
    pub lp_mint: &'a AccountView,
    /// The token account for receive token_0
    pub recipient_token_0_account: &'a AccountView,
    /// The token account for receive token_1
    pub recipient_token_1_account: &'a AccountView,
    /// The address that holds pool tokens for token_0
    pub token_0_vault: &'a AccountView,
    /// The address that holds pool tokens for token_1
    pub token_1_vault: &'a AccountView,
    /// The mint of token_0 vault
    pub vault_0_mint: &'a AccountView,
    /// The mint of token_1 vault
    pub vault_1_mint: &'a AccountView,
    /// locked lp token account
    pub locked_lp_vault: &'a AccountView,
    /// System program
    pub system_program: &'a AccountView,
    /// Associated token program
    pub associated_token_program: &'a AccountView,
    /// token Program
    pub token_program: &'a AccountView,
    /// Token program 2022
    pub token_program_2022: &'a AccountView,
    /// memo program
    pub memo_program: &'a AccountView,
}

impl<'a> CollectTradingFeesAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 24] {
        [
            InstructionAccount::writable_signer(self.operator.address()),
            InstructionAccount::readonly(self.protocol_fee_recipient.address()),
            InstructionAccount::readonly(self.config.address()),
            InstructionAccount::readonly(self.lock_program.address()),
            InstructionAccount::readonly(self.vault_authority.address()),
            InstructionAccount::readonly(self.authority.address()),
            InstructionAccount::readonly(self.fee_nft_account.address()),
            InstructionAccount::writable(self.locked_liquidity.address()),
            InstructionAccount::readonly(self.cpmm_program.address()),
            InstructionAccount::readonly(self.cp_authority.address()),
            InstructionAccount::writable(self.pool_state.address()),
            InstructionAccount::writable(self.lp_mint.address()),
            InstructionAccount::writable(self.recipient_token_0_account.address()),
            InstructionAccount::writable(self.recipient_token_1_account.address()),
            InstructionAccount::writable(self.token_0_vault.address()),
            InstructionAccount::writable(self.token_1_vault.address()),
            InstructionAccount::readonly(self.vault_0_mint.address()),
            InstructionAccount::readonly(self.vault_1_mint.address()),
            InstructionAccount::writable(self.locked_lp_vault.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.token_program_2022.address()),
            InstructionAccount::readonly(self.memo_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 24] {
        [
            self.operator, self.protocol_fee_recipient, self.config, self.lock_program, self.vault_authority, self.authority, self.fee_nft_account, self.locked_liquidity, self.cpmm_program, self.cp_authority, self.pool_state, self.lp_mint, self.recipient_token_0_account, self.recipient_token_1_account, self.token_0_vault, self.token_1_vault, self.vault_0_mint, self.vault_1_mint, self.locked_lp_vault, self.system_program, self.associated_token_program, self.token_program, self.token_program_2022, self.memo_program
        ]
    }
}

/// Accounts for `collect_trading_fees_v2`
pub struct CollectTradingFeesV2Accounts<'a> {
    /// operator
    pub operator: &'a AccountView,
    /// protocol_fee_recipient
    pub protocol_fee_recipient: &'a AccountView,
    /// config
    pub config: &'a AccountView,
    /// lock_program
    pub lock_program: &'a AccountView,
    /// vault_authority
    pub vault_authority: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
    /// Fee token account
    pub fee_nft_account: &'a AccountView,
    /// Store the locked the information of liquidity
    pub locked_liquidity: &'a AccountView,
    /// cpmm_program
    pub cpmm_program: &'a AccountView,
    /// cp_authority
    pub cp_authority: &'a AccountView,
    /// pool_state
    pub pool_state: &'a AccountView,
    /// lp mint
    pub lp_mint: &'a AccountView,
    /// The token account for receive token_0
    pub recipient_token_0_account: &'a AccountView,
    /// The token account for receive token_1
    pub recipient_token_1_account: &'a AccountView,
    /// The address that holds pool tokens for token_0
    pub token_0_vault: &'a AccountView,
    /// The address that holds pool tokens for token_1
    pub token_1_vault: &'a AccountView,
    /// The mint of token_0 vault
    pub vault_0_mint: &'a AccountView,
    /// The mint of token_1 vault
    pub vault_1_mint: &'a AccountView,
    /// locked lp token account
    pub locked_lp_vault: &'a AccountView,
    /// System program
    pub system_program: &'a AccountView,
    /// Associated token program
    pub associated_token_program: &'a AccountView,
    /// token Program
    pub token_program: &'a AccountView,
    /// Token program 2022
    pub token_program_2022: &'a AccountView,
    /// memo program
    pub memo_program: &'a AccountView,
}

impl<'a> CollectTradingFeesV2Accounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 24] {
        [
            InstructionAccount::writable_signer(self.operator.address()),
            InstructionAccount::readonly(self.protocol_fee_recipient.address()),
            InstructionAccount::readonly(self.config.address()),
            InstructionAccount::readonly(self.lock_program.address()),
            InstructionAccount::readonly(self.vault_authority.address()),
            InstructionAccount::readonly(self.authority.address()),
            InstructionAccount::readonly(self.fee_nft_account.address()),
            InstructionAccount::writable(self.locked_liquidity.address()),
            InstructionAccount::readonly(self.cpmm_program.address()),
            InstructionAccount::readonly(self.cp_authority.address()),
            InstructionAccount::writable(self.pool_state.address()),
            InstructionAccount::writable(self.lp_mint.address()),
            InstructionAccount::writable(self.recipient_token_0_account.address()),
            InstructionAccount::writable(self.recipient_token_1_account.address()),
            InstructionAccount::writable(self.token_0_vault.address()),
            InstructionAccount::writable(self.token_1_vault.address()),
            InstructionAccount::readonly(self.vault_0_mint.address()),
            InstructionAccount::readonly(self.vault_1_mint.address()),
            InstructionAccount::writable(self.locked_lp_vault.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.token_program_2022.address()),
            InstructionAccount::readonly(self.memo_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 24] {
        [
            self.operator, self.protocol_fee_recipient, self.config, self.lock_program, self.vault_authority, self.authority, self.fee_nft_account, self.locked_liquidity, self.cpmm_program, self.cp_authority, self.pool_state, self.lp_mint, self.recipient_token_0_account, self.recipient_token_1_account, self.token_0_vault, self.token_1_vault, self.vault_0_mint, self.vault_1_mint, self.locked_lp_vault, self.system_program, self.associated_token_program, self.token_program, self.token_program_2022, self.memo_program
        ]
    }
}

/// Accounts for `complete_authority_transfer`
pub struct CompleteAuthorityTransferAccounts<'a> {
    /// pending_authority
    pub pending_authority: &'a AccountView,
    /// config
    pub config: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
}

impl<'a> CompleteAuthorityTransferAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 3] {
        [
            InstructionAccount::readonly_signer(self.pending_authority.address()),
            InstructionAccount::writable(self.config.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 3] {
        [
            self.pending_authority, self.config, self.system_program
        ]
    }
}

/// Accounts for `create_meteora_pool`
pub struct CreateMeteoraPoolAccounts<'a> {
    /// operator
    pub operator: &'a AccountView,
    /// config
    pub config: &'a AccountView,
    /// vault_authority
    pub vault_authority: &'a AccountView,
    /// cp_amm_config
    pub cp_amm_config: &'a AccountView,
    /// pool_authority
    pub pool_authority: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// position
    pub position: &'a AccountView,
    /// position_nft_mint
    pub position_nft_mint: &'a AccountView,
    /// position_nft_account
    pub position_nft_account: &'a AccountView,
    /// token_a_mint
    pub token_a_mint: &'a AccountView,
    /// token_b_mint
    pub token_b_mint: &'a AccountView,
    /// token_a_vault
    pub token_a_vault: &'a AccountView,
    /// token_b_vault
    pub token_b_vault: &'a AccountView,
    /// bonding_curve
    pub bonding_curve: &'a AccountView,
    /// bonding_curve_vault
    pub bonding_curve_vault: &'a AccountView,
    /// bonding_curve_wsol_vault
    pub bonding_curve_wsol_vault: &'a AccountView,
    /// token_program
    pub token_program: &'a AccountView,
    /// token_2022_program
    pub token_2022_program: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// cp_amm_program
    pub cp_amm_program: &'a AccountView,
}

impl<'a> CreateMeteoraPoolAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 21] {
        [
            InstructionAccount::writable_signer(self.operator.address()),
            InstructionAccount::readonly(self.config.address()),
            InstructionAccount::writable(self.vault_authority.address()),
            InstructionAccount::readonly(self.cp_amm_config.address()),
            InstructionAccount::readonly(self.pool_authority.address()),
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.position.address()),
            InstructionAccount::writable_signer(self.position_nft_mint.address()),
            InstructionAccount::writable(self.position_nft_account.address()),
            InstructionAccount::readonly(self.token_a_mint.address()),
            InstructionAccount::readonly(self.token_b_mint.address()),
            InstructionAccount::writable(self.token_a_vault.address()),
            InstructionAccount::writable(self.token_b_vault.address()),
            InstructionAccount::writable(self.bonding_curve.address()),
            InstructionAccount::writable(self.bonding_curve_vault.address()),
            InstructionAccount::writable(self.bonding_curve_wsol_vault.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.token_2022_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.cp_amm_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 21] {
        [
            self.operator, self.config, self.vault_authority, self.cp_amm_config, self.pool_authority, self.pool, self.position, self.position_nft_mint, self.position_nft_account, self.token_a_mint, self.token_b_mint, self.token_a_vault, self.token_b_vault, self.bonding_curve, self.bonding_curve_vault, self.bonding_curve_wsol_vault, self.token_program, self.token_2022_program, self.system_program, self.event_authority, self.cp_amm_program
        ]
    }
}

/// Accounts for `create_raydium_pool`
pub struct CreateRaydiumPoolAccounts<'a> {
    /// cpmm_program
    pub cpmm_program: &'a AccountView,
    /// Which config the pool belongs to.
    pub amm_config: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
    /// pool_state
    pub pool_state: &'a AccountView,
    /// token_0_mint
    pub token_0_mint: &'a AccountView,
    /// Token_1 mint, the key must be greater than token_0 mint.
    pub token_1_mint: &'a AccountView,
    /// lp_mint
    pub lp_mint: &'a AccountView,
    /// vault_authority
    pub vault_authority: &'a AccountView,
    /// bonding_curve
    pub bonding_curve: &'a AccountView,
    /// bonding_curve_vault
    pub bonding_curve_vault: &'a AccountView,
    /// bonding_curve_wsol_vault
    pub bonding_curve_wsol_vault: &'a AccountView,
    /// creator_lp_token
    pub creator_lp_token: &'a AccountView,
    /// token_0_vault
    pub token_0_vault: &'a AccountView,
    /// token_1_vault
    pub token_1_vault: &'a AccountView,
    /// create pool fee account
    pub create_pool_fee: &'a AccountView,
    /// observation_state
    pub observation_state: &'a AccountView,
    /// operator
    pub operator: &'a AccountView,
    /// config
    pub config: &'a AccountView,
    /// Program to create mint account and mint tokens
    pub token_program: &'a AccountView,
    /// Program to create an ATA for receiving position NFT
    pub associated_token_program: &'a AccountView,
    /// To create a new program account
    pub system_program: &'a AccountView,
    /// Sysvar for program account
    pub rent: &'a AccountView,
}

impl<'a> CreateRaydiumPoolAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 22] {
        [
            InstructionAccount::readonly(self.cpmm_program.address()),
            InstructionAccount::readonly(self.amm_config.address()),
            InstructionAccount::readonly(self.authority.address()),
            InstructionAccount::writable(self.pool_state.address()),
            InstructionAccount::writable(self.token_0_mint.address()),
            InstructionAccount::readonly(self.token_1_mint.address()),
            InstructionAccount::writable(self.lp_mint.address()),
            InstructionAccount::writable(self.vault_authority.address()),
            InstructionAccount::writable(self.bonding_curve.address()),
            InstructionAccount::writable(self.bonding_curve_vault.address()),
            InstructionAccount::writable(self.bonding_curve_wsol_vault.address()),
            InstructionAccount::writable(self.creator_lp_token.address()),
            InstructionAccount::writable(self.token_0_vault.address()),
            InstructionAccount::writable(self.token_1_vault.address()),
            InstructionAccount::writable(self.create_pool_fee.address()),
            InstructionAccount::writable(self.observation_state.address()),
            InstructionAccount::writable_signer(self.operator.address()),
            InstructionAccount::readonly(self.config.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.rent.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 22] {
        [
            self.cpmm_program, self.amm_config, self.authority, self.pool_state, self.token_0_mint, self.token_1_mint, self.lp_mint, self.vault_authority, self.bonding_curve, self.bonding_curve_vault, self.bonding_curve_wsol_vault, self.creator_lp_token, self.token_0_vault, self.token_1_vault, self.create_pool_fee, self.observation_state, self.operator, self.config, self.token_program, self.associated_token_program, self.system_program, self.rent
        ]
    }
}

/// Accounts for `create_raydium_random_pool`
pub struct CreateRaydiumRandomPoolAccounts<'a> {
    /// cpmm_program
    pub cpmm_program: &'a AccountView,
    /// Which config the pool belongs to.
    pub amm_config: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
    /// pool_state
    pub pool_state: &'a AccountView,
    /// token_0_mint
    pub token_0_mint: &'a AccountView,
    /// Token_1 mint, the key must be greater than token_0 mint.
    pub token_1_mint: &'a AccountView,
    /// lp_mint
    pub lp_mint: &'a AccountView,
    /// vault_authority
    pub vault_authority: &'a AccountView,
    /// bonding_curve
    pub bonding_curve: &'a AccountView,
    /// bonding_curve_vault
    pub bonding_curve_vault: &'a AccountView,
    /// bonding_curve_wsol_vault
    pub bonding_curve_wsol_vault: &'a AccountView,
    /// creator_lp_token
    pub creator_lp_token: &'a AccountView,
    /// token_0_vault
    pub token_0_vault: &'a AccountView,
    /// token_1_vault
    pub token_1_vault: &'a AccountView,
    /// create pool fee account
    pub create_pool_fee: &'a AccountView,
    /// observation_state
    pub observation_state: &'a AccountView,
    /// operator
    pub operator: &'a AccountView,
    /// config
    pub config: &'a AccountView,
    /// Program to create mint account and mint tokens
    pub token_program: &'a AccountView,
    /// Program to create an ATA for receiving position NFT
    pub associated_token_program: &'a AccountView,
    /// To create a new program account
    pub system_program: &'a AccountView,
    /// Sysvar for program account
    pub rent: &'a AccountView,
}

impl<'a> CreateRaydiumRandomPoolAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 22] {
        [
            InstructionAccount::readonly(self.cpmm_program.address()),
            InstructionAccount::readonly(self.amm_config.address()),
            InstructionAccount::readonly(self.authority.address()),
            InstructionAccount::writable_signer(self.pool_state.address()),
            InstructionAccount::writable(self.token_0_mint.address()),
            InstructionAccount::readonly(self.token_1_mint.address()),
            InstructionAccount::writable(self.lp_mint.address()),
            InstructionAccount::writable(self.vault_authority.address()),
            InstructionAccount::writable(self.bonding_curve.address()),
            InstructionAccount::writable(self.bonding_curve_vault.address()),
            InstructionAccount::writable(self.bonding_curve_wsol_vault.address()),
            InstructionAccount::writable(self.creator_lp_token.address()),
            InstructionAccount::writable(self.token_0_vault.address()),
            InstructionAccount::writable(self.token_1_vault.address()),
            InstructionAccount::writable(self.create_pool_fee.address()),
            InstructionAccount::writable(self.observation_state.address()),
            InstructionAccount::writable_signer(self.operator.address()),
            InstructionAccount::readonly(self.config.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.rent.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 22] {
        [
            self.cpmm_program, self.amm_config, self.authority, self.pool_state, self.token_0_mint, self.token_1_mint, self.lp_mint, self.vault_authority, self.bonding_curve, self.bonding_curve_vault, self.bonding_curve_wsol_vault, self.creator_lp_token, self.token_0_vault, self.token_1_vault, self.create_pool_fee, self.observation_state, self.operator, self.config, self.token_program, self.associated_token_program, self.system_program, self.rent
        ]
    }
}

/// Accounts for `create_token`
pub struct CreateTokenAccounts<'a> {
    /// config
    pub config: &'a AccountView,
    /// metadata
    pub metadata: &'a AccountView,
    /// mint
    pub mint: &'a AccountView,
    /// payer
    pub payer: &'a AccountView,
    /// rent
    pub rent: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
    /// token_program
    pub token_program: &'a AccountView,
    /// token_metadata_program
    pub token_metadata_program: &'a AccountView,
}

impl<'a> CreateTokenAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 8] {
        [
            InstructionAccount::readonly(self.config.address()),
            InstructionAccount::writable(self.metadata.address()),
            InstructionAccount::writable(self.mint.address()),
            InstructionAccount::writable_signer(self.payer.address()),
            InstructionAccount::readonly(self.rent.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.token_metadata_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 8] {
        [
            self.config, self.metadata, self.mint, self.payer, self.rent, self.system_program, self.token_program, self.token_metadata_program
        ]
    }
}

/// Accounts for `create_token_fallback`
pub struct CreateTokenFallbackAccounts<'a> {
    /// config
    pub config: &'a AccountView,
    /// metadata
    pub metadata: &'a AccountView,
    /// mint
    pub mint: &'a AccountView,
    /// payer
    pub payer: &'a AccountView,
    /// rent
    pub rent: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
    /// token_program
    pub token_program: &'a AccountView,
    /// token_metadata_program
    pub token_metadata_program: &'a AccountView,
}

impl<'a> CreateTokenFallbackAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 8] {
        [
            InstructionAccount::readonly(self.config.address()),
            InstructionAccount::writable(self.metadata.address()),
            InstructionAccount::writable(self.mint.address()),
            InstructionAccount::writable_signer(self.payer.address()),
            InstructionAccount::readonly(self.rent.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.token_metadata_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 8] {
        [
            self.config, self.metadata, self.mint, self.payer, self.rent, self.system_program, self.token_program, self.token_metadata_program
        ]
    }
}

/// Accounts for `deploy_bonding_curve`
pub struct DeployBondingCurveAccounts<'a> {
    /// mint
    pub mint: &'a AccountView,
    /// vault_authority
    pub vault_authority: &'a AccountView,
    /// bonding_curve
    pub bonding_curve: &'a AccountView,
    /// bonding_curve_sol_vault
    pub bonding_curve_sol_vault: &'a AccountView,
    /// bonding_curve_vault
    pub bonding_curve_vault: &'a AccountView,
    /// config
    pub config: &'a AccountView,
    /// payer
    pub payer: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
    /// token_program
    pub token_program: &'a AccountView,
    /// associated_token_program
    pub associated_token_program: &'a AccountView,
}

impl<'a> DeployBondingCurveAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 10] {
        [
            InstructionAccount::writable(self.mint.address()),
            InstructionAccount::readonly(self.vault_authority.address()),
            InstructionAccount::writable(self.bonding_curve.address()),
            InstructionAccount::writable(self.bonding_curve_sol_vault.address()),
            InstructionAccount::writable(self.bonding_curve_vault.address()),
            InstructionAccount::readonly(self.config.address()),
            InstructionAccount::writable_signer(self.payer.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 10] {
        [
            self.mint, self.vault_authority, self.bonding_curve, self.bonding_curve_sol_vault, self.bonding_curve_vault, self.config, self.payer, self.system_program, self.token_program, self.associated_token_program
        ]
    }
}

/// Accounts for `deploy_bonding_curve_fallback`
pub struct DeployBondingCurveFallbackAccounts<'a> {
    /// mint
    pub mint: &'a AccountView,
    /// vault_authority
    pub vault_authority: &'a AccountView,
    /// bonding_curve
    pub bonding_curve: &'a AccountView,
    /// bonding_curve_sol_vault
    pub bonding_curve_sol_vault: &'a AccountView,
    /// bonding_curve_vault
    pub bonding_curve_vault: &'a AccountView,
    /// config
    pub config: &'a AccountView,
    /// payer
    pub payer: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
    /// token_program
    pub token_program: &'a AccountView,
    /// associated_token_program
    pub associated_token_program: &'a AccountView,
}

impl<'a> DeployBondingCurveFallbackAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 10] {
        [
            InstructionAccount::writable(self.mint.address()),
            InstructionAccount::readonly(self.vault_authority.address()),
            InstructionAccount::writable(self.bonding_curve.address()),
            InstructionAccount::writable(self.bonding_curve_sol_vault.address()),
            InstructionAccount::writable(self.bonding_curve_vault.address()),
            InstructionAccount::readonly(self.config.address()),
            InstructionAccount::writable_signer(self.payer.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 10] {
        [
            self.mint, self.vault_authority, self.bonding_curve, self.bonding_curve_sol_vault, self.bonding_curve_vault, self.config, self.payer, self.system_program, self.token_program, self.associated_token_program
        ]
    }
}

/// Accounts for `deposit_into_raydium`
pub struct DepositIntoRaydiumAccounts<'a> {
    /// config
    pub config: &'a AccountView,
    /// Which config the pool belongs to.
    pub amm_config: &'a AccountView,
    /// operator
    pub operator: &'a AccountView,
    /// operator_wsol_account
    pub operator_wsol_account: &'a AccountView,
    /// vault_authority
    pub vault_authority: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
    /// pool_state
    pub pool_state: &'a AccountView,
    /// token_0_vault
    pub token_0_vault: &'a AccountView,
    /// token_1_vault
    pub token_1_vault: &'a AccountView,
    /// bonding_curve_vault
    pub bonding_curve_vault: &'a AccountView,
    /// bonding_curve_wsol_vault
    pub bonding_curve_wsol_vault: &'a AccountView,
    /// token Program
    pub token_program: &'a AccountView,
    /// Token program 2022
    pub token_program_2022: &'a AccountView,
    /// System program
    pub system_program: &'a AccountView,
    /// Program to create an ATA for the operator to recoup any potential remaining WSOL
    pub associated_token_program: &'a AccountView,
    /// lp_mint
    pub lp_mint: &'a AccountView,
    /// cpmm_program
    pub cpmm_program: &'a AccountView,
    /// owner_lp_token
    pub owner_lp_token: &'a AccountView,
    /// bonding_curve
    pub bonding_curve: &'a AccountView,
    /// token_0_mint
    pub token_0_mint: &'a AccountView,
    /// token_1 mint, the key must greater than token_0 mint
    pub token_1_mint: &'a AccountView,
}

impl<'a> DepositIntoRaydiumAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 21] {
        [
            InstructionAccount::readonly(self.config.address()),
            InstructionAccount::readonly(self.amm_config.address()),
            InstructionAccount::writable_signer(self.operator.address()),
            InstructionAccount::writable(self.operator_wsol_account.address()),
            InstructionAccount::writable(self.vault_authority.address()),
            InstructionAccount::readonly(self.authority.address()),
            InstructionAccount::writable(self.pool_state.address()),
            InstructionAccount::writable(self.token_0_vault.address()),
            InstructionAccount::writable(self.token_1_vault.address()),
            InstructionAccount::writable(self.bonding_curve_vault.address()),
            InstructionAccount::writable(self.bonding_curve_wsol_vault.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.token_program_2022.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
            InstructionAccount::writable(self.lp_mint.address()),
            InstructionAccount::readonly(self.cpmm_program.address()),
            InstructionAccount::writable(self.owner_lp_token.address()),
            InstructionAccount::writable(self.bonding_curve.address()),
            InstructionAccount::writable(self.token_0_mint.address()),
            InstructionAccount::readonly(self.token_1_mint.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 21] {
        [
            self.config, self.amm_config, self.operator, self.operator_wsol_account, self.vault_authority, self.authority, self.pool_state, self.token_0_vault, self.token_1_vault, self.bonding_curve_vault, self.bonding_curve_wsol_vault, self.token_program, self.token_program_2022, self.system_program, self.associated_token_program, self.lp_mint, self.cpmm_program, self.owner_lp_token, self.bonding_curve, self.token_0_mint, self.token_1_mint
        ]
    }
}

/// Accounts for `graduate`
pub struct GraduateAccounts<'a> {
    /// mint
    pub mint: &'a AccountView,
    /// wsol
    pub wsol: &'a AccountView,
    /// protocol_fee_recipient
    pub protocol_fee_recipient: &'a AccountView,
    /// token_distributor
    pub token_distributor: &'a AccountView,
    /// token_distributor_token_account
    pub token_distributor_token_account: &'a AccountView,
    /// vault_authority
    pub vault_authority: &'a AccountView,
    /// bonding_curve_sol_vault
    pub bonding_curve_sol_vault: &'a AccountView,
    /// bonding_curve
    pub bonding_curve: &'a AccountView,
    /// bonding_curve_vault
    pub bonding_curve_vault: &'a AccountView,
    /// bonding_curve_wsol_account
    pub bonding_curve_wsol_account: &'a AccountView,
    /// operator
    pub operator: &'a AccountView,
    /// config
    pub config: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
    /// token_program
    pub token_program: &'a AccountView,
    /// associated_token_program
    pub associated_token_program: &'a AccountView,
}

impl<'a> GraduateAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 15] {
        [
            InstructionAccount::readonly(self.mint.address()),
            InstructionAccount::readonly(self.wsol.address()),
            InstructionAccount::writable(self.protocol_fee_recipient.address()),
            InstructionAccount::readonly(self.token_distributor.address()),
            InstructionAccount::writable(self.token_distributor_token_account.address()),
            InstructionAccount::readonly(self.vault_authority.address()),
            InstructionAccount::writable(self.bonding_curve_sol_vault.address()),
            InstructionAccount::writable(self.bonding_curve.address()),
            InstructionAccount::writable(self.bonding_curve_vault.address()),
            InstructionAccount::writable(self.bonding_curve_wsol_account.address()),
            InstructionAccount::writable_signer(self.operator.address()),
            InstructionAccount::readonly(self.config.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 15] {
        [
            self.mint, self.wsol, self.protocol_fee_recipient, self.token_distributor, self.token_distributor_token_account, self.vault_authority, self.bonding_curve_sol_vault, self.bonding_curve, self.bonding_curve_vault, self.bonding_curve_wsol_account, self.operator, self.config, self.system_program, self.token_program, self.associated_token_program
        ]
    }
}

/// Accounts for `initialize`
pub struct InitializeAccounts<'a> {
    /// config
    pub config: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
}

impl<'a> InitializeAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 3] {
        [
            InstructionAccount::writable(self.config.address()),
            InstructionAccount::writable_signer(self.authority.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 3] {
        [
            self.config, self.authority, self.system_program
        ]
    }
}

/// Accounts for `initiate_authority_transfer`
pub struct InitiateAuthorityTransferAccounts<'a> {
    /// authority
    pub authority: &'a AccountView,
    /// config
    pub config: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
}

impl<'a> InitiateAuthorityTransferAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 3] {
        [
            InstructionAccount::readonly_signer(self.authority.address()),
            InstructionAccount::writable(self.config.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 3] {
        [
            self.authority, self.config, self.system_program
        ]
    }
}

/// Accounts for `lock_raydium_liquidity`
pub struct LockRaydiumLiquidityAccounts<'a> {
    /// lock_program
    pub lock_program: &'a AccountView,
    /// vault_authority
    pub vault_authority: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
    /// fee_nft_owner
    pub fee_nft_owner: &'a AccountView,
    /// fee_nft_mint
    pub fee_nft_mint: &'a AccountView,
    /// fee_nft_account
    pub fee_nft_account: &'a AccountView,
    /// pool_state
    pub pool_state: &'a AccountView,
    /// Store the locked information of liquidity
    pub locked_liquidity: &'a AccountView,
    /// The mint of liquidity token
    pub lp_mint: &'a AccountView,
    /// liquidity owner lp token account
    pub liquidity_owner_lp: &'a AccountView,
    /// locked_lp_vault
    pub locked_lp_vault: &'a AccountView,
    /// The address that holds pool tokens for token_0
    pub token_0_vault: &'a AccountView,
    /// The address that holds pool tokens for token_1
    pub token_1_vault: &'a AccountView,
    /// operator
    pub operator: &'a AccountView,
    /// config
    pub config: &'a AccountView,
    /// bonding_curve
    pub bonding_curve: &'a AccountView,
    /// To store metaplex metadata
    pub metadata_account: &'a AccountView,
    /// Sysvar for token mint and ATA creation
    pub rent: &'a AccountView,
    /// Program to create the new account
    pub system_program: &'a AccountView,
    /// Program to create/transfer mint/token account
    pub token_program: &'a AccountView,
    /// Program to create an ATA for receiving fee NFT
    pub associated_token_program: &'a AccountView,
    /// Program to create NFT metadata accunt
    pub metadata_program: &'a AccountView,
}

impl<'a> LockRaydiumLiquidityAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 22] {
        [
            InstructionAccount::readonly(self.lock_program.address()),
            InstructionAccount::writable(self.vault_authority.address()),
            InstructionAccount::readonly(self.authority.address()),
            InstructionAccount::readonly(self.fee_nft_owner.address()),
            InstructionAccount::writable_signer(self.fee_nft_mint.address()),
            InstructionAccount::writable(self.fee_nft_account.address()),
            InstructionAccount::writable(self.pool_state.address()),
            InstructionAccount::writable(self.locked_liquidity.address()),
            InstructionAccount::writable(self.lp_mint.address()),
            InstructionAccount::writable(self.liquidity_owner_lp.address()),
            InstructionAccount::writable(self.locked_lp_vault.address()),
            InstructionAccount::writable(self.token_0_vault.address()),
            InstructionAccount::writable(self.token_1_vault.address()),
            InstructionAccount::writable_signer(self.operator.address()),
            InstructionAccount::readonly(self.config.address()),
            InstructionAccount::writable(self.bonding_curve.address()),
            InstructionAccount::writable(self.metadata_account.address()),
            InstructionAccount::readonly(self.rent.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
            InstructionAccount::readonly(self.metadata_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 22] {
        [
            self.lock_program, self.vault_authority, self.authority, self.fee_nft_owner, self.fee_nft_mint, self.fee_nft_account, self.pool_state, self.locked_liquidity, self.lp_mint, self.liquidity_owner_lp, self.locked_lp_vault, self.token_0_vault, self.token_1_vault, self.operator, self.config, self.bonding_curve, self.metadata_account, self.rent, self.system_program, self.token_program, self.associated_token_program, self.metadata_program
        ]
    }
}

/// Accounts for `remove_operators`
pub struct RemoveOperatorsAccounts<'a> {
    /// config
    pub config: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
}

impl<'a> RemoveOperatorsAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 3] {
        [
            InstructionAccount::writable(self.config.address()),
            InstructionAccount::writable_signer(self.authority.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 3] {
        [
            self.config, self.authority, self.system_program
        ]
    }
}

/// Accounts for `sell_token`
pub struct SellTokenAccounts<'a> {
    /// mint
    pub mint: &'a AccountView,
    /// bonding_curve
    pub bonding_curve: &'a AccountView,
    /// trading_fees_vault
    pub trading_fees_vault: &'a AccountView,
    /// bonding_curve_vault
    pub bonding_curve_vault: &'a AccountView,
    /// bonding_curve_sol_vault
    pub bonding_curve_sol_vault: &'a AccountView,
    /// seller_token_account
    pub seller_token_account: &'a AccountView,
    /// seller
    pub seller: &'a AccountView,
    /// recipient
    pub recipient: &'a AccountView,
    /// config
    pub config: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
    /// token_program
    pub token_program: &'a AccountView,
    /// associated_token_program
    pub associated_token_program: &'a AccountView,
}

impl<'a> SellTokenAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 12] {
        [
            InstructionAccount::readonly(self.mint.address()),
            InstructionAccount::writable(self.bonding_curve.address()),
            InstructionAccount::writable(self.trading_fees_vault.address()),
            InstructionAccount::writable(self.bonding_curve_vault.address()),
            InstructionAccount::writable(self.bonding_curve_sol_vault.address()),
            InstructionAccount::writable(self.seller_token_account.address()),
            InstructionAccount::writable_signer(self.seller.address()),
            InstructionAccount::writable(self.recipient.address()),
            InstructionAccount::readonly(self.config.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 12] {
        [
            self.mint, self.bonding_curve, self.trading_fees_vault, self.bonding_curve_vault, self.bonding_curve_sol_vault, self.seller_token_account, self.seller, self.recipient, self.config, self.system_program, self.token_program, self.associated_token_program
        ]
    }
}

/// Accounts for `split_post_graduation_trading_fees`
pub struct SplitPostGraduationTradingFeesAccounts<'a> {
    /// operator
    pub operator: &'a AccountView,
    /// mint
    pub mint: &'a AccountView,
    /// wsol
    pub wsol: &'a AccountView,
    /// config
    pub config: &'a AccountView,
    /// vault_authority
    pub vault_authority: &'a AccountView,
    /// bonding_curve
    pub bonding_curve: &'a AccountView,
    /// trading_fees_vault
    pub trading_fees_vault: &'a AccountView,
    /// fee_splitter_program
    pub fee_splitter_program: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
    /// token_program
    pub token_program: &'a AccountView,
    /// associated_token_program
    pub associated_token_program: &'a AccountView,
    /// fee_splitter_config
    pub fee_splitter_config: &'a AccountView,
    /// fee_splitter_creator_vault
    pub fee_splitter_creator_vault: &'a AccountView,
    /// fee_splitter_vault_authority
    pub fee_splitter_vault_authority: &'a AccountView,
    /// fee_splitter_creator_vault_authority
    pub fee_splitter_creator_vault_authority: &'a AccountView,
    /// fee_splitter_staking_mint
    pub fee_splitter_staking_mint: &'a AccountView,
    /// fee_splitter_wsol_vault
    pub fee_splitter_wsol_vault: &'a AccountView,
    /// fee_splitter_creator_vault_authority_wsol_vault
    pub fee_splitter_creator_vault_authority_wsol_vault: &'a AccountView,
    /// fee_splitter_treasury_wsol_vault
    pub fee_splitter_treasury_wsol_vault: &'a AccountView,
    /// fee_splitter_team_wsol_vault
    pub fee_splitter_team_wsol_vault: &'a AccountView,
    /// fee_splitter_reward_pool
    pub fee_splitter_reward_pool: &'a AccountView,
    /// fee_splitter_reward_pool_staking_vault
    pub fee_splitter_reward_pool_staking_vault: &'a AccountView,
    /// fee_splitter_reward_pool_reward_vault
    pub fee_splitter_reward_pool_reward_vault: &'a AccountView,
    /// fee_splitter_reward_pool_program
    pub fee_splitter_reward_pool_program: &'a AccountView,
}

impl<'a> SplitPostGraduationTradingFeesAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 24] {
        [
            InstructionAccount::writable_signer(self.operator.address()),
            InstructionAccount::readonly(self.mint.address()),
            InstructionAccount::readonly(self.wsol.address()),
            InstructionAccount::readonly(self.config.address()),
            InstructionAccount::readonly(self.vault_authority.address()),
            InstructionAccount::writable(self.bonding_curve.address()),
            InstructionAccount::writable(self.trading_fees_vault.address()),
            InstructionAccount::readonly(self.fee_splitter_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
            InstructionAccount::readonly(self.fee_splitter_config.address()),
            InstructionAccount::writable(self.fee_splitter_creator_vault.address()),
            InstructionAccount::writable(self.fee_splitter_vault_authority.address()),
            InstructionAccount::writable(self.fee_splitter_creator_vault_authority.address()),
            InstructionAccount::readonly(self.fee_splitter_staking_mint.address()),
            InstructionAccount::writable(self.fee_splitter_wsol_vault.address()),
            InstructionAccount::writable(self.fee_splitter_creator_vault_authority_wsol_vault.address()),
            InstructionAccount::writable(self.fee_splitter_treasury_wsol_vault.address()),
            InstructionAccount::writable(self.fee_splitter_team_wsol_vault.address()),
            InstructionAccount::writable(self.fee_splitter_reward_pool.address()),
            InstructionAccount::writable(self.fee_splitter_reward_pool_staking_vault.address()),
            InstructionAccount::writable(self.fee_splitter_reward_pool_reward_vault.address()),
            InstructionAccount::readonly(self.fee_splitter_reward_pool_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 24] {
        [
            self.operator, self.mint, self.wsol, self.config, self.vault_authority, self.bonding_curve, self.trading_fees_vault, self.fee_splitter_program, self.system_program, self.token_program, self.associated_token_program, self.fee_splitter_config, self.fee_splitter_creator_vault, self.fee_splitter_vault_authority, self.fee_splitter_creator_vault_authority, self.fee_splitter_staking_mint, self.fee_splitter_wsol_vault, self.fee_splitter_creator_vault_authority_wsol_vault, self.fee_splitter_treasury_wsol_vault, self.fee_splitter_team_wsol_vault, self.fee_splitter_reward_pool, self.fee_splitter_reward_pool_staking_vault, self.fee_splitter_reward_pool_reward_vault, self.fee_splitter_reward_pool_program
        ]
    }
}

/// Accounts for `split_trading_fees`
pub struct SplitTradingFeesAccounts<'a> {
    /// operator
    pub operator: &'a AccountView,
    /// mint
    pub mint: &'a AccountView,
    /// wsol
    pub wsol: &'a AccountView,
    /// config
    pub config: &'a AccountView,
    /// vault_authority
    pub vault_authority: &'a AccountView,
    /// bonding_curve
    pub bonding_curve: &'a AccountView,
    /// trading_fees_vault
    pub trading_fees_vault: &'a AccountView,
    /// fee_splitter_program
    pub fee_splitter_program: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
    /// token_program
    pub token_program: &'a AccountView,
    /// associated_token_program
    pub associated_token_program: &'a AccountView,
    /// fee_splitter_config
    pub fee_splitter_config: &'a AccountView,
    /// fee_splitter_creator_vault
    pub fee_splitter_creator_vault: &'a AccountView,
    /// fee_splitter_vault_authority
    pub fee_splitter_vault_authority: &'a AccountView,
    /// fee_splitter_creator_vault_authority
    pub fee_splitter_creator_vault_authority: &'a AccountView,
    /// fee_splitter_staking_mint
    pub fee_splitter_staking_mint: &'a AccountView,
    /// fee_splitter_wsol_vault
    pub fee_splitter_wsol_vault: &'a AccountView,
    /// fee_splitter_creator_vault_authority_wsol_vault
    pub fee_splitter_creator_vault_authority_wsol_vault: &'a AccountView,
    /// fee_splitter_treasury_wsol_vault
    pub fee_splitter_treasury_wsol_vault: &'a AccountView,
    /// fee_splitter_team_wsol_vault
    pub fee_splitter_team_wsol_vault: &'a AccountView,
    /// fee_splitter_reward_pool
    pub fee_splitter_reward_pool: &'a AccountView,
    /// fee_splitter_reward_pool_staking_vault
    pub fee_splitter_reward_pool_staking_vault: &'a AccountView,
    /// fee_splitter_reward_pool_reward_vault
    pub fee_splitter_reward_pool_reward_vault: &'a AccountView,
    /// fee_splitter_reward_pool_program
    pub fee_splitter_reward_pool_program: &'a AccountView,
}

impl<'a> SplitTradingFeesAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 24] {
        [
            InstructionAccount::writable_signer(self.operator.address()),
            InstructionAccount::readonly(self.mint.address()),
            InstructionAccount::readonly(self.wsol.address()),
            InstructionAccount::readonly(self.config.address()),
            InstructionAccount::readonly(self.vault_authority.address()),
            InstructionAccount::writable(self.bonding_curve.address()),
            InstructionAccount::writable(self.trading_fees_vault.address()),
            InstructionAccount::readonly(self.fee_splitter_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
            InstructionAccount::readonly(self.fee_splitter_config.address()),
            InstructionAccount::writable(self.fee_splitter_creator_vault.address()),
            InstructionAccount::writable(self.fee_splitter_vault_authority.address()),
            InstructionAccount::writable(self.fee_splitter_creator_vault_authority.address()),
            InstructionAccount::readonly(self.fee_splitter_staking_mint.address()),
            InstructionAccount::writable(self.fee_splitter_wsol_vault.address()),
            InstructionAccount::writable(self.fee_splitter_creator_vault_authority_wsol_vault.address()),
            InstructionAccount::writable(self.fee_splitter_treasury_wsol_vault.address()),
            InstructionAccount::writable(self.fee_splitter_team_wsol_vault.address()),
            InstructionAccount::writable(self.fee_splitter_reward_pool.address()),
            InstructionAccount::writable(self.fee_splitter_reward_pool_staking_vault.address()),
            InstructionAccount::writable(self.fee_splitter_reward_pool_reward_vault.address()),
            InstructionAccount::readonly(self.fee_splitter_reward_pool_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 24] {
        [
            self.operator, self.mint, self.wsol, self.config, self.vault_authority, self.bonding_curve, self.trading_fees_vault, self.fee_splitter_program, self.system_program, self.token_program, self.associated_token_program, self.fee_splitter_config, self.fee_splitter_creator_vault, self.fee_splitter_vault_authority, self.fee_splitter_creator_vault_authority, self.fee_splitter_staking_mint, self.fee_splitter_wsol_vault, self.fee_splitter_creator_vault_authority_wsol_vault, self.fee_splitter_treasury_wsol_vault, self.fee_splitter_team_wsol_vault, self.fee_splitter_reward_pool, self.fee_splitter_reward_pool_staking_vault, self.fee_splitter_reward_pool_reward_vault, self.fee_splitter_reward_pool_program
        ]
    }
}

/// Accounts for `swap_sol_for_tokens_on_raydium`
pub struct SwapSolForTokensOnRaydiumAccounts<'a> {
    /// config
    pub config: &'a AccountView,
    /// bonding_curve
    pub bonding_curve: &'a AccountView,
    /// Which config the pool belongs to.
    pub amm_config: &'a AccountView,
    /// operator
    pub operator: &'a AccountView,
    /// vault_authority
    pub vault_authority: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
    /// pool_state
    pub pool_state: &'a AccountView,
    /// input_vault
    pub input_vault: &'a AccountView,
    /// output_vault
    pub output_vault: &'a AccountView,
    /// bonding_curve_vault
    pub bonding_curve_vault: &'a AccountView,
    /// bonding_curve_wsol_vault
    pub bonding_curve_wsol_vault: &'a AccountView,
    /// token_0 mint, the key must smaller than token_1 mint
    pub output_token_mint: &'a AccountView,
    /// token_1 mint, the key must greater than token_0 mint
    pub input_token_mint: &'a AccountView,
    /// input_token_mint and output_token_mint have the same token program
    pub token_program: &'a AccountView,
    /// cp_swap_program
    pub cp_swap_program: &'a AccountView,
    /// observation_state
    pub observation_state: &'a AccountView,
}

impl<'a> SwapSolForTokensOnRaydiumAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 16] {
        [
            InstructionAccount::readonly(self.config.address()),
            InstructionAccount::writable(self.bonding_curve.address()),
            InstructionAccount::readonly(self.amm_config.address()),
            InstructionAccount::writable_signer(self.operator.address()),
            InstructionAccount::writable(self.vault_authority.address()),
            InstructionAccount::readonly(self.authority.address()),
            InstructionAccount::writable(self.pool_state.address()),
            InstructionAccount::writable(self.input_vault.address()),
            InstructionAccount::writable(self.output_vault.address()),
            InstructionAccount::writable(self.bonding_curve_vault.address()),
            InstructionAccount::writable(self.bonding_curve_wsol_vault.address()),
            InstructionAccount::readonly(self.output_token_mint.address()),
            InstructionAccount::readonly(self.input_token_mint.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.cp_swap_program.address()),
            InstructionAccount::writable(self.observation_state.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 16] {
        [
            self.config, self.bonding_curve, self.amm_config, self.operator, self.vault_authority, self.authority, self.pool_state, self.input_vault, self.output_vault, self.bonding_curve_vault, self.bonding_curve_wsol_vault, self.output_token_mint, self.input_token_mint, self.token_program, self.cp_swap_program, self.observation_state
        ]
    }
}

/// Accounts for `swap_tokens_for_sol_on_raydium`
pub struct SwapTokensForSolOnRaydiumAccounts<'a> {
    /// config
    pub config: &'a AccountView,
    /// bonding_curve
    pub bonding_curve: &'a AccountView,
    /// Which config the pool belongs to.
    pub amm_config: &'a AccountView,
    /// operator
    pub operator: &'a AccountView,
    /// vault_authority
    pub vault_authority: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
    /// pool_state
    pub pool_state: &'a AccountView,
    /// input_vault
    pub input_vault: &'a AccountView,
    /// output_vault
    pub output_vault: &'a AccountView,
    /// bonding_curve_vault
    pub bonding_curve_vault: &'a AccountView,
    /// bonding_curve_wsol_vault
    pub bonding_curve_wsol_vault: &'a AccountView,
    /// token_0 mint, the key must smaller than token_1 mint
    pub input_token_mint: &'a AccountView,
    /// token_1 mint, the key must greater than token_0 mint
    pub output_token_mint: &'a AccountView,
    /// input_token_mint and output_token_mint have the same token program
    pub token_program: &'a AccountView,
    /// cp_swap_program
    pub cp_swap_program: &'a AccountView,
    /// observation_state
    pub observation_state: &'a AccountView,
}

impl<'a> SwapTokensForSolOnRaydiumAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 16] {
        [
            InstructionAccount::readonly(self.config.address()),
            InstructionAccount::writable(self.bonding_curve.address()),
            InstructionAccount::readonly(self.amm_config.address()),
            InstructionAccount::writable_signer(self.operator.address()),
            InstructionAccount::writable(self.vault_authority.address()),
            InstructionAccount::readonly(self.authority.address()),
            InstructionAccount::writable(self.pool_state.address()),
            InstructionAccount::writable(self.input_vault.address()),
            InstructionAccount::writable(self.output_vault.address()),
            InstructionAccount::writable(self.bonding_curve_vault.address()),
            InstructionAccount::writable(self.bonding_curve_wsol_vault.address()),
            InstructionAccount::readonly(self.input_token_mint.address()),
            InstructionAccount::readonly(self.output_token_mint.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.cp_swap_program.address()),
            InstructionAccount::writable(self.observation_state.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 16] {
        [
            self.config, self.bonding_curve, self.amm_config, self.operator, self.vault_authority, self.authority, self.pool_state, self.input_vault, self.output_vault, self.bonding_curve_vault, self.bonding_curve_wsol_vault, self.input_token_mint, self.output_token_mint, self.token_program, self.cp_swap_program, self.observation_state
        ]
    }
}

/// Accounts for `toggle_paused`
pub struct TogglePausedAccounts<'a> {
    /// authority
    pub authority: &'a AccountView,
    /// config
    pub config: &'a AccountView,
}

impl<'a> TogglePausedAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 2] {
        [
            InstructionAccount::writable_signer(self.authority.address()),
            InstructionAccount::writable(self.config.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 2] {
        [
            self.authority, self.config
        ]
    }
}

/// Accounts for `update_config`
pub struct UpdateConfigAccounts<'a> {
    /// config
    pub config: &'a AccountView,
    /// authority
    pub authority: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
}

impl<'a> UpdateConfigAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 3] {
        [
            InstructionAccount::writable(self.config.address()),
            InstructionAccount::readonly_signer(self.authority.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 3] {
        [
            self.config, self.authority, self.system_program
        ]
    }
}


// ============================================
// CPI Functions
// ============================================
/// CPI: add_operators
#[inline(always)]
pub fn add_operators<'a>(
    accounts: &AddOperatorsAccounts<'a>, args: &AddOperatorsArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<AddOperatorsArgs>()];
    data[0..8].copy_from_slice(&ADD_OPERATORS);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const AddOperatorsArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<AddOperatorsArgs>(),
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

/// CPI: buy_token
#[inline(always)]
pub fn buy_token<'a>(
    accounts: &BuyTokenAccounts<'a>, args: &BuyTokenArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<BuyTokenArgs>()];
    data[0..8].copy_from_slice(&BUY_TOKEN);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const BuyTokenArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<BuyTokenArgs>(),
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

/// CPI: cancel_authority_transfer
#[inline(always)]
pub fn cancel_authority_transfer<'a>(
    accounts: &CancelAuthorityTransferAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&CANCEL_AUTHORITY_TRANSFER);
    
    
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

/// CPI: close_bonding_curve_vault
#[inline(always)]
pub fn close_bonding_curve_vault<'a>(
    accounts: &CloseBondingCurveVaultAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&CLOSE_BONDING_CURVE_VAULT);
    
    
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

/// CPI: collect_meteora_trading_fees
#[inline(always)]
pub fn collect_meteora_trading_fees<'a>(
    accounts: &CollectMeteoraTradingFeesAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&COLLECT_METEORA_TRADING_FEES);
    
    
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

/// CPI: collect_meteora_trading_fees_v2
#[inline(always)]
pub fn collect_meteora_trading_fees_v2<'a>(
    accounts: &CollectMeteoraTradingFeesV2Accounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&COLLECT_METEORA_TRADING_FEES_V2);
    
    
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
    invoke_signed::<19>(&instruction, &account_views, signers)
}

/// CPI: collect_trading_fees
#[inline(always)]
pub fn collect_trading_fees<'a>(
    accounts: &CollectTradingFeesAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&COLLECT_TRADING_FEES);
    
    
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
    invoke_signed::<24>(&instruction, &account_views, signers)
}

/// CPI: collect_trading_fees_v2
#[inline(always)]
pub fn collect_trading_fees_v2<'a>(
    accounts: &CollectTradingFeesV2Accounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&COLLECT_TRADING_FEES_V2);
    
    
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
    invoke_signed::<24>(&instruction, &account_views, signers)
}

/// CPI: complete_authority_transfer
#[inline(always)]
pub fn complete_authority_transfer<'a>(
    accounts: &CompleteAuthorityTransferAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&COMPLETE_AUTHORITY_TRANSFER);
    
    
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
    invoke_signed::<21>(&instruction, &account_views, signers)
}

/// CPI: create_raydium_pool
#[inline(always)]
pub fn create_raydium_pool<'a>(
    accounts: &CreateRaydiumPoolAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&CREATE_RAYDIUM_POOL);
    
    
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

/// CPI: create_raydium_random_pool
#[inline(always)]
pub fn create_raydium_random_pool<'a>(
    accounts: &CreateRaydiumRandomPoolAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&CREATE_RAYDIUM_RANDOM_POOL);
    
    
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

/// CPI: create_token
#[inline(always)]
pub fn create_token<'a>(
    accounts: &CreateTokenAccounts<'a>, args: &CreateTokenArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<CreateTokenArgs>()];
    data[0..8].copy_from_slice(&CREATE_TOKEN);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const CreateTokenArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<CreateTokenArgs>(),
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

/// CPI: create_token_fallback
#[inline(always)]
pub fn create_token_fallback<'a>(
    accounts: &CreateTokenFallbackAccounts<'a>, args: &CreateTokenFallbackArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<CreateTokenFallbackArgs>()];
    data[0..8].copy_from_slice(&CREATE_TOKEN_FALLBACK);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const CreateTokenFallbackArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<CreateTokenFallbackArgs>(),
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

/// CPI: deploy_bonding_curve
#[inline(always)]
pub fn deploy_bonding_curve<'a>(
    accounts: &DeployBondingCurveAccounts<'a>, args: &DeployBondingCurveArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<DeployBondingCurveArgs>()];
    data[0..8].copy_from_slice(&DEPLOY_BONDING_CURVE);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const DeployBondingCurveArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<DeployBondingCurveArgs>(),
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

/// CPI: deploy_bonding_curve_fallback
#[inline(always)]
pub fn deploy_bonding_curve_fallback<'a>(
    accounts: &DeployBondingCurveFallbackAccounts<'a>, args: &DeployBondingCurveFallbackArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<DeployBondingCurveFallbackArgs>()];
    data[0..8].copy_from_slice(&DEPLOY_BONDING_CURVE_FALLBACK);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const DeployBondingCurveFallbackArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<DeployBondingCurveFallbackArgs>(),
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

/// CPI: deposit_into_raydium
#[inline(always)]
pub fn deposit_into_raydium<'a>(
    accounts: &DepositIntoRaydiumAccounts<'a>, args: &DepositIntoRaydiumArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<DepositIntoRaydiumArgs>()];
    data[0..8].copy_from_slice(&DEPOSIT_INTO_RAYDIUM);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const DepositIntoRaydiumArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<DepositIntoRaydiumArgs>(),
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

/// CPI: graduate
#[inline(always)]
pub fn graduate<'a>(
    accounts: &GraduateAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&GRADUATE);
    
    
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
    invoke_signed::<3>(&instruction, &account_views, signers)
}

/// CPI: initiate_authority_transfer
#[inline(always)]
pub fn initiate_authority_transfer<'a>(
    accounts: &InitiateAuthorityTransferAccounts<'a>, args: &InitiateAuthorityTransferArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<InitiateAuthorityTransferArgs>()];
    data[0..8].copy_from_slice(&INITIATE_AUTHORITY_TRANSFER);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const InitiateAuthorityTransferArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<InitiateAuthorityTransferArgs>(),
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

/// CPI: lock_raydium_liquidity
#[inline(always)]
pub fn lock_raydium_liquidity<'a>(
    accounts: &LockRaydiumLiquidityAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&LOCK_RAYDIUM_LIQUIDITY);
    
    
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

/// CPI: remove_operators
#[inline(always)]
pub fn remove_operators<'a>(
    accounts: &RemoveOperatorsAccounts<'a>, args: &RemoveOperatorsArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<RemoveOperatorsArgs>()];
    data[0..8].copy_from_slice(&REMOVE_OPERATORS);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const RemoveOperatorsArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<RemoveOperatorsArgs>(),
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

/// CPI: sell_token
#[inline(always)]
pub fn sell_token<'a>(
    accounts: &SellTokenAccounts<'a>, args: &SellTokenArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SellTokenArgs>()];
    data[0..8].copy_from_slice(&SELL_TOKEN);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SellTokenArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SellTokenArgs>(),
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

/// CPI: split_post_graduation_trading_fees
#[inline(always)]
pub fn split_post_graduation_trading_fees<'a>(
    accounts: &SplitPostGraduationTradingFeesAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&SPLIT_POST_GRADUATION_TRADING_FEES);
    
    
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
    invoke_signed::<24>(&instruction, &account_views, signers)
}

/// CPI: split_trading_fees
#[inline(always)]
pub fn split_trading_fees<'a>(
    accounts: &SplitTradingFeesAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&SPLIT_TRADING_FEES);
    
    
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
    invoke_signed::<24>(&instruction, &account_views, signers)
}

/// CPI: swap_sol_for_tokens_on_raydium
#[inline(always)]
pub fn swap_sol_for_tokens_on_raydium<'a>(
    accounts: &SwapSolForTokensOnRaydiumAccounts<'a>, args: &SwapSolForTokensOnRaydiumArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SwapSolForTokensOnRaydiumArgs>()];
    data[0..8].copy_from_slice(&SWAP_SOL_FOR_TOKENS_ON_RAYDIUM);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SwapSolForTokensOnRaydiumArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SwapSolForTokensOnRaydiumArgs>(),
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

/// CPI: swap_tokens_for_sol_on_raydium
#[inline(always)]
pub fn swap_tokens_for_sol_on_raydium<'a>(
    accounts: &SwapTokensForSolOnRaydiumAccounts<'a>, args: &SwapTokensForSolOnRaydiumArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SwapTokensForSolOnRaydiumArgs>()];
    data[0..8].copy_from_slice(&SWAP_TOKENS_FOR_SOL_ON_RAYDIUM);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SwapTokensForSolOnRaydiumArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SwapTokensForSolOnRaydiumArgs>(),
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

/// CPI: toggle_paused
#[inline(always)]
pub fn toggle_paused<'a>(
    accounts: &TogglePausedAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&TOGGLE_PAUSED);
    
    
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

/// CPI: update_config
#[inline(always)]
pub fn update_config<'a>(
    accounts: &UpdateConfigAccounts<'a>, args: &UpdateConfigArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<UpdateConfigArgs>()];
    data[0..8].copy_from_slice(&UPDATE_CONFIG);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const UpdateConfigArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<UpdateConfigArgs>(),
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

