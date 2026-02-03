//! Raydium AMM V4 Pinocchio CPI Client
//!
//! Program ID: 675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8
//!
//! Raydium AMM V4 is a native Solana program (non-Anchor) with custom instruction format.
//! 
//! Official source: https://github.com/raydium-io/raydium-amm
//!
//! Supported instructions:
//! - SwapBaseIn (9): Fixed input swap (requires OpenBook market accounts)
//! - SwapBaseOut (11): Fixed output swap (requires OpenBook market accounts)
//! - SwapBaseInV2 (16): Simplified fixed input swap (no OpenBook required)
//! - SwapBaseOutV2 (17): Simplified fixed output swap (no OpenBook required)
//! - Deposit (3): Add liquidity
//! - Withdraw (4): Remove liquidity

use pinocchio::{
    AccountView, Address, ProgramResult,
    cpi::{invoke_signed, Signer},
    instruction::{InstructionView, InstructionAccount},
};

// ============================================
// Constants
// ============================================

/// Raydium AMM V4 Program ID
pub const PROGRAM_ID: Address = Address::new_from_array(
    five8_const::decode_32_const("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8")
);

/// Raydium AMM Authority PDA
/// Derived from: create_program_address(&[AUTHORITY_AMM, &[nonce]])
pub const AMM_AUTHORITY: Address = Address::new_from_array(
    five8_const::decode_32_const("5Q544fKrFoe6tsEbD7S8EmxGTJYAKtTVhAW5Q5pge4j1")
);

/// SPL Token Program
pub const TOKEN_PROGRAM: Address = Address::new_from_array(
    five8_const::decode_32_const("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")
);

// Instruction discriminators
pub const SWAP_BASE_IN_DISCRIMINATOR: u8 = 9;
pub const SWAP_BASE_OUT_DISCRIMINATOR: u8 = 11;
pub const SWAP_BASE_IN_V2_DISCRIMINATOR: u8 = 16;
pub const SWAP_BASE_OUT_V2_DISCRIMINATOR: u8 = 17;
pub const DEPOSIT_DISCRIMINATOR: u8 = 3;
pub const WITHDRAW_DISCRIMINATOR: u8 = 4;

// ============================================
// Account Data Layout (AmmInfo)
// ============================================

/// AMM state account layout (AmmInfo)
/// 
/// Based on official source: https://github.com/raydium-io/raydium-amm/blob/master/program/src/state.rs
/// 
/// NOTE: Pool reserves are NOT stored in AmmInfo! 
/// You need to read balances from coin_vault and pc_vault token accounts!
pub struct AmmStateLayout;

impl AmmStateLayout {
    // ============================================
    // Basic fields (each u64 = 8 bytes)
    // ============================================
    
    /// Status flags
    pub const STATUS_OFFSET: usize = 0;
    /// Nonce (for PDA derivation)
    pub const NONCE_OFFSET: usize = 8;
    /// Max order count
    pub const ORDER_NUM_OFFSET: usize = 16;
    /// Depth range
    pub const DEPTH_OFFSET: usize = 24;
    /// Coin decimals
    pub const COIN_DECIMALS_OFFSET: usize = 32;
    /// PC decimals
    pub const PC_DECIMALS_OFFSET: usize = 40;
    /// AMM state
    pub const STATE_OFFSET: usize = 48;
    /// Reset flag
    pub const RESET_FLAG_OFFSET: usize = 56;
    /// Min size
    pub const MIN_SIZE_OFFSET: usize = 64;
    /// Vol max cut ratio
    pub const VOL_MAX_CUT_RATIO_OFFSET: usize = 72;
    /// Amount wave
    pub const AMOUNT_WAVE_OFFSET: usize = 80;
    /// Coin lot size
    pub const COIN_LOT_SIZE_OFFSET: usize = 88;
    /// PC lot size
    pub const PC_LOT_SIZE_OFFSET: usize = 96;
    /// Min price multiplier
    pub const MIN_PRICE_MULTIPLIER_OFFSET: usize = 104;
    /// Max price multiplier
    pub const MAX_PRICE_MULTIPLIER_OFFSET: usize = 112;
    /// System decimal value
    pub const SYS_DECIMAL_VALUE_OFFSET: usize = 120;
    
