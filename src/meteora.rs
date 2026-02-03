//! CPI module for Meteora
//!
//! Program: amm
//! Program ID: Eo7WjKq67rjJQSZxS6z3YkapzY3eMj6Xy8X5EQVn5UaB
//! Instructions: 26

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
pub const ID: Address = Address::new_from_array(five8_const::decode_32_const("Eo7WjKq67rjJQSZxS6z3YkapzY3eMj6Xy8X5EQVn5UaB"));

// ============================================
// Instruction Discriminators
// ============================================
pub const INITIALIZE_PERMISSIONED_POOL: [u8; 8] = [77, 85, 178, 157, 50, 48, 212, 126];
pub const INITIALIZE_PERMISSIONLESS_POOL: [u8; 8] = [118, 173, 41, 157, 173, 72, 97, 103];
pub const INITIALIZE_PERMISSIONLESS_POOL_WITH_FEE_TIER: [u8; 8] = [6, 135, 68, 147, 229, 82, 169, 113];
pub const ENABLE_OR_DISABLE_POOL: [u8; 8] = [128, 6, 228, 131, 55, 161, 52, 169];
pub const SWAP: [u8; 8] = [248, 198, 158, 145, 225, 117, 135, 200];
pub const REMOVE_LIQUIDITY_SINGLE_SIDE: [u8; 8] = [84, 84, 177, 66, 254, 185, 10, 251];
pub const ADD_IMBALANCE_LIQUIDITY: [u8; 8] = [79, 35, 122, 84, 173, 15, 93, 191];
pub const REMOVE_BALANCE_LIQUIDITY: [u8; 8] = [133, 109, 44, 179, 56, 238, 114, 33];
pub const ADD_BALANCE_LIQUIDITY: [u8; 8] = [168, 227, 50, 62, 189, 171, 84, 176];
pub const SET_POOL_FEES: [u8; 8] = [102, 44, 158, 54, 205, 37, 126, 78];
pub const OVERRIDE_CURVE_PARAM: [u8; 8] = [98, 86, 204, 51, 94, 71, 69, 187];
pub const GET_POOL_INFO: [u8; 8] = [9, 48, 220, 101, 22, 240, 78, 200];
pub const BOOTSTRAP_LIQUIDITY: [u8; 8] = [4, 228, 215, 71, 225, 253, 119, 206];
pub const CREATE_MINT_METADATA: [u8; 8] = [13, 70, 168, 41, 250, 100, 148, 90];
pub const CREATE_LOCK_ESCROW: [u8; 8] = [54, 87, 165, 19, 69, 227, 218, 224];
pub const LOCK: [u8; 8] = [21, 19, 208, 43, 237, 62, 255, 87];
pub const CLAIM_FEE: [u8; 8] = [169, 32, 79, 137, 136, 232, 70, 137];
pub const CREATE_CONFIG: [u8; 8] = [201, 207, 243, 114, 75, 111, 47, 189];
pub const CLOSE_CONFIG: [u8; 8] = [145, 9, 72, 157, 95, 125, 61, 85];
pub const INITIALIZE_PERMISSIONLESS_CONSTANT_PRODUCT_POOL_WITH_CONFIG: [u8; 8] = [7, 166, 138, 171, 206, 171, 236, 244];
pub const INITIALIZE_PERMISSIONLESS_CONSTANT_PRODUCT_POOL_WITH_CONFIG2: [u8; 8] = [48, 149, 220, 130, 61, 11, 9, 178];
pub const INITIALIZE_CUSTOMIZABLE_PERMISSIONLESS_CONSTANT_PRODUCT_POOL: [u8; 8] = [145, 24, 172, 194, 219, 125, 3, 190];
pub const UPDATE_ACTIVATION_POINT: [u8; 8] = [150, 62, 125, 219, 171, 220, 26, 237];
pub const WITHDRAW_PROTOCOL_FEES: [u8; 8] = [11, 68, 165, 98, 18, 208, 134, 73];
pub const SET_WHITELISTED_VAULT: [u8; 8] = [12, 148, 94, 42, 55, 57, 83, 247];
pub const PARTNER_CLAIM_FEE: [u8; 8] = [57, 53, 176, 30, 123, 70, 52, 64];

// ============================================
// Instruction Arguments
// ============================================
/// Arguments for `initializePermissionedPool`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct InitializepermissionedpoolArgs {
    pub curve_type: [u8; 32],
}

/// Arguments for `initializePermissionlessPool`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct InitializepermissionlesspoolArgs {
    pub curve_type: [u8; 32],
    pub token_a_amount: u64,
    pub token_b_amount: u64,
}

/// Arguments for `initializePermissionlessPoolWithFeeTier`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct InitializepermissionlesspoolwithfeetierArgs {
    pub curve_type: [u8; 32],
    pub trade_fee_bps: u64,
    pub token_a_amount: u64,
    pub token_b_amount: u64,
}

/// Arguments for `enableOrDisablePool`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct EnableordisablepoolArgs {
    pub enable: bool,
}

/// Arguments for `swap`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SwapArgs {
    pub in_amount: u64,
    pub minimum_out_amount: u64,
}

/// Arguments for `removeLiquiditySingleSide`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct RemoveliquiditysinglesideArgs {
    pub pool_token_amount: u64,
    pub minimum_out_amount: u64,
}

/// Arguments for `addImbalanceLiquidity`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct AddimbalanceliquidityArgs {
    pub minimum_pool_token_amount: u64,
    pub token_a_amount: u64,
    pub token_b_amount: u64,
}

/// Arguments for `removeBalanceLiquidity`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct RemovebalanceliquidityArgs {
    pub pool_token_amount: u64,
    pub minimum_a_token_out: u64,
    pub minimum_b_token_out: u64,
}

/// Arguments for `addBalanceLiquidity`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct AddbalanceliquidityArgs {
    pub pool_token_amount: u64,
    pub maximum_token_a_amount: u64,
    pub maximum_token_b_amount: u64,
}

/// Arguments for `setPoolFees`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SetpoolfeesArgs {
    pub fees: [u8; 32],
    pub new_partner_fee_numerator: u64,
}

/// Arguments for `overrideCurveParam`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct OverridecurveparamArgs {
    pub curve_type: [u8; 32],
}

/// Arguments for `bootstrapLiquidity`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct BootstrapliquidityArgs {
    pub token_a_amount: u64,
    pub token_b_amount: u64,
}

/// Arguments for `lock`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct LockArgs {
    pub max_amount: u64,
}

/// Arguments for `claimFee`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct ClaimfeeArgs {
    pub max_amount: u64,
}

/// Arguments for `createConfig`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct CreateconfigArgs {
    pub config_parameters: [u8; 32],
}

/// Arguments for `initializePermissionlessConstantProductPoolWithConfig`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct InitializepermissionlessconstantproductpoolwithconfigArgs {
    pub token_a_amount: u64,
    pub token_b_amount: u64,
}

/// Arguments for `initializePermissionlessConstantProductPoolWithConfig2`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct Initializepermissionlessconstantproductpoolwithconfig2Args {
    pub token_a_amount: u64,
    pub token_b_amount: u64,
    pub activation_point: Option<u64>,
}

/// Arguments for `initializeCustomizablePermissionlessConstantProductPool`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct InitializecustomizablepermissionlessconstantproductpoolArgs {
    pub token_a_amount: u64,
    pub token_b_amount: u64,
    pub params: [u8; 32],
}

/// Arguments for `updateActivationPoint`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct UpdateactivationpointArgs {
    pub new_activation_point: u64,
}

/// Arguments for `setWhitelistedVault`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SetwhitelistedvaultArgs {
    pub whitelisted_vault: [u8; 32],
}

/// Arguments for `partnerClaimFee`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct PartnerclaimfeeArgs {
    pub max_amount_a: u64,
    pub max_amount_b: u64,
}


// ============================================
// Instruction Accounts
// ============================================
/// Accounts for `initializePermissionedPool`
pub struct InitializepermissionedpoolAccounts<'a> {
    /// Pool account (arbitrary address)
    pub pool: &'a AccountView,
    /// LP token mint of the pool
    pub lp_mint: &'a AccountView,
    /// Token A mint of the pool. Eg: USDT
    pub token_a_mint: &'a AccountView,
    /// Token B mint of the pool. Eg: USDC
    pub token_b_mint: &'a AccountView,
    /// Vault account for token A. Token A of the pool will be deposit / withdraw from this vault account.
    pub a_vault: &'a AccountView,
    /// Vault account for token B. Token B of the pool will be deposit / withdraw from this vault account.
    pub b_vault: &'a AccountView,
    /// LP token mint of vault A
    pub a_vault_lp_mint: &'a AccountView,
    /// LP token mint of vault B
    pub b_vault_lp_mint: &'a AccountView,
    /// LP token account of vault A. Used to receive/burn the vault LP upon deposit/withdraw from the vault.
    pub a_vault_lp: &'a AccountView,
    /// LP token account of vault B. Used to receive/burn vault LP upon deposit/withdraw from the vault.
    pub b_vault_lp: &'a AccountView,
    /// Admin token account for pool token A mint. Used to bootstrap the pool with initial liquidity.
    pub admin_token_a: &'a AccountView,
    /// Admin token account for pool token B mint. Used to bootstrap the pool with initial liquidity.
    pub admin_token_b: &'a AccountView,
    /// Admin pool LP token account. Used to receive LP during first deposit (initialize pool)
    pub admin_pool_lp: &'a AccountView,
    /// Protocol fee token account for token A. Used to receive trading fee.
    pub protocol_token_a_fee: &'a AccountView,
    /// Protocol fee token account for token B. Used to receive trading fee.
    pub protocol_token_b_fee: &'a AccountView,
    /// Admin account. This account will be the admin of the pool, and the payer for PDA during initialize pool.
    pub admin: &'a AccountView,
    /// feeOwner
    pub fee_owner: &'a AccountView,
    /// Rent account.
    pub rent: &'a AccountView,
    /// mintMetadata
    pub mint_metadata: &'a AccountView,
    /// metadataProgram
    pub metadata_program: &'a AccountView,
    /// Vault program. The pool will deposit/withdraw liquidity from the vault.
    pub vault_program: &'a AccountView,
    /// Token program.
    pub token_program: &'a AccountView,
    /// Associated token program.
    pub associated_token_program: &'a AccountView,
    /// System program.
    pub system_program: &'a AccountView,
}

impl<'a> InitializepermissionedpoolAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 24] {
        [
            InstructionAccount::writable_signer(self.pool.address()),
            InstructionAccount::writable(self.lp_mint.address()),
            InstructionAccount::readonly(self.token_a_mint.address()),
            InstructionAccount::readonly(self.token_b_mint.address()),
            InstructionAccount::writable(self.a_vault.address()),
            InstructionAccount::writable(self.b_vault.address()),
            InstructionAccount::writable(self.a_vault_lp_mint.address()),
            InstructionAccount::writable(self.b_vault_lp_mint.address()),
            InstructionAccount::writable(self.a_vault_lp.address()),
            InstructionAccount::writable(self.b_vault_lp.address()),
            InstructionAccount::writable(self.admin_token_a.address()),
            InstructionAccount::writable(self.admin_token_b.address()),
            InstructionAccount::writable(self.admin_pool_lp.address()),
            InstructionAccount::writable(self.protocol_token_a_fee.address()),
            InstructionAccount::writable(self.protocol_token_b_fee.address()),
            InstructionAccount::writable_signer(self.admin.address()),
            InstructionAccount::readonly(self.fee_owner.address()),
            InstructionAccount::readonly(self.rent.address()),
            InstructionAccount::writable(self.mint_metadata.address()),
            InstructionAccount::readonly(self.metadata_program.address()),
            InstructionAccount::readonly(self.vault_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 24] {
        [
            self.pool, self.lp_mint, self.token_a_mint, self.token_b_mint, self.a_vault, self.b_vault, self.a_vault_lp_mint, self.b_vault_lp_mint, self.a_vault_lp, self.b_vault_lp, self.admin_token_a, self.admin_token_b, self.admin_pool_lp, self.protocol_token_a_fee, self.protocol_token_b_fee, self.admin, self.fee_owner, self.rent, self.mint_metadata, self.metadata_program, self.vault_program, self.token_program, self.associated_token_program, self.system_program
        ]
    }
}

/// Accounts for `initializePermissionlessPool`
pub struct InitializepermissionlesspoolAccounts<'a> {
    /// Pool account (PDA address)
    pub pool: &'a AccountView,
    /// LP token mint of the pool
    pub lp_mint: &'a AccountView,
    /// Token A mint of the pool. Eg: USDT
    pub token_a_mint: &'a AccountView,
    /// Token B mint of the pool. Eg: USDC
    pub token_b_mint: &'a AccountView,
    /// Vault account for token A. Token A of the pool will be deposit / withdraw from this vault account.
    pub a_vault: &'a AccountView,
    /// Vault account for token B. Token B of the pool will be deposit / withdraw from this vault account.
    pub b_vault: &'a AccountView,
    /// Token vault account of vault A
    pub a_token_vault: &'a AccountView,
    /// Token vault account of vault B
    pub b_token_vault: &'a AccountView,
    /// LP token mint of vault A
    pub a_vault_lp_mint: &'a AccountView,
    /// LP token mint of vault B
    pub b_vault_lp_mint: &'a AccountView,
    /// LP token account of vault A. Used to receive/burn the vault LP upon deposit/withdraw from the vault.
    pub a_vault_lp: &'a AccountView,
    /// LP token account of vault B. Used to receive/burn vault LP upon deposit/withdraw from the vault.
    pub b_vault_lp: &'a AccountView,
    /// Payer token account for pool token A mint. Used to bootstrap the pool with initial liquidity.
    pub payer_token_a: &'a AccountView,
    /// Admin token account for pool token B mint. Used to bootstrap the pool with initial liquidity.
    pub payer_token_b: &'a AccountView,
    /// payerPoolLp
    pub payer_pool_lp: &'a AccountView,
    /// Protocol fee token account for token A. Used to receive trading fee.
    pub protocol_token_a_fee: &'a AccountView,
    /// Protocol fee token account for token B. Used to receive trading fee.
    pub protocol_token_b_fee: &'a AccountView,
    /// Admin account. This account will be the admin of the pool, and the payer for PDA during initialize pool.
    pub payer: &'a AccountView,
    /// feeOwner
    pub fee_owner: &'a AccountView,
    /// Rent account.
    pub rent: &'a AccountView,
    /// mintMetadata
    pub mint_metadata: &'a AccountView,
    /// metadataProgram
    pub metadata_program: &'a AccountView,
    /// Vault program. The pool will deposit/withdraw liquidity from the vault.
    pub vault_program: &'a AccountView,
    /// Token program.
    pub token_program: &'a AccountView,
    /// Associated token program.
    pub associated_token_program: &'a AccountView,
    /// System program.
    pub system_program: &'a AccountView,
}

impl<'a> InitializepermissionlesspoolAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 26] {
        [
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.lp_mint.address()),
            InstructionAccount::readonly(self.token_a_mint.address()),
            InstructionAccount::readonly(self.token_b_mint.address()),
            InstructionAccount::writable(self.a_vault.address()),
            InstructionAccount::writable(self.b_vault.address()),
            InstructionAccount::writable(self.a_token_vault.address()),
            InstructionAccount::writable(self.b_token_vault.address()),
            InstructionAccount::writable(self.a_vault_lp_mint.address()),
            InstructionAccount::writable(self.b_vault_lp_mint.address()),
            InstructionAccount::writable(self.a_vault_lp.address()),
            InstructionAccount::writable(self.b_vault_lp.address()),
            InstructionAccount::writable(self.payer_token_a.address()),
            InstructionAccount::writable(self.payer_token_b.address()),
            InstructionAccount::writable(self.payer_pool_lp.address()),
            InstructionAccount::writable(self.protocol_token_a_fee.address()),
            InstructionAccount::writable(self.protocol_token_b_fee.address()),
            InstructionAccount::writable_signer(self.payer.address()),
            InstructionAccount::readonly(self.fee_owner.address()),
            InstructionAccount::readonly(self.rent.address()),
            InstructionAccount::writable(self.mint_metadata.address()),
            InstructionAccount::readonly(self.metadata_program.address()),
            InstructionAccount::readonly(self.vault_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 26] {
        [
            self.pool, self.lp_mint, self.token_a_mint, self.token_b_mint, self.a_vault, self.b_vault, self.a_token_vault, self.b_token_vault, self.a_vault_lp_mint, self.b_vault_lp_mint, self.a_vault_lp, self.b_vault_lp, self.payer_token_a, self.payer_token_b, self.payer_pool_lp, self.protocol_token_a_fee, self.protocol_token_b_fee, self.payer, self.fee_owner, self.rent, self.mint_metadata, self.metadata_program, self.vault_program, self.token_program, self.associated_token_program, self.system_program
        ]
    }
}

/// Accounts for `initializePermissionlessPoolWithFeeTier`
pub struct InitializepermissionlesspoolwithfeetierAccounts<'a> {
    /// Pool account (PDA address)
    pub pool: &'a AccountView,
    /// LP token mint of the pool
    pub lp_mint: &'a AccountView,
    /// Token A mint of the pool. Eg: USDT
    pub token_a_mint: &'a AccountView,
    /// Token B mint of the pool. Eg: USDC
    pub token_b_mint: &'a AccountView,
    /// Vault account for token A. Token A of the pool will be deposit / withdraw from this vault account.
    pub a_vault: &'a AccountView,
    /// Vault account for token B. Token B of the pool will be deposit / withdraw from this vault account.
    pub b_vault: &'a AccountView,
    /// Token vault account of vault A
    pub a_token_vault: &'a AccountView,
    /// Token vault account of vault B
    pub b_token_vault: &'a AccountView,
    /// LP token mint of vault A
    pub a_vault_lp_mint: &'a AccountView,
    /// LP token mint of vault B
    pub b_vault_lp_mint: &'a AccountView,
    /// LP token account of vault A. Used to receive/burn the vault LP upon deposit/withdraw from the vault.
    pub a_vault_lp: &'a AccountView,
    /// LP token account of vault B. Used to receive/burn vault LP upon deposit/withdraw from the vault.
    pub b_vault_lp: &'a AccountView,
    /// Payer token account for pool token A mint. Used to bootstrap the pool with initial liquidity.
    pub payer_token_a: &'a AccountView,
    /// Admin token account for pool token B mint. Used to bootstrap the pool with initial liquidity.
    pub payer_token_b: &'a AccountView,
    /// payerPoolLp
    pub payer_pool_lp: &'a AccountView,
    /// Protocol fee token account for token A. Used to receive trading fee.
    pub protocol_token_a_fee: &'a AccountView,
    /// Protocol fee token account for token B. Used to receive trading fee.
    pub protocol_token_b_fee: &'a AccountView,
    /// Admin account. This account will be the admin of the pool, and the payer for PDA during initialize pool.
    pub payer: &'a AccountView,
    /// feeOwner
    pub fee_owner: &'a AccountView,
    /// Rent account.
    pub rent: &'a AccountView,
    /// mintMetadata
    pub mint_metadata: &'a AccountView,
    /// metadataProgram
    pub metadata_program: &'a AccountView,
    /// Vault program. The pool will deposit/withdraw liquidity from the vault.
    pub vault_program: &'a AccountView,
    /// Token program.
    pub token_program: &'a AccountView,
    /// Associated token program.
    pub associated_token_program: &'a AccountView,
    /// System program.
    pub system_program: &'a AccountView,
}

impl<'a> InitializepermissionlesspoolwithfeetierAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 26] {
        [
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.lp_mint.address()),
            InstructionAccount::readonly(self.token_a_mint.address()),
            InstructionAccount::readonly(self.token_b_mint.address()),
            InstructionAccount::writable(self.a_vault.address()),
            InstructionAccount::writable(self.b_vault.address()),
            InstructionAccount::writable(self.a_token_vault.address()),
            InstructionAccount::writable(self.b_token_vault.address()),
            InstructionAccount::writable(self.a_vault_lp_mint.address()),
            InstructionAccount::writable(self.b_vault_lp_mint.address()),
            InstructionAccount::writable(self.a_vault_lp.address()),
            InstructionAccount::writable(self.b_vault_lp.address()),
            InstructionAccount::writable(self.payer_token_a.address()),
            InstructionAccount::writable(self.payer_token_b.address()),
            InstructionAccount::writable(self.payer_pool_lp.address()),
            InstructionAccount::writable(self.protocol_token_a_fee.address()),
            InstructionAccount::writable(self.protocol_token_b_fee.address()),
            InstructionAccount::writable_signer(self.payer.address()),
            InstructionAccount::readonly(self.fee_owner.address()),
            InstructionAccount::readonly(self.rent.address()),
            InstructionAccount::writable(self.mint_metadata.address()),
            InstructionAccount::readonly(self.metadata_program.address()),
            InstructionAccount::readonly(self.vault_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 26] {
        [
            self.pool, self.lp_mint, self.token_a_mint, self.token_b_mint, self.a_vault, self.b_vault, self.a_token_vault, self.b_token_vault, self.a_vault_lp_mint, self.b_vault_lp_mint, self.a_vault_lp, self.b_vault_lp, self.payer_token_a, self.payer_token_b, self.payer_pool_lp, self.protocol_token_a_fee, self.protocol_token_b_fee, self.payer, self.fee_owner, self.rent, self.mint_metadata, self.metadata_program, self.vault_program, self.token_program, self.associated_token_program, self.system_program
        ]
    }
}

/// Accounts for `enableOrDisablePool`
pub struct EnableordisablepoolAccounts<'a> {
    /// Pool account (PDA)
    pub pool: &'a AccountView,
    /// Admin account. Must be owner of the pool.
    pub admin: &'a AccountView,
}

impl<'a> EnableordisablepoolAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 2] {
        [
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::readonly_signer(self.admin.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 2] {
        [
            self.pool, self.admin
        ]
    }
}

/// Accounts for `swap`
pub struct SwapAccounts<'a> {
    /// Pool account (PDA)
    pub pool: &'a AccountView,
    /// User token account. Token from this account will be transfer into the vault by the pool in exchange for another token of the pool.
    pub user_source_token: &'a AccountView,
    /// User token account. The exchanged token will be transfer into this account from the pool.
    pub user_destination_token: &'a AccountView,
    /// Vault account for token a. token a of the pool will be deposit / withdraw from this vault account.
    pub a_vault: &'a AccountView,
    /// Vault account for token b. token b of the pool will be deposit / withdraw from this vault account.
    pub b_vault: &'a AccountView,
    /// Token vault account of vault A
    pub a_token_vault: &'a AccountView,
    /// Token vault account of vault B
    pub b_token_vault: &'a AccountView,
    /// Lp token mint of vault a
    pub a_vault_lp_mint: &'a AccountView,
    /// Lp token mint of vault b
    pub b_vault_lp_mint: &'a AccountView,
    /// LP token account of vault A. Used to receive/burn the vault LP upon deposit/withdraw from the vault.
    pub a_vault_lp: &'a AccountView,
    /// LP token account of vault B. Used to receive/burn the vault LP upon deposit/withdraw from the vault.
    pub b_vault_lp: &'a AccountView,
    /// Protocol fee token account. Used to receive trading fee. It's mint field must matched with user_source_token mint field.
    pub protocol_token_fee: &'a AccountView,
    /// User account. Must be owner of user_source_token.
    pub user: &'a AccountView,
    /// Vault program. the pool will deposit/withdraw liquidity from the vault.
    pub vault_program: &'a AccountView,
    /// Token program.
    pub token_program: &'a AccountView,
}

impl<'a> SwapAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 15] {
        [
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.user_source_token.address()),
            InstructionAccount::writable(self.user_destination_token.address()),
            InstructionAccount::writable(self.a_vault.address()),
            InstructionAccount::writable(self.b_vault.address()),
            InstructionAccount::writable(self.a_token_vault.address()),
            InstructionAccount::writable(self.b_token_vault.address()),
            InstructionAccount::writable(self.a_vault_lp_mint.address()),
            InstructionAccount::writable(self.b_vault_lp_mint.address()),
            InstructionAccount::writable(self.a_vault_lp.address()),
            InstructionAccount::writable(self.b_vault_lp.address()),
            InstructionAccount::writable(self.protocol_token_fee.address()),
            InstructionAccount::readonly_signer(self.user.address()),
            InstructionAccount::readonly(self.vault_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 15] {
        [
            self.pool, self.user_source_token, self.user_destination_token, self.a_vault, self.b_vault, self.a_token_vault, self.b_token_vault, self.a_vault_lp_mint, self.b_vault_lp_mint, self.a_vault_lp, self.b_vault_lp, self.protocol_token_fee, self.user, self.vault_program, self.token_program
        ]
    }
}

/// Accounts for `removeLiquiditySingleSide`
pub struct RemoveliquiditysinglesideAccounts<'a> {
    /// Pool account (PDA)
    pub pool: &'a AccountView,
    /// LP token mint of the pool
    pub lp_mint: &'a AccountView,
    /// User pool lp token account. LP will be burned from this account upon success liquidity removal.
    pub user_pool_lp: &'a AccountView,
    /// LP token account of vault A. Used to receive/burn the vault LP upon deposit/withdraw from the vault.
    pub a_vault_lp: &'a AccountView,
    /// LP token account of vault B. Used to receive/burn the vault LP upon deposit/withdraw from the vault.
    pub b_vault_lp: &'a AccountView,
    /// Vault account for token A. Token A of the pool will be deposit / withdraw from this vault account.
    pub a_vault: &'a AccountView,
    /// Vault account for token B. Token B of the pool will be deposit / withdraw from this vault account.
    pub b_vault: &'a AccountView,
    /// LP token mint of vault A
    pub a_vault_lp_mint: &'a AccountView,
    /// LP token mint of vault B
    pub b_vault_lp_mint: &'a AccountView,
    /// Token vault account of vault A
    pub a_token_vault: &'a AccountView,
    /// Token vault account of vault B
    pub b_token_vault: &'a AccountView,
    /// User token account to receive token upon success liquidity removal.
    pub user_destination_token: &'a AccountView,
    /// User account. Must be owner of the user_pool_lp account.
    pub user: &'a AccountView,
    /// Vault program. The pool will deposit/withdraw liquidity from the vault.
    pub vault_program: &'a AccountView,
    /// Token program.
    pub token_program: &'a AccountView,
}

impl<'a> RemoveliquiditysinglesideAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 15] {
        [
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.lp_mint.address()),
            InstructionAccount::writable(self.user_pool_lp.address()),
            InstructionAccount::writable(self.a_vault_lp.address()),
            InstructionAccount::writable(self.b_vault_lp.address()),
            InstructionAccount::writable(self.a_vault.address()),
            InstructionAccount::writable(self.b_vault.address()),
            InstructionAccount::writable(self.a_vault_lp_mint.address()),
            InstructionAccount::writable(self.b_vault_lp_mint.address()),
            InstructionAccount::writable(self.a_token_vault.address()),
            InstructionAccount::writable(self.b_token_vault.address()),
            InstructionAccount::writable(self.user_destination_token.address()),
            InstructionAccount::readonly_signer(self.user.address()),
            InstructionAccount::readonly(self.vault_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 15] {
        [
            self.pool, self.lp_mint, self.user_pool_lp, self.a_vault_lp, self.b_vault_lp, self.a_vault, self.b_vault, self.a_vault_lp_mint, self.b_vault_lp_mint, self.a_token_vault, self.b_token_vault, self.user_destination_token, self.user, self.vault_program, self.token_program
        ]
    }
}

/// Accounts for `addImbalanceLiquidity`
pub struct AddimbalanceliquidityAccounts<'a> {
    /// Pool account (PDA)
    pub pool: &'a AccountView,
    /// LP token mint of the pool
    pub lp_mint: &'a AccountView,
    /// user pool lp token account. lp will be burned from this account upon success liquidity removal.
    pub user_pool_lp: &'a AccountView,
    /// LP token account of vault A. Used to receive/burn the vault LP upon deposit/withdraw from the vault.
    pub a_vault_lp: &'a AccountView,
    /// LP token account of vault B. Used to receive/burn the vault LP upon deposit/withdraw from the vault.
    pub b_vault_lp: &'a AccountView,
    /// Vault account for token a. token a of the pool will be deposit / withdraw from this vault account.
    pub a_vault: &'a AccountView,
    /// Vault account for token b. token b of the pool will be deposit / withdraw from this vault account.
    pub b_vault: &'a AccountView,
    /// LP token mint of vault a
    pub a_vault_lp_mint: &'a AccountView,
    /// LP token mint of vault b
    pub b_vault_lp_mint: &'a AccountView,
    /// Token vault account of vault A
    pub a_token_vault: &'a AccountView,
    /// Token vault account of vault B
    pub b_token_vault: &'a AccountView,
    /// User token A account. Token will be transfer from this account if it is add liquidity operation. Else, token will be transfer into this account.
    pub user_a_token: &'a AccountView,
    /// User token B account. Token will be transfer from this account if it is add liquidity operation. Else, token will be transfer into this account.
    pub user_b_token: &'a AccountView,
    /// User account. Must be owner of user_a_token, and user_b_token.
    pub user: &'a AccountView,
    /// Vault program. the pool will deposit/withdraw liquidity from the vault.
    pub vault_program: &'a AccountView,
    /// Token program.
    pub token_program: &'a AccountView,
}

impl<'a> AddimbalanceliquidityAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 16] {
        [
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.lp_mint.address()),
            InstructionAccount::writable(self.user_pool_lp.address()),
            InstructionAccount::writable(self.a_vault_lp.address()),
            InstructionAccount::writable(self.b_vault_lp.address()),
            InstructionAccount::writable(self.a_vault.address()),
            InstructionAccount::writable(self.b_vault.address()),
            InstructionAccount::writable(self.a_vault_lp_mint.address()),
            InstructionAccount::writable(self.b_vault_lp_mint.address()),
            InstructionAccount::writable(self.a_token_vault.address()),
            InstructionAccount::writable(self.b_token_vault.address()),
            InstructionAccount::writable(self.user_a_token.address()),
            InstructionAccount::writable(self.user_b_token.address()),
            InstructionAccount::readonly_signer(self.user.address()),
            InstructionAccount::readonly(self.vault_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 16] {
        [
            self.pool, self.lp_mint, self.user_pool_lp, self.a_vault_lp, self.b_vault_lp, self.a_vault, self.b_vault, self.a_vault_lp_mint, self.b_vault_lp_mint, self.a_token_vault, self.b_token_vault, self.user_a_token, self.user_b_token, self.user, self.vault_program, self.token_program
        ]
    }
}

/// Accounts for `removeBalanceLiquidity`
pub struct RemovebalanceliquidityAccounts<'a> {
    /// Pool account (PDA)
    pub pool: &'a AccountView,
    /// LP token mint of the pool
    pub lp_mint: &'a AccountView,
    /// user pool lp token account. lp will be burned from this account upon success liquidity removal.
    pub user_pool_lp: &'a AccountView,
    /// LP token account of vault A. Used to receive/burn the vault LP upon deposit/withdraw from the vault.
    pub a_vault_lp: &'a AccountView,
    /// LP token account of vault B. Used to receive/burn the vault LP upon deposit/withdraw from the vault.
    pub b_vault_lp: &'a AccountView,
    /// Vault account for token a. token a of the pool will be deposit / withdraw from this vault account.
    pub a_vault: &'a AccountView,
    /// Vault account for token b. token b of the pool will be deposit / withdraw from this vault account.
    pub b_vault: &'a AccountView,
    /// LP token mint of vault a
    pub a_vault_lp_mint: &'a AccountView,
    /// LP token mint of vault b
    pub b_vault_lp_mint: &'a AccountView,
    /// Token vault account of vault A
    pub a_token_vault: &'a AccountView,
    /// Token vault account of vault B
    pub b_token_vault: &'a AccountView,
    /// User token A account. Token will be transfer from this account if it is add liquidity operation. Else, token will be transfer into this account.
    pub user_a_token: &'a AccountView,
    /// User token B account. Token will be transfer from this account if it is add liquidity operation. Else, token will be transfer into this account.
    pub user_b_token: &'a AccountView,
    /// User account. Must be owner of user_a_token, and user_b_token.
    pub user: &'a AccountView,
    /// Vault program. the pool will deposit/withdraw liquidity from the vault.
    pub vault_program: &'a AccountView,
    /// Token program.
    pub token_program: &'a AccountView,
}

impl<'a> RemovebalanceliquidityAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 16] {
        [
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.lp_mint.address()),
            InstructionAccount::writable(self.user_pool_lp.address()),
            InstructionAccount::writable(self.a_vault_lp.address()),
            InstructionAccount::writable(self.b_vault_lp.address()),
            InstructionAccount::writable(self.a_vault.address()),
            InstructionAccount::writable(self.b_vault.address()),
            InstructionAccount::writable(self.a_vault_lp_mint.address()),
            InstructionAccount::writable(self.b_vault_lp_mint.address()),
            InstructionAccount::writable(self.a_token_vault.address()),
            InstructionAccount::writable(self.b_token_vault.address()),
            InstructionAccount::writable(self.user_a_token.address()),
            InstructionAccount::writable(self.user_b_token.address()),
            InstructionAccount::readonly_signer(self.user.address()),
            InstructionAccount::readonly(self.vault_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 16] {
        [
            self.pool, self.lp_mint, self.user_pool_lp, self.a_vault_lp, self.b_vault_lp, self.a_vault, self.b_vault, self.a_vault_lp_mint, self.b_vault_lp_mint, self.a_token_vault, self.b_token_vault, self.user_a_token, self.user_b_token, self.user, self.vault_program, self.token_program
        ]
    }
}

/// Accounts for `addBalanceLiquidity`
pub struct AddbalanceliquidityAccounts<'a> {
    /// Pool account (PDA)
    pub pool: &'a AccountView,
    /// LP token mint of the pool
    pub lp_mint: &'a AccountView,
    /// user pool lp token account. lp will be burned from this account upon success liquidity removal.
    pub user_pool_lp: &'a AccountView,
    /// LP token account of vault A. Used to receive/burn the vault LP upon deposit/withdraw from the vault.
    pub a_vault_lp: &'a AccountView,
    /// LP token account of vault B. Used to receive/burn the vault LP upon deposit/withdraw from the vault.
    pub b_vault_lp: &'a AccountView,
    /// Vault account for token a. token a of the pool will be deposit / withdraw from this vault account.
    pub a_vault: &'a AccountView,
    /// Vault account for token b. token b of the pool will be deposit / withdraw from this vault account.
    pub b_vault: &'a AccountView,
    /// LP token mint of vault a
    pub a_vault_lp_mint: &'a AccountView,
    /// LP token mint of vault b
    pub b_vault_lp_mint: &'a AccountView,
    /// Token vault account of vault A
    pub a_token_vault: &'a AccountView,
    /// Token vault account of vault B
    pub b_token_vault: &'a AccountView,
    /// User token A account. Token will be transfer from this account if it is add liquidity operation. Else, token will be transfer into this account.
    pub user_a_token: &'a AccountView,
    /// User token B account. Token will be transfer from this account if it is add liquidity operation. Else, token will be transfer into this account.
    pub user_b_token: &'a AccountView,
    /// User account. Must be owner of user_a_token, and user_b_token.
    pub user: &'a AccountView,
    /// Vault program. the pool will deposit/withdraw liquidity from the vault.
    pub vault_program: &'a AccountView,
    /// Token program.
    pub token_program: &'a AccountView,
}

impl<'a> AddbalanceliquidityAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 16] {
        [
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.lp_mint.address()),
            InstructionAccount::writable(self.user_pool_lp.address()),
            InstructionAccount::writable(self.a_vault_lp.address()),
            InstructionAccount::writable(self.b_vault_lp.address()),
            InstructionAccount::writable(self.a_vault.address()),
            InstructionAccount::writable(self.b_vault.address()),
            InstructionAccount::writable(self.a_vault_lp_mint.address()),
            InstructionAccount::writable(self.b_vault_lp_mint.address()),
            InstructionAccount::writable(self.a_token_vault.address()),
            InstructionAccount::writable(self.b_token_vault.address()),
            InstructionAccount::writable(self.user_a_token.address()),
            InstructionAccount::writable(self.user_b_token.address()),
            InstructionAccount::readonly_signer(self.user.address()),
            InstructionAccount::readonly(self.vault_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 16] {
        [
            self.pool, self.lp_mint, self.user_pool_lp, self.a_vault_lp, self.b_vault_lp, self.a_vault, self.b_vault, self.a_vault_lp_mint, self.b_vault_lp_mint, self.a_token_vault, self.b_token_vault, self.user_a_token, self.user_b_token, self.user, self.vault_program, self.token_program
        ]
    }
}

/// Accounts for `setPoolFees`
pub struct SetpoolfeesAccounts<'a> {
    /// Pool account (PDA)
    pub pool: &'a AccountView,
    /// Fee operator account
    pub fee_operator: &'a AccountView,
}

impl<'a> SetpoolfeesAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 2] {
        [
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::readonly_signer(self.fee_operator.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 2] {
        [
            self.pool, self.fee_operator
        ]
    }
}

/// Accounts for `overrideCurveParam`
pub struct OverridecurveparamAccounts<'a> {
    /// Pool account (PDA)
    pub pool: &'a AccountView,
    /// Admin account.
    pub admin: &'a AccountView,
}

impl<'a> OverridecurveparamAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 2] {
        [
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::readonly_signer(self.admin.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 2] {
        [
            self.pool, self.admin
        ]
    }
}

/// Accounts for `getPoolInfo`
pub struct GetpoolinfoAccounts<'a> {
    /// Pool account (PDA)
    pub pool: &'a AccountView,
    /// LP token mint of the pool
    pub lp_mint: &'a AccountView,
    /// LP token account of vault A. Used to receive/burn the vault LP upon deposit/withdraw from the vault.
    pub a_vault_lp: &'a AccountView,
    /// LP token account of vault B. Used to receive/burn the vault LP upon deposit/withdraw from the vault.
    pub b_vault_lp: &'a AccountView,
    /// Vault account for token a. token a of the pool will be deposit / withdraw from this vault account.
    pub a_vault: &'a AccountView,
    /// Vault account for token b. token b of the pool will be deposit / withdraw from this vault account.
    pub b_vault: &'a AccountView,
    /// LP token mint of vault a
    pub a_vault_lp_mint: &'a AccountView,
    /// LP token mint of vault b
    pub b_vault_lp_mint: &'a AccountView,
}

impl<'a> GetpoolinfoAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 8] {
        [
            InstructionAccount::readonly(self.pool.address()),
            InstructionAccount::readonly(self.lp_mint.address()),
            InstructionAccount::readonly(self.a_vault_lp.address()),
            InstructionAccount::readonly(self.b_vault_lp.address()),
            InstructionAccount::readonly(self.a_vault.address()),
            InstructionAccount::readonly(self.b_vault.address()),
            InstructionAccount::readonly(self.a_vault_lp_mint.address()),
            InstructionAccount::readonly(self.b_vault_lp_mint.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 8] {
        [
            self.pool, self.lp_mint, self.a_vault_lp, self.b_vault_lp, self.a_vault, self.b_vault, self.a_vault_lp_mint, self.b_vault_lp_mint
        ]
    }
}

