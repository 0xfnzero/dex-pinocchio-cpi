//! CPI module for Perena
//!
//! Program: numeraire
//! Program ID: NUMERUNsFCP3kuNmWZuXtm1AaQCPj9uw6Guv2Ekoi5P
//! Instructions: 22

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
pub const ID: Address = Address::new_from_array(five8_const::decode_32_const("NUMERUNsFCP3kuNmWZuXtm1AaQCPj9uw6Guv2Ekoi5P"));

// ============================================
// Instruction Discriminators
// ============================================
pub const ADD_LIQUIDITY: [u8; 8] = [181, 157, 89, 67, 143, 182, 52, 72];
pub const COMPOUND: [u8; 8] = [165, 208, 251, 78, 242, 160, 141, 47];
pub const CREATE_POOL: [u8; 8] = [233, 146, 209, 142, 207, 104, 64, 188];
pub const INIT_VIRTUAL_STABLE_PAIR: [u8; 8] = [228, 18, 163, 161, 101, 204, 106, 172];
pub const REMOVE_ALL_LIQUIDITY: [u8; 8] = [10, 51, 61, 35, 112, 105, 24, 85];
pub const REMOVE_LIQUIDITY: [u8; 8] = [80, 85, 209, 72, 24, 206, 177, 108];
pub const SET_FEE: [u8; 8] = [18, 154, 24, 18, 237, 214, 19, 80];
pub const SET_INV_T_MAX: [u8; 8] = [218, 209, 244, 237, 211, 236, 98, 58];
pub const SET_LP_TOKEN_METADATA: [u8; 8] = [71, 73, 56, 155, 202, 142, 100, 150];
pub const SET_NUMERAIRE_OWNER: [u8; 8] = [6, 199, 177, 104, 86, 61, 93, 253];
pub const SET_NUMERAIRE_STATUS: [u8; 8] = [10, 17, 5, 71, 204, 171, 126, 173];
pub const SET_NUMERAIRE_WHITELISTED_POOL_CREATOR: [u8; 8] = [147, 51, 31, 255, 111, 2, 189, 173];
pub const SET_OWNER: [u8; 8] = [72, 202, 120, 52, 77, 128, 96, 197];
pub const SET_RATE: [u8; 8] = [99, 58, 170, 238, 160, 120, 74, 11];
pub const SET_STATUS: [u8; 8] = [181, 184, 224, 203, 193, 29, 177, 224];
pub const SET_WHITELISTED_ADDER: [u8; 8] = [74, 38, 140, 129, 228, 73, 236, 105];
pub const SKIM: [u8; 8] = [238, 120, 221, 138, 82, 60, 100, 218];
pub const SWAP_EXACT_IN: [u8; 8] = [104, 104, 131, 86, 161, 189, 180, 216];
pub const SWAP_EXACT_IN_HINTED: [u8; 8] = [98, 239, 244, 233, 16, 236, 40, 49];
pub const SWAP_EXACT_IN_QUOTE: [u8; 8] = [68, 209, 177, 170, 185, 100, 29, 191];
pub const SWAP_EXACT_OUT: [u8; 8] = [250, 73, 101, 33, 38, 207, 75, 184];
pub const SWAP_EXACT_OUT_HINTED: [u8; 8] = [153, 208, 206, 70, 62, 234, 98, 182];

// ============================================
// Instruction Arguments
// ============================================
/// Arguments for `add_liquidity`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct AddLiquidityArgs {
    pub data: [u8; 32],
}

/// Arguments for `create_pool`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct CreatePoolArgs {
    pub data: [u8; 32],
}

/// Arguments for `init_virtual_stable_pair`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct InitVirtualStablePairArgs {
    pub data: [u8; 32],
}

/// Arguments for `remove_liquidity`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct RemoveLiquidityArgs {
    pub data: [u8; 32],
}

/// Arguments for `set_fee`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SetFeeArgs {
    pub data: [u8; 32],
}

/// Arguments for `set_inv_t_max`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SetInvTMaxArgs {
    pub data: [u8; 32],
}

/// Arguments for `set_lp_token_metadata`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SetLpTokenMetadataArgs {
    pub data: [u8; 32],
}

/// Arguments for `set_numeraire_owner`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SetNumeraireOwnerArgs {
    pub data: [u8; 32],
}

/// Arguments for `set_numeraire_status`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SetNumeraireStatusArgs {
    pub data: [u8; 32],
}