    // ============================================
    // Fees struct (64 bytes, offset 128)
    // ============================================
    
    /// Min separate numerator
    pub const MIN_SEPARATE_NUM_OFFSET: usize = 128;
    /// Min separate denominator
    pub const MIN_SEPARATE_DEN_OFFSET: usize = 136;
    /// Trade fee numerator
    pub const TRADE_FEE_NUM_OFFSET: usize = 144;
    /// Trade fee denominator
    pub const TRADE_FEE_DEN_OFFSET: usize = 152;
    /// Pnl numerator
    pub const PNL_NUM_OFFSET: usize = 160;
    /// Pnl denominator
    pub const PNL_DEN_OFFSET: usize = 168;
    /// Swap fee numerator (default 25)
    pub const SWAP_FEE_NUM_OFFSET: usize = 176;
    /// Swap fee denominator (default 10000)
    pub const SWAP_FEE_DEN_OFFSET: usize = 184;
    
    // ============================================
    // StateData struct (136 bytes, offset 192)
    // ============================================
    
    /// Need take pnl coin
    pub const NEED_TAKE_PNL_COIN_OFFSET: usize = 192;
    /// Need take pnl pc
    pub const NEED_TAKE_PNL_PC_OFFSET: usize = 200;
    /// Total pnl pc
    pub const TOTAL_PNL_PC_OFFSET: usize = 208;
    /// Total pnl coin
    pub const TOTAL_PNL_COIN_OFFSET: usize = 216;
    /// Pool open time
    pub const POOL_OPEN_TIME_OFFSET: usize = 224;
    /// Padding [u64; 2]
    pub const PADDING_OFFSET: usize = 232;
    /// Orderbook to init time
    pub const ORDERBOOK_TO_INIT_TIME_OFFSET: usize = 248;
    /// Swap coin in amount (u128)
    pub const SWAP_COIN_IN_AMOUNT_OFFSET: usize = 256;
    /// Swap pc out amount (u128)
    pub const SWAP_PC_OUT_AMOUNT_OFFSET: usize = 272;
    /// Swap acc pc fee
    pub const SWAP_ACC_PC_FEE_OFFSET: usize = 288;
    /// Swap pc in amount (u128)
    pub const SWAP_PC_IN_AMOUNT_OFFSET: usize = 296;
    /// Swap coin out amount (u128)
    pub const SWAP_COIN_OUT_AMOUNT_OFFSET: usize = 312;
    /// Swap acc coin fee
    pub const SWAP_ACC_COIN_FEE_OFFSET: usize = 320;
    
    // ============================================
    // Pubkey fields (each 32 bytes)
    // ============================================
    
    /// Coin vault pubkey (offset 328)
    pub const COIN_VAULT_OFFSET: usize = 328;
    /// PC vault pubkey
    pub const PC_VAULT_OFFSET: usize = 360;
    /// Coin vault mint
    pub const COIN_VAULT_MINT_OFFSET: usize = 392;
    /// PC vault mint
    pub const PC_VAULT_MINT_OFFSET: usize = 424;
    /// LP mint
    pub const LP_MINT_OFFSET: usize = 456;
    /// Open orders
    pub const OPEN_ORDERS_OFFSET: usize = 488;
    /// Market
    pub const MARKET_OFFSET: usize = 520;
    /// Market program
    pub const MARKET_PROGRAM_OFFSET: usize = 552;
    /// Target orders
    pub const TARGET_ORDERS_OFFSET: usize = 584;
    /// Padding1 [u64; 8]
    pub const PADDING1_OFFSET: usize = 616;
    /// AMM owner
    pub const AMM_OWNER_OFFSET: usize = 680;
    /// LP amount
    pub const LP_AMOUNT_OFFSET: usize = 712;
    /// Client order ID
    pub const CLIENT_ORDER_ID_OFFSET: usize = 720;
    /// Recent epoch
    pub const RECENT_EPOCH_OFFSET: usize = 728;
    /// Padding2
    pub const PADDING2_OFFSET: usize = 736;
    
    /// Total size
    pub const SIZE: usize = 744;
}

