//! CPI module for OpenBook V2
//!
//! Program: openbook_v2
//! Program ID: opnb2LAfJYbRMAHHvqjCwQxanZn7ReEHp1k81EohpZb
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
pub const ID: Address = Address::new_from_array(five8_const::decode_32_const("opnb2LAfJYbRMAHHvqjCwQxanZn7ReEHp1k81EohpZb"));

// ============================================
// Instruction Discriminators
// ============================================
pub const CREATE_MARKET: [u8; 8] = [103, 226, 97, 235, 200, 188, 251, 254];
pub const CLOSE_MARKET: [u8; 8] = [88, 154, 248, 186, 48, 14, 123, 244];
pub const CREATE_OPEN_ORDERS_INDEXER: [u8; 8] = [64, 64, 153, 255, 217, 71, 249, 133];
pub const CLOSE_OPEN_ORDERS_INDEXER: [u8; 8] = [103, 249, 229, 231, 247, 253, 197, 136];
pub const CREATE_OPEN_ORDERS_ACCOUNT: [u8; 8] = [204, 181, 175, 222, 40, 125, 188, 71];
pub const CLOSE_OPEN_ORDERS_ACCOUNT: [u8; 8] = [176, 74, 115, 210, 54, 179, 91, 103];
pub const PLACE_ORDER: [u8; 8] = [51, 194, 155, 175, 109, 130, 96, 106];
pub const EDIT_ORDER: [u8; 8] = [254, 208, 118, 29, 173, 248, 200, 70];
pub const EDIT_ORDER_PEGGED: [u8; 8] = [62, 187, 125, 69, 26, 221, 157, 133];
pub const PLACE_ORDERS: [u8; 8] = [60, 63, 50, 123, 12, 197, 60, 190];
pub const CANCEL_ALL_AND_PLACE_ORDERS: [u8; 8] = [128, 155, 222, 60, 186, 40, 225, 50];
pub const PLACE_ORDER_PEGGED: [u8; 8] = [141, 185, 251, 63, 74, 85, 210, 145];
pub const PLACE_TAKE_ORDER: [u8; 8] = [3, 44, 71, 3, 26, 199, 203, 85];
pub const CONSUME_EVENTS: [u8; 8] = [221, 145, 177, 52, 31, 47, 63, 201];
pub const CONSUME_GIVEN_EVENTS: [u8; 8] = [209, 227, 54, 4, 109, 172, 41, 71];
pub const CANCEL_ORDER: [u8; 8] = [95, 129, 237, 240, 8, 49, 223, 132];
pub const CANCEL_ORDER_BY_CLIENT_ORDER_ID: [u8; 8] = [115, 178, 201, 8, 175, 183, 123, 119];
pub const CANCEL_ALL_ORDERS: [u8; 8] = [196, 83, 243, 171, 17, 100, 160, 143];
pub const DEPOSIT: [u8; 8] = [242, 35, 198, 137, 82, 225, 242, 182];
pub const REFILL: [u8; 8] = [128, 207, 142, 11, 54, 232, 38, 201];
pub const SETTLE_FUNDS: [u8; 8] = [238, 64, 163, 96, 75, 171, 16, 33];
pub const SETTLE_FUNDS_EXPIRED: [u8; 8] = [107, 18, 56, 69, 228, 56, 55, 164];
pub const SWEEP_FEES: [u8; 8] = [175, 225, 98, 71, 118, 66, 34, 148];
pub const SET_DELEGATE: [u8; 8] = [242, 30, 46, 76, 108, 235, 128, 181];
pub const SET_MARKET_EXPIRED: [u8; 8] = [219, 82, 219, 236, 60, 115, 197, 64];
pub const PRUNE_ORDERS: [u8; 8] = [27, 213, 159, 191, 12, 116, 112, 121];
pub const STUB_ORACLE_CREATE: [u8; 8] = [172, 63, 101, 83, 141, 76, 199, 216];
pub const STUB_ORACLE_CLOSE: [u8; 8] = [92, 137, 45, 3, 45, 60, 117, 224];
pub const STUB_ORACLE_SET: [u8; 8] = [109, 198, 79, 121, 65, 202, 161, 142];

// ============================================
// Instruction Arguments
// ============================================
/// Arguments for `createMarket`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct CreatemarketArgs {
    pub name: [u8; 32],
    pub oracle_config: [u8; 32],
    pub quote_lot_size: i64,
    pub base_lot_size: i64,
    pub maker_fee: i64,
    pub taker_fee: i64,
    pub time_expiry: i64,
}

/// Arguments for `createOpenOrdersAccount`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct CreateopenordersaccountArgs {
    pub name: [u8; 32],
}

/// Arguments for `placeOrder`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct PlaceorderArgs {
    pub args: [u8; 32],
}

/// Arguments for `editOrder`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct EditorderArgs {
    pub client_order_id: u64,
    pub expected_cancel_size: i64,
    pub place_order: [u8; 32],
}

/// Arguments for `editOrderPegged`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct EditorderpeggedArgs {
    pub client_order_id: u64,
    pub expected_cancel_size: i64,
    pub place_order: [u8; 32],
}

/// Arguments for `placeOrders`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct PlaceordersArgs {
    pub orders_type: [u8; 32],
    pub bids: [u8; 32],
    pub asks: [u8; 32],
    pub limit: u8,
}

/// Arguments for `cancelAllAndPlaceOrders`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct CancelallandplaceordersArgs {
    pub orders_type: [u8; 32],
    pub bids: [u8; 32],
    pub asks: [u8; 32],
    pub limit: u8,
}

/// Arguments for `placeOrderPegged`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct PlaceorderpeggedArgs {
    pub args: [u8; 32],
}

/// Arguments for `placeTakeOrder`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct PlacetakeorderArgs {
    pub args: [u8; 32],
}

/// Arguments for `consumeEvents`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct ConsumeeventsArgs {
    pub limit: u64,
}

/// Arguments for `consumeGivenEvents`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct ConsumegiveneventsArgs {
    pub slots: [u8; 32],
}

/// Arguments for `cancelOrder`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct CancelorderArgs {
    pub order_id: u128,
}

/// Arguments for `cancelOrderByClientOrderId`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct CancelorderbyclientorderidArgs {
    pub client_order_id: u64,
}

/// Arguments for `cancelAllOrders`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct CancelallordersArgs {
    pub side_option: Option<[u8; 32]>,
    pub limit: u8,
}