/// Arguments for `set_numeraire_whitelisted_pool_creator`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SetNumeraireWhitelistedPoolCreatorArgs {
    pub data: [u8; 32],
}

/// Arguments for `set_owner`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SetOwnerArgs {
    pub data: [u8; 32],
}

/// Arguments for `set_rate`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SetRateArgs {
    pub data: [u8; 32],
}

/// Arguments for `set_status`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SetStatusArgs {
    pub data: [u8; 32],
}

/// Arguments for `set_whitelisted_adder`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SetWhitelistedAdderArgs {
    pub data: [u8; 32],
}

/// Arguments for `swap_exact_in`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SwapExactInArgs {
    pub data: [u8; 32],
}

/// Arguments for `swap_exact_in_hinted`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SwapExactInHintedArgs {
    pub data: [u8; 32],
}

/// Arguments for `swap_exact_in_quote`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SwapExactInQuoteArgs {
    pub data: [u8; 32],
}

/// Arguments for `swap_exact_out`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SwapExactOutArgs {
    pub data: [u8; 32],
}

/// Arguments for `swap_exact_out_hinted`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SwapExactOutHintedArgs {
    pub data: [u8; 32],
}


// ============================================
// Instruction Accounts
// ============================================
/// Accounts for `add_liquidity`
pub struct AddLiquidityAccounts<'a> {
    /// pool
    pub pool: &'a AccountView,
    /// lp_mint
    pub lp_mint: &'a AccountView,
    /// payer_lp_account
    pub payer_lp_account: &'a AccountView,
    /// numeraire_config
    pub numeraire_config: &'a AccountView,
    /// payer
    pub payer: &'a AccountView,
    /// Solana ecosystem accounts
    pub token_program: &'a AccountView,
    /// token_2022_program
    pub token_2022_program: &'a AccountView,
}

impl<'a> AddLiquidityAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 7] {
        [
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.lp_mint.address()),
            InstructionAccount::writable(self.payer_lp_account.address()),
            InstructionAccount::readonly(self.numeraire_config.address()),
            InstructionAccount::readonly_signer(self.payer.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.token_2022_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 7] {
        [
            self.pool, self.lp_mint, self.payer_lp_account, self.numeraire_config, self.payer, self.token_program, self.token_2022_program
        ]
    }
}

/// Accounts for `compound`
pub struct CompoundAccounts<'a> {
    /// pool
    pub pool: &'a AccountView,
    /// lp_mint
    pub lp_mint: &'a AccountView,
    /// payer_lp_account
    pub payer_lp_account: &'a AccountView,
    /// numeraire_config
    pub numeraire_config: &'a AccountView,
    /// payer
    pub payer: &'a AccountView,
    /// Solana ecosystem accounts
    pub token_program: &'a AccountView,
    /// token_2022_program
    pub token_2022_program: &'a AccountView,
}

impl<'a> CompoundAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 7] {
        [
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.lp_mint.address()),
            InstructionAccount::writable(self.payer_lp_account.address()),
            InstructionAccount::readonly(self.numeraire_config.address()),
            InstructionAccount::readonly_signer(self.payer.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.token_2022_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 7] {
        [
            self.pool, self.lp_mint, self.payer_lp_account, self.numeraire_config, self.payer, self.token_program, self.token_2022_program
        ]
    }
}

/// Accounts for `create_pool`
pub struct CreatePoolAccounts<'a> {
    /// pool
    pub pool: &'a AccountView,
    /// lp_mint
    pub lp_mint: &'a AccountView,
    /// creator_lp_account
    pub creator_lp_account: &'a AccountView,
    /// numeraire_config
    pub numeraire_config: &'a AccountView,
    /// payer
    pub payer: &'a AccountView,
    /// Solana ecosystem accounts
    pub token_program: &'a AccountView,
    /// token_program_2022
    pub token_program_2022: &'a AccountView,
    /// associated_token_program
    pub associated_token_program: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
}

impl<'a> CreatePoolAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 9] {
        [
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.lp_mint.address()),
            InstructionAccount::writable(self.creator_lp_account.address()),
            InstructionAccount::readonly(self.numeraire_config.address()),
            InstructionAccount::writable_signer(self.payer.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.token_program_2022.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 9] {
        [
            self.pool, self.lp_mint, self.creator_lp_account, self.numeraire_config, self.payer, self.token_program, self.token_program_2022, self.associated_token_program, self.system_program
        ]
    }
}