/// Parse swap fee from AMM state account data
/// 
/// Returns (numerator, denominator), default is (25, 10000) = 0.25%
#[inline(always)]
pub fn parse_swap_fee(data: &[u8]) -> Option<(u64, u64)> {
    if data.len() < 192 {
        return None;
    }
    
    let numerator = u64::from_le_bytes(
        data[AmmStateLayout::SWAP_FEE_NUM_OFFSET..AmmStateLayout::SWAP_FEE_NUM_OFFSET + 8]
            .try_into().ok()?
    );
    let denominator = u64::from_le_bytes(
        data[AmmStateLayout::SWAP_FEE_DEN_OFFSET..AmmStateLayout::SWAP_FEE_DEN_OFFSET + 8]
            .try_into().ok()?
    );
    
    Some((numerator, denominator))
}

/// Parse vault addresses from AMM state account data
/// 
/// Returns (coin_vault, pc_vault) as 32-byte addresses
#[inline(always)]
pub fn parse_vault_addresses(data: &[u8]) -> Option<([u8; 32], [u8; 32])> {
    if data.len() < 392 {
        return None;
    }
    
    let mut coin_vault = [0u8; 32];
    let mut pc_vault = [0u8; 32];
    
    coin_vault.copy_from_slice(&data[AmmStateLayout::COIN_VAULT_OFFSET..AmmStateLayout::COIN_VAULT_OFFSET + 32]);
    pc_vault.copy_from_slice(&data[AmmStateLayout::PC_VAULT_OFFSET..AmmStateLayout::PC_VAULT_OFFSET + 32]);
    
    Some((coin_vault, pc_vault))
}

/// Parse balance from SPL Token account data
/// 
/// SPL Token Account layout:
/// - [0..32] mint
/// - [32..64] owner
/// - [64..72] amount (u64)
/// - ...
#[inline(always)]
pub fn parse_token_account_balance(data: &[u8]) -> Option<u64> {
    if data.len() < 72 {
        return None;
    }
    
    let amount = u64::from_le_bytes(
        data[64..72].try_into().ok()?
    );
    
    Some(amount)
}

/// Get pool reserves from vault accounts
/// 
/// NOTE: This requires coin_vault and pc_vault account data, NOT AmmInfo!
#[inline(always)]
pub fn parse_pool_reserves_from_vaults(
    coin_vault_data: &[u8],
    pc_vault_data: &[u8],
) -> Option<(u64, u64)> {
    let coin_amount = parse_token_account_balance(coin_vault_data)?;
    let pc_amount = parse_token_account_balance(pc_vault_data)?;
    Some((coin_amount, pc_amount))
}

// ============================================
// SwapBaseIn (Instruction 9)
// Fixed input swap, requires OpenBook market accounts
// ============================================

/// SwapBaseIn accounts
/// 
/// Requires 17 accounts (including OpenBook market accounts)
pub struct SwapBaseInAccounts<'a> {
    /// SPL Token Program
    pub token_program: &'a AccountView,
    /// AMM pool account
    pub amm: &'a AccountView,
    /// AMM authority PDA
    pub amm_authority: &'a AccountView,
    /// AMM open orders account
    pub amm_open_orders: &'a AccountView,
    /// AMM coin vault
    pub amm_coin_vault: &'a AccountView,
    /// AMM pc vault
    pub amm_pc_vault: &'a AccountView,
    /// Market program id
    pub market_program: &'a AccountView,
    /// Market account
    pub market: &'a AccountView,
    /// Market bids
    pub market_bids: &'a AccountView,
    /// Market asks
    pub market_asks: &'a AccountView,
    /// Market event queue
    pub market_event_queue: &'a AccountView,
    /// Market coin vault
    pub market_coin_vault: &'a AccountView,
    /// Market pc vault
    pub market_pc_vault: &'a AccountView,
    /// Market vault signer
    pub market_vault_signer: &'a AccountView,
    /// User source token account
    pub user_source: &'a AccountView,
    /// User destination token account
    pub user_destination: &'a AccountView,
    /// User wallet
    pub user_owner: &'a AccountView,
}