/// Arguments for `deposit`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct DepositArgs {
    pub base_amount: u64,
    pub quote_amount: u64,
}

/// Arguments for `refill`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct RefillArgs {
    pub base_amount: u64,
    pub quote_amount: u64,
}

/// Arguments for `pruneOrders`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct PruneordersArgs {
    pub limit: u8,
}

/// Arguments for `stubOracleCreate`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct StuboraclecreateArgs {
    pub price: f64,
}

/// Arguments for `stubOracleSet`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct StuboraclesetArgs {
    pub price: f64,
}


// ============================================
// Instruction Accounts
// ============================================
/// Accounts for `createMarket`
pub struct CreatemarketAccounts<'a> {
    /// market
    pub market: &'a AccountView,
    /// marketAuthority
    pub market_authority: &'a AccountView,
    /// Accounts are initialized by client,
    pub bids: &'a AccountView,
    /// asks
    pub asks: &'a AccountView,
    /// eventHeap
    pub event_heap: &'a AccountView,
    /// payer
    pub payer: &'a AccountView,
    /// marketBaseVault
    pub market_base_vault: &'a AccountView,
    /// marketQuoteVault
    pub market_quote_vault: &'a AccountView,
    /// baseMint
    pub base_mint: &'a AccountView,
    /// quoteMint
    pub quote_mint: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// associatedTokenProgram
    pub associated_token_program: &'a AccountView,
    /// oracleA
    pub oracle_a: &'a AccountView,
    /// oracleB
    pub oracle_b: &'a AccountView,
    /// collectFeeAdmin
    pub collect_fee_admin: &'a AccountView,
    /// openOrdersAdmin
    pub open_orders_admin: &'a AccountView,
    /// consumeEventsAdmin
    pub consume_events_admin: &'a AccountView,
    /// closeMarketAdmin
    pub close_market_admin: &'a AccountView,
    /// eventAuthority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> CreatemarketAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 21] {
        [
            InstructionAccount::writable_signer(self.market.address()),
            InstructionAccount::readonly(self.market_authority.address()),
            InstructionAccount::writable(self.bids.address()),
            InstructionAccount::writable(self.asks.address()),
            InstructionAccount::writable(self.event_heap.address()),
            InstructionAccount::writable_signer(self.payer.address()),
            InstructionAccount::writable(self.market_base_vault.address()),
            InstructionAccount::writable(self.market_quote_vault.address()),
            InstructionAccount::readonly(self.base_mint.address()),
            InstructionAccount::readonly(self.quote_mint.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
            InstructionAccount::readonly(self.oracle_a.address()),
            InstructionAccount::readonly(self.oracle_b.address()),
            InstructionAccount::readonly(self.collect_fee_admin.address()),
            InstructionAccount::readonly(self.open_orders_admin.address()),
            InstructionAccount::readonly(self.consume_events_admin.address()),
            InstructionAccount::readonly(self.close_market_admin.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 21] {
        [
            self.market, self.market_authority, self.bids, self.asks, self.event_heap, self.payer, self.market_base_vault, self.market_quote_vault, self.base_mint, self.quote_mint, self.system_program, self.token_program, self.associated_token_program, self.oracle_a, self.oracle_b, self.collect_fee_admin, self.open_orders_admin, self.consume_events_admin, self.close_market_admin, self.event_authority, self.program
        ]
    }
}

/// Accounts for `closeMarket`
pub struct ClosemarketAccounts<'a> {
    /// closeMarketAdmin
    pub close_market_admin: &'a AccountView,
    /// market
    pub market: &'a AccountView,
    /// bids
    pub bids: &'a AccountView,
    /// asks
    pub asks: &'a AccountView,
    /// eventHeap
    pub event_heap: &'a AccountView,
    /// solDestination
    pub sol_destination: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
}

impl<'a> ClosemarketAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 7] {
        [
            InstructionAccount::readonly_signer(self.close_market_admin.address()),
            InstructionAccount::writable(self.market.address()),
            InstructionAccount::writable(self.bids.address()),
            InstructionAccount::writable(self.asks.address()),
            InstructionAccount::writable(self.event_heap.address()),
            InstructionAccount::writable(self.sol_destination.address()),
            InstructionAccount::readonly(self.token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 7] {
        [
            self.close_market_admin, self.market, self.bids, self.asks, self.event_heap, self.sol_destination, self.token_program
        ]
    }
}

/// Accounts for `createOpenOrdersIndexer`
pub struct CreateopenordersindexerAccounts<'a> {
    /// payer
    pub payer: &'a AccountView,
    /// owner
    pub owner: &'a AccountView,
    /// openOrdersIndexer
    pub open_orders_indexer: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
}

impl<'a> CreateopenordersindexerAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 4] {
        [
            InstructionAccount::writable_signer(self.payer.address()),
            InstructionAccount::readonly_signer(self.owner.address()),
            InstructionAccount::writable(self.open_orders_indexer.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 4] {
        [
            self.payer, self.owner, self.open_orders_indexer, self.system_program
        ]
    }
}

/// Accounts for `closeOpenOrdersIndexer`
pub struct CloseopenordersindexerAccounts<'a> {
    /// owner
    pub owner: &'a AccountView,
    /// openOrdersIndexer
    pub open_orders_indexer: &'a AccountView,
    /// solDestination
    pub sol_destination: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
}

impl<'a> CloseopenordersindexerAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 4] {
        [
            InstructionAccount::readonly_signer(self.owner.address()),
            InstructionAccount::writable(self.open_orders_indexer.address()),
            InstructionAccount::writable(self.sol_destination.address()),
            InstructionAccount::readonly(self.token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 4] {
        [
            self.owner, self.open_orders_indexer, self.sol_destination, self.token_program
        ]
    }
}

/// Accounts for `createOpenOrdersAccount`
pub struct CreateopenordersaccountAccounts<'a> {
    /// payer
    pub payer: &'a AccountView,
    /// owner
    pub owner: &'a AccountView,
    /// delegateAccount
    pub delegate_account: &'a AccountView,
    /// openOrdersIndexer
    pub open_orders_indexer: &'a AccountView,
    /// openOrdersAccount
    pub open_orders_account: &'a AccountView,
    /// market
    pub market: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
}