/// Accounts for `init_virtual_stable_pair`
pub struct InitVirtualStablePairAccounts<'a> {
    /// x_mint
    pub x_mint: &'a AccountView,
    /// pair
    pub pair: &'a AccountView,
    /// pair_authority
    pub pair_authority: &'a AccountView,
    /// x_vault
    pub x_vault: &'a AccountView,
    /// x_adder
    pub x_adder: &'a AccountView,
    /// numeraire_config
    pub numeraire_config: &'a AccountView,
    /// payer
    pub payer: &'a AccountView,
    /// Solana ecosystem accounts
    pub token_program: &'a AccountView,
    /// associated_token_program
    pub associated_token_program: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
}

impl<'a> InitVirtualStablePairAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 10] {
        [
            InstructionAccount::readonly(self.x_mint.address()),
            InstructionAccount::writable(self.pair.address()),
            InstructionAccount::readonly(self.pair_authority.address()),
            InstructionAccount::writable(self.x_vault.address()),
            InstructionAccount::writable(self.x_adder.address()),
            InstructionAccount::readonly(self.numeraire_config.address()),
            InstructionAccount::writable_signer(self.payer.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 10] {
        [
            self.x_mint, self.pair, self.pair_authority, self.x_vault, self.x_adder, self.numeraire_config, self.payer, self.token_program, self.associated_token_program, self.system_program
        ]
    }
}

/// Accounts for `remove_all_liquidity`
pub struct RemoveAllLiquidityAccounts<'a> {
    /// pool
    pub pool: &'a AccountView,
    /// lp_mint
    pub lp_mint: &'a AccountView,
    /// payer_lp_account
    pub payer_lp_account: &'a AccountView,
    /// numeraire_config
    pub numeraire_config: &'a AccountView,
    /// payer
    pub payer: &'a AccountView,
    /// Solana ecosystem accounts
    pub token_program: &'a AccountView,
    /// token_2022_program
    pub token_2022_program: &'a AccountView,
}

impl<'a> RemoveAllLiquidityAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 7] {
        [
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.lp_mint.address()),
            InstructionAccount::writable(self.payer_lp_account.address()),
            InstructionAccount::readonly(self.numeraire_config.address()),
            InstructionAccount::readonly_signer(self.payer.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.token_2022_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 7] {
        [
            self.pool, self.lp_mint, self.payer_lp_account, self.numeraire_config, self.payer, self.token_program, self.token_2022_program
        ]
    }
}

/// Accounts for `remove_liquidity`
pub struct RemoveLiquidityAccounts<'a> {
    /// pool
    pub pool: &'a AccountView,
    /// lp_mint
    pub lp_mint: &'a AccountView,
    /// payer_lp_account
    pub payer_lp_account: &'a AccountView,
    /// numeraire_config
    pub numeraire_config: &'a AccountView,
    /// payer
    pub payer: &'a AccountView,
    /// Solana ecosystem accounts
    pub token_program: &'a AccountView,
    /// token_2022_program
    pub token_2022_program: &'a AccountView,
}

impl<'a> RemoveLiquidityAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 7] {
        [
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.lp_mint.address()),
            InstructionAccount::writable(self.payer_lp_account.address()),
            InstructionAccount::readonly(self.numeraire_config.address()),
            InstructionAccount::readonly_signer(self.payer.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.token_2022_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 7] {
        [
            self.pool, self.lp_mint, self.payer_lp_account, self.numeraire_config, self.payer, self.token_program, self.token_2022_program
        ]
    }
}

/// Accounts for `set_fee`
pub struct SetFeeAccounts<'a> {
    /// pool
    pub pool: &'a AccountView,
    /// payer
    pub payer: &'a AccountView,
}

impl<'a> SetFeeAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 2] {
        [
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::readonly_signer(self.payer.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 2] {
        [
            self.pool, self.payer
        ]
    }
}

/// Accounts for `set_inv_t_max`
pub struct SetInvTMaxAccounts<'a> {
    /// pool
    pub pool: &'a AccountView,
    /// payer
    pub payer: &'a AccountView,
}

impl<'a> SetInvTMaxAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 2] {
        [
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::readonly_signer(self.payer.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 2] {
        [
            self.pool, self.payer
        ]
    }
}