impl<'a> SwapBaseInAccounts<'a> {
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 17] {
        [
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::writable(self.amm.address()),
            InstructionAccount::readonly(self.amm_authority.address()),
            InstructionAccount::writable(self.amm_open_orders.address()),
            InstructionAccount::writable(self.amm_coin_vault.address()),
            InstructionAccount::writable(self.amm_pc_vault.address()),
            InstructionAccount::readonly(self.market_program.address()),
            InstructionAccount::writable(self.market.address()),
            InstructionAccount::writable(self.market_bids.address()),
            InstructionAccount::writable(self.market_asks.address()),
            InstructionAccount::writable(self.market_event_queue.address()),
            InstructionAccount::writable(self.market_coin_vault.address()),
            InstructionAccount::writable(self.market_pc_vault.address()),
            InstructionAccount::readonly(self.market_vault_signer.address()),
            InstructionAccount::writable(self.user_source.address()),
            InstructionAccount::writable(self.user_destination.address()),
            InstructionAccount::readonly_signer(self.user_owner.address()),
        ]
    }
    
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 17] {
        [
            self.token_program,
            self.amm,
            self.amm_authority,
            self.amm_open_orders,
            self.amm_coin_vault,
            self.amm_pc_vault,
            self.market_program,
            self.market,
            self.market_bids,
            self.market_asks,
            self.market_event_queue,
            self.market_coin_vault,
            self.market_pc_vault,
            self.market_vault_signer,
            self.user_source,
            self.user_destination,
            self.user_owner,
        ]
    }
}

/// SwapBaseIn arguments
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct SwapBaseInArgs {
    /// Input amount
    pub amount_in: u64,
    /// Minimum output amount
    pub minimum_amount_out: u64,
}

/// Execute SwapBaseIn instruction
#[inline(always)]
pub fn swap_base_in<'a>(
    accounts: &SwapBaseInAccounts<'a>,
    args: &SwapBaseInArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    let mut data = [0u8; 17];
    data[0] = SWAP_BASE_IN_DISCRIMINATOR;
    data[1..9].copy_from_slice(&args.amount_in.to_le_bytes());
    data[9..17].copy_from_slice(&args.minimum_amount_out.to_le_bytes());
    
    let instruction_accounts = accounts.to_instruction_accounts();
    let account_views = accounts.to_views();
    
    let instruction = InstructionView {
        program_id: &PROGRAM_ID,
        accounts: &instruction_accounts,
        data: &data,
    };
    
    invoke_signed::<17>(&instruction, &account_views, signers)
}

// ============================================
// SwapBaseOut (Instruction 11)
// Fixed output swap, requires OpenBook market accounts
// ============================================

/// SwapBaseOut arguments
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct SwapBaseOutArgs {
    /// Maximum input amount
    pub max_amount_in: u64,
    /// Expected output amount
    pub amount_out: u64,
}

/// Execute SwapBaseOut instruction
#[inline(always)]
pub fn swap_base_out<'a>(
    accounts: &SwapBaseInAccounts<'a>,  // Same account structure
    args: &SwapBaseOutArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    let mut data = [0u8; 17];
    data[0] = SWAP_BASE_OUT_DISCRIMINATOR;
    data[1..9].copy_from_slice(&args.max_amount_in.to_le_bytes());
    data[9..17].copy_from_slice(&args.amount_out.to_le_bytes());
    
    let instruction_accounts = accounts.to_instruction_accounts();
    let account_views = accounts.to_views();
    
    let instruction = InstructionView {
        program_id: &PROGRAM_ID,
        accounts: &instruction_accounts,
        data: &data,
    };
    
    invoke_signed::<17>(&instruction, &account_views, signers)
}

// ============================================
// SwapBaseInV2 (Instruction 16)
// Simplified fixed input swap, no OpenBook required
// ============================================

/// SwapBaseInV2 accounts (simplified, only 8 accounts)
pub struct SwapBaseInV2Accounts<'a> {
    /// SPL Token Program
    pub token_program: &'a AccountView,
    /// AMM pool account
    pub amm: &'a AccountView,
    /// AMM authority PDA
    pub amm_authority: &'a AccountView,
    /// AMM coin vault
    pub amm_coin_vault: &'a AccountView,
    /// AMM pc vault
    pub amm_pc_vault: &'a AccountView,
    /// User source token account
    pub user_source: &'a AccountView,
    /// User destination token account
    pub user_destination: &'a AccountView,
    /// User wallet
    pub user_owner: &'a AccountView,
}