/// Accounts for `bootstrapLiquidity`
pub struct BootstrapliquidityAccounts<'a> {
    /// Pool account (PDA)
    pub pool: &'a AccountView,
    /// LP token mint of the pool
    pub lp_mint: &'a AccountView,
    /// user pool lp token account. lp will be burned from this account upon success liquidity removal.
    pub user_pool_lp: &'a AccountView,
    /// LP token account of vault A. Used to receive/burn the vault LP upon deposit/withdraw from the vault.
    pub a_vault_lp: &'a AccountView,
    /// LP token account of vault B. Used to receive/burn the vault LP upon deposit/withdraw from the vault.
    pub b_vault_lp: &'a AccountView,
    /// Vault account for token a. token a of the pool will be deposit / withdraw from this vault account.
    pub a_vault: &'a AccountView,
    /// Vault account for token b. token b of the pool will be deposit / withdraw from this vault account.
    pub b_vault: &'a AccountView,
    /// LP token mint of vault a
    pub a_vault_lp_mint: &'a AccountView,
    /// LP token mint of vault b
    pub b_vault_lp_mint: &'a AccountView,
    /// Token vault account of vault A
    pub a_token_vault: &'a AccountView,
    /// Token vault account of vault B
    pub b_token_vault: &'a AccountView,
    /// User token A account. Token will be transfer from this account if it is add liquidity operation. Else, token will be transfer into this account.
    pub user_a_token: &'a AccountView,
    /// User token B account. Token will be transfer from this account if it is add liquidity operation. Else, token will be transfer into this account.
    pub user_b_token: &'a AccountView,
    /// User account. Must be owner of user_a_token, and user_b_token.
    pub user: &'a AccountView,
    /// Vault program. the pool will deposit/withdraw liquidity from the vault.
    pub vault_program: &'a AccountView,
    /// Token program.
    pub token_program: &'a AccountView,
}

impl<'a> BootstrapliquidityAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 16] {
        [
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.lp_mint.address()),
            InstructionAccount::writable(self.user_pool_lp.address()),
            InstructionAccount::writable(self.a_vault_lp.address()),
            InstructionAccount::writable(self.b_vault_lp.address()),
            InstructionAccount::writable(self.a_vault.address()),
            InstructionAccount::writable(self.b_vault.address()),
            InstructionAccount::writable(self.a_vault_lp_mint.address()),
            InstructionAccount::writable(self.b_vault_lp_mint.address()),
            InstructionAccount::writable(self.a_token_vault.address()),
            InstructionAccount::writable(self.b_token_vault.address()),
            InstructionAccount::writable(self.user_a_token.address()),
            InstructionAccount::writable(self.user_b_token.address()),
            InstructionAccount::readonly_signer(self.user.address()),
            InstructionAccount::readonly(self.vault_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 16] {
        [
            self.pool, self.lp_mint, self.user_pool_lp, self.a_vault_lp, self.b_vault_lp, self.a_vault, self.b_vault, self.a_vault_lp_mint, self.b_vault_lp_mint, self.a_token_vault, self.b_token_vault, self.user_a_token, self.user_b_token, self.user, self.vault_program, self.token_program
        ]
    }
}

/// Accounts for `createMintMetadata`
pub struct CreatemintmetadataAccounts<'a> {
    /// Pool account
    pub pool: &'a AccountView,
    /// LP mint account of the pool
    pub lp_mint: &'a AccountView,
    /// Vault A LP account of the pool
    pub a_vault_lp: &'a AccountView,
    /// mintMetadata
    pub mint_metadata: &'a AccountView,
    /// metadataProgram
    pub metadata_program: &'a AccountView,
    /// System program.
    pub system_program: &'a AccountView,
    /// Payer
    pub payer: &'a AccountView,
}

impl<'a> CreatemintmetadataAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 7] {
        [
            InstructionAccount::readonly(self.pool.address()),
            InstructionAccount::readonly(self.lp_mint.address()),
            InstructionAccount::readonly(self.a_vault_lp.address()),
            InstructionAccount::writable(self.mint_metadata.address()),
            InstructionAccount::readonly(self.metadata_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::writable_signer(self.payer.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 7] {
        [
            self.pool, self.lp_mint, self.a_vault_lp, self.mint_metadata, self.metadata_program, self.system_program, self.payer
        ]
    }
}

/// Accounts for `createLockEscrow`
pub struct CreatelockescrowAccounts<'a> {
    /// Pool account
    pub pool: &'a AccountView,
    /// Lock account
    pub lock_escrow: &'a AccountView,
    /// owner
    pub owner: &'a AccountView,
    /// LP token mint of the pool
    pub lp_mint: &'a AccountView,
    /// Payer account
    pub payer: &'a AccountView,
    /// System program.
    pub system_program: &'a AccountView,
}

impl<'a> CreatelockescrowAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 6] {
        [
            InstructionAccount::readonly(self.pool.address()),
            InstructionAccount::writable(self.lock_escrow.address()),
            InstructionAccount::readonly(self.owner.address()),
            InstructionAccount::readonly(self.lp_mint.address()),
            InstructionAccount::writable_signer(self.payer.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 6] {
        [
            self.pool, self.lock_escrow, self.owner, self.lp_mint, self.payer, self.system_program
        ]
    }
}

/// Accounts for `lock`
pub struct LockAccounts<'a> {
    /// Pool account
    pub pool: &'a AccountView,
    /// LP token mint of the pool
    pub lp_mint: &'a AccountView,
    /// Lock account
    pub lock_escrow: &'a AccountView,
    /// Can be anyone
    pub owner: &'a AccountView,
    /// owner lp token account
    pub source_tokens: &'a AccountView,
    /// Escrow vault
    pub escrow_vault: &'a AccountView,
    /// Token program.
    pub token_program: &'a AccountView,
    /// Vault account for token a. token a of the pool will be deposit / withdraw from this vault account.
    pub a_vault: &'a AccountView,
    /// Vault account for token b. token b of the pool will be deposit / withdraw from this vault account.
    pub b_vault: &'a AccountView,
    /// LP token account of vault A. Used to receive/burn the vault LP upon deposit/withdraw from the vault.
    pub a_vault_lp: &'a AccountView,
    /// LP token account of vault B. Used to receive/burn the vault LP upon deposit/withdraw from the vault.
    pub b_vault_lp: &'a AccountView,
    /// LP token mint of vault a
    pub a_vault_lp_mint: &'a AccountView,
    /// LP token mint of vault b
    pub b_vault_lp_mint: &'a AccountView,
}

impl<'a> LockAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 13] {
        [
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::readonly(self.lp_mint.address()),
            InstructionAccount::writable(self.lock_escrow.address()),
            InstructionAccount::writable_signer(self.owner.address()),
            InstructionAccount::writable(self.source_tokens.address()),
            InstructionAccount::writable(self.escrow_vault.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.a_vault.address()),
            InstructionAccount::readonly(self.b_vault.address()),
            InstructionAccount::readonly(self.a_vault_lp.address()),
            InstructionAccount::readonly(self.b_vault_lp.address()),
            InstructionAccount::readonly(self.a_vault_lp_mint.address()),
            InstructionAccount::readonly(self.b_vault_lp_mint.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 13] {
        [
            self.pool, self.lp_mint, self.lock_escrow, self.owner, self.source_tokens, self.escrow_vault, self.token_program, self.a_vault, self.b_vault, self.a_vault_lp, self.b_vault_lp, self.a_vault_lp_mint, self.b_vault_lp_mint
        ]
    }
}

/// Accounts for `claimFee`
pub struct ClaimfeeAccounts<'a> {
    /// Pool account
    pub pool: &'a AccountView,
    /// LP token mint of the pool
    pub lp_mint: &'a AccountView,
    /// Lock account
    pub lock_escrow: &'a AccountView,
    /// Owner of lock account
    pub owner: &'a AccountView,
    /// owner lp token account
    pub source_tokens: &'a AccountView,
    /// Escrow vault
    pub escrow_vault: &'a AccountView,
    /// Token program.
    pub token_program: &'a AccountView,
    /// Token vault account of vault A
    pub a_token_vault: &'a AccountView,
    /// Token vault account of vault B
    pub b_token_vault: &'a AccountView,
    /// Vault account for token a. token a of the pool will be deposit / withdraw from this vault account.
    pub a_vault: &'a AccountView,
    /// Vault account for token b. token b of the pool will be deposit / withdraw from this vault account.
    pub b_vault: &'a AccountView,
    /// LP token account of vault A. Used to receive/burn the vault LP upon deposit/withdraw from the vault.
    pub a_vault_lp: &'a AccountView,
    /// LP token account of vault B. Used to receive/burn the vault LP upon deposit/withdraw from the vault.
    pub b_vault_lp: &'a AccountView,
    /// LP token mint of vault a
    pub a_vault_lp_mint: &'a AccountView,
    /// LP token mint of vault b
    pub b_vault_lp_mint: &'a AccountView,
    /// User token A account. Token will be transfer from this account if it is add liquidity operation. Else, token will be transfer into this account.
    pub user_a_token: &'a AccountView,
    /// User token B account. Token will be transfer from this account if it is add liquidity operation. Else, token will be transfer into this account.
    pub user_b_token: &'a AccountView,
    /// Vault program. the pool will deposit/withdraw liquidity from the vault.
    pub vault_program: &'a AccountView,
}