/// Accounts for `set_lp_token_metadata`
pub struct SetLpTokenMetadataAccounts<'a> {
    /// pool
    pub pool: &'a AccountView,
    /// lp_mint
    pub lp_mint: &'a AccountView,
    /// metadata_account
    pub metadata_account: &'a AccountView,
    /// payer
    pub payer: &'a AccountView,
    /// Solana ecosystem accounts
    pub token_metadata_program: &'a AccountView,
    /// token_program
    pub token_program: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
    /// instructions
    pub instructions: &'a AccountView,
}

impl<'a> SetLpTokenMetadataAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 8] {
        [
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.lp_mint.address()),
            InstructionAccount::writable(self.metadata_account.address()),
            InstructionAccount::readonly_signer(self.payer.address()),
            InstructionAccount::readonly(self.token_metadata_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.instructions.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 8] {
        [
            self.pool, self.lp_mint, self.metadata_account, self.payer, self.token_metadata_program, self.token_program, self.system_program, self.instructions
        ]
    }
}

/// Accounts for `set_numeraire_owner`
pub struct SetNumeraireOwnerAccounts<'a> {
    /// numeraire_config
    pub numeraire_config: &'a AccountView,
    /// pair_mint
    pub pair_mint: &'a AccountView,
    /// payer
    pub payer: &'a AccountView,
    /// Solana ecosystem accounts
    pub system_program: &'a AccountView,
}

impl<'a> SetNumeraireOwnerAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 4] {
        [
            InstructionAccount::writable(self.numeraire_config.address()),
            InstructionAccount::readonly(self.pair_mint.address()),
            InstructionAccount::writable_signer(self.payer.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 4] {
        [
            self.numeraire_config, self.pair_mint, self.payer, self.system_program
        ]
    }
}

/// Accounts for `set_numeraire_status`
pub struct SetNumeraireStatusAccounts<'a> {
    /// numeraire_config
    pub numeraire_config: &'a AccountView,
    /// pair_mint
    pub pair_mint: &'a AccountView,
    /// payer
    pub payer: &'a AccountView,
    /// Solana ecosystem accounts
    pub system_program: &'a AccountView,
}

impl<'a> SetNumeraireStatusAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 4] {
        [
            InstructionAccount::writable(self.numeraire_config.address()),
            InstructionAccount::readonly(self.pair_mint.address()),
            InstructionAccount::writable_signer(self.payer.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 4] {
        [
            self.numeraire_config, self.pair_mint, self.payer, self.system_program
        ]
    }
}

/// Accounts for `set_numeraire_whitelisted_pool_creator`
pub struct SetNumeraireWhitelistedPoolCreatorAccounts<'a> {
    /// numeraire_config
    pub numeraire_config: &'a AccountView,
    /// pair_mint
    pub pair_mint: &'a AccountView,
    /// payer
    pub payer: &'a AccountView,
    /// Solana ecosystem accounts
    pub system_program: &'a AccountView,
}

impl<'a> SetNumeraireWhitelistedPoolCreatorAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 4] {
        [
            InstructionAccount::writable(self.numeraire_config.address()),
            InstructionAccount::readonly(self.pair_mint.address()),
            InstructionAccount::writable_signer(self.payer.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 4] {
        [
            self.numeraire_config, self.pair_mint, self.payer, self.system_program
        ]
    }
}

/// Accounts for `set_owner`
pub struct SetOwnerAccounts<'a> {
    /// pool
    pub pool: &'a AccountView,
    /// payer
    pub payer: &'a AccountView,
}

impl<'a> SetOwnerAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 2] {
        [
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::readonly_signer(self.payer.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 2] {
        [
            self.pool, self.payer
        ]
    }
}

/// Accounts for `set_rate`
pub struct SetRateAccounts<'a> {
    /// numeraire_config
    pub numeraire_config: &'a AccountView,
    /// pair_mint
    pub pair_mint: &'a AccountView,
    /// payer
    pub payer: &'a AccountView,
    /// Solana ecosystem accounts
    pub system_program: &'a AccountView,
}

impl<'a> SetRateAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 4] {
        [
            InstructionAccount::writable(self.numeraire_config.address()),
            InstructionAccount::readonly(self.pair_mint.address()),
            InstructionAccount::writable_signer(self.payer.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 4] {
        [
            self.numeraire_config, self.pair_mint, self.payer, self.system_program
        ]
    }
}

/// Accounts for `set_status`
pub struct SetStatusAccounts<'a> {
    /// pool
    pub pool: &'a AccountView,
    /// payer
    pub payer: &'a AccountView,
}