impl<'a> CreateopenordersaccountAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 7] {
        [
            InstructionAccount::writable_signer(self.payer.address()),
            InstructionAccount::readonly_signer(self.owner.address()),
            InstructionAccount::readonly(self.delegate_account.address()),
            InstructionAccount::writable(self.open_orders_indexer.address()),
            InstructionAccount::writable(self.open_orders_account.address()),
            InstructionAccount::readonly(self.market.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 7] {
        [
            self.payer, self.owner, self.delegate_account, self.open_orders_indexer, self.open_orders_account, self.market, self.system_program
        ]
    }
}

/// Accounts for `closeOpenOrdersAccount`
pub struct CloseopenordersaccountAccounts<'a> {
    /// owner
    pub owner: &'a AccountView,
    /// openOrdersIndexer
    pub open_orders_indexer: &'a AccountView,
    /// openOrdersAccount
    pub open_orders_account: &'a AccountView,
    /// solDestination
    pub sol_destination: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
}

impl<'a> CloseopenordersaccountAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 5] {
        [
            InstructionAccount::readonly_signer(self.owner.address()),
            InstructionAccount::writable(self.open_orders_indexer.address()),
            InstructionAccount::writable(self.open_orders_account.address()),
            InstructionAccount::writable(self.sol_destination.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 5] {
        [
            self.owner, self.open_orders_indexer, self.open_orders_account, self.sol_destination, self.system_program
        ]
    }
}

/// Accounts for `placeOrder`
pub struct PlaceorderAccounts<'a> {
    /// signer
    pub signer: &'a AccountView,
    /// openOrdersAccount
    pub open_orders_account: &'a AccountView,
    /// openOrdersAdmin
    pub open_orders_admin: &'a AccountView,
    /// userTokenAccount
    pub user_token_account: &'a AccountView,
    /// market
    pub market: &'a AccountView,
    /// bids
    pub bids: &'a AccountView,
    /// asks
    pub asks: &'a AccountView,
    /// eventHeap
    pub event_heap: &'a AccountView,
    /// marketVault
    pub market_vault: &'a AccountView,
    /// oracleA
    pub oracle_a: &'a AccountView,
    /// oracleB
    pub oracle_b: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
}

impl<'a> PlaceorderAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 12] {
        [
            InstructionAccount::readonly_signer(self.signer.address()),
            InstructionAccount::writable(self.open_orders_account.address()),
            InstructionAccount::readonly_signer(self.open_orders_admin.address()),
            InstructionAccount::writable(self.user_token_account.address()),
            InstructionAccount::writable(self.market.address()),
            InstructionAccount::writable(self.bids.address()),
            InstructionAccount::writable(self.asks.address()),
            InstructionAccount::writable(self.event_heap.address()),
            InstructionAccount::writable(self.market_vault.address()),
            InstructionAccount::readonly(self.oracle_a.address()),
            InstructionAccount::readonly(self.oracle_b.address()),
            InstructionAccount::readonly(self.token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 12] {
        [
            self.signer, self.open_orders_account, self.open_orders_admin, self.user_token_account, self.market, self.bids, self.asks, self.event_heap, self.market_vault, self.oracle_a, self.oracle_b, self.token_program
        ]
    }
}

/// Accounts for `editOrder`
pub struct EditorderAccounts<'a> {
    /// signer
    pub signer: &'a AccountView,
    /// openOrdersAccount
    pub open_orders_account: &'a AccountView,
    /// openOrdersAdmin
    pub open_orders_admin: &'a AccountView,
    /// userTokenAccount
    pub user_token_account: &'a AccountView,
    /// market
    pub market: &'a AccountView,
    /// bids
    pub bids: &'a AccountView,
    /// asks
    pub asks: &'a AccountView,
    /// eventHeap
    pub event_heap: &'a AccountView,
    /// marketVault
    pub market_vault: &'a AccountView,
    /// oracleA
    pub oracle_a: &'a AccountView,
    /// oracleB
    pub oracle_b: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
}

impl<'a> EditorderAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 12] {
        [
            InstructionAccount::readonly_signer(self.signer.address()),
            InstructionAccount::writable(self.open_orders_account.address()),
            InstructionAccount::readonly_signer(self.open_orders_admin.address()),
            InstructionAccount::writable(self.user_token_account.address()),
            InstructionAccount::writable(self.market.address()),
            InstructionAccount::writable(self.bids.address()),
            InstructionAccount::writable(self.asks.address()),
            InstructionAccount::writable(self.event_heap.address()),
            InstructionAccount::writable(self.market_vault.address()),
            InstructionAccount::readonly(self.oracle_a.address()),
            InstructionAccount::readonly(self.oracle_b.address()),
            InstructionAccount::readonly(self.token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 12] {
        [
            self.signer, self.open_orders_account, self.open_orders_admin, self.user_token_account, self.market, self.bids, self.asks, self.event_heap, self.market_vault, self.oracle_a, self.oracle_b, self.token_program
        ]
    }
}

/// Accounts for `editOrderPegged`
pub struct EditorderpeggedAccounts<'a> {
    /// signer
    pub signer: &'a AccountView,
    /// openOrdersAccount
    pub open_orders_account: &'a AccountView,
    /// openOrdersAdmin
    pub open_orders_admin: &'a AccountView,
    /// userTokenAccount
    pub user_token_account: &'a AccountView,
    /// market
    pub market: &'a AccountView,
    /// bids
    pub bids: &'a AccountView,
    /// asks
    pub asks: &'a AccountView,
    /// eventHeap
    pub event_heap: &'a AccountView,
    /// marketVault
    pub market_vault: &'a AccountView,
    /// oracleA
    pub oracle_a: &'a AccountView,
    /// oracleB
    pub oracle_b: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
}

impl<'a> EditorderpeggedAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 12] {
        [
            InstructionAccount::readonly_signer(self.signer.address()),
            InstructionAccount::writable(self.open_orders_account.address()),
            InstructionAccount::readonly_signer(self.open_orders_admin.address()),
            InstructionAccount::writable(self.user_token_account.address()),
            InstructionAccount::writable(self.market.address()),
            InstructionAccount::writable(self.bids.address()),
            InstructionAccount::writable(self.asks.address()),
            InstructionAccount::writable(self.event_heap.address()),
            InstructionAccount::writable(self.market_vault.address()),
            InstructionAccount::readonly(self.oracle_a.address()),
            InstructionAccount::readonly(self.oracle_b.address()),
            InstructionAccount::readonly(self.token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 12] {
        [
            self.signer, self.open_orders_account, self.open_orders_admin, self.user_token_account, self.market, self.bids, self.asks, self.event_heap, self.market_vault, self.oracle_a, self.oracle_b, self.token_program
        ]
    }
}