impl<'a> ClaimfeeAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 18] {
        [
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.lp_mint.address()),
            InstructionAccount::writable(self.lock_escrow.address()),
            InstructionAccount::writable_signer(self.owner.address()),
            InstructionAccount::writable(self.source_tokens.address()),
            InstructionAccount::writable(self.escrow_vault.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::writable(self.a_token_vault.address()),
            InstructionAccount::writable(self.b_token_vault.address()),
            InstructionAccount::writable(self.a_vault.address()),
            InstructionAccount::writable(self.b_vault.address()),
            InstructionAccount::writable(self.a_vault_lp.address()),
            InstructionAccount::writable(self.b_vault_lp.address()),
            InstructionAccount::writable(self.a_vault_lp_mint.address()),
            InstructionAccount::writable(self.b_vault_lp_mint.address()),
            InstructionAccount::writable(self.user_a_token.address()),
            InstructionAccount::writable(self.user_b_token.address()),
            InstructionAccount::readonly(self.vault_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 18] {
        [
            self.pool, self.lp_mint, self.lock_escrow, self.owner, self.source_tokens, self.escrow_vault, self.token_program, self.a_token_vault, self.b_token_vault, self.a_vault, self.b_vault, self.a_vault_lp, self.b_vault_lp, self.a_vault_lp_mint, self.b_vault_lp_mint, self.user_a_token, self.user_b_token, self.vault_program
        ]
    }
}

/// Accounts for `createConfig`
pub struct CreateconfigAccounts<'a> {
    /// config
    pub config: &'a AccountView,
    /// admin
    pub admin: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
}

impl<'a> CreateconfigAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 3] {
        [
            InstructionAccount::writable(self.config.address()),
            InstructionAccount::writable_signer(self.admin.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 3] {
        [
            self.config, self.admin, self.system_program
        ]
    }
}

/// Accounts for `closeConfig`
pub struct CloseconfigAccounts<'a> {
    /// config
    pub config: &'a AccountView,
    /// admin
    pub admin: &'a AccountView,
    /// rentReceiver
    pub rent_receiver: &'a AccountView,
}

impl<'a> CloseconfigAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 3] {
        [
            InstructionAccount::writable(self.config.address()),
            InstructionAccount::writable_signer(self.admin.address()),
            InstructionAccount::writable(self.rent_receiver.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 3] {
        [
            self.config, self.admin, self.rent_receiver
        ]
    }
}

/// Accounts for `initializePermissionlessConstantProductPoolWithConfig`
pub struct InitializepermissionlessconstantproductpoolwithconfigAccounts<'a> {
    /// Pool account (PDA address)
    pub pool: &'a AccountView,
    /// config
    pub config: &'a AccountView,
    /// LP token mint of the pool
    pub lp_mint: &'a AccountView,
    /// Token A mint of the pool. Eg: USDT
    pub token_a_mint: &'a AccountView,
    /// Token B mint of the pool. Eg: USDC
    pub token_b_mint: &'a AccountView,
    /// Vault account for token A. Token A of the pool will be deposit / withdraw from this vault account.
    pub a_vault: &'a AccountView,
    /// Vault account for token B. Token B of the pool will be deposit / withdraw from this vault account.
    pub b_vault: &'a AccountView,
    /// Token vault account of vault A
    pub a_token_vault: &'a AccountView,
    /// Token vault account of vault B
    pub b_token_vault: &'a AccountView,
    /// LP token mint of vault A
    pub a_vault_lp_mint: &'a AccountView,
    /// LP token mint of vault B
    pub b_vault_lp_mint: &'a AccountView,
    /// LP token account of vault A. Used to receive/burn the vault LP upon deposit/withdraw from the vault.
    pub a_vault_lp: &'a AccountView,
    /// LP token account of vault B. Used to receive/burn vault LP upon deposit/withdraw from the vault.
    pub b_vault_lp: &'a AccountView,
    /// Payer token account for pool token A mint. Used to bootstrap the pool with initial liquidity.
    pub payer_token_a: &'a AccountView,
    /// Admin token account for pool token B mint. Used to bootstrap the pool with initial liquidity.
    pub payer_token_b: &'a AccountView,
    /// payerPoolLp
    pub payer_pool_lp: &'a AccountView,
    /// Protocol fee token account for token A. Used to receive trading fee.
    pub protocol_token_a_fee: &'a AccountView,
    /// Protocol fee token account for token B. Used to receive trading fee.
    pub protocol_token_b_fee: &'a AccountView,
    /// Admin account. This account will be the admin of the pool, and the payer for PDA during initialize pool.
    pub payer: &'a AccountView,
    /// Rent account.
    pub rent: &'a AccountView,
    /// mintMetadata
    pub mint_metadata: &'a AccountView,
    /// metadataProgram
    pub metadata_program: &'a AccountView,
    /// Vault program. The pool will deposit/withdraw liquidity from the vault.
    pub vault_program: &'a AccountView,
    /// Token program.
    pub token_program: &'a AccountView,
    /// Associated token program.
    pub associated_token_program: &'a AccountView,
    /// System program.
    pub system_program: &'a AccountView,
}

impl<'a> InitializepermissionlessconstantproductpoolwithconfigAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 26] {
        [
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::readonly(self.config.address()),
            InstructionAccount::writable(self.lp_mint.address()),
            InstructionAccount::readonly(self.token_a_mint.address()),
            InstructionAccount::readonly(self.token_b_mint.address()),
            InstructionAccount::writable(self.a_vault.address()),
            InstructionAccount::writable(self.b_vault.address()),
            InstructionAccount::writable(self.a_token_vault.address()),
            InstructionAccount::writable(self.b_token_vault.address()),
            InstructionAccount::writable(self.a_vault_lp_mint.address()),
            InstructionAccount::writable(self.b_vault_lp_mint.address()),
            InstructionAccount::writable(self.a_vault_lp.address()),
            InstructionAccount::writable(self.b_vault_lp.address()),
            InstructionAccount::writable(self.payer_token_a.address()),
            InstructionAccount::writable(self.payer_token_b.address()),
            InstructionAccount::writable(self.payer_pool_lp.address()),
            InstructionAccount::writable(self.protocol_token_a_fee.address()),
            InstructionAccount::writable(self.protocol_token_b_fee.address()),
            InstructionAccount::writable_signer(self.payer.address()),
            InstructionAccount::readonly(self.rent.address()),
            InstructionAccount::writable(self.mint_metadata.address()),
            InstructionAccount::readonly(self.metadata_program.address()),
            InstructionAccount::readonly(self.vault_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 26] {
        [
            self.pool, self.config, self.lp_mint, self.token_a_mint, self.token_b_mint, self.a_vault, self.b_vault, self.a_token_vault, self.b_token_vault, self.a_vault_lp_mint, self.b_vault_lp_mint, self.a_vault_lp, self.b_vault_lp, self.payer_token_a, self.payer_token_b, self.payer_pool_lp, self.protocol_token_a_fee, self.protocol_token_b_fee, self.payer, self.rent, self.mint_metadata, self.metadata_program, self.vault_program, self.token_program, self.associated_token_program, self.system_program
        ]
    }
}

/// Accounts for `initializePermissionlessConstantProductPoolWithConfig2`
pub struct Initializepermissionlessconstantproductpoolwithconfig2Accounts<'a> {
    /// Pool account (PDA address)
    pub pool: &'a AccountView,
    /// config
    pub config: &'a AccountView,
    /// LP token mint of the pool
    pub lp_mint: &'a AccountView,
    /// Token A mint of the pool. Eg: USDT
    pub token_a_mint: &'a AccountView,
    /// Token B mint of the pool. Eg: USDC
    pub token_b_mint: &'a AccountView,
    /// Vault account for token A. Token A of the pool will be deposit / withdraw from this vault account.
    pub a_vault: &'a AccountView,
    /// Vault account for token B. Token B of the pool will be deposit / withdraw from this vault account.
    pub b_vault: &'a AccountView,
    /// Token vault account of vault A
    pub a_token_vault: &'a AccountView,
    /// Token vault account of vault B
    pub b_token_vault: &'a AccountView,
    /// LP token mint of vault A
    pub a_vault_lp_mint: &'a AccountView,
    /// LP token mint of vault B
    pub b_vault_lp_mint: &'a AccountView,
    /// LP token account of vault A. Used to receive/burn the vault LP upon deposit/withdraw from the vault.
    pub a_vault_lp: &'a AccountView,
    /// LP token account of vault B. Used to receive/burn vault LP upon deposit/withdraw from the vault.
    pub b_vault_lp: &'a AccountView,
    /// Payer token account for pool token A mint. Used to bootstrap the pool with initial liquidity.
    pub payer_token_a: &'a AccountView,
    /// Admin token account for pool token B mint. Used to bootstrap the pool with initial liquidity.
    pub payer_token_b: &'a AccountView,
    /// payerPoolLp
    pub payer_pool_lp: &'a AccountView,
    /// Protocol fee token account for token A. Used to receive trading fee.
    pub protocol_token_a_fee: &'a AccountView,
    /// Protocol fee token account for token B. Used to receive trading fee.
    pub protocol_token_b_fee: &'a AccountView,
    /// Admin account. This account will be the admin of the pool, and the payer for PDA during initialize pool.
    pub payer: &'a AccountView,
    /// Rent account.
    pub rent: &'a AccountView,
    /// mintMetadata
    pub mint_metadata: &'a AccountView,
    /// metadataProgram
    pub metadata_program: &'a AccountView,
    /// Vault program. The pool will deposit/withdraw liquidity from the vault.
    pub vault_program: &'a AccountView,
    /// Token program.
    pub token_program: &'a AccountView,
    /// Associated token program.
    pub associated_token_program: &'a AccountView,
    /// System program.
    pub system_program: &'a AccountView,
}