impl<'a> SwapBaseInV2Accounts<'a> {
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 8] {
        [
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::writable(self.amm.address()),
            InstructionAccount::readonly(self.amm_authority.address()),
            InstructionAccount::writable(self.amm_coin_vault.address()),
            InstructionAccount::writable(self.amm_pc_vault.address()),
            InstructionAccount::writable(self.user_source.address()),
            InstructionAccount::writable(self.user_destination.address()),
            InstructionAccount::readonly_signer(self.user_owner.address()),
        ]
    }
    
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 8] {
        [
            self.token_program,
            self.amm,
            self.amm_authority,
            self.amm_coin_vault,
            self.amm_pc_vault,
            self.user_source,
            self.user_destination,
            self.user_owner,
        ]
    }
}

/// SwapBaseInV2 arguments
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct SwapBaseInV2Args {
    /// Input amount
    pub amount_in: u64,
    /// Minimum output amount
    pub minimum_amount_out: u64,
}

/// Execute SwapBaseInV2 instruction
#[inline(always)]
pub fn swap_base_in_v2<'a>(
    accounts: &SwapBaseInV2Accounts<'a>,
    args: &SwapBaseInV2Args,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    let mut data = [0u8; 17];
    data[0] = SWAP_BASE_IN_V2_DISCRIMINATOR;
    data[1..9].copy_from_slice(&args.amount_in.to_le_bytes());
    data[9..17].copy_from_slice(&args.minimum_amount_out.to_le_bytes());
    
    let instruction_accounts = accounts.to_instruction_accounts();
    let account_views = accounts.to_views();
    
    let instruction = InstructionView {
        program_id: &PROGRAM_ID,
        accounts: &instruction_accounts,
        data: &data,
    };
    
    invoke_signed::<8>(&instruction, &account_views, signers)
}

// ============================================
// SwapBaseOutV2 (Instruction 17)
// Simplified fixed output swap, no OpenBook required
// ============================================

/// SwapBaseOutV2 arguments
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct SwapBaseOutV2Args {
    /// Maximum input amount
    pub max_amount_in: u64,
    /// Expected output amount
    pub amount_out: u64,
}

/// Execute SwapBaseOutV2 instruction
#[inline(always)]
pub fn swap_base_out_v2<'a>(
    accounts: &SwapBaseInV2Accounts<'a>,  // Same account structure
    args: &SwapBaseOutV2Args,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    let mut data = [0u8; 17];
    data[0] = SWAP_BASE_OUT_V2_DISCRIMINATOR;
    data[1..9].copy_from_slice(&args.max_amount_in.to_le_bytes());
    data[9..17].copy_from_slice(&args.amount_out.to_le_bytes());
    
    let instruction_accounts = accounts.to_instruction_accounts();
    let account_views = accounts.to_views();
    
    let instruction = InstructionView {
        program_id: &PROGRAM_ID,
        accounts: &instruction_accounts,
        data: &data,
    };
    
    invoke_signed::<8>(&instruction, &account_views, signers)
}

// ============================================
// Deposit (Instruction 3)
// Add liquidity
// ============================================

/// Deposit accounts
pub struct DepositAccounts<'a> {
    /// SPL Token Program
    pub token_program: &'a AccountView,
    /// AMM pool account
    pub amm: &'a AccountView,
    /// AMM authority PDA
    pub amm_authority: &'a AccountView,
    /// AMM open orders
    pub amm_open_orders: &'a AccountView,
    /// AMM target orders
    pub amm_target_orders: &'a AccountView,
    /// AMM LP mint
    pub amm_lp_mint: &'a AccountView,
    /// AMM coin vault
    pub amm_coin_vault: &'a AccountView,
    /// AMM pc vault
    pub amm_pc_vault: &'a AccountView,
    /// Market
    pub market: &'a AccountView,
    /// User coin account
    pub user_coin: &'a AccountView,
    /// User pc account
    pub user_pc: &'a AccountView,
    /// User LP account
    pub user_lp: &'a AccountView,
    /// User wallet
    pub user_owner: &'a AccountView,
    /// Market event queue
    pub market_event_queue: &'a AccountView,
}