/// Accounts for `placeOrders`
pub struct PlaceordersAccounts<'a> {
    /// signer
    pub signer: &'a AccountView,
    /// openOrdersAccount
    pub open_orders_account: &'a AccountView,
    /// openOrdersAdmin
    pub open_orders_admin: &'a AccountView,
    /// userQuoteAccount
    pub user_quote_account: &'a AccountView,
    /// userBaseAccount
    pub user_base_account: &'a AccountView,
    /// market
    pub market: &'a AccountView,
    /// bids
    pub bids: &'a AccountView,
    /// asks
    pub asks: &'a AccountView,
    /// eventHeap
    pub event_heap: &'a AccountView,
    /// marketQuoteVault
    pub market_quote_vault: &'a AccountView,
    /// marketBaseVault
    pub market_base_vault: &'a AccountView,
    /// oracleA
    pub oracle_a: &'a AccountView,
    /// oracleB
    pub oracle_b: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
}

impl<'a> PlaceordersAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 14] {
        [
            InstructionAccount::readonly_signer(self.signer.address()),
            InstructionAccount::writable(self.open_orders_account.address()),
            InstructionAccount::readonly_signer(self.open_orders_admin.address()),
            InstructionAccount::writable(self.user_quote_account.address()),
            InstructionAccount::writable(self.user_base_account.address()),
            InstructionAccount::writable(self.market.address()),
            InstructionAccount::writable(self.bids.address()),
            InstructionAccount::writable(self.asks.address()),
            InstructionAccount::writable(self.event_heap.address()),
            InstructionAccount::writable(self.market_quote_vault.address()),
            InstructionAccount::writable(self.market_base_vault.address()),
            InstructionAccount::readonly(self.oracle_a.address()),
            InstructionAccount::readonly(self.oracle_b.address()),
            InstructionAccount::readonly(self.token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 14] {
        [
            self.signer, self.open_orders_account, self.open_orders_admin, self.user_quote_account, self.user_base_account, self.market, self.bids, self.asks, self.event_heap, self.market_quote_vault, self.market_base_vault, self.oracle_a, self.oracle_b, self.token_program
        ]
    }
}

/// Accounts for `cancelAllAndPlaceOrders`
pub struct CancelallandplaceordersAccounts<'a> {
    /// signer
    pub signer: &'a AccountView,
    /// openOrdersAccount
    pub open_orders_account: &'a AccountView,
    /// openOrdersAdmin
    pub open_orders_admin: &'a AccountView,
    /// userQuoteAccount
    pub user_quote_account: &'a AccountView,
    /// userBaseAccount
    pub user_base_account: &'a AccountView,
    /// market
    pub market: &'a AccountView,
    /// bids
    pub bids: &'a AccountView,
    /// asks
    pub asks: &'a AccountView,
    /// eventHeap
    pub event_heap: &'a AccountView,
    /// marketQuoteVault
    pub market_quote_vault: &'a AccountView,
    /// marketBaseVault
    pub market_base_vault: &'a AccountView,
    /// oracleA
    pub oracle_a: &'a AccountView,
    /// oracleB
    pub oracle_b: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
}

impl<'a> CancelallandplaceordersAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 14] {
        [
            InstructionAccount::readonly_signer(self.signer.address()),
            InstructionAccount::writable(self.open_orders_account.address()),
            InstructionAccount::readonly_signer(self.open_orders_admin.address()),
            InstructionAccount::writable(self.user_quote_account.address()),
            InstructionAccount::writable(self.user_base_account.address()),
            InstructionAccount::writable(self.market.address()),
            InstructionAccount::writable(self.bids.address()),
            InstructionAccount::writable(self.asks.address()),
            InstructionAccount::writable(self.event_heap.address()),
            InstructionAccount::writable(self.market_quote_vault.address()),
            InstructionAccount::writable(self.market_base_vault.address()),
            InstructionAccount::readonly(self.oracle_a.address()),
            InstructionAccount::readonly(self.oracle_b.address()),
            InstructionAccount::readonly(self.token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 14] {
        [
            self.signer, self.open_orders_account, self.open_orders_admin, self.user_quote_account, self.user_base_account, self.market, self.bids, self.asks, self.event_heap, self.market_quote_vault, self.market_base_vault, self.oracle_a, self.oracle_b, self.token_program
        ]
    }
}

/// Accounts for `placeOrderPegged`
pub struct PlaceorderpeggedAccounts<'a> {
    /// signer
    pub signer: &'a AccountView,
    /// openOrdersAccount
    pub open_orders_account: &'a AccountView,
    /// openOrdersAdmin
    pub open_orders_admin: &'a AccountView,
    /// userTokenAccount
    pub user_token_account: &'a AccountView,
    /// market
    pub market: &'a AccountView,
    /// bids
    pub bids: &'a AccountView,
    /// asks
    pub asks: &'a AccountView,
    /// eventHeap
    pub event_heap: &'a AccountView,
    /// marketVault
    pub market_vault: &'a AccountView,
    /// oracleA
    pub oracle_a: &'a AccountView,
    /// oracleB
    pub oracle_b: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
}

impl<'a> PlaceorderpeggedAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 12] {
        [
            InstructionAccount::readonly_signer(self.signer.address()),
            InstructionAccount::writable(self.open_orders_account.address()),
            InstructionAccount::readonly_signer(self.open_orders_admin.address()),
            InstructionAccount::writable(self.user_token_account.address()),
            InstructionAccount::writable(self.market.address()),
            InstructionAccount::writable(self.bids.address()),
            InstructionAccount::writable(self.asks.address()),
            InstructionAccount::writable(self.event_heap.address()),
            InstructionAccount::writable(self.market_vault.address()),
            InstructionAccount::readonly(self.oracle_a.address()),
            InstructionAccount::readonly(self.oracle_b.address()),
            InstructionAccount::readonly(self.token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 12] {
        [
            self.signer, self.open_orders_account, self.open_orders_admin, self.user_token_account, self.market, self.bids, self.asks, self.event_heap, self.market_vault, self.oracle_a, self.oracle_b, self.token_program
        ]
    }
}