impl<'a> Initializepermissionlessconstantproductpoolwithconfig2Accounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 26] {
        [
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::readonly(self.config.address()),
            InstructionAccount::writable(self.lp_mint.address()),
            InstructionAccount::readonly(self.token_a_mint.address()),
            InstructionAccount::readonly(self.token_b_mint.address()),
            InstructionAccount::writable(self.a_vault.address()),
            InstructionAccount::writable(self.b_vault.address()),
            InstructionAccount::writable(self.a_token_vault.address()),
            InstructionAccount::writable(self.b_token_vault.address()),
            InstructionAccount::writable(self.a_vault_lp_mint.address()),
            InstructionAccount::writable(self.b_vault_lp_mint.address()),
            InstructionAccount::writable(self.a_vault_lp.address()),
            InstructionAccount::writable(self.b_vault_lp.address()),
            InstructionAccount::writable(self.payer_token_a.address()),
            InstructionAccount::writable(self.payer_token_b.address()),
            InstructionAccount::writable(self.payer_pool_lp.address()),
            InstructionAccount::writable(self.protocol_token_a_fee.address()),
            InstructionAccount::writable(self.protocol_token_b_fee.address()),
            InstructionAccount::writable_signer(self.payer.address()),
            InstructionAccount::readonly(self.rent.address()),
            InstructionAccount::writable(self.mint_metadata.address()),
            InstructionAccount::readonly(self.metadata_program.address()),
            InstructionAccount::readonly(self.vault_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 26] {
        [
            self.pool, self.config, self.lp_mint, self.token_a_mint, self.token_b_mint, self.a_vault, self.b_vault, self.a_token_vault, self.b_token_vault, self.a_vault_lp_mint, self.b_vault_lp_mint, self.a_vault_lp, self.b_vault_lp, self.payer_token_a, self.payer_token_b, self.payer_pool_lp, self.protocol_token_a_fee, self.protocol_token_b_fee, self.payer, self.rent, self.mint_metadata, self.metadata_program, self.vault_program, self.token_program, self.associated_token_program, self.system_program
        ]
    }
}

/// Accounts for `initializeCustomizablePermissionlessConstantProductPool`
pub struct InitializecustomizablepermissionlessconstantproductpoolAccounts<'a> {
    /// Pool account (PDA address)
    pub pool: &'a AccountView,
    /// LP token mint of the pool
    pub lp_mint: &'a AccountView,
    /// Token A mint of the pool. Eg: USDT
    pub token_a_mint: &'a AccountView,
    /// Token B mint of the pool. Eg: USDC
    pub token_b_mint: &'a AccountView,
    /// Vault account for token A. Token A of the pool will be deposit / withdraw from this vault account.
    pub a_vault: &'a AccountView,
    /// Vault account for token B. Token B of the pool will be deposit / withdraw from this vault account.
    pub b_vault: &'a AccountView,
    /// Token vault account of vault A
    pub a_token_vault: &'a AccountView,
    /// Token vault account of vault B
    pub b_token_vault: &'a AccountView,
    /// LP token mint of vault A
    pub a_vault_lp_mint: &'a AccountView,
    /// LP token mint of vault B
    pub b_vault_lp_mint: &'a AccountView,
    /// LP token account of vault A. Used to receive/burn the vault LP upon deposit/withdraw from the vault.
    pub a_vault_lp: &'a AccountView,
    /// LP token account of vault B. Used to receive/burn vault LP upon deposit/withdraw from the vault.
    pub b_vault_lp: &'a AccountView,
    /// Payer token account for pool token A mint. Used to bootstrap the pool with initial liquidity.
    pub payer_token_a: &'a AccountView,
    /// Admin token account for pool token B mint. Used to bootstrap the pool with initial liquidity.
    pub payer_token_b: &'a AccountView,
    /// payerPoolLp
    pub payer_pool_lp: &'a AccountView,
    /// Protocol fee token account for token A. Used to receive trading fee.
    pub protocol_token_a_fee: &'a AccountView,
    /// Protocol fee token account for token B. Used to receive trading fee.
    pub protocol_token_b_fee: &'a AccountView,
    /// Admin account. This account will be the admin of the pool, and the payer for PDA during initialize pool.
    pub payer: &'a AccountView,
    /// Rent account.
    pub rent: &'a AccountView,
    /// mintMetadata
    pub mint_metadata: &'a AccountView,
    /// metadataProgram
    pub metadata_program: &'a AccountView,
    /// Vault program. The pool will deposit/withdraw liquidity from the vault.
    pub vault_program: &'a AccountView,
    /// Token program.
    pub token_program: &'a AccountView,
    /// Associated token program.
    pub associated_token_program: &'a AccountView,
    /// System program.
    pub system_program: &'a AccountView,
}

impl<'a> InitializecustomizablepermissionlessconstantproductpoolAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 25] {
        [
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.lp_mint.address()),
            InstructionAccount::readonly(self.token_a_mint.address()),
            InstructionAccount::readonly(self.token_b_mint.address()),
            InstructionAccount::writable(self.a_vault.address()),
            InstructionAccount::writable(self.b_vault.address()),
            InstructionAccount::writable(self.a_token_vault.address()),
            InstructionAccount::writable(self.b_token_vault.address()),
            InstructionAccount::writable(self.a_vault_lp_mint.address()),
            InstructionAccount::writable(self.b_vault_lp_mint.address()),
            InstructionAccount::writable(self.a_vault_lp.address()),
            InstructionAccount::writable(self.b_vault_lp.address()),
            InstructionAccount::writable(self.payer_token_a.address()),
            InstructionAccount::writable(self.payer_token_b.address()),
            InstructionAccount::writable(self.payer_pool_lp.address()),
            InstructionAccount::writable(self.protocol_token_a_fee.address()),
            InstructionAccount::writable(self.protocol_token_b_fee.address()),
            InstructionAccount::writable_signer(self.payer.address()),
            InstructionAccount::readonly(self.rent.address()),
            InstructionAccount::writable(self.mint_metadata.address()),
            InstructionAccount::readonly(self.metadata_program.address()),
            InstructionAccount::readonly(self.vault_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 25] {
        [
            self.pool, self.lp_mint, self.token_a_mint, self.token_b_mint, self.a_vault, self.b_vault, self.a_token_vault, self.b_token_vault, self.a_vault_lp_mint, self.b_vault_lp_mint, self.a_vault_lp, self.b_vault_lp, self.payer_token_a, self.payer_token_b, self.payer_pool_lp, self.protocol_token_a_fee, self.protocol_token_b_fee, self.payer, self.rent, self.mint_metadata, self.metadata_program, self.vault_program, self.token_program, self.associated_token_program, self.system_program
        ]
    }
}

/// Accounts for `updateActivationPoint`
pub struct UpdateactivationpointAccounts<'a> {
    /// Pool account (PDA)
    pub pool: &'a AccountView,
    /// Admin account.
    pub admin: &'a AccountView,
}

impl<'a> UpdateactivationpointAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 2] {
        [
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::readonly_signer(self.admin.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 2] {
        [
            self.pool, self.admin
        ]
    }
}

/// Accounts for `withdrawProtocolFees`
pub struct WithdrawprotocolfeesAccounts<'a> {
    /// Pool account (PDA)
    pub pool: &'a AccountView,
    /// aVaultLp
    pub a_vault_lp: &'a AccountView,
    /// protocolTokenAFee
    pub protocol_token_a_fee: &'a AccountView,
    /// protocolTokenBFee
    pub protocol_token_b_fee: &'a AccountView,
    /// treasuryTokenA
    pub treasury_token_a: &'a AccountView,
    /// treasuryTokenB
    pub treasury_token_b: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
}

impl<'a> WithdrawprotocolfeesAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 7] {
        [
            InstructionAccount::readonly(self.pool.address()),
            InstructionAccount::readonly(self.a_vault_lp.address()),
            InstructionAccount::writable(self.protocol_token_a_fee.address()),
            InstructionAccount::writable(self.protocol_token_b_fee.address()),
            InstructionAccount::writable(self.treasury_token_a.address()),
            InstructionAccount::writable(self.treasury_token_b.address()),
            InstructionAccount::readonly(self.token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 7] {
        [
            self.pool, self.a_vault_lp, self.protocol_token_a_fee, self.protocol_token_b_fee, self.treasury_token_a, self.treasury_token_b, self.token_program
        ]
    }
}

/// Accounts for `setWhitelistedVault`
pub struct SetwhitelistedvaultAccounts<'a> {
    /// pool
    pub pool: &'a AccountView,
    /// admin
    pub admin: &'a AccountView,
}

impl<'a> SetwhitelistedvaultAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 2] {
        [
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::readonly_signer(self.admin.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 2] {
        [
            self.pool, self.admin
        ]
    }
}

/// Accounts for `partnerClaimFee`
pub struct PartnerclaimfeeAccounts<'a> {
    /// Pool account (PDA)
    pub pool: &'a AccountView,
    /// aVaultLp
    pub a_vault_lp: &'a AccountView,
    /// protocolTokenAFee
    pub protocol_token_a_fee: &'a AccountView,
    /// protocolTokenBFee
    pub protocol_token_b_fee: &'a AccountView,
    /// partnerTokenA
    pub partner_token_a: &'a AccountView,
    /// partnerTokenB
    pub partner_token_b: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// partnerAuthority
    pub partner_authority: &'a AccountView,
}

impl<'a> PartnerclaimfeeAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 8] {
        [
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::readonly(self.a_vault_lp.address()),
            InstructionAccount::writable(self.protocol_token_a_fee.address()),
            InstructionAccount::writable(self.protocol_token_b_fee.address()),
            InstructionAccount::writable(self.partner_token_a.address()),
            InstructionAccount::writable(self.partner_token_b.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly_signer(self.partner_authority.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 8] {
        [
            self.pool, self.a_vault_lp, self.protocol_token_a_fee, self.protocol_token_b_fee, self.partner_token_a, self.partner_token_b, self.token_program, self.partner_authority
        ]
    }
}


// ============================================
// CPI Functions
// ============================================
/// CPI: initializePermissionedPool
#[inline(always)]
pub fn initialize_permissioned_pool<'a>(
    accounts: &InitializepermissionedpoolAccounts<'a>, args: &InitializepermissionedpoolArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<InitializepermissionedpoolArgs>()];
    data[0..8].copy_from_slice(&INITIALIZE_PERMISSIONED_POOL);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const InitializepermissionedpoolArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<InitializepermissionedpoolArgs>(),
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
    invoke_signed::<24>(&instruction, &account_views, signers)
}