impl<'a> SetStatusAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 2] {
        [
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::readonly_signer(self.payer.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 2] {
        [
            self.pool, self.payer
        ]
    }
}

/// Accounts for `set_whitelisted_adder`
pub struct SetWhitelistedAdderAccounts<'a> {
    /// pool
    pub pool: &'a AccountView,
    /// payer
    pub payer: &'a AccountView,
}

impl<'a> SetWhitelistedAdderAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 2] {
        [
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::readonly_signer(self.payer.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 2] {
        [
            self.pool, self.payer
        ]
    }
}

/// Accounts for `skim`
pub struct SkimAccounts<'a> {
    /// pool
    pub pool: &'a AccountView,
    /// lp_mint
    pub lp_mint: &'a AccountView,
    /// payer_lp_account
    pub payer_lp_account: &'a AccountView,
    /// numeraire_config
    pub numeraire_config: &'a AccountView,
    /// payer
    pub payer: &'a AccountView,
    /// Solana ecosystem accounts
    pub token_program: &'a AccountView,
    /// token_2022_program
    pub token_2022_program: &'a AccountView,
}

impl<'a> SkimAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 7] {
        [
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.lp_mint.address()),
            InstructionAccount::writable(self.payer_lp_account.address()),
            InstructionAccount::readonly(self.numeraire_config.address()),
            InstructionAccount::readonly_signer(self.payer.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.token_2022_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 7] {
        [
            self.pool, self.lp_mint, self.payer_lp_account, self.numeraire_config, self.payer, self.token_program, self.token_2022_program
        ]
    }
}

/// Accounts for `swap_exact_in`
pub struct SwapExactInAccounts<'a> {
    /// pool
    pub pool: &'a AccountView,
    /// in_mint
    pub in_mint: &'a AccountView,
    /// out_mint
    pub out_mint: &'a AccountView,
    /// in_trader
    pub in_trader: &'a AccountView,
    /// out_trader
    pub out_trader: &'a AccountView,
    /// in_vault
    pub in_vault: &'a AccountView,
    /// out_vault
    pub out_vault: &'a AccountView,
    /// numeraire_config
    pub numeraire_config: &'a AccountView,
    /// The trader account (executing the swap, paying for all rents)
    pub payer: &'a AccountView,
    /// Solana ecosystem accounts
    pub token_program: &'a AccountView,
    /// token_2022_program
    pub token_2022_program: &'a AccountView,
}

impl<'a> SwapExactInAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 11] {
        [
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.in_mint.address()),
            InstructionAccount::writable(self.out_mint.address()),
            InstructionAccount::writable(self.in_trader.address()),
            InstructionAccount::writable(self.out_trader.address()),
            InstructionAccount::writable(self.in_vault.address()),
            InstructionAccount::writable(self.out_vault.address()),
            InstructionAccount::readonly(self.numeraire_config.address()),
            InstructionAccount::readonly_signer(self.payer.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.token_2022_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 11] {
        [
            self.pool, self.in_mint, self.out_mint, self.in_trader, self.out_trader, self.in_vault, self.out_vault, self.numeraire_config, self.payer, self.token_program, self.token_2022_program
        ]
    }
}

/// Accounts for `swap_exact_in_hinted`
pub struct SwapExactInHintedAccounts<'a> {
    /// pool
    pub pool: &'a AccountView,
    /// in_mint
    pub in_mint: &'a AccountView,
    /// out_mint
    pub out_mint: &'a AccountView,
    /// in_trader
    pub in_trader: &'a AccountView,
    /// out_trader
    pub out_trader: &'a AccountView,
    /// in_vault
    pub in_vault: &'a AccountView,
    /// out_vault
    pub out_vault: &'a AccountView,
    /// numeraire_config
    pub numeraire_config: &'a AccountView,
    /// The trader account (executing the swap, paying for all rents)
    pub payer: &'a AccountView,
    /// Solana ecosystem accounts
    pub token_program: &'a AccountView,
    /// token_2022_program
    pub token_2022_program: &'a AccountView,
}

impl<'a> SwapExactInHintedAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 11] {
        [
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.in_mint.address()),
            InstructionAccount::writable(self.out_mint.address()),
            InstructionAccount::writable(self.in_trader.address()),
            InstructionAccount::writable(self.out_trader.address()),
            InstructionAccount::writable(self.in_vault.address()),
            InstructionAccount::writable(self.out_vault.address()),
            InstructionAccount::readonly(self.numeraire_config.address()),
            InstructionAccount::readonly_signer(self.payer.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.token_2022_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 11] {
        [
            self.pool, self.in_mint, self.out_mint, self.in_trader, self.out_trader, self.in_vault, self.out_vault, self.numeraire_config, self.payer, self.token_program, self.token_2022_program
        ]
    }
}