/// Accounts for `placeTakeOrder`
pub struct PlacetakeorderAccounts<'a> {
    /// signer
    pub signer: &'a AccountView,
    /// penaltyPayer
    pub penalty_payer: &'a AccountView,
    /// market
    pub market: &'a AccountView,
    /// marketAuthority
    pub market_authority: &'a AccountView,
    /// bids
    pub bids: &'a AccountView,
    /// asks
    pub asks: &'a AccountView,
    /// marketBaseVault
    pub market_base_vault: &'a AccountView,
    /// marketQuoteVault
    pub market_quote_vault: &'a AccountView,
    /// eventHeap
    pub event_heap: &'a AccountView,
    /// userBaseAccount
    pub user_base_account: &'a AccountView,
    /// userQuoteAccount
    pub user_quote_account: &'a AccountView,
    /// oracleA
    pub oracle_a: &'a AccountView,
    /// oracleB
    pub oracle_b: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
    /// openOrdersAdmin
    pub open_orders_admin: &'a AccountView,
}

impl<'a> PlacetakeorderAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 16] {
        [
            InstructionAccount::writable_signer(self.signer.address()),
            InstructionAccount::writable_signer(self.penalty_payer.address()),
            InstructionAccount::writable(self.market.address()),
            InstructionAccount::readonly(self.market_authority.address()),
            InstructionAccount::writable(self.bids.address()),
            InstructionAccount::writable(self.asks.address()),
            InstructionAccount::writable(self.market_base_vault.address()),
            InstructionAccount::writable(self.market_quote_vault.address()),
            InstructionAccount::writable(self.event_heap.address()),
            InstructionAccount::writable(self.user_base_account.address()),
            InstructionAccount::writable(self.user_quote_account.address()),
            InstructionAccount::readonly(self.oracle_a.address()),
            InstructionAccount::readonly(self.oracle_b.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly_signer(self.open_orders_admin.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 16] {
        [
            self.signer, self.penalty_payer, self.market, self.market_authority, self.bids, self.asks, self.market_base_vault, self.market_quote_vault, self.event_heap, self.user_base_account, self.user_quote_account, self.oracle_a, self.oracle_b, self.token_program, self.system_program, self.open_orders_admin
        ]
    }
}

/// Accounts for `consumeEvents`
pub struct ConsumeeventsAccounts<'a> {
    /// consumeEventsAdmin
    pub consume_events_admin: &'a AccountView,
    /// market
    pub market: &'a AccountView,
    /// eventHeap
    pub event_heap: &'a AccountView,
}

impl<'a> ConsumeeventsAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 3] {
        [
            InstructionAccount::readonly_signer(self.consume_events_admin.address()),
            InstructionAccount::writable(self.market.address()),
            InstructionAccount::writable(self.event_heap.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 3] {
        [
            self.consume_events_admin, self.market, self.event_heap
        ]
    }
}

/// Accounts for `consumeGivenEvents`
pub struct ConsumegiveneventsAccounts<'a> {
    /// consumeEventsAdmin
    pub consume_events_admin: &'a AccountView,
    /// market
    pub market: &'a AccountView,
    /// eventHeap
    pub event_heap: &'a AccountView,
}

impl<'a> ConsumegiveneventsAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 3] {
        [
            InstructionAccount::readonly_signer(self.consume_events_admin.address()),
            InstructionAccount::writable(self.market.address()),
            InstructionAccount::writable(self.event_heap.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 3] {
        [
            self.consume_events_admin, self.market, self.event_heap
        ]
    }
}

/// Accounts for `cancelOrder`
pub struct CancelorderAccounts<'a> {
    /// signer
    pub signer: &'a AccountView,
    /// openOrdersAccount
    pub open_orders_account: &'a AccountView,
    /// market
    pub market: &'a AccountView,
    /// bids
    pub bids: &'a AccountView,
    /// asks
    pub asks: &'a AccountView,
}

impl<'a> CancelorderAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 5] {
        [
            InstructionAccount::readonly_signer(self.signer.address()),
            InstructionAccount::writable(self.open_orders_account.address()),
            InstructionAccount::readonly(self.market.address()),
            InstructionAccount::writable(self.bids.address()),
            InstructionAccount::writable(self.asks.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 5] {
        [
            self.signer, self.open_orders_account, self.market, self.bids, self.asks
        ]
    }
}

/// Accounts for `cancelOrderByClientOrderId`
pub struct CancelorderbyclientorderidAccounts<'a> {
    /// signer
    pub signer: &'a AccountView,
    /// openOrdersAccount
    pub open_orders_account: &'a AccountView,
    /// market
    pub market: &'a AccountView,
    /// bids
    pub bids: &'a AccountView,
    /// asks
    pub asks: &'a AccountView,
}

impl<'a> CancelorderbyclientorderidAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 5] {
        [
            InstructionAccount::readonly_signer(self.signer.address()),
            InstructionAccount::writable(self.open_orders_account.address()),
            InstructionAccount::readonly(self.market.address()),
            InstructionAccount::writable(self.bids.address()),
            InstructionAccount::writable(self.asks.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 5] {
        [
            self.signer, self.open_orders_account, self.market, self.bids, self.asks
        ]
    }
}

/// Accounts for `cancelAllOrders`
pub struct CancelallordersAccounts<'a> {
    /// signer
    pub signer: &'a AccountView,
    /// openOrdersAccount
    pub open_orders_account: &'a AccountView,
    /// market
    pub market: &'a AccountView,
    /// bids
    pub bids: &'a AccountView,
    /// asks
    pub asks: &'a AccountView,
}

impl<'a> CancelallordersAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 5] {
        [
            InstructionAccount::readonly_signer(self.signer.address()),
            InstructionAccount::writable(self.open_orders_account.address()),
            InstructionAccount::readonly(self.market.address()),
            InstructionAccount::writable(self.bids.address()),
            InstructionAccount::writable(self.asks.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 5] {
        [
            self.signer, self.open_orders_account, self.market, self.bids, self.asks
        ]
    }
}

/// Accounts for `deposit`
pub struct DepositAccounts<'a> {
    /// owner
    pub owner: &'a AccountView,
    /// userBaseAccount
    pub user_base_account: &'a AccountView,
    /// userQuoteAccount
    pub user_quote_account: &'a AccountView,
    /// openOrdersAccount
    pub open_orders_account: &'a AccountView,
    /// market
    pub market: &'a AccountView,
    /// marketBaseVault
    pub market_base_vault: &'a AccountView,
    /// marketQuoteVault
    pub market_quote_vault: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
}