impl<'a> DepositAccounts<'a> {
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 14] {
        [
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::writable(self.amm.address()),
            InstructionAccount::readonly(self.amm_authority.address()),
            InstructionAccount::readonly(self.amm_open_orders.address()),
            InstructionAccount::writable(self.amm_target_orders.address()),
            InstructionAccount::writable(self.amm_lp_mint.address()),
            InstructionAccount::writable(self.amm_coin_vault.address()),
            InstructionAccount::writable(self.amm_pc_vault.address()),
            InstructionAccount::readonly(self.market.address()),
            InstructionAccount::writable(self.user_coin.address()),
            InstructionAccount::writable(self.user_pc.address()),
            InstructionAccount::writable(self.user_lp.address()),
            InstructionAccount::readonly_signer(self.user_owner.address()),
            InstructionAccount::readonly(self.market_event_queue.address()),
        ]
    }
    
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 14] {
        [
            self.token_program,
            self.amm,
            self.amm_authority,
            self.amm_open_orders,
            self.amm_target_orders,
            self.amm_lp_mint,
            self.amm_coin_vault,
            self.amm_pc_vault,
            self.market,
            self.user_coin,
            self.user_pc,
            self.user_lp,
            self.user_owner,
            self.market_event_queue,
        ]
    }
}

/// Deposit arguments
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct DepositArgs {
    /// Max coin amount
    pub max_coin_amount: u64,
    /// Max pc amount
    pub max_pc_amount: u64,
    /// Base side (0 = coin, 1 = pc)
    pub base_side: u64,
}

/// Execute Deposit instruction
#[inline(always)]
pub fn deposit<'a>(
    accounts: &DepositAccounts<'a>,
    args: &DepositArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    let mut data = [0u8; 25];
    data[0] = DEPOSIT_DISCRIMINATOR;
    data[1..9].copy_from_slice(&args.max_coin_amount.to_le_bytes());
    data[9..17].copy_from_slice(&args.max_pc_amount.to_le_bytes());
    data[17..25].copy_from_slice(&args.base_side.to_le_bytes());
    
    let instruction_accounts = accounts.to_instruction_accounts();
    let account_views = accounts.to_views();
    
    let instruction = InstructionView {
        program_id: &PROGRAM_ID,
        accounts: &instruction_accounts,
        data: &data,
    };
    
    invoke_signed::<14>(&instruction, &account_views, signers)
}

// ============================================
// Withdraw (Instruction 4)
// Remove liquidity
// ============================================

/// Withdraw accounts
pub struct WithdrawAccounts<'a> {
    /// SPL Token Program
    pub token_program: &'a AccountView,
    /// AMM pool account
    pub amm: &'a AccountView,
    /// AMM authority PDA
    pub amm_authority: &'a AccountView,
    /// AMM open orders
    pub amm_open_orders: &'a AccountView,
    /// AMM target orders
    pub amm_target_orders: &'a AccountView,
    /// AMM LP mint
    pub amm_lp_mint: &'a AccountView,
    /// AMM coin vault
    pub amm_coin_vault: &'a AccountView,
    /// AMM pc vault
    pub amm_pc_vault: &'a AccountView,
    /// Market program
    pub market_program: &'a AccountView,
    /// Market
    pub market: &'a AccountView,
    /// Market coin vault
    pub market_coin_vault: &'a AccountView,
    /// Market pc vault
    pub market_pc_vault: &'a AccountView,
    /// Market vault signer
    pub market_vault_signer: &'a AccountView,
    /// User LP account
    pub user_lp: &'a AccountView,
    /// User coin account
    pub user_coin: &'a AccountView,
    /// User pc account
    pub user_pc: &'a AccountView,
    /// User wallet
    pub user_owner: &'a AccountView,
    /// Market event queue
    pub market_event_queue: &'a AccountView,
    /// Market bids
    pub market_bids: &'a AccountView,
    /// Market asks
    pub market_asks: &'a AccountView,
}