/// Accounts for `swap_exact_in_quote`
pub struct SwapExactInQuoteAccounts<'a> {
    /// pool
    pub pool: &'a AccountView,
    /// in_mint
    pub in_mint: &'a AccountView,
    /// out_mint
    pub out_mint: &'a AccountView,
    /// in_trader
    pub in_trader: &'a AccountView,
    /// out_trader
    pub out_trader: &'a AccountView,
    /// in_vault
    pub in_vault: &'a AccountView,
    /// out_vault
    pub out_vault: &'a AccountView,
    /// numeraire_config
    pub numeraire_config: &'a AccountView,
    /// The trader account (executing the swap, paying for all rents)
    pub payer: &'a AccountView,
    /// Solana ecosystem accounts
    pub token_program: &'a AccountView,
    /// token_2022_program
    pub token_2022_program: &'a AccountView,
}

impl<'a> SwapExactInQuoteAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 11] {
        [
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.in_mint.address()),
            InstructionAccount::writable(self.out_mint.address()),
            InstructionAccount::writable(self.in_trader.address()),
            InstructionAccount::writable(self.out_trader.address()),
            InstructionAccount::writable(self.in_vault.address()),
            InstructionAccount::writable(self.out_vault.address()),
            InstructionAccount::readonly(self.numeraire_config.address()),
            InstructionAccount::readonly_signer(self.payer.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.token_2022_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 11] {
        [
            self.pool, self.in_mint, self.out_mint, self.in_trader, self.out_trader, self.in_vault, self.out_vault, self.numeraire_config, self.payer, self.token_program, self.token_2022_program
        ]
    }
}

/// Accounts for `swap_exact_out`
pub struct SwapExactOutAccounts<'a> {
    /// pool
    pub pool: &'a AccountView,
    /// in_mint
    pub in_mint: &'a AccountView,
    /// out_mint
    pub out_mint: &'a AccountView,
    /// in_trader
    pub in_trader: &'a AccountView,
    /// out_trader
    pub out_trader: &'a AccountView,
    /// in_vault
    pub in_vault: &'a AccountView,
    /// out_vault
    pub out_vault: &'a AccountView,
    /// numeraire_config
    pub numeraire_config: &'a AccountView,
    /// The trader account (executing the swap, paying for all rents)
    pub payer: &'a AccountView,
    /// Solana ecosystem accounts
    pub token_program: &'a AccountView,
    /// token_2022_program
    pub token_2022_program: &'a AccountView,
}

impl<'a> SwapExactOutAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 11] {
        [
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.in_mint.address()),
            InstructionAccount::writable(self.out_mint.address()),
            InstructionAccount::writable(self.in_trader.address()),
            InstructionAccount::writable(self.out_trader.address()),
            InstructionAccount::writable(self.in_vault.address()),
            InstructionAccount::writable(self.out_vault.address()),
            InstructionAccount::readonly(self.numeraire_config.address()),
            InstructionAccount::readonly_signer(self.payer.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.token_2022_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 11] {
        [
            self.pool, self.in_mint, self.out_mint, self.in_trader, self.out_trader, self.in_vault, self.out_vault, self.numeraire_config, self.payer, self.token_program, self.token_2022_program
        ]
    }
}

/// Accounts for `swap_exact_out_hinted`
pub struct SwapExactOutHintedAccounts<'a> {
    /// pool
    pub pool: &'a AccountView,
    /// in_mint
    pub in_mint: &'a AccountView,
    /// out_mint
    pub out_mint: &'a AccountView,
    /// in_trader
    pub in_trader: &'a AccountView,
    /// out_trader
    pub out_trader: &'a AccountView,
    /// in_vault
    pub in_vault: &'a AccountView,
    /// out_vault
    pub out_vault: &'a AccountView,
    /// numeraire_config
    pub numeraire_config: &'a AccountView,
    /// The trader account (executing the swap, paying for all rents)
    pub payer: &'a AccountView,
    /// Solana ecosystem accounts
    pub token_program: &'a AccountView,
    /// token_2022_program
    pub token_2022_program: &'a AccountView,
}