impl<'a> DepositAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 8] {
        [
            InstructionAccount::readonly_signer(self.owner.address()),
            InstructionAccount::writable(self.user_base_account.address()),
            InstructionAccount::writable(self.user_quote_account.address()),
            InstructionAccount::writable(self.open_orders_account.address()),
            InstructionAccount::writable(self.market.address()),
            InstructionAccount::writable(self.market_base_vault.address()),
            InstructionAccount::writable(self.market_quote_vault.address()),
            InstructionAccount::readonly(self.token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 8] {
        [
            self.owner, self.user_base_account, self.user_quote_account, self.open_orders_account, self.market, self.market_base_vault, self.market_quote_vault, self.token_program
        ]
    }
}

/// Accounts for `refill`
pub struct RefillAccounts<'a> {
    /// owner
    pub owner: &'a AccountView,
    /// userBaseAccount
    pub user_base_account: &'a AccountView,
    /// userQuoteAccount
    pub user_quote_account: &'a AccountView,
    /// openOrdersAccount
    pub open_orders_account: &'a AccountView,
    /// market
    pub market: &'a AccountView,
    /// marketBaseVault
    pub market_base_vault: &'a AccountView,
    /// marketQuoteVault
    pub market_quote_vault: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
}

impl<'a> RefillAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 8] {
        [
            InstructionAccount::readonly_signer(self.owner.address()),
            InstructionAccount::writable(self.user_base_account.address()),
            InstructionAccount::writable(self.user_quote_account.address()),
            InstructionAccount::writable(self.open_orders_account.address()),
            InstructionAccount::writable(self.market.address()),
            InstructionAccount::writable(self.market_base_vault.address()),
            InstructionAccount::writable(self.market_quote_vault.address()),
            InstructionAccount::readonly(self.token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 8] {
        [
            self.owner, self.user_base_account, self.user_quote_account, self.open_orders_account, self.market, self.market_base_vault, self.market_quote_vault, self.token_program
        ]
    }
}

/// Accounts for `settleFunds`
pub struct SettlefundsAccounts<'a> {
    /// owner
    pub owner: &'a AccountView,
    /// penaltyPayer
    pub penalty_payer: &'a AccountView,
    /// openOrdersAccount
    pub open_orders_account: &'a AccountView,
    /// market
    pub market: &'a AccountView,
    /// marketAuthority
    pub market_authority: &'a AccountView,
    /// marketBaseVault
    pub market_base_vault: &'a AccountView,
    /// marketQuoteVault
    pub market_quote_vault: &'a AccountView,
    /// userBaseAccount
    pub user_base_account: &'a AccountView,
    /// userQuoteAccount
    pub user_quote_account: &'a AccountView,
    /// referrerAccount
    pub referrer_account: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
}

impl<'a> SettlefundsAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 12] {
        [
            InstructionAccount::writable_signer(self.owner.address()),
            InstructionAccount::writable_signer(self.penalty_payer.address()),
            InstructionAccount::writable(self.open_orders_account.address()),
            InstructionAccount::writable(self.market.address()),
            InstructionAccount::readonly(self.market_authority.address()),
            InstructionAccount::writable(self.market_base_vault.address()),
            InstructionAccount::writable(self.market_quote_vault.address()),
            InstructionAccount::writable(self.user_base_account.address()),
            InstructionAccount::writable(self.user_quote_account.address()),
            InstructionAccount::writable(self.referrer_account.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 12] {
        [
            self.owner, self.penalty_payer, self.open_orders_account, self.market, self.market_authority, self.market_base_vault, self.market_quote_vault, self.user_base_account, self.user_quote_account, self.referrer_account, self.token_program, self.system_program
        ]
    }
}

/// Accounts for `settleFundsExpired`
pub struct SettlefundsexpiredAccounts<'a> {
    /// closeMarketAdmin
    pub close_market_admin: &'a AccountView,
    /// owner
    pub owner: &'a AccountView,
    /// penaltyPayer
    pub penalty_payer: &'a AccountView,
    /// openOrdersAccount
    pub open_orders_account: &'a AccountView,
    /// market
    pub market: &'a AccountView,
    /// marketAuthority
    pub market_authority: &'a AccountView,
    /// marketBaseVault
    pub market_base_vault: &'a AccountView,
    /// marketQuoteVault
    pub market_quote_vault: &'a AccountView,
    /// userBaseAccount
    pub user_base_account: &'a AccountView,
    /// userQuoteAccount
    pub user_quote_account: &'a AccountView,
    /// referrerAccount
    pub referrer_account: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
}

impl<'a> SettlefundsexpiredAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 13] {
        [
            InstructionAccount::readonly_signer(self.close_market_admin.address()),
            InstructionAccount::writable_signer(self.owner.address()),
            InstructionAccount::writable_signer(self.penalty_payer.address()),
            InstructionAccount::writable(self.open_orders_account.address()),
            InstructionAccount::writable(self.market.address()),
            InstructionAccount::readonly(self.market_authority.address()),
            InstructionAccount::writable(self.market_base_vault.address()),
            InstructionAccount::writable(self.market_quote_vault.address()),
            InstructionAccount::writable(self.user_base_account.address()),
            InstructionAccount::writable(self.user_quote_account.address()),
            InstructionAccount::writable(self.referrer_account.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 13] {
        [
            self.close_market_admin, self.owner, self.penalty_payer, self.open_orders_account, self.market, self.market_authority, self.market_base_vault, self.market_quote_vault, self.user_base_account, self.user_quote_account, self.referrer_account, self.token_program, self.system_program
        ]
    }
}

/// Accounts for `sweepFees`
pub struct SweepfeesAccounts<'a> {
    /// collectFeeAdmin
    pub collect_fee_admin: &'a AccountView,
    /// market
    pub market: &'a AccountView,
    /// marketAuthority
    pub market_authority: &'a AccountView,
    /// marketQuoteVault
    pub market_quote_vault: &'a AccountView,
    /// tokenReceiverAccount
    pub token_receiver_account: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
}

impl<'a> SweepfeesAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 6] {
        [
            InstructionAccount::readonly_signer(self.collect_fee_admin.address()),
            InstructionAccount::writable(self.market.address()),
            InstructionAccount::readonly(self.market_authority.address()),
            InstructionAccount::writable(self.market_quote_vault.address()),
            InstructionAccount::writable(self.token_receiver_account.address()),
            InstructionAccount::readonly(self.token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 6] {
        [
            self.collect_fee_admin, self.market, self.market_authority, self.market_quote_vault, self.token_receiver_account, self.token_program
        ]
    }
}

/// Accounts for `setDelegate`
pub struct SetdelegateAccounts<'a> {
    /// owner
    pub owner: &'a AccountView,
    /// openOrdersAccount
    pub open_orders_account: &'a AccountView,
    /// delegateAccount
    pub delegate_account: &'a AccountView,
}