impl<'a> WithdrawAccounts<'a> {
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 20] {
        [
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::writable(self.amm.address()),
            InstructionAccount::readonly(self.amm_authority.address()),
            InstructionAccount::writable(self.amm_open_orders.address()),
            InstructionAccount::writable(self.amm_target_orders.address()),
            InstructionAccount::writable(self.amm_lp_mint.address()),
            InstructionAccount::writable(self.amm_coin_vault.address()),
            InstructionAccount::writable(self.amm_pc_vault.address()),
            InstructionAccount::readonly(self.market_program.address()),
            InstructionAccount::writable(self.market.address()),
            InstructionAccount::writable(self.market_coin_vault.address()),
            InstructionAccount::writable(self.market_pc_vault.address()),
            InstructionAccount::readonly(self.market_vault_signer.address()),
            InstructionAccount::writable(self.user_lp.address()),
            InstructionAccount::writable(self.user_coin.address()),
            InstructionAccount::writable(self.user_pc.address()),
            InstructionAccount::readonly_signer(self.user_owner.address()),
            InstructionAccount::writable(self.market_event_queue.address()),
            InstructionAccount::writable(self.market_bids.address()),
            InstructionAccount::writable(self.market_asks.address()),
        ]
    }
    
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 20] {
        [
            self.token_program,
            self.amm,
            self.amm_authority,
            self.amm_open_orders,
            self.amm_target_orders,
            self.amm_lp_mint,
            self.amm_coin_vault,
            self.amm_pc_vault,
            self.market_program,
            self.market,
            self.market_coin_vault,
            self.market_pc_vault,
            self.market_vault_signer,
            self.user_lp,
            self.user_coin,
            self.user_pc,
            self.user_owner,
            self.market_event_queue,
            self.market_bids,
            self.market_asks,
        ]
    }
}

/// Withdraw arguments
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct WithdrawArgs {
    /// LP amount
    pub amount: u64,
}

/// Execute Withdraw instruction
#[inline(always)]
pub fn withdraw<'a>(
    accounts: &WithdrawAccounts<'a>,
    args: &WithdrawArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    let mut data = [0u8; 9];
    data[0] = WITHDRAW_DISCRIMINATOR;
    data[1..9].copy_from_slice(&args.amount.to_le_bytes());
    
    let instruction_accounts = accounts.to_instruction_accounts();
    let account_views = accounts.to_views();
    
    let instruction = InstructionView {
        program_id: &PROGRAM_ID,
        accounts: &instruction_accounts,
        data: &data,
    };
    
    invoke_signed::<20>(&instruction, &account_views, signers)
}

// ============================================
// Helper Functions
// ============================================

/// Calculate swap output amount (without slippage)
/// 
/// Uses constant product formula: x * y = k
/// output = (reserve_out * amount_in) / (reserve_in + amount_in)
#[inline(always)]
pub fn calculate_swap_output(
    amount_in: u64,
    reserve_in: u64,
    reserve_out: u64,
    fee_numerator: u64,
    fee_denominator: u64,
) -> u64 {
    if reserve_in == 0 || reserve_out == 0 || amount_in == 0 {
        return 0;
    }
    
    // Amount after fee deduction
    let amount_in_with_fee = (amount_in as u128)
        .checked_mul(fee_denominator.saturating_sub(fee_numerator) as u128)
        .unwrap_or(0);
    
    // Calculate output
    let numerator = amount_in_with_fee
        .checked_mul(reserve_out as u128)
        .unwrap_or(0);
    
    let denominator = (reserve_in as u128)
        .checked_mul(fee_denominator as u128)
        .unwrap_or(1)
        .checked_add(amount_in_with_fee)
        .unwrap_or(1);
    
    (numerator / denominator) as u64
}

/// Calculate swap input amount (given expected output)
#[inline(always)]
pub fn calculate_swap_input(
    amount_out: u64,
    reserve_in: u64,
    reserve_out: u64,
    fee_numerator: u64,
    fee_denominator: u64,
) -> u64 {
    if reserve_in == 0 || reserve_out == 0 || amount_out == 0 || amount_out >= reserve_out {
        return u64::MAX;
    }
    
    let numerator = (reserve_in as u128)
        .checked_mul(amount_out as u128)
        .unwrap_or(u128::MAX)
        .checked_mul(fee_denominator as u128)
        .unwrap_or(u128::MAX);
    
    let denominator = (reserve_out.saturating_sub(amount_out) as u128)
        .checked_mul(fee_denominator.saturating_sub(fee_numerator) as u128)
        .unwrap_or(1);
    
    ((numerator / denominator) + 1) as u64
}

/// Get default swap fee (0.25%)
#[inline(always)]
pub const fn default_swap_fee() -> (u64, u64) {
    (25, 10000) // 25 / 10000 = 0.25%
}