impl<'a> SwapExactOutHintedAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 11] {
        [
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.in_mint.address()),
            InstructionAccount::writable(self.out_mint.address()),
            InstructionAccount::writable(self.in_trader.address()),
            InstructionAccount::writable(self.out_trader.address()),
            InstructionAccount::writable(self.in_vault.address()),
            InstructionAccount::writable(self.out_vault.address()),
            InstructionAccount::readonly(self.numeraire_config.address()),
            InstructionAccount::readonly_signer(self.payer.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.token_2022_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 11] {
        [
            self.pool, self.in_mint, self.out_mint, self.in_trader, self.out_trader, self.in_vault, self.out_vault, self.numeraire_config, self.payer, self.token_program, self.token_2022_program
        ]
    }
}


// ============================================
// CPI Functions
// ============================================
/// CPI: add_liquidity
#[inline(always)]
pub fn add_liquidity<'a>(
    accounts: &AddLiquidityAccounts<'a>, args: &AddLiquidityArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<AddLiquidityArgs>()];
    data[0..8].copy_from_slice(&ADD_LIQUIDITY);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const AddLiquidityArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<AddLiquidityArgs>(),
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

/// CPI: compound
#[inline(always)]
pub fn compound<'a>(
    accounts: &CompoundAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&COMPOUND);
    
    
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

/// CPI: create_pool
#[inline(always)]
pub fn create_pool<'a>(
    accounts: &CreatePoolAccounts<'a>, args: &CreatePoolArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<CreatePoolArgs>()];
    data[0..8].copy_from_slice(&CREATE_POOL);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const CreatePoolArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<CreatePoolArgs>(),
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

/// CPI: init_virtual_stable_pair
#[inline(always)]
pub fn init_virtual_stable_pair<'a>(
    accounts: &InitVirtualStablePairAccounts<'a>, args: &InitVirtualStablePairArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<InitVirtualStablePairArgs>()];
    data[0..8].copy_from_slice(&INIT_VIRTUAL_STABLE_PAIR);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const InitVirtualStablePairArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<InitVirtualStablePairArgs>(),
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

/// CPI: remove_all_liquidity
#[inline(always)]
pub fn remove_all_liquidity<'a>(
    accounts: &RemoveAllLiquidityAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&REMOVE_ALL_LIQUIDITY);
    
    
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

/// CPI: remove_liquidity
#[inline(always)]
pub fn remove_liquidity<'a>(
    accounts: &RemoveLiquidityAccounts<'a>, args: &RemoveLiquidityArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<RemoveLiquidityArgs>()];
    data[0..8].copy_from_slice(&REMOVE_LIQUIDITY);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const RemoveLiquidityArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<RemoveLiquidityArgs>(),
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

/// CPI: set_fee
#[inline(always)]
pub fn set_fee<'a>(
    accounts: &SetFeeAccounts<'a>, args: &SetFeeArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SetFeeArgs>()];
    data[0..8].copy_from_slice(&SET_FEE);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SetFeeArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SetFeeArgs>(),
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

/// CPI: set_inv_t_max
#[inline(always)]
pub fn set_inv_t_max<'a>(
    accounts: &SetInvTMaxAccounts<'a>, args: &SetInvTMaxArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SetInvTMaxArgs>()];
    data[0..8].copy_from_slice(&SET_INV_T_MAX);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SetInvTMaxArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SetInvTMaxArgs>(),
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

/// CPI: set_lp_token_metadata
#[inline(always)]
pub fn set_lp_token_metadata<'a>(
    accounts: &SetLpTokenMetadataAccounts<'a>, args: &SetLpTokenMetadataArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SetLpTokenMetadataArgs>()];
    data[0..8].copy_from_slice(&SET_LP_TOKEN_METADATA);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SetLpTokenMetadataArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SetLpTokenMetadataArgs>(),
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

/// CPI: set_numeraire_owner
#[inline(always)]
pub fn set_numeraire_owner<'a>(
    accounts: &SetNumeraireOwnerAccounts<'a>, args: &SetNumeraireOwnerArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SetNumeraireOwnerArgs>()];
    data[0..8].copy_from_slice(&SET_NUMERAIRE_OWNER);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SetNumeraireOwnerArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SetNumeraireOwnerArgs>(),
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