impl<'a> SetdelegateAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 3] {
        [
            InstructionAccount::writable_signer(self.owner.address()),
            InstructionAccount::writable(self.open_orders_account.address()),
            InstructionAccount::readonly(self.delegate_account.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 3] {
        [
            self.owner, self.open_orders_account, self.delegate_account
        ]
    }
}

/// Accounts for `setMarketExpired`
pub struct SetmarketexpiredAccounts<'a> {
    /// closeMarketAdmin
    pub close_market_admin: &'a AccountView,
    /// market
    pub market: &'a AccountView,
}

impl<'a> SetmarketexpiredAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 2] {
        [
            InstructionAccount::readonly_signer(self.close_market_admin.address()),
            InstructionAccount::writable(self.market.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 2] {
        [
            self.close_market_admin, self.market
        ]
    }
}

/// Accounts for `pruneOrders`
pub struct PruneordersAccounts<'a> {
    /// closeMarketAdmin
    pub close_market_admin: &'a AccountView,
    /// openOrdersAccount
    pub open_orders_account: &'a AccountView,
    /// market
    pub market: &'a AccountView,
    /// bids
    pub bids: &'a AccountView,
    /// asks
    pub asks: &'a AccountView,
}

impl<'a> PruneordersAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 5] {
        [
            InstructionAccount::readonly_signer(self.close_market_admin.address()),
            InstructionAccount::writable(self.open_orders_account.address()),
            InstructionAccount::readonly(self.market.address()),
            InstructionAccount::writable(self.bids.address()),
            InstructionAccount::writable(self.asks.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 5] {
        [
            self.close_market_admin, self.open_orders_account, self.market, self.bids, self.asks
        ]
    }
}

/// Accounts for `stubOracleCreate`
pub struct StuboraclecreateAccounts<'a> {
    /// payer
    pub payer: &'a AccountView,
    /// owner
    pub owner: &'a AccountView,
    /// oracle
    pub oracle: &'a AccountView,
    /// mint
    pub mint: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
}

impl<'a> StuboraclecreateAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 5] {
        [
            InstructionAccount::writable_signer(self.payer.address()),
            InstructionAccount::readonly_signer(self.owner.address()),
            InstructionAccount::writable(self.oracle.address()),
            InstructionAccount::readonly(self.mint.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 5] {
        [
            self.payer, self.owner, self.oracle, self.mint, self.system_program
        ]
    }
}

/// Accounts for `stubOracleClose`
pub struct StuboraclecloseAccounts<'a> {
    /// owner
    pub owner: &'a AccountView,
    /// oracle
    pub oracle: &'a AccountView,
    /// solDestination
    pub sol_destination: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
}

impl<'a> StuboraclecloseAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 4] {
        [
            InstructionAccount::readonly_signer(self.owner.address()),
            InstructionAccount::writable(self.oracle.address()),
            InstructionAccount::writable(self.sol_destination.address()),
            InstructionAccount::readonly(self.token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 4] {
        [
            self.owner, self.oracle, self.sol_destination, self.token_program
        ]
    }
}

/// Accounts for `stubOracleSet`
pub struct StuboraclesetAccounts<'a> {
    /// owner
    pub owner: &'a AccountView,
    /// oracle
    pub oracle: &'a AccountView,
}

impl<'a> StuboraclesetAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 2] {
        [
            InstructionAccount::readonly_signer(self.owner.address()),
            InstructionAccount::writable(self.oracle.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 2] {
        [
            self.owner, self.oracle
        ]
    }
}


// ============================================
// CPI Functions
// ============================================
/// CPI: createMarket
#[inline(always)]
pub fn create_market<'a>(
    accounts: &CreatemarketAccounts<'a>, args: &CreatemarketArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<CreatemarketArgs>()];
    data[0..8].copy_from_slice(&CREATE_MARKET);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const CreatemarketArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<CreatemarketArgs>(),
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

/// CPI: closeMarket
#[inline(always)]
pub fn close_market<'a>(
    accounts: &ClosemarketAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&CLOSE_MARKET);
    
    
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

/// CPI: createOpenOrdersIndexer
#[inline(always)]
pub fn create_open_orders_indexer<'a>(
    accounts: &CreateopenordersindexerAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&CREATE_OPEN_ORDERS_INDEXER);
    
    
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

/// CPI: closeOpenOrdersIndexer
#[inline(always)]
pub fn close_open_orders_indexer<'a>(
    accounts: &CloseopenordersindexerAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&CLOSE_OPEN_ORDERS_INDEXER);
    
    
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

/// CPI: createOpenOrdersAccount
#[inline(always)]
pub fn create_open_orders_account<'a>(
    accounts: &CreateopenordersaccountAccounts<'a>, args: &CreateopenordersaccountArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<CreateopenordersaccountArgs>()];
    data[0..8].copy_from_slice(&CREATE_OPEN_ORDERS_ACCOUNT);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const CreateopenordersaccountArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<CreateopenordersaccountArgs>(),
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

/// CPI: closeOpenOrdersAccount
#[inline(always)]
pub fn close_open_orders_account<'a>(
    accounts: &CloseopenordersaccountAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&CLOSE_OPEN_ORDERS_ACCOUNT);
    
    
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
    invoke_signed::<5>(&instruction, &account_views, signers)
}

/// CPI: placeOrder
#[inline(always)]
pub fn place_order<'a>(
    accounts: &PlaceorderAccounts<'a>, args: &PlaceorderArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<PlaceorderArgs>()];
    data[0..8].copy_from_slice(&PLACE_ORDER);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const PlaceorderArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<PlaceorderArgs>(),
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

/// CPI: editOrder
#[inline(always)]
pub fn edit_order<'a>(
    accounts: &EditorderAccounts<'a>, args: &EditorderArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<EditorderArgs>()];
    data[0..8].copy_from_slice(&EDIT_ORDER);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const EditorderArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<EditorderArgs>(),
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

/// CPI: editOrderPegged
#[inline(always)]
pub fn edit_order_pegged<'a>(
    accounts: &EditorderpeggedAccounts<'a>, args: &EditorderpeggedArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<EditorderpeggedArgs>()];
    data[0..8].copy_from_slice(&EDIT_ORDER_PEGGED);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const EditorderpeggedArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<EditorderpeggedArgs>(),
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