/// CPI: initializePermissionlessPool
#[inline(always)]
pub fn initialize_permissionless_pool<'a>(
    accounts: &InitializepermissionlesspoolAccounts<'a>, args: &InitializepermissionlesspoolArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<InitializepermissionlesspoolArgs>()];
    data[0..8].copy_from_slice(&INITIALIZE_PERMISSIONLESS_POOL);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const InitializepermissionlesspoolArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<InitializepermissionlesspoolArgs>(),
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
    invoke_signed::<26>(&instruction, &account_views, signers)
}

/// CPI: initializePermissionlessPoolWithFeeTier
#[inline(always)]
pub fn initialize_permissionless_pool_with_fee_tier<'a>(
    accounts: &InitializepermissionlesspoolwithfeetierAccounts<'a>, args: &InitializepermissionlesspoolwithfeetierArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<InitializepermissionlesspoolwithfeetierArgs>()];
    data[0..8].copy_from_slice(&INITIALIZE_PERMISSIONLESS_POOL_WITH_FEE_TIER);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const InitializepermissionlesspoolwithfeetierArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<InitializepermissionlesspoolwithfeetierArgs>(),
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
    invoke_signed::<26>(&instruction, &account_views, signers)
}

/// CPI: enableOrDisablePool
#[inline(always)]
pub fn enable_or_disable_pool<'a>(
    accounts: &EnableordisablepoolAccounts<'a>, args: &EnableordisablepoolArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<EnableordisablepoolArgs>()];
    data[0..8].copy_from_slice(&ENABLE_OR_DISABLE_POOL);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const EnableordisablepoolArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<EnableordisablepoolArgs>(),
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
    invoke_signed::<15>(&instruction, &account_views, signers)
}

/// CPI: removeLiquiditySingleSide
#[inline(always)]
pub fn remove_liquidity_single_side<'a>(
    accounts: &RemoveliquiditysinglesideAccounts<'a>, args: &RemoveliquiditysinglesideArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<RemoveliquiditysinglesideArgs>()];
    data[0..8].copy_from_slice(&REMOVE_LIQUIDITY_SINGLE_SIDE);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const RemoveliquiditysinglesideArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<RemoveliquiditysinglesideArgs>(),
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

/// CPI: addImbalanceLiquidity
#[inline(always)]
pub fn add_imbalance_liquidity<'a>(
    accounts: &AddimbalanceliquidityAccounts<'a>, args: &AddimbalanceliquidityArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<AddimbalanceliquidityArgs>()];
    data[0..8].copy_from_slice(&ADD_IMBALANCE_LIQUIDITY);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const AddimbalanceliquidityArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<AddimbalanceliquidityArgs>(),
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

/// CPI: removeBalanceLiquidity
#[inline(always)]
pub fn remove_balance_liquidity<'a>(
    accounts: &RemovebalanceliquidityAccounts<'a>, args: &RemovebalanceliquidityArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<RemovebalanceliquidityArgs>()];
    data[0..8].copy_from_slice(&REMOVE_BALANCE_LIQUIDITY);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const RemovebalanceliquidityArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<RemovebalanceliquidityArgs>(),
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

/// CPI: addBalanceLiquidity
#[inline(always)]
pub fn add_balance_liquidity<'a>(
    accounts: &AddbalanceliquidityAccounts<'a>, args: &AddbalanceliquidityArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<AddbalanceliquidityArgs>()];
    data[0..8].copy_from_slice(&ADD_BALANCE_LIQUIDITY);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const AddbalanceliquidityArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<AddbalanceliquidityArgs>(),
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

/// CPI: setPoolFees
#[inline(always)]
pub fn set_pool_fees<'a>(
    accounts: &SetpoolfeesAccounts<'a>, args: &SetpoolfeesArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SetpoolfeesArgs>()];
    data[0..8].copy_from_slice(&SET_POOL_FEES);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SetpoolfeesArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SetpoolfeesArgs>(),
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

/// CPI: overrideCurveParam
#[inline(always)]
pub fn override_curve_param<'a>(
    accounts: &OverridecurveparamAccounts<'a>, args: &OverridecurveparamArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<OverridecurveparamArgs>()];
    data[0..8].copy_from_slice(&OVERRIDE_CURVE_PARAM);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const OverridecurveparamArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<OverridecurveparamArgs>(),
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

/// CPI: getPoolInfo
#[inline(always)]
pub fn get_pool_info<'a>(
    accounts: &GetpoolinfoAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&GET_POOL_INFO);
    
    
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

/// CPI: bootstrapLiquidity
#[inline(always)]
pub fn bootstrap_liquidity<'a>(
    accounts: &BootstrapliquidityAccounts<'a>, args: &BootstrapliquidityArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<BootstrapliquidityArgs>()];
    data[0..8].copy_from_slice(&BOOTSTRAP_LIQUIDITY);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const BootstrapliquidityArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<BootstrapliquidityArgs>(),
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

/// CPI: createMintMetadata
#[inline(always)]
pub fn create_mint_metadata<'a>(
    accounts: &CreatemintmetadataAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&CREATE_MINT_METADATA);
    
    
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

/// CPI: createLockEscrow
#[inline(always)]
pub fn create_lock_escrow<'a>(
    accounts: &CreatelockescrowAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&CREATE_LOCK_ESCROW);
    
    
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

/// CPI: lock
#[inline(always)]
pub fn lock<'a>(
    accounts: &LockAccounts<'a>, args: &LockArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<LockArgs>()];
    data[0..8].copy_from_slice(&LOCK);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const LockArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<LockArgs>(),
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

/// CPI: claimFee
#[inline(always)]
pub fn claim_fee<'a>(
    accounts: &ClaimfeeAccounts<'a>, args: &ClaimfeeArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<ClaimfeeArgs>()];
    data[0..8].copy_from_slice(&CLAIM_FEE);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const ClaimfeeArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<ClaimfeeArgs>(),
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

/// CPI: createConfig
#[inline(always)]
pub fn create_config<'a>(
    accounts: &CreateconfigAccounts<'a>, args: &CreateconfigArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<CreateconfigArgs>()];
    data[0..8].copy_from_slice(&CREATE_CONFIG);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const CreateconfigArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<CreateconfigArgs>(),
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

/// CPI: closeConfig
#[inline(always)]
pub fn close_config<'a>(
    accounts: &CloseconfigAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&CLOSE_CONFIG);
    
    
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

/// CPI: initializePermissionlessConstantProductPoolWithConfig
#[inline(always)]
pub fn initialize_permissionless_constant_product_pool_with_config<'a>(
    accounts: &InitializepermissionlessconstantproductpoolwithconfigAccounts<'a>, args: &InitializepermissionlessconstantproductpoolwithconfigArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<InitializepermissionlessconstantproductpoolwithconfigArgs>()];
    data[0..8].copy_from_slice(&INITIALIZE_PERMISSIONLESS_CONSTANT_PRODUCT_POOL_WITH_CONFIG);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const InitializepermissionlessconstantproductpoolwithconfigArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<InitializepermissionlessconstantproductpoolwithconfigArgs>(),
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
    invoke_signed::<26>(&instruction, &account_views, signers)
}

/// CPI: initializePermissionlessConstantProductPoolWithConfig2
#[inline(always)]
pub fn initialize_permissionless_constant_product_pool_with_config2<'a>(
    accounts: &Initializepermissionlessconstantproductpoolwithconfig2Accounts<'a>, args: &Initializepermissionlessconstantproductpoolwithconfig2Args,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<Initializepermissionlessconstantproductpoolwithconfig2Args>()];
    data[0..8].copy_from_slice(&INITIALIZE_PERMISSIONLESS_CONSTANT_PRODUCT_POOL_WITH_CONFIG2);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const Initializepermissionlessconstantproductpoolwithconfig2Args as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<Initializepermissionlessconstantproductpoolwithconfig2Args>(),
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
    invoke_signed::<26>(&instruction, &account_views, signers)
}

/// CPI: initializeCustomizablePermissionlessConstantProductPool
#[inline(always)]
pub fn initialize_customizable_permissionless_constant_product_pool<'a>(
    accounts: &InitializecustomizablepermissionlessconstantproductpoolAccounts<'a>, args: &InitializecustomizablepermissionlessconstantproductpoolArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<InitializecustomizablepermissionlessconstantproductpoolArgs>()];
    data[0..8].copy_from_slice(&INITIALIZE_CUSTOMIZABLE_PERMISSIONLESS_CONSTANT_PRODUCT_POOL);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const InitializecustomizablepermissionlessconstantproductpoolArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<InitializecustomizablepermissionlessconstantproductpoolArgs>(),
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
    invoke_signed::<25>(&instruction, &account_views, signers)
}

/// CPI: updateActivationPoint
#[inline(always)]
pub fn update_activation_point<'a>(
    accounts: &UpdateactivationpointAccounts<'a>, args: &UpdateactivationpointArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<UpdateactivationpointArgs>()];
    data[0..8].copy_from_slice(&UPDATE_ACTIVATION_POINT);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const UpdateactivationpointArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<UpdateactivationpointArgs>(),
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

/// CPI: withdrawProtocolFees
#[inline(always)]
pub fn withdraw_protocol_fees<'a>(
    accounts: &WithdrawprotocolfeesAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&WITHDRAW_PROTOCOL_FEES);
    
    
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

/// CPI: setWhitelistedVault
#[inline(always)]
pub fn set_whitelisted_vault<'a>(
    accounts: &SetwhitelistedvaultAccounts<'a>, args: &SetwhitelistedvaultArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SetwhitelistedvaultArgs>()];
    data[0..8].copy_from_slice(&SET_WHITELISTED_VAULT);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SetwhitelistedvaultArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SetwhitelistedvaultArgs>(),
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

/// CPI: partnerClaimFee
#[inline(always)]
pub fn partner_claim_fee<'a>(
    accounts: &PartnerclaimfeeAccounts<'a>, args: &PartnerclaimfeeArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<PartnerclaimfeeArgs>()];
    data[0..8].copy_from_slice(&PARTNER_CLAIM_FEE);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const PartnerclaimfeeArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<PartnerclaimfeeArgs>(),
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