/// CPI: set_numeraire_status
#[inline(always)]
pub fn set_numeraire_status<'a>(
    accounts: &SetNumeraireStatusAccounts<'a>, args: &SetNumeraireStatusArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SetNumeraireStatusArgs>()];
    data[0..8].copy_from_slice(&SET_NUMERAIRE_STATUS);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SetNumeraireStatusArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SetNumeraireStatusArgs>(),
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

/// CPI: set_numeraire_whitelisted_pool_creator
#[inline(always)]
pub fn set_numeraire_whitelisted_pool_creator<'a>(
    accounts: &SetNumeraireWhitelistedPoolCreatorAccounts<'a>, args: &SetNumeraireWhitelistedPoolCreatorArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SetNumeraireWhitelistedPoolCreatorArgs>()];
    data[0..8].copy_from_slice(&SET_NUMERAIRE_WHITELISTED_POOL_CREATOR);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SetNumeraireWhitelistedPoolCreatorArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SetNumeraireWhitelistedPoolCreatorArgs>(),
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

/// CPI: set_owner
#[inline(always)]
pub fn set_owner<'a>(
    accounts: &SetOwnerAccounts<'a>, args: &SetOwnerArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SetOwnerArgs>()];
    data[0..8].copy_from_slice(&SET_OWNER);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SetOwnerArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SetOwnerArgs>(),
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

/// CPI: set_rate
#[inline(always)]
pub fn set_rate<'a>(
    accounts: &SetRateAccounts<'a>, args: &SetRateArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SetRateArgs>()];
    data[0..8].copy_from_slice(&SET_RATE);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SetRateArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SetRateArgs>(),
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

/// CPI: set_status
#[inline(always)]
pub fn set_status<'a>(
    accounts: &SetStatusAccounts<'a>, args: &SetStatusArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SetStatusArgs>()];
    data[0..8].copy_from_slice(&SET_STATUS);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SetStatusArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SetStatusArgs>(),
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

/// CPI: set_whitelisted_adder
#[inline(always)]
pub fn set_whitelisted_adder<'a>(
    accounts: &SetWhitelistedAdderAccounts<'a>, args: &SetWhitelistedAdderArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SetWhitelistedAdderArgs>()];
    data[0..8].copy_from_slice(&SET_WHITELISTED_ADDER);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SetWhitelistedAdderArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SetWhitelistedAdderArgs>(),
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

/// CPI: skim
#[inline(always)]
pub fn skim<'a>(
    accounts: &SkimAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&SKIM);
    
    
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

/// CPI: swap_exact_in
#[inline(always)]
pub fn swap_exact_in<'a>(
    accounts: &SwapExactInAccounts<'a>, args: &SwapExactInArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SwapExactInArgs>()];
    data[0..8].copy_from_slice(&SWAP_EXACT_IN);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SwapExactInArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SwapExactInArgs>(),
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

/// CPI: swap_exact_in_hinted
#[inline(always)]
pub fn swap_exact_in_hinted<'a>(
    accounts: &SwapExactInHintedAccounts<'a>, args: &SwapExactInHintedArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SwapExactInHintedArgs>()];
    data[0..8].copy_from_slice(&SWAP_EXACT_IN_HINTED);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SwapExactInHintedArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SwapExactInHintedArgs>(),
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

/// CPI: swap_exact_in_quote
#[inline(always)]
pub fn swap_exact_in_quote<'a>(
    accounts: &SwapExactInQuoteAccounts<'a>, args: &SwapExactInQuoteArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SwapExactInQuoteArgs>()];
    data[0..8].copy_from_slice(&SWAP_EXACT_IN_QUOTE);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SwapExactInQuoteArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SwapExactInQuoteArgs>(),
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

/// CPI: swap_exact_out
#[inline(always)]
pub fn swap_exact_out<'a>(
    accounts: &SwapExactOutAccounts<'a>, args: &SwapExactOutArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SwapExactOutArgs>()];
    data[0..8].copy_from_slice(&SWAP_EXACT_OUT);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SwapExactOutArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SwapExactOutArgs>(),
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

/// CPI: swap_exact_out_hinted
#[inline(always)]
pub fn swap_exact_out_hinted<'a>(
    accounts: &SwapExactOutHintedAccounts<'a>, args: &SwapExactOutHintedArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SwapExactOutHintedArgs>()];
    data[0..8].copy_from_slice(&SWAP_EXACT_OUT_HINTED);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SwapExactOutHintedArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SwapExactOutHintedArgs>(),
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