/// CPI: placeOrders
#[inline(always)]
pub fn place_orders<'a>(
    accounts: &PlaceordersAccounts<'a>, args: &PlaceordersArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<PlaceordersArgs>()];
    data[0..8].copy_from_slice(&PLACE_ORDERS);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const PlaceordersArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<PlaceordersArgs>(),
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

/// CPI: cancelAllAndPlaceOrders
#[inline(always)]
pub fn cancel_all_and_place_orders<'a>(
    accounts: &CancelallandplaceordersAccounts<'a>, args: &CancelallandplaceordersArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<CancelallandplaceordersArgs>()];
    data[0..8].copy_from_slice(&CANCEL_ALL_AND_PLACE_ORDERS);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const CancelallandplaceordersArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<CancelallandplaceordersArgs>(),
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

/// CPI: placeOrderPegged
#[inline(always)]
pub fn place_order_pegged<'a>(
    accounts: &PlaceorderpeggedAccounts<'a>, args: &PlaceorderpeggedArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<PlaceorderpeggedArgs>()];
    data[0..8].copy_from_slice(&PLACE_ORDER_PEGGED);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const PlaceorderpeggedArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<PlaceorderpeggedArgs>(),
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

/// CPI: placeTakeOrder
#[inline(always)]
pub fn place_take_order<'a>(
    accounts: &PlacetakeorderAccounts<'a>, args: &PlacetakeorderArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<PlacetakeorderArgs>()];
    data[0..8].copy_from_slice(&PLACE_TAKE_ORDER);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const PlacetakeorderArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<PlacetakeorderArgs>(),
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

/// CPI: consumeEvents
#[inline(always)]
pub fn consume_events<'a>(
    accounts: &ConsumeeventsAccounts<'a>, args: &ConsumeeventsArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<ConsumeeventsArgs>()];
    data[0..8].copy_from_slice(&CONSUME_EVENTS);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const ConsumeeventsArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<ConsumeeventsArgs>(),
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

/// CPI: consumeGivenEvents
#[inline(always)]
pub fn consume_given_events<'a>(
    accounts: &ConsumegiveneventsAccounts<'a>, args: &ConsumegiveneventsArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<ConsumegiveneventsArgs>()];
    data[0..8].copy_from_slice(&CONSUME_GIVEN_EVENTS);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const ConsumegiveneventsArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<ConsumegiveneventsArgs>(),
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

/// CPI: cancelOrder
#[inline(always)]
pub fn cancel_order<'a>(
    accounts: &CancelorderAccounts<'a>, args: &CancelorderArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<CancelorderArgs>()];
    data[0..8].copy_from_slice(&CANCEL_ORDER);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const CancelorderArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<CancelorderArgs>(),
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
    invoke_signed::<5>(&instruction, &account_views, signers)
}

/// CPI: cancelOrderByClientOrderId
#[inline(always)]
pub fn cancel_order_by_client_order_id<'a>(
    accounts: &CancelorderbyclientorderidAccounts<'a>, args: &CancelorderbyclientorderidArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<CancelorderbyclientorderidArgs>()];
    data[0..8].copy_from_slice(&CANCEL_ORDER_BY_CLIENT_ORDER_ID);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const CancelorderbyclientorderidArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<CancelorderbyclientorderidArgs>(),
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
    invoke_signed::<5>(&instruction, &account_views, signers)
}

/// CPI: cancelAllOrders
#[inline(always)]
pub fn cancel_all_orders<'a>(
    accounts: &CancelallordersAccounts<'a>, args: &CancelallordersArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<CancelallordersArgs>()];
    data[0..8].copy_from_slice(&CANCEL_ALL_ORDERS);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const CancelallordersArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<CancelallordersArgs>(),
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
    invoke_signed::<5>(&instruction, &account_views, signers)
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
    invoke_signed::<8>(&instruction, &account_views, signers)
}

/// CPI: refill
#[inline(always)]
pub fn refill<'a>(
    accounts: &RefillAccounts<'a>, args: &RefillArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<RefillArgs>()];
    data[0..8].copy_from_slice(&REFILL);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const RefillArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<RefillArgs>(),
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

/// CPI: settleFunds
#[inline(always)]
pub fn settle_funds<'a>(
    accounts: &SettlefundsAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&SETTLE_FUNDS);
    
    
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

/// CPI: settleFundsExpired
#[inline(always)]
pub fn settle_funds_expired<'a>(
    accounts: &SettlefundsexpiredAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&SETTLE_FUNDS_EXPIRED);
    
    
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

/// CPI: sweepFees
#[inline(always)]
pub fn sweep_fees<'a>(
    accounts: &SweepfeesAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&SWEEP_FEES);
    
    
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

/// CPI: setDelegate
#[inline(always)]
pub fn set_delegate<'a>(
    accounts: &SetdelegateAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&SET_DELEGATE);
    
    
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

/// CPI: setMarketExpired
#[inline(always)]
pub fn set_market_expired<'a>(
    accounts: &SetmarketexpiredAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&SET_MARKET_EXPIRED);
    
    
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

/// CPI: pruneOrders
#[inline(always)]
pub fn prune_orders<'a>(
    accounts: &PruneordersAccounts<'a>, args: &PruneordersArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<PruneordersArgs>()];
    data[0..8].copy_from_slice(&PRUNE_ORDERS);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const PruneordersArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<PruneordersArgs>(),
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
    invoke_signed::<5>(&instruction, &account_views, signers)
}

/// CPI: stubOracleCreate
#[inline(always)]
pub fn stub_oracle_create<'a>(
    accounts: &StuboraclecreateAccounts<'a>, args: &StuboraclecreateArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<StuboraclecreateArgs>()];
    data[0..8].copy_from_slice(&STUB_ORACLE_CREATE);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const StuboraclecreateArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<StuboraclecreateArgs>(),
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
    invoke_signed::<5>(&instruction, &account_views, signers)
}

/// CPI: stubOracleClose
#[inline(always)]
pub fn stub_oracle_close<'a>(
    accounts: &StuboraclecloseAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&STUB_ORACLE_CLOSE);
    
    
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

/// CPI: stubOracleSet
#[inline(always)]
pub fn stub_oracle_set<'a>(
    accounts: &StuboraclesetAccounts<'a>, args: &StuboraclesetArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<StuboraclesetArgs>()];
    data[0..8].copy_from_slice(&STUB_ORACLE_SET);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const StuboraclesetArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<StuboraclesetArgs>(),
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

