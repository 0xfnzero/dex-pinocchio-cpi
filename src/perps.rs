//! CPI module for Perps
//!
//! Program: perpetuals
//! Program ID: PERPHjGBqRHArX4DySjwM6UJHiR3sWAatqfdBS2qQJu
//! Instructions: 59

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
pub const ID: Address = Address::new_from_array(five8_const::decode_32_const("PERPHjGBqRHArX4DySjwM6UJHiR3sWAatqfdBS2qQJu"));

// ============================================
// Instruction Discriminators
// ============================================
pub const INIT: [u8; 8] = [220, 59, 207, 236, 108, 250, 47, 100];
pub const ADD_POOL: [u8; 8] = [115, 230, 212, 211, 175, 49, 39, 169];
pub const ADD_CUSTODY: [u8; 8] = [247, 254, 126, 17, 26, 6, 215, 117];
pub const SET_CUSTODY_CONFIG: [u8; 8] = [133, 97, 130, 143, 215, 229, 36, 176];
pub const SET_POOL_CONFIG: [u8; 8] = [216, 87, 65, 125, 113, 110, 185, 120];
pub const SET_PERPETUALS_CONFIG: [u8; 8] = [80, 72, 21, 191, 29, 121, 45, 111];
pub const TRANSFER_ADMIN: [u8; 8] = [42, 242, 66, 106, 228, 10, 111, 156];
pub const WITHDRAW_FEES2: [u8; 8] = [252, 128, 143, 145, 225, 221, 159, 207];
pub const CREATE_TOKEN_METADATA: [u8; 8] = [221, 80, 176, 37, 153, 188, 160, 68];
pub const CREATE_TOKEN_LEDGER: [u8; 8] = [232, 242, 197, 253, 240, 143, 129, 52];
pub const REALLOC_CUSTODY: [u8; 8] = [123, 58, 109, 139, 133, 7, 225, 200];
pub const REALLOC_POOL: [u8; 8] = [114, 128, 37, 167, 71, 227, 40, 178];
pub const CREATE_AND_DELEGATE_STAKE_ACCOUNT: [u8; 8] = [98, 209, 122, 27, 222, 137, 94, 134];
pub const UNSTAKE: [u8; 8] = [90, 95, 107, 42, 205, 124, 50, 225];
pub const WITHDRAW_STAKE: [u8; 8] = [153, 8, 22, 138, 105, 176, 87, 66];
pub const REDEEM_STAKE: [u8; 8] = [178, 203, 250, 105, 133, 118, 255, 69];
pub const OPERATOR_SET_CUSTODY_CONFIG: [u8; 8] = [166, 137, 92, 204, 145, 224, 24, 218];
pub const OPERATOR_SET_POOL_CONFIG: [u8; 8] = [76, 201, 80, 18, 199, 92, 246, 105];
pub const TEST_INIT: [u8; 8] = [48, 51, 92, 122, 81, 19, 112, 41];
pub const SET_TEST_TIME: [u8; 8] = [242, 231, 177, 251, 126, 145, 159, 104];
pub const SET_TOKEN_LEDGER: [u8; 8] = [228, 85, 185, 112, 78, 79, 77, 2];
pub const SWAP2: [u8; 8] = [65, 75, 63, 76, 235, 91, 91, 136];
pub const SWAP_WITH_TOKEN_LEDGER: [u8; 8] = [139, 141, 238, 197, 41, 211, 172, 19];
pub const INSTANT_INCREASE_POSITION_PRE_SWAP: [u8; 8] = [197, 38, 86, 165, 199, 23, 38, 234];
pub const ADD_LIQUIDITY2: [u8; 8] = [228, 162, 78, 28, 70, 219, 116, 115];
pub const REMOVE_LIQUIDITY2: [u8; 8] = [230, 215, 82, 127, 241, 101, 227, 146];
pub const CREATE_INCREASE_POSITION_MARKET_REQUEST: [u8; 8] = [184, 85, 199, 24, 105, 171, 156, 56];
pub const CREATE_DECREASE_POSITION_REQUEST2: [u8; 8] = [105, 64, 201, 82, 250, 14, 109, 77];
pub const CREATE_DECREASE_POSITION_MARKET_REQUEST: [u8; 8] = [74, 198, 195, 86, 193, 99, 1, 79];
pub const UPDATE_DECREASE_POSITION_REQUEST2: [u8; 8] = [144, 200, 249, 255, 108, 217, 249, 116];
pub const CLOSE_POSITION_REQUEST2: [u8; 8] = [121, 68, 162, 28, 216, 47, 200, 66];
pub const CLOSE_POSITION_REQUEST3: [u8; 8] = [122, 130, 33, 18, 211, 44, 161, 58];
pub const INCREASE_POSITION4: [u8; 8] = [67, 147, 53, 23, 43, 57, 16, 67];
pub const INCREASE_POSITION_PRE_SWAP: [u8; 8] = [26, 136, 225, 217, 22, 21, 83, 20];
pub const INCREASE_POSITION_WITH_INTERNAL_SWAP: [u8; 8] = [114, 55, 106, 140, 199, 221, 32, 112];
pub const DECREASE_POSITION4: [u8; 8] = [185, 161, 114, 175, 96, 148, 3, 170];
pub const DECREASE_POSITION_WITH_INTERNAL_SWAP: [u8; 8] = [131, 17, 153, 110, 119, 100, 97, 38];
pub const DECREASE_POSITION_WITH_TPSL: [u8; 8] = [108, 18, 203, 209, 227, 103, 65, 165];
pub const DECREASE_POSITION_WITH_TPSL_AND_INTERNAL_SWAP: [u8; 8] = [2, 111, 200, 231, 35, 65, 123, 235];
pub const LIQUIDATE_FULL_POSITION4: [u8; 8] = [64, 176, 88, 51, 168, 188, 156, 175];
pub const REFRESH_ASSETS_UNDER_MANAGEMENT: [u8; 8] = [162, 0, 215, 55, 225, 15, 185, 0];
pub const SET_MAX_GLOBAL_SIZES: [u8; 8] = [89, 2, 210, 24, 167, 227, 13, 214];
pub const INSTANT_CREATE_TPSL: [u8; 8] = [117, 98, 66, 127, 30, 50, 73, 185];
pub const INSTANT_CREATE_LIMIT_ORDER: [u8; 8] = [194, 37, 195, 123, 40, 127, 126, 156];
pub const INSTANT_INCREASE_POSITION: [u8; 8] = [164, 126, 68, 182, 223, 166, 64, 183];
pub const INSTANT_DECREASE_POSITION: [u8; 8] = [46, 23, 240, 44, 30, 138, 94, 140];
pub const INSTANT_DECREASE_POSITION2: [u8; 8] = [162, 191, 200, 62, 139, 62, 176, 17];
pub const INSTANT_UPDATE_LIMIT_ORDER: [u8; 8] = [136, 245, 229, 58, 121, 141, 12, 207];
pub const INSTANT_UPDATE_TPSL: [u8; 8] = [144, 228, 114, 37, 165, 242, 111, 101];
pub const GET_ADD_LIQUIDITY_AMOUNT_AND_FEE2: [u8; 8] = [109, 157, 55, 169, 8, 81, 4, 118];
pub const GET_REMOVE_LIQUIDITY_AMOUNT_AND_FEE2: [u8; 8] = [183, 59, 72, 110, 223, 243, 150, 142];
pub const GET_ASSETS_UNDER_MANAGEMENT2: [u8; 8] = [193, 210, 13, 249, 113, 149, 29, 84];
pub const BORROW_FROM_CUSTODY: [u8; 8] = [153, 183, 65, 65, 113, 33, 249, 45];
pub const REPAY_TO_CUSTODY: [u8; 8] = [211, 219, 183, 222, 248, 74, 5, 26];
pub const DEPOSIT_COLLATERAL_FOR_BORROWS: [u8; 8] = [17, 2, 195, 190, 76, 16, 238, 74];
pub const WITHDRAW_COLLATERAL_FOR_BORROWS: [u8; 8] = [117, 160, 60, 82, 237, 233, 46, 182];
pub const LIQUIDATE_BORROW_POSITION: [u8; 8] = [235, 201, 17, 133, 234, 72, 84, 210];
pub const PARTIAL_LIQUIDATE_BORROW_POSITION: [u8; 8] = [250, 166, 13, 74, 97, 204, 130, 209];
pub const CLOSE_BORROW_POSITION: [u8; 8] = [204, 226, 145, 205, 232, 37, 3, 140];

// ============================================
// Instruction Arguments
// ============================================
/// Arguments for `init`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct InitArgs {
    pub params: [u8; 32],
}

/// Arguments for `addPool`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct AddpoolArgs {
    pub params: [u8; 32],
}

/// Arguments for `addCustody`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct AddcustodyArgs {
    pub params: [u8; 32],
}

/// Arguments for `setCustodyConfig`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SetcustodyconfigArgs {
    pub params: [u8; 32],
}

/// Arguments for `setPoolConfig`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SetpoolconfigArgs {
    pub params: [u8; 32],
}

/// Arguments for `setPerpetualsConfig`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SetperpetualsconfigArgs {
    pub params: [u8; 32],
}

/// Arguments for `transferAdmin`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct TransferadminArgs {
    pub params: [u8; 32],
}

/// Arguments for `withdrawFees2`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct Withdrawfees2Args {
    pub params: [u8; 32],
}

/// Arguments for `createTokenMetadata`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct CreatetokenmetadataArgs {
    pub params: [u8; 32],
}

/// Arguments for `createAndDelegateStakeAccount`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct CreateanddelegatestakeaccountArgs {
    pub params: [u8; 32],
}

/// Arguments for `operatorSetCustodyConfig`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct OperatorsetcustodyconfigArgs {
    pub params: [u8; 32],
}

/// Arguments for `operatorSetPoolConfig`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct OperatorsetpoolconfigArgs {
    pub params: [u8; 32],
}

/// Arguments for `testInit`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct TestinitArgs {
    pub params: [u8; 32],
}

/// Arguments for `setTestTime`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SettesttimeArgs {
    pub params: [u8; 32],
}

/// Arguments for `swap2`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct Swap2Args {
    pub params: [u8; 32],
}

/// Arguments for `swapWithTokenLedger`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SwapwithtokenledgerArgs {
    pub params: [u8; 32],
}

/// Arguments for `instantIncreasePositionPreSwap`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct InstantincreasepositionpreswapArgs {
    pub params: [u8; 32],
}

/// Arguments for `addLiquidity2`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct Addliquidity2Args {
    pub params: [u8; 32],
}

/// Arguments for `removeLiquidity2`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct Removeliquidity2Args {
    pub params: [u8; 32],
}

/// Arguments for `createIncreasePositionMarketRequest`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct CreateincreasepositionmarketrequestArgs {
    pub params: [u8; 32],
}

/// Arguments for `createDecreasePositionRequest2`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct Createdecreasepositionrequest2Args {
    pub params: [u8; 32],
}

/// Arguments for `createDecreasePositionMarketRequest`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct CreatedecreasepositionmarketrequestArgs {
    pub params: [u8; 32],
}

/// Arguments for `updateDecreasePositionRequest2`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct Updatedecreasepositionrequest2Args {
    pub params: [u8; 32],
}

/// Arguments for `increasePosition4`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct Increaseposition4Args {
    pub params: [u8; 32],
}

/// Arguments for `increasePositionPreSwap`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct IncreasepositionpreswapArgs {
    pub params: [u8; 32],
}

/// Arguments for `increasePositionWithInternalSwap`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct IncreasepositionwithinternalswapArgs {
    pub params: [u8; 32],
}

/// Arguments for `decreasePosition4`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct Decreaseposition4Args {
    pub params: [u8; 32],
}

/// Arguments for `decreasePositionWithInternalSwap`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct DecreasepositionwithinternalswapArgs {
    pub params: [u8; 32],
}

/// Arguments for `decreasePositionWithTpsl`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct DecreasepositionwithtpslArgs {
    pub params: [u8; 32],
}

/// Arguments for `decreasePositionWithTpslAndInternalSwap`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct DecreasepositionwithtpslandinternalswapArgs {
    pub params: [u8; 32],
}

/// Arguments for `liquidateFullPosition4`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct Liquidatefullposition4Args {
    pub params: [u8; 32],
}

/// Arguments for `refreshAssetsUnderManagement`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct RefreshassetsundermanagementArgs {
    pub params: [u8; 32],
}

/// Arguments for `setMaxGlobalSizes`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SetmaxglobalsizesArgs {
    pub params: [u8; 32],
}

/// Arguments for `instantCreateTpsl`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct InstantcreatetpslArgs {
    pub params: [u8; 32],
}

/// Arguments for `instantCreateLimitOrder`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct InstantcreatelimitorderArgs {
    pub params: [u8; 32],
}

/// Arguments for `instantIncreasePosition`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct InstantincreasepositionArgs {
    pub params: [u8; 32],
}

/// Arguments for `instantDecreasePosition`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct InstantdecreasepositionArgs {
    pub params: [u8; 32],
}

/// Arguments for `instantDecreasePosition2`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct Instantdecreaseposition2Args {
    pub params: [u8; 32],
}

/// Arguments for `instantUpdateLimitOrder`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct InstantupdatelimitorderArgs {
    pub params: [u8; 32],
}

/// Arguments for `instantUpdateTpsl`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct InstantupdatetpslArgs {
    pub params: [u8; 32],
}

/// Arguments for `getAddLiquidityAmountAndFee2`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct Getaddliquidityamountandfee2Args {
    pub params: [u8; 32],
}

/// Arguments for `getRemoveLiquidityAmountAndFee2`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct Getremoveliquidityamountandfee2Args {
    pub params: [u8; 32],
}

/// Arguments for `getAssetsUnderManagement2`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct Getassetsundermanagement2Args {
    pub params: [u8; 32],
}

/// Arguments for `borrowFromCustody`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct BorrowfromcustodyArgs {
    pub params: [u8; 32],
}

/// Arguments for `repayToCustody`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct RepaytocustodyArgs {
    pub params: [u8; 32],
}

/// Arguments for `depositCollateralForBorrows`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct DepositcollateralforborrowsArgs {
    pub params: [u8; 32],
}

/// Arguments for `withdrawCollateralForBorrows`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct WithdrawcollateralforborrowsArgs {
    pub params: [u8; 32],
}

/// Arguments for `liquidateBorrowPosition`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct LiquidateborrowpositionArgs {
    pub params: [u8; 32],
}

/// Arguments for `partialLiquidateBorrowPosition`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct PartialliquidateborrowpositionArgs {
    pub params: [u8; 32],
}

/// Arguments for `closeBorrowPosition`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct CloseborrowpositionArgs {
    pub params: [u8; 32],
}


// ============================================
// Instruction Accounts
// ============================================
/// Accounts for `init`
pub struct InitAccounts<'a> {
    /// upgradeAuthority
    pub upgrade_authority: &'a AccountView,
    /// admin
    pub admin: &'a AccountView,
    /// transferAuthority
    pub transfer_authority: &'a AccountView,
    /// perpetuals
    pub perpetuals: &'a AccountView,
    /// perpetualsProgram
    pub perpetuals_program: &'a AccountView,
    /// perpetualsProgramData
    pub perpetuals_program_data: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
}

impl<'a> InitAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 8] {
        [
            InstructionAccount::writable_signer(self.upgrade_authority.address()),
            InstructionAccount::readonly(self.admin.address()),
            InstructionAccount::writable(self.transfer_authority.address()),
            InstructionAccount::writable(self.perpetuals.address()),
            InstructionAccount::readonly(self.perpetuals_program.address()),
            InstructionAccount::readonly(self.perpetuals_program_data.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 8] {
        [
            self.upgrade_authority, self.admin, self.transfer_authority, self.perpetuals, self.perpetuals_program, self.perpetuals_program_data, self.system_program, self.token_program
        ]
    }
}

/// Accounts for `addPool`
pub struct AddpoolAccounts<'a> {
    /// admin
    pub admin: &'a AccountView,
    /// transferAuthority
    pub transfer_authority: &'a AccountView,
    /// perpetuals
    pub perpetuals: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// lpTokenMint
    pub lp_token_mint: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// rent
    pub rent: &'a AccountView,
}

impl<'a> AddpoolAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 8] {
        [
            InstructionAccount::writable_signer(self.admin.address()),
            InstructionAccount::readonly(self.transfer_authority.address()),
            InstructionAccount::writable(self.perpetuals.address()),
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.lp_token_mint.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.rent.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 8] {
        [
            self.admin, self.transfer_authority, self.perpetuals, self.pool, self.lp_token_mint, self.system_program, self.token_program, self.rent
        ]
    }
}

/// Accounts for `addCustody`
pub struct AddcustodyAccounts<'a> {
    /// admin
    pub admin: &'a AccountView,
    /// transferAuthority
    pub transfer_authority: &'a AccountView,
    /// perpetuals
    pub perpetuals: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// custody
    pub custody: &'a AccountView,
    /// custodyTokenAccount
    pub custody_token_account: &'a AccountView,
    /// custodyTokenMint
    pub custody_token_mint: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// rent
    pub rent: &'a AccountView,
}

impl<'a> AddcustodyAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 10] {
        [
            InstructionAccount::writable_signer(self.admin.address()),
            InstructionAccount::readonly(self.transfer_authority.address()),
            InstructionAccount::readonly(self.perpetuals.address()),
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.custody.address()),
            InstructionAccount::writable(self.custody_token_account.address()),
            InstructionAccount::readonly(self.custody_token_mint.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.rent.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 10] {
        [
            self.admin, self.transfer_authority, self.perpetuals, self.pool, self.custody, self.custody_token_account, self.custody_token_mint, self.system_program, self.token_program, self.rent
        ]
    }
}

/// Accounts for `setCustodyConfig`
pub struct SetcustodyconfigAccounts<'a> {
    /// admin
    pub admin: &'a AccountView,
    /// perpetuals
    pub perpetuals: &'a AccountView,
    /// custody
    pub custody: &'a AccountView,
}

impl<'a> SetcustodyconfigAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 3] {
        [
            InstructionAccount::writable_signer(self.admin.address()),
            InstructionAccount::readonly(self.perpetuals.address()),
            InstructionAccount::writable(self.custody.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 3] {
        [
            self.admin, self.perpetuals, self.custody
        ]
    }
}

/// Accounts for `setPoolConfig`
pub struct SetpoolconfigAccounts<'a> {
    /// admin
    pub admin: &'a AccountView,
    /// perpetuals
    pub perpetuals: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
}

impl<'a> SetpoolconfigAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 3] {
        [
            InstructionAccount::readonly_signer(self.admin.address()),
            InstructionAccount::readonly(self.perpetuals.address()),
            InstructionAccount::writable(self.pool.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 3] {
        [
            self.admin, self.perpetuals, self.pool
        ]
    }
}

/// Accounts for `setPerpetualsConfig`
pub struct SetperpetualsconfigAccounts<'a> {
    /// admin
    pub admin: &'a AccountView,
    /// perpetuals
    pub perpetuals: &'a AccountView,
}

impl<'a> SetperpetualsconfigAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 2] {
        [
            InstructionAccount::readonly_signer(self.admin.address()),
            InstructionAccount::writable(self.perpetuals.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 2] {
        [
            self.admin, self.perpetuals
        ]
    }
}

/// Accounts for `transferAdmin`
pub struct TransferadminAccounts<'a> {
    /// admin
    pub admin: &'a AccountView,
    /// newAdmin
    pub new_admin: &'a AccountView,
    /// perpetuals
    pub perpetuals: &'a AccountView,
}

impl<'a> TransferadminAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 3] {
        [
            InstructionAccount::readonly_signer(self.admin.address()),
            InstructionAccount::readonly(self.new_admin.address()),
            InstructionAccount::writable(self.perpetuals.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 3] {
        [
            self.admin, self.new_admin, self.perpetuals
        ]
    }
}

/// Accounts for `withdrawFees2`
pub struct Withdrawfees2Accounts<'a> {
    /// keeper
    pub keeper: &'a AccountView,
    /// transferAuthority
    pub transfer_authority: &'a AccountView,
    /// perpetuals
    pub perpetuals: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// custody
    pub custody: &'a AccountView,
    /// custodyTokenAccount
    pub custody_token_account: &'a AccountView,
    /// custodyDovesPriceAccount
    pub custody_doves_price_account: &'a AccountView,
    /// custodyPythnetPriceAccount
    pub custody_pythnet_price_account: &'a AccountView,
    /// receivingTokenAccount
    pub receiving_token_account: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
}

impl<'a> Withdrawfees2Accounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 10] {
        [
            InstructionAccount::readonly_signer(self.keeper.address()),
            InstructionAccount::readonly(self.transfer_authority.address()),
            InstructionAccount::readonly(self.perpetuals.address()),
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.custody.address()),
            InstructionAccount::writable(self.custody_token_account.address()),
            InstructionAccount::readonly(self.custody_doves_price_account.address()),
            InstructionAccount::readonly(self.custody_pythnet_price_account.address()),
            InstructionAccount::writable(self.receiving_token_account.address()),
            InstructionAccount::readonly(self.token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 10] {
        [
            self.keeper, self.transfer_authority, self.perpetuals, self.pool, self.custody, self.custody_token_account, self.custody_doves_price_account, self.custody_pythnet_price_account, self.receiving_token_account, self.token_program
        ]
    }
}

/// Accounts for `createTokenMetadata`
pub struct CreatetokenmetadataAccounts<'a> {
    /// admin
    pub admin: &'a AccountView,
    /// perpetuals
    pub perpetuals: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// transferAuthority
    pub transfer_authority: &'a AccountView,
    /// metadata
    pub metadata: &'a AccountView,
    /// lpTokenMint
    pub lp_token_mint: &'a AccountView,
    /// tokenMetadataProgram
    pub token_metadata_program: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
    /// rent
    pub rent: &'a AccountView,
}

impl<'a> CreatetokenmetadataAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 9] {
        [
            InstructionAccount::writable_signer(self.admin.address()),
            InstructionAccount::readonly(self.perpetuals.address()),
            InstructionAccount::readonly(self.pool.address()),
            InstructionAccount::readonly(self.transfer_authority.address()),
            InstructionAccount::writable(self.metadata.address()),
            InstructionAccount::readonly(self.lp_token_mint.address()),
            InstructionAccount::readonly(self.token_metadata_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.rent.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 9] {
        [
            self.admin, self.perpetuals, self.pool, self.transfer_authority, self.metadata, self.lp_token_mint, self.token_metadata_program, self.system_program, self.rent
        ]
    }
}

/// Accounts for `createTokenLedger`
pub struct CreatetokenledgerAccounts<'a> {
    /// tokenLedger
    pub token_ledger: &'a AccountView,
    /// payer
    pub payer: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
}

impl<'a> CreatetokenledgerAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 3] {
        [
            InstructionAccount::writable_signer(self.token_ledger.address()),
            InstructionAccount::writable_signer(self.payer.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 3] {
        [
            self.token_ledger, self.payer, self.system_program
        ]
    }
}

/// Accounts for `reallocCustody`
pub struct RealloccustodyAccounts<'a> {
    /// keeper
    pub keeper: &'a AccountView,
    /// custody
    pub custody: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
    /// rent
    pub rent: &'a AccountView,
}

impl<'a> RealloccustodyAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 4] {
        [
            InstructionAccount::writable_signer(self.keeper.address()),
            InstructionAccount::writable(self.custody.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.rent.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 4] {
        [
            self.keeper, self.custody, self.system_program, self.rent
        ]
    }
}

/// Accounts for `reallocPool`
pub struct ReallocpoolAccounts<'a> {
    /// keeper
    pub keeper: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
    /// rent
    pub rent: &'a AccountView,
}

impl<'a> ReallocpoolAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 4] {
        [
            InstructionAccount::writable_signer(self.keeper.address()),
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.rent.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 4] {
        [
            self.keeper, self.pool, self.system_program, self.rent
        ]
    }
}

/// Accounts for `createAndDelegateStakeAccount`
pub struct CreateanddelegatestakeaccountAccounts<'a> {
    /// keeper
    pub keeper: &'a AccountView,
    /// perpetuals
    pub perpetuals: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// custody
    pub custody: &'a AccountView,
    /// custodyTokenAccount
    pub custody_token_account: &'a AccountView,
    /// transferAuthority
    pub transfer_authority: &'a AccountView,
    /// stakeAccount
    pub stake_account: &'a AccountView,
    /// stakeInfo
    pub stake_info: &'a AccountView,
    /// validatorVoteAccount
    pub validator_vote_account: &'a AccountView,
    /// stakeConfig
    pub stake_config: &'a AccountView,
    /// wsolMint
    pub wsol_mint: &'a AccountView,
    /// tempWsolAccount
    pub temp_wsol_account: &'a AccountView,
    /// rent
    pub rent: &'a AccountView,
    /// clock
    pub clock: &'a AccountView,
    /// stakeHistory
    pub stake_history: &'a AccountView,
    /// stakeProgram
    pub stake_program: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
}

impl<'a> CreateanddelegatestakeaccountAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 18] {
        [
            InstructionAccount::writable_signer(self.keeper.address()),
            InstructionAccount::readonly(self.perpetuals.address()),
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.custody.address()),
            InstructionAccount::writable(self.custody_token_account.address()),
            InstructionAccount::readonly(self.transfer_authority.address()),
            InstructionAccount::writable(self.stake_account.address()),
            InstructionAccount::writable(self.stake_info.address()),
            InstructionAccount::readonly(self.validator_vote_account.address()),
            InstructionAccount::readonly(self.stake_config.address()),
            InstructionAccount::readonly(self.wsol_mint.address()),
            InstructionAccount::writable(self.temp_wsol_account.address()),
            InstructionAccount::readonly(self.rent.address()),
            InstructionAccount::readonly(self.clock.address()),
            InstructionAccount::readonly(self.stake_history.address()),
            InstructionAccount::readonly(self.stake_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 18] {
        [
            self.keeper, self.perpetuals, self.pool, self.custody, self.custody_token_account, self.transfer_authority, self.stake_account, self.stake_info, self.validator_vote_account, self.stake_config, self.wsol_mint, self.temp_wsol_account, self.rent, self.clock, self.stake_history, self.stake_program, self.system_program, self.token_program
        ]
    }
}

/// Accounts for `unstake`
pub struct UnstakeAccounts<'a> {
    /// operator
    pub operator: &'a AccountView,
    /// perpetuals
    pub perpetuals: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// custody
    pub custody: &'a AccountView,
    /// transferAuthority
    pub transfer_authority: &'a AccountView,
    /// stakeAccount
    pub stake_account: &'a AccountView,
    /// stakeInfo
    pub stake_info: &'a AccountView,
    /// clock
    pub clock: &'a AccountView,
    /// stakeProgram
    pub stake_program: &'a AccountView,
}

impl<'a> UnstakeAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 9] {
        [
            InstructionAccount::readonly_signer(self.operator.address()),
            InstructionAccount::readonly(self.perpetuals.address()),
            InstructionAccount::readonly(self.pool.address()),
            InstructionAccount::readonly(self.custody.address()),
            InstructionAccount::readonly(self.transfer_authority.address()),
            InstructionAccount::writable(self.stake_account.address()),
            InstructionAccount::writable(self.stake_info.address()),
            InstructionAccount::readonly(self.clock.address()),
            InstructionAccount::readonly(self.stake_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 9] {
        [
            self.operator, self.perpetuals, self.pool, self.custody, self.transfer_authority, self.stake_account, self.stake_info, self.clock, self.stake_program
        ]
    }
}

/// Accounts for `withdrawStake`
pub struct WithdrawstakeAccounts<'a> {
    /// keeper
    pub keeper: &'a AccountView,
    /// perpetuals
    pub perpetuals: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// custody
    pub custody: &'a AccountView,
    /// custodyTokenAccount
    pub custody_token_account: &'a AccountView,
    /// transferAuthority
    pub transfer_authority: &'a AccountView,
    /// stakeAccount
    pub stake_account: &'a AccountView,
    /// stakeInfo
    pub stake_info: &'a AccountView,
    /// clock
    pub clock: &'a AccountView,
    /// stakeHistory
    pub stake_history: &'a AccountView,
    /// stakeProgram
    pub stake_program: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
}

impl<'a> WithdrawstakeAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 13] {
        [
            InstructionAccount::writable_signer(self.keeper.address()),
            InstructionAccount::readonly(self.perpetuals.address()),
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.custody.address()),
            InstructionAccount::writable(self.custody_token_account.address()),
            InstructionAccount::writable(self.transfer_authority.address()),
            InstructionAccount::writable(self.stake_account.address()),
            InstructionAccount::writable(self.stake_info.address()),
            InstructionAccount::readonly(self.clock.address()),
            InstructionAccount::readonly(self.stake_history.address()),
            InstructionAccount::readonly(self.stake_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 13] {
        [
            self.keeper, self.perpetuals, self.pool, self.custody, self.custody_token_account, self.transfer_authority, self.stake_account, self.stake_info, self.clock, self.stake_history, self.stake_program, self.system_program, self.token_program
        ]
    }
}

/// Accounts for `redeemStake`
pub struct RedeemstakeAccounts<'a> {
    /// keeper
    pub keeper: &'a AccountView,
    /// perpetuals
    pub perpetuals: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// custody
    pub custody: &'a AccountView,
    /// stakeAccount
    pub stake_account: &'a AccountView,
    /// stakeInfo
    pub stake_info: &'a AccountView,
}

impl<'a> RedeemstakeAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 6] {
        [
            InstructionAccount::readonly_signer(self.keeper.address()),
            InstructionAccount::readonly(self.perpetuals.address()),
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.custody.address()),
            InstructionAccount::readonly(self.stake_account.address()),
            InstructionAccount::writable(self.stake_info.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 6] {
        [
            self.keeper, self.perpetuals, self.pool, self.custody, self.stake_account, self.stake_info
        ]
    }
}

/// Accounts for `operatorSetCustodyConfig`
pub struct OperatorsetcustodyconfigAccounts<'a> {
    /// operator
    pub operator: &'a AccountView,
    /// custody
    pub custody: &'a AccountView,
}

impl<'a> OperatorsetcustodyconfigAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 2] {
        [
            InstructionAccount::readonly_signer(self.operator.address()),
            InstructionAccount::writable(self.custody.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 2] {
        [
            self.operator, self.custody
        ]
    }
}

/// Accounts for `operatorSetPoolConfig`
pub struct OperatorsetpoolconfigAccounts<'a> {
    /// operator
    pub operator: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
}

impl<'a> OperatorsetpoolconfigAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 2] {
        [
            InstructionAccount::readonly_signer(self.operator.address()),
            InstructionAccount::writable(self.pool.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 2] {
        [
            self.operator, self.pool
        ]
    }
}

/// Accounts for `testInit`
pub struct TestinitAccounts<'a> {
    /// upgradeAuthority
    pub upgrade_authority: &'a AccountView,
    /// admin
    pub admin: &'a AccountView,
    /// transferAuthority
    pub transfer_authority: &'a AccountView,
    /// perpetuals
    pub perpetuals: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
}

impl<'a> TestinitAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 6] {
        [
            InstructionAccount::writable_signer(self.upgrade_authority.address()),
            InstructionAccount::readonly(self.admin.address()),
            InstructionAccount::writable(self.transfer_authority.address()),
            InstructionAccount::writable(self.perpetuals.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 6] {
        [
            self.upgrade_authority, self.admin, self.transfer_authority, self.perpetuals, self.system_program, self.token_program
        ]
    }
}

/// Accounts for `setTestTime`
pub struct SettesttimeAccounts<'a> {
    /// admin
    pub admin: &'a AccountView,
    /// perpetuals
    pub perpetuals: &'a AccountView,
}

impl<'a> SettesttimeAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 2] {
        [
            InstructionAccount::readonly_signer(self.admin.address()),
            InstructionAccount::writable(self.perpetuals.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 2] {
        [
            self.admin, self.perpetuals
        ]
    }
}

/// Accounts for `setTokenLedger`
pub struct SettokenledgerAccounts<'a> {
    /// tokenLedger
    pub token_ledger: &'a AccountView,
    /// tokenAccount
    pub token_account: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
}

impl<'a> SettokenledgerAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 3] {
        [
            InstructionAccount::writable(self.token_ledger.address()),
            InstructionAccount::readonly(self.token_account.address()),
            InstructionAccount::readonly(self.token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 3] {
        [
            self.token_ledger, self.token_account, self.token_program
        ]
    }
}

/// Accounts for `swap2`
pub struct Swap2Accounts<'a> {
    /// owner
    pub owner: &'a AccountView,
    /// fundingAccount
    pub funding_account: &'a AccountView,
    /// receivingAccount
    pub receiving_account: &'a AccountView,
    /// transferAuthority
    pub transfer_authority: &'a AccountView,
    /// perpetuals
    pub perpetuals: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// receivingCustody
    pub receiving_custody: &'a AccountView,
    /// receivingCustodyDovesPriceAccount
    pub receiving_custody_doves_price_account: &'a AccountView,
    /// receivingCustodyPythnetPriceAccount
    pub receiving_custody_pythnet_price_account: &'a AccountView,
    /// receivingCustodyTokenAccount
    pub receiving_custody_token_account: &'a AccountView,
    /// dispensingCustody
    pub dispensing_custody: &'a AccountView,
    /// dispensingCustodyDovesPriceAccount
    pub dispensing_custody_doves_price_account: &'a AccountView,
    /// dispensingCustodyPythnetPriceAccount
    pub dispensing_custody_pythnet_price_account: &'a AccountView,
    /// dispensingCustodyTokenAccount
    pub dispensing_custody_token_account: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// eventAuthority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> Swap2Accounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 17] {
        [
            InstructionAccount::readonly_signer(self.owner.address()),
            InstructionAccount::writable(self.funding_account.address()),
            InstructionAccount::writable(self.receiving_account.address()),
            InstructionAccount::readonly(self.transfer_authority.address()),
            InstructionAccount::readonly(self.perpetuals.address()),
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.receiving_custody.address()),
            InstructionAccount::readonly(self.receiving_custody_doves_price_account.address()),
            InstructionAccount::readonly(self.receiving_custody_pythnet_price_account.address()),
            InstructionAccount::writable(self.receiving_custody_token_account.address()),
            InstructionAccount::writable(self.dispensing_custody.address()),
            InstructionAccount::readonly(self.dispensing_custody_doves_price_account.address()),
            InstructionAccount::readonly(self.dispensing_custody_pythnet_price_account.address()),
            InstructionAccount::writable(self.dispensing_custody_token_account.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 17] {
        [
            self.owner, self.funding_account, self.receiving_account, self.transfer_authority, self.perpetuals, self.pool, self.receiving_custody, self.receiving_custody_doves_price_account, self.receiving_custody_pythnet_price_account, self.receiving_custody_token_account, self.dispensing_custody, self.dispensing_custody_doves_price_account, self.dispensing_custody_pythnet_price_account, self.dispensing_custody_token_account, self.token_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `swapWithTokenLedger`
pub struct SwapwithtokenledgerAccounts<'a> {
    /// owner
    pub owner: &'a AccountView,
    /// fundingAccount
    pub funding_account: &'a AccountView,
    /// receivingAccount
    pub receiving_account: &'a AccountView,
    /// transferAuthority
    pub transfer_authority: &'a AccountView,
    /// perpetuals
    pub perpetuals: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// receivingCustody
    pub receiving_custody: &'a AccountView,
    /// receivingCustodyDovesPriceAccount
    pub receiving_custody_doves_price_account: &'a AccountView,
    /// receivingCustodyTokenAccount
    pub receiving_custody_token_account: &'a AccountView,
    /// dispensingCustody
    pub dispensing_custody: &'a AccountView,
    /// dispensingCustodyDovesPriceAccount
    pub dispensing_custody_doves_price_account: &'a AccountView,
    /// dispensingCustodyTokenAccount
    pub dispensing_custody_token_account: &'a AccountView,
    /// tokenLedger
    pub token_ledger: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// instruction
    pub instruction: &'a AccountView,
    /// eventAuthority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> SwapwithtokenledgerAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 17] {
        [
            InstructionAccount::readonly_signer(self.owner.address()),
            InstructionAccount::writable(self.funding_account.address()),
            InstructionAccount::writable(self.receiving_account.address()),
            InstructionAccount::readonly(self.transfer_authority.address()),
            InstructionAccount::readonly(self.perpetuals.address()),
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.receiving_custody.address()),
            InstructionAccount::readonly(self.receiving_custody_doves_price_account.address()),
            InstructionAccount::writable(self.receiving_custody_token_account.address()),
            InstructionAccount::writable(self.dispensing_custody.address()),
            InstructionAccount::readonly(self.dispensing_custody_doves_price_account.address()),
            InstructionAccount::writable(self.dispensing_custody_token_account.address()),
            InstructionAccount::readonly(self.token_ledger.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.instruction.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 17] {
        [
            self.owner, self.funding_account, self.receiving_account, self.transfer_authority, self.perpetuals, self.pool, self.receiving_custody, self.receiving_custody_doves_price_account, self.receiving_custody_token_account, self.dispensing_custody, self.dispensing_custody_doves_price_account, self.dispensing_custody_token_account, self.token_ledger, self.token_program, self.instruction, self.event_authority, self.program
        ]
    }
}

/// Accounts for `instantIncreasePositionPreSwap`
pub struct InstantincreasepositionpreswapAccounts<'a> {
    /// owner
    pub owner: &'a AccountView,
    /// fundingAccount
    pub funding_account: &'a AccountView,
    /// receivingAccount
    pub receiving_account: &'a AccountView,
    /// transferAuthority
    pub transfer_authority: &'a AccountView,
    /// perpetuals
    pub perpetuals: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// receivingCustody
    pub receiving_custody: &'a AccountView,
    /// receivingCustodyDovesPriceAccount
    pub receiving_custody_doves_price_account: &'a AccountView,
    /// receivingCustodyTokenAccount
    pub receiving_custody_token_account: &'a AccountView,
    /// dispensingCustody
    pub dispensing_custody: &'a AccountView,
    /// dispensingCustodyDovesPriceAccount
    pub dispensing_custody_doves_price_account: &'a AccountView,
    /// dispensingCustodyTokenAccount
    pub dispensing_custody_token_account: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// instruction
    pub instruction: &'a AccountView,
    /// eventAuthority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> InstantincreasepositionpreswapAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 16] {
        [
            InstructionAccount::readonly_signer(self.owner.address()),
            InstructionAccount::writable(self.funding_account.address()),
            InstructionAccount::writable(self.receiving_account.address()),
            InstructionAccount::readonly(self.transfer_authority.address()),
            InstructionAccount::readonly(self.perpetuals.address()),
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.receiving_custody.address()),
            InstructionAccount::readonly(self.receiving_custody_doves_price_account.address()),
            InstructionAccount::writable(self.receiving_custody_token_account.address()),
            InstructionAccount::writable(self.dispensing_custody.address()),
            InstructionAccount::readonly(self.dispensing_custody_doves_price_account.address()),
            InstructionAccount::writable(self.dispensing_custody_token_account.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.instruction.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 16] {
        [
            self.owner, self.funding_account, self.receiving_account, self.transfer_authority, self.perpetuals, self.pool, self.receiving_custody, self.receiving_custody_doves_price_account, self.receiving_custody_token_account, self.dispensing_custody, self.dispensing_custody_doves_price_account, self.dispensing_custody_token_account, self.token_program, self.instruction, self.event_authority, self.program
        ]
    }
}

/// Accounts for `addLiquidity2`
pub struct Addliquidity2Accounts<'a> {
    /// owner
    pub owner: &'a AccountView,
    /// fundingAccount
    pub funding_account: &'a AccountView,
    /// lpTokenAccount
    pub lp_token_account: &'a AccountView,
    /// transferAuthority
    pub transfer_authority: &'a AccountView,
    /// perpetuals
    pub perpetuals: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// custody
    pub custody: &'a AccountView,
    /// custodyDovesPriceAccount
    pub custody_doves_price_account: &'a AccountView,
    /// custodyPythnetPriceAccount
    pub custody_pythnet_price_account: &'a AccountView,
    /// custodyTokenAccount
    pub custody_token_account: &'a AccountView,
    /// lpTokenMint
    pub lp_token_mint: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// eventAuthority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> Addliquidity2Accounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 14] {
        [
            InstructionAccount::readonly_signer(self.owner.address()),
            InstructionAccount::writable(self.funding_account.address()),
            InstructionAccount::writable(self.lp_token_account.address()),
            InstructionAccount::readonly(self.transfer_authority.address()),
            InstructionAccount::readonly(self.perpetuals.address()),
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.custody.address()),
            InstructionAccount::readonly(self.custody_doves_price_account.address()),
            InstructionAccount::readonly(self.custody_pythnet_price_account.address()),
            InstructionAccount::writable(self.custody_token_account.address()),
            InstructionAccount::writable(self.lp_token_mint.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 14] {
        [
            self.owner, self.funding_account, self.lp_token_account, self.transfer_authority, self.perpetuals, self.pool, self.custody, self.custody_doves_price_account, self.custody_pythnet_price_account, self.custody_token_account, self.lp_token_mint, self.token_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `removeLiquidity2`
pub struct Removeliquidity2Accounts<'a> {
    /// owner
    pub owner: &'a AccountView,
    /// receivingAccount
    pub receiving_account: &'a AccountView,
    /// lpTokenAccount
    pub lp_token_account: &'a AccountView,
    /// transferAuthority
    pub transfer_authority: &'a AccountView,
    /// perpetuals
    pub perpetuals: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// custody
    pub custody: &'a AccountView,
    /// custodyDovesPriceAccount
    pub custody_doves_price_account: &'a AccountView,
    /// custodyPythnetPriceAccount
    pub custody_pythnet_price_account: &'a AccountView,
    /// custodyTokenAccount
    pub custody_token_account: &'a AccountView,
    /// lpTokenMint
    pub lp_token_mint: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// eventAuthority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> Removeliquidity2Accounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 14] {
        [
            InstructionAccount::readonly_signer(self.owner.address()),
            InstructionAccount::writable(self.receiving_account.address()),
            InstructionAccount::writable(self.lp_token_account.address()),
            InstructionAccount::readonly(self.transfer_authority.address()),
            InstructionAccount::readonly(self.perpetuals.address()),
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.custody.address()),
            InstructionAccount::readonly(self.custody_doves_price_account.address()),
            InstructionAccount::readonly(self.custody_pythnet_price_account.address()),
            InstructionAccount::writable(self.custody_token_account.address()),
            InstructionAccount::writable(self.lp_token_mint.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 14] {
        [
            self.owner, self.receiving_account, self.lp_token_account, self.transfer_authority, self.perpetuals, self.pool, self.custody, self.custody_doves_price_account, self.custody_pythnet_price_account, self.custody_token_account, self.lp_token_mint, self.token_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `createIncreasePositionMarketRequest`
pub struct CreateincreasepositionmarketrequestAccounts<'a> {
    /// owner
    pub owner: &'a AccountView,
    /// fundingAccount
    pub funding_account: &'a AccountView,
    /// perpetuals
    pub perpetuals: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// position
    pub position: &'a AccountView,
    /// positionRequest
    pub position_request: &'a AccountView,
    /// positionRequestAta
    pub position_request_ata: &'a AccountView,
    /// custody
    pub custody: &'a AccountView,
    /// collateralCustody
    pub collateral_custody: &'a AccountView,
    /// inputMint
    pub input_mint: &'a AccountView,
    /// referral
    pub referral: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// associatedTokenProgram
    pub associated_token_program: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
    /// eventAuthority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> CreateincreasepositionmarketrequestAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 16] {
        [
            InstructionAccount::writable_signer(self.owner.address()),
            InstructionAccount::writable(self.funding_account.address()),
            InstructionAccount::readonly(self.perpetuals.address()),
            InstructionAccount::readonly(self.pool.address()),
            InstructionAccount::writable(self.position.address()),
            InstructionAccount::writable(self.position_request.address()),
            InstructionAccount::writable(self.position_request_ata.address()),
            InstructionAccount::readonly(self.custody.address()),
            InstructionAccount::readonly(self.collateral_custody.address()),
            InstructionAccount::readonly(self.input_mint.address()),
            InstructionAccount::readonly(self.referral.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 16] {
        [
            self.owner, self.funding_account, self.perpetuals, self.pool, self.position, self.position_request, self.position_request_ata, self.custody, self.collateral_custody, self.input_mint, self.referral, self.token_program, self.associated_token_program, self.system_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `createDecreasePositionRequest2`
pub struct Createdecreasepositionrequest2Accounts<'a> {
    /// owner
    pub owner: &'a AccountView,
    /// receivingAccount
    pub receiving_account: &'a AccountView,
    /// perpetuals
    pub perpetuals: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// position
    pub position: &'a AccountView,
    /// positionRequest
    pub position_request: &'a AccountView,
    /// positionRequestAta
    pub position_request_ata: &'a AccountView,
    /// custody
    pub custody: &'a AccountView,
    /// custodyDovesPriceAccount
    pub custody_doves_price_account: &'a AccountView,
    /// custodyPythnetPriceAccount
    pub custody_pythnet_price_account: &'a AccountView,
    /// collateralCustody
    pub collateral_custody: &'a AccountView,
    /// desiredMint
    pub desired_mint: &'a AccountView,
    /// referral
    pub referral: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// associatedTokenProgram
    pub associated_token_program: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
    /// eventAuthority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> Createdecreasepositionrequest2Accounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 18] {
        [
            InstructionAccount::writable_signer(self.owner.address()),
            InstructionAccount::writable(self.receiving_account.address()),
            InstructionAccount::readonly(self.perpetuals.address()),
            InstructionAccount::readonly(self.pool.address()),
            InstructionAccount::readonly(self.position.address()),
            InstructionAccount::writable(self.position_request.address()),
            InstructionAccount::writable(self.position_request_ata.address()),
            InstructionAccount::readonly(self.custody.address()),
            InstructionAccount::readonly(self.custody_doves_price_account.address()),
            InstructionAccount::readonly(self.custody_pythnet_price_account.address()),
            InstructionAccount::readonly(self.collateral_custody.address()),
            InstructionAccount::readonly(self.desired_mint.address()),
            InstructionAccount::readonly(self.referral.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 18] {
        [
            self.owner, self.receiving_account, self.perpetuals, self.pool, self.position, self.position_request, self.position_request_ata, self.custody, self.custody_doves_price_account, self.custody_pythnet_price_account, self.collateral_custody, self.desired_mint, self.referral, self.token_program, self.associated_token_program, self.system_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `createDecreasePositionMarketRequest`
pub struct CreatedecreasepositionmarketrequestAccounts<'a> {
    /// owner
    pub owner: &'a AccountView,
    /// receivingAccount
    pub receiving_account: &'a AccountView,
    /// perpetuals
    pub perpetuals: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// position
    pub position: &'a AccountView,
    /// positionRequest
    pub position_request: &'a AccountView,
    /// positionRequestAta
    pub position_request_ata: &'a AccountView,
    /// custody
    pub custody: &'a AccountView,
    /// collateralCustody
    pub collateral_custody: &'a AccountView,
    /// desiredMint
    pub desired_mint: &'a AccountView,
    /// referral
    pub referral: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// associatedTokenProgram
    pub associated_token_program: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
    /// eventAuthority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> CreatedecreasepositionmarketrequestAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 16] {
        [
            InstructionAccount::writable_signer(self.owner.address()),
            InstructionAccount::writable(self.receiving_account.address()),
            InstructionAccount::readonly(self.perpetuals.address()),
            InstructionAccount::readonly(self.pool.address()),
            InstructionAccount::readonly(self.position.address()),
            InstructionAccount::writable(self.position_request.address()),
            InstructionAccount::writable(self.position_request_ata.address()),
            InstructionAccount::readonly(self.custody.address()),
            InstructionAccount::readonly(self.collateral_custody.address()),
            InstructionAccount::readonly(self.desired_mint.address()),
            InstructionAccount::readonly(self.referral.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 16] {
        [
            self.owner, self.receiving_account, self.perpetuals, self.pool, self.position, self.position_request, self.position_request_ata, self.custody, self.collateral_custody, self.desired_mint, self.referral, self.token_program, self.associated_token_program, self.system_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `updateDecreasePositionRequest2`
pub struct Updatedecreasepositionrequest2Accounts<'a> {
    /// owner
    pub owner: &'a AccountView,
    /// perpetuals
    pub perpetuals: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// position
    pub position: &'a AccountView,
    /// positionRequest
    pub position_request: &'a AccountView,
    /// custody
    pub custody: &'a AccountView,
    /// custodyDovesPriceAccount
    pub custody_doves_price_account: &'a AccountView,
    /// custodyPythnetPriceAccount
    pub custody_pythnet_price_account: &'a AccountView,
}

impl<'a> Updatedecreasepositionrequest2Accounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 8] {
        [
            InstructionAccount::readonly_signer(self.owner.address()),
            InstructionAccount::readonly(self.perpetuals.address()),
            InstructionAccount::readonly(self.pool.address()),
            InstructionAccount::readonly(self.position.address()),
            InstructionAccount::writable(self.position_request.address()),
            InstructionAccount::readonly(self.custody.address()),
            InstructionAccount::readonly(self.custody_doves_price_account.address()),
            InstructionAccount::readonly(self.custody_pythnet_price_account.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 8] {
        [
            self.owner, self.perpetuals, self.pool, self.position, self.position_request, self.custody, self.custody_doves_price_account, self.custody_pythnet_price_account
        ]
    }
}

/// Accounts for `closePositionRequest2`
pub struct Closepositionrequest2Accounts<'a> {
    /// keeper
    pub keeper: &'a AccountView,
    /// owner
    pub owner: &'a AccountView,
    /// ownerAta
    pub owner_ata: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// positionRequest
    pub position_request: &'a AccountView,
    /// positionRequestAta
    pub position_request_ata: &'a AccountView,
    /// position
    pub position: &'a AccountView,
    /// mint
    pub mint: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
    /// associatedTokenProgram
    pub associated_token_program: &'a AccountView,
    /// eventAuthority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> Closepositionrequest2Accounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 13] {
        [
            InstructionAccount::writable_signer(self.keeper.address()),
            InstructionAccount::writable(self.owner.address()),
            InstructionAccount::writable(self.owner_ata.address()),
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.position_request.address()),
            InstructionAccount::writable(self.position_request_ata.address()),
            InstructionAccount::readonly(self.position.address()),
            InstructionAccount::readonly(self.mint.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 13] {
        [
            self.keeper, self.owner, self.owner_ata, self.pool, self.position_request, self.position_request_ata, self.position, self.mint, self.token_program, self.system_program, self.associated_token_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `closePositionRequest3`
pub struct Closepositionrequest3Accounts<'a> {
    /// keeper
    pub keeper: &'a AccountView,
    /// owner
    pub owner: &'a AccountView,
    /// ownerAta
    pub owner_ata: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// positionRequest
    pub position_request: &'a AccountView,
    /// positionRequestAta
    pub position_request_ata: &'a AccountView,
    /// position
    pub position: &'a AccountView,
    /// custody
    pub custody: &'a AccountView,
    /// mint
    pub mint: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
    /// associatedTokenProgram
    pub associated_token_program: &'a AccountView,
    /// eventAuthority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> Closepositionrequest3Accounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 14] {
        [
            InstructionAccount::writable_signer(self.keeper.address()),
            InstructionAccount::writable(self.owner.address()),
            InstructionAccount::writable(self.owner_ata.address()),
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.position_request.address()),
            InstructionAccount::writable(self.position_request_ata.address()),
            InstructionAccount::readonly(self.position.address()),
            InstructionAccount::writable(self.custody.address()),
            InstructionAccount::readonly(self.mint.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 14] {
        [
            self.keeper, self.owner, self.owner_ata, self.pool, self.position_request, self.position_request_ata, self.position, self.custody, self.mint, self.token_program, self.system_program, self.associated_token_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `increasePosition4`
pub struct Increaseposition4Accounts<'a> {
    /// keeper
    pub keeper: &'a AccountView,
    /// perpetuals
    pub perpetuals: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// positionRequest
    pub position_request: &'a AccountView,
    /// positionRequestAta
    pub position_request_ata: &'a AccountView,
    /// position
    pub position: &'a AccountView,
    /// custody
    pub custody: &'a AccountView,
    /// custodyDovesPriceAccount
    pub custody_doves_price_account: &'a AccountView,
    /// custodyPythnetPriceAccount
    pub custody_pythnet_price_account: &'a AccountView,
    /// collateralCustody
    pub collateral_custody: &'a AccountView,
    /// collateralCustodyDovesPriceAccount
    pub collateral_custody_doves_price_account: &'a AccountView,
    /// collateralCustodyPythnetPriceAccount
    pub collateral_custody_pythnet_price_account: &'a AccountView,
    /// collateralCustodyTokenAccount
    pub collateral_custody_token_account: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// eventAuthority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> Increaseposition4Accounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 16] {
        [
            InstructionAccount::readonly_signer(self.keeper.address()),
            InstructionAccount::readonly(self.perpetuals.address()),
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.position_request.address()),
            InstructionAccount::writable(self.position_request_ata.address()),
            InstructionAccount::writable(self.position.address()),
            InstructionAccount::writable(self.custody.address()),
            InstructionAccount::readonly(self.custody_doves_price_account.address()),
            InstructionAccount::readonly(self.custody_pythnet_price_account.address()),
            InstructionAccount::writable(self.collateral_custody.address()),
            InstructionAccount::readonly(self.collateral_custody_doves_price_account.address()),
            InstructionAccount::readonly(self.collateral_custody_pythnet_price_account.address()),
            InstructionAccount::writable(self.collateral_custody_token_account.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 16] {
        [
            self.keeper, self.perpetuals, self.pool, self.position_request, self.position_request_ata, self.position, self.custody, self.custody_doves_price_account, self.custody_pythnet_price_account, self.collateral_custody, self.collateral_custody_doves_price_account, self.collateral_custody_pythnet_price_account, self.collateral_custody_token_account, self.token_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `increasePositionPreSwap`
pub struct IncreasepositionpreswapAccounts<'a> {
    /// keeper
    pub keeper: &'a AccountView,
    /// keeperAta
    pub keeper_ata: &'a AccountView,
    /// positionRequest
    pub position_request: &'a AccountView,
    /// positionRequestAta
    pub position_request_ata: &'a AccountView,
    /// position
    pub position: &'a AccountView,
    /// collateralCustody
    pub collateral_custody: &'a AccountView,
    /// collateralCustodyTokenAccount
    pub collateral_custody_token_account: &'a AccountView,
    /// instruction
    pub instruction: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// eventAuthority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> IncreasepositionpreswapAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 11] {
        [
            InstructionAccount::readonly_signer(self.keeper.address()),
            InstructionAccount::writable(self.keeper_ata.address()),
            InstructionAccount::writable(self.position_request.address()),
            InstructionAccount::writable(self.position_request_ata.address()),
            InstructionAccount::readonly(self.position.address()),
            InstructionAccount::readonly(self.collateral_custody.address()),
            InstructionAccount::readonly(self.collateral_custody_token_account.address()),
            InstructionAccount::readonly(self.instruction.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 11] {
        [
            self.keeper, self.keeper_ata, self.position_request, self.position_request_ata, self.position, self.collateral_custody, self.collateral_custody_token_account, self.instruction, self.token_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `increasePositionWithInternalSwap`
pub struct IncreasepositionwithinternalswapAccounts<'a> {
    /// keeper
    pub keeper: &'a AccountView,
    /// perpetuals
    pub perpetuals: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// positionRequest
    pub position_request: &'a AccountView,
    /// positionRequestAta
    pub position_request_ata: &'a AccountView,
    /// position
    pub position: &'a AccountView,
    /// custody
    pub custody: &'a AccountView,
    /// custodyDovesPriceAccount
    pub custody_doves_price_account: &'a AccountView,
    /// custodyPythnetPriceAccount
    pub custody_pythnet_price_account: &'a AccountView,
    /// collateralCustody
    pub collateral_custody: &'a AccountView,
    /// collateralCustodyDovesPriceAccount
    pub collateral_custody_doves_price_account: &'a AccountView,
    /// collateralCustodyPythnetPriceAccount
    pub collateral_custody_pythnet_price_account: &'a AccountView,
    /// collateralCustodyTokenAccount
    pub collateral_custody_token_account: &'a AccountView,
    /// receivingCustody
    pub receiving_custody: &'a AccountView,
    /// receivingCustodyDovesPriceAccount
    pub receiving_custody_doves_price_account: &'a AccountView,
    /// receivingCustodyPythnetPriceAccount
    pub receiving_custody_pythnet_price_account: &'a AccountView,
    /// receivingCustodyTokenAccount
    pub receiving_custody_token_account: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// eventAuthority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> IncreasepositionwithinternalswapAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 20] {
        [
            InstructionAccount::readonly_signer(self.keeper.address()),
            InstructionAccount::readonly(self.perpetuals.address()),
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.position_request.address()),
            InstructionAccount::writable(self.position_request_ata.address()),
            InstructionAccount::writable(self.position.address()),
            InstructionAccount::writable(self.custody.address()),
            InstructionAccount::readonly(self.custody_doves_price_account.address()),
            InstructionAccount::readonly(self.custody_pythnet_price_account.address()),
            InstructionAccount::writable(self.collateral_custody.address()),
            InstructionAccount::readonly(self.collateral_custody_doves_price_account.address()),
            InstructionAccount::readonly(self.collateral_custody_pythnet_price_account.address()),
            InstructionAccount::writable(self.collateral_custody_token_account.address()),
            InstructionAccount::writable(self.receiving_custody.address()),
            InstructionAccount::readonly(self.receiving_custody_doves_price_account.address()),
            InstructionAccount::readonly(self.receiving_custody_pythnet_price_account.address()),
            InstructionAccount::writable(self.receiving_custody_token_account.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 20] {
        [
            self.keeper, self.perpetuals, self.pool, self.position_request, self.position_request_ata, self.position, self.custody, self.custody_doves_price_account, self.custody_pythnet_price_account, self.collateral_custody, self.collateral_custody_doves_price_account, self.collateral_custody_pythnet_price_account, self.collateral_custody_token_account, self.receiving_custody, self.receiving_custody_doves_price_account, self.receiving_custody_pythnet_price_account, self.receiving_custody_token_account, self.token_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `decreasePosition4`
pub struct Decreaseposition4Accounts<'a> {
    /// keeper
    pub keeper: &'a AccountView,
    /// owner
    pub owner: &'a AccountView,
    /// transferAuthority
    pub transfer_authority: &'a AccountView,
    /// perpetuals
    pub perpetuals: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// positionRequest
    pub position_request: &'a AccountView,
    /// positionRequestAta
    pub position_request_ata: &'a AccountView,
    /// position
    pub position: &'a AccountView,
    /// custody
    pub custody: &'a AccountView,
    /// custodyDovesPriceAccount
    pub custody_doves_price_account: &'a AccountView,
    /// custodyPythnetPriceAccount
    pub custody_pythnet_price_account: &'a AccountView,
    /// collateralCustody
    pub collateral_custody: &'a AccountView,
    /// collateralCustodyDovesPriceAccount
    pub collateral_custody_doves_price_account: &'a AccountView,
    /// collateralCustodyPythnetPriceAccount
    pub collateral_custody_pythnet_price_account: &'a AccountView,
    /// collateralCustodyTokenAccount
    pub collateral_custody_token_account: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// eventAuthority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> Decreaseposition4Accounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 18] {
        [
            InstructionAccount::readonly_signer(self.keeper.address()),
            InstructionAccount::writable(self.owner.address()),
            InstructionAccount::readonly(self.transfer_authority.address()),
            InstructionAccount::readonly(self.perpetuals.address()),
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.position_request.address()),
            InstructionAccount::writable(self.position_request_ata.address()),
            InstructionAccount::writable(self.position.address()),
            InstructionAccount::writable(self.custody.address()),
            InstructionAccount::readonly(self.custody_doves_price_account.address()),
            InstructionAccount::readonly(self.custody_pythnet_price_account.address()),
            InstructionAccount::writable(self.collateral_custody.address()),
            InstructionAccount::readonly(self.collateral_custody_doves_price_account.address()),
            InstructionAccount::readonly(self.collateral_custody_pythnet_price_account.address()),
            InstructionAccount::writable(self.collateral_custody_token_account.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 18] {
        [
            self.keeper, self.owner, self.transfer_authority, self.perpetuals, self.pool, self.position_request, self.position_request_ata, self.position, self.custody, self.custody_doves_price_account, self.custody_pythnet_price_account, self.collateral_custody, self.collateral_custody_doves_price_account, self.collateral_custody_pythnet_price_account, self.collateral_custody_token_account, self.token_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `decreasePositionWithInternalSwap`
pub struct DecreasepositionwithinternalswapAccounts<'a> {
    /// keeper
    pub keeper: &'a AccountView,
    /// owner
    pub owner: &'a AccountView,
    /// transferAuthority
    pub transfer_authority: &'a AccountView,
    /// perpetuals
    pub perpetuals: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// positionRequest
    pub position_request: &'a AccountView,
    /// positionRequestAta
    pub position_request_ata: &'a AccountView,
    /// position
    pub position: &'a AccountView,
    /// custody
    pub custody: &'a AccountView,
    /// custodyDovesPriceAccount
    pub custody_doves_price_account: &'a AccountView,
    /// custodyPythnetPriceAccount
    pub custody_pythnet_price_account: &'a AccountView,
    /// collateralCustody
    pub collateral_custody: &'a AccountView,
    /// collateralCustodyDovesPriceAccount
    pub collateral_custody_doves_price_account: &'a AccountView,
    /// collateralCustodyPythnetPriceAccount
    pub collateral_custody_pythnet_price_account: &'a AccountView,
    /// collateralCustodyTokenAccount
    pub collateral_custody_token_account: &'a AccountView,
    /// dispensingCustody
    pub dispensing_custody: &'a AccountView,
    /// dispensingCustodyDovesPriceAccount
    pub dispensing_custody_doves_price_account: &'a AccountView,
    /// dispensingCustodyPythnetPriceAccount
    pub dispensing_custody_pythnet_price_account: &'a AccountView,
    /// dispensingCustodyTokenAccount
    pub dispensing_custody_token_account: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// eventAuthority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> DecreasepositionwithinternalswapAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 22] {
        [
            InstructionAccount::readonly_signer(self.keeper.address()),
            InstructionAccount::writable(self.owner.address()),
            InstructionAccount::readonly(self.transfer_authority.address()),
            InstructionAccount::readonly(self.perpetuals.address()),
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.position_request.address()),
            InstructionAccount::writable(self.position_request_ata.address()),
            InstructionAccount::writable(self.position.address()),
            InstructionAccount::writable(self.custody.address()),
            InstructionAccount::readonly(self.custody_doves_price_account.address()),
            InstructionAccount::readonly(self.custody_pythnet_price_account.address()),
            InstructionAccount::writable(self.collateral_custody.address()),
            InstructionAccount::readonly(self.collateral_custody_doves_price_account.address()),
            InstructionAccount::readonly(self.collateral_custody_pythnet_price_account.address()),
            InstructionAccount::writable(self.collateral_custody_token_account.address()),
            InstructionAccount::writable(self.dispensing_custody.address()),
            InstructionAccount::readonly(self.dispensing_custody_doves_price_account.address()),
            InstructionAccount::readonly(self.dispensing_custody_pythnet_price_account.address()),
            InstructionAccount::writable(self.dispensing_custody_token_account.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 22] {
        [
            self.keeper, self.owner, self.transfer_authority, self.perpetuals, self.pool, self.position_request, self.position_request_ata, self.position, self.custody, self.custody_doves_price_account, self.custody_pythnet_price_account, self.collateral_custody, self.collateral_custody_doves_price_account, self.collateral_custody_pythnet_price_account, self.collateral_custody_token_account, self.dispensing_custody, self.dispensing_custody_doves_price_account, self.dispensing_custody_pythnet_price_account, self.dispensing_custody_token_account, self.token_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `decreasePositionWithTpsl`
pub struct DecreasepositionwithtpslAccounts<'a> {
    /// keeper
    pub keeper: &'a AccountView,
    /// owner
    pub owner: &'a AccountView,
    /// transferAuthority
    pub transfer_authority: &'a AccountView,
    /// perpetuals
    pub perpetuals: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// positionRequest
    pub position_request: &'a AccountView,
    /// positionRequestAta
    pub position_request_ata: &'a AccountView,
    /// position
    pub position: &'a AccountView,
    /// custody
    pub custody: &'a AccountView,
    /// custodyDovesPriceAccount
    pub custody_doves_price_account: &'a AccountView,
    /// collateralCustody
    pub collateral_custody: &'a AccountView,
    /// collateralCustodyDovesPriceAccount
    pub collateral_custody_doves_price_account: &'a AccountView,
    /// collateralCustodyTokenAccount
    pub collateral_custody_token_account: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// eventAuthority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> DecreasepositionwithtpslAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 16] {
        [
            InstructionAccount::readonly_signer(self.keeper.address()),
            InstructionAccount::readonly(self.owner.address()),
            InstructionAccount::readonly(self.transfer_authority.address()),
            InstructionAccount::readonly(self.perpetuals.address()),
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.position_request.address()),
            InstructionAccount::writable(self.position_request_ata.address()),
            InstructionAccount::writable(self.position.address()),
            InstructionAccount::writable(self.custody.address()),
            InstructionAccount::readonly(self.custody_doves_price_account.address()),
            InstructionAccount::writable(self.collateral_custody.address()),
            InstructionAccount::readonly(self.collateral_custody_doves_price_account.address()),
            InstructionAccount::writable(self.collateral_custody_token_account.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 16] {
        [
            self.keeper, self.owner, self.transfer_authority, self.perpetuals, self.pool, self.position_request, self.position_request_ata, self.position, self.custody, self.custody_doves_price_account, self.collateral_custody, self.collateral_custody_doves_price_account, self.collateral_custody_token_account, self.token_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `decreasePositionWithTpslAndInternalSwap`
pub struct DecreasepositionwithtpslandinternalswapAccounts<'a> {
    /// keeper
    pub keeper: &'a AccountView,
    /// owner
    pub owner: &'a AccountView,
    /// transferAuthority
    pub transfer_authority: &'a AccountView,
    /// perpetuals
    pub perpetuals: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// positionRequest
    pub position_request: &'a AccountView,
    /// positionRequestAta
    pub position_request_ata: &'a AccountView,
    /// position
    pub position: &'a AccountView,
    /// custody
    pub custody: &'a AccountView,
    /// custodyDovesPriceAccount
    pub custody_doves_price_account: &'a AccountView,
    /// collateralCustody
    pub collateral_custody: &'a AccountView,
    /// collateralCustodyDovesPriceAccount
    pub collateral_custody_doves_price_account: &'a AccountView,
    /// collateralCustodyTokenAccount
    pub collateral_custody_token_account: &'a AccountView,
    /// dispensingCustody
    pub dispensing_custody: &'a AccountView,
    /// dispensingCustodyDovesPriceAccount
    pub dispensing_custody_doves_price_account: &'a AccountView,
    /// dispensingCustodyTokenAccount
    pub dispensing_custody_token_account: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// eventAuthority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> DecreasepositionwithtpslandinternalswapAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 19] {
        [
            InstructionAccount::readonly_signer(self.keeper.address()),
            InstructionAccount::readonly(self.owner.address()),
            InstructionAccount::readonly(self.transfer_authority.address()),
            InstructionAccount::readonly(self.perpetuals.address()),
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.position_request.address()),
            InstructionAccount::writable(self.position_request_ata.address()),
            InstructionAccount::writable(self.position.address()),
            InstructionAccount::writable(self.custody.address()),
            InstructionAccount::readonly(self.custody_doves_price_account.address()),
            InstructionAccount::writable(self.collateral_custody.address()),
            InstructionAccount::readonly(self.collateral_custody_doves_price_account.address()),
            InstructionAccount::writable(self.collateral_custody_token_account.address()),
            InstructionAccount::writable(self.dispensing_custody.address()),
            InstructionAccount::readonly(self.dispensing_custody_doves_price_account.address()),
            InstructionAccount::writable(self.dispensing_custody_token_account.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 19] {
        [
            self.keeper, self.owner, self.transfer_authority, self.perpetuals, self.pool, self.position_request, self.position_request_ata, self.position, self.custody, self.custody_doves_price_account, self.collateral_custody, self.collateral_custody_doves_price_account, self.collateral_custody_token_account, self.dispensing_custody, self.dispensing_custody_doves_price_account, self.dispensing_custody_token_account, self.token_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `liquidateFullPosition4`
pub struct Liquidatefullposition4Accounts<'a> {
    /// signer
    pub signer: &'a AccountView,
    /// perpetuals
    pub perpetuals: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// position
    pub position: &'a AccountView,
    /// custody
    pub custody: &'a AccountView,
    /// custodyDovesPriceAccount
    pub custody_doves_price_account: &'a AccountView,
    /// custodyPythnetPriceAccount
    pub custody_pythnet_price_account: &'a AccountView,
    /// collateralCustody
    pub collateral_custody: &'a AccountView,
    /// collateralCustodyDovesPriceAccount
    pub collateral_custody_doves_price_account: &'a AccountView,
    /// collateralCustodyPythnetPriceAccount
    pub collateral_custody_pythnet_price_account: &'a AccountView,
    /// collateralCustodyTokenAccount
    pub collateral_custody_token_account: &'a AccountView,
    /// eventAuthority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> Liquidatefullposition4Accounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 13] {
        [
            InstructionAccount::readonly_signer(self.signer.address()),
            InstructionAccount::readonly(self.perpetuals.address()),
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.position.address()),
            InstructionAccount::writable(self.custody.address()),
            InstructionAccount::readonly(self.custody_doves_price_account.address()),
            InstructionAccount::readonly(self.custody_pythnet_price_account.address()),
            InstructionAccount::writable(self.collateral_custody.address()),
            InstructionAccount::readonly(self.collateral_custody_doves_price_account.address()),
            InstructionAccount::readonly(self.collateral_custody_pythnet_price_account.address()),
            InstructionAccount::writable(self.collateral_custody_token_account.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 13] {
        [
            self.signer, self.perpetuals, self.pool, self.position, self.custody, self.custody_doves_price_account, self.custody_pythnet_price_account, self.collateral_custody, self.collateral_custody_doves_price_account, self.collateral_custody_pythnet_price_account, self.collateral_custody_token_account, self.event_authority, self.program
        ]
    }
}

/// Accounts for `refreshAssetsUnderManagement`
pub struct RefreshassetsundermanagementAccounts<'a> {
    /// keeper
    pub keeper: &'a AccountView,
    /// perpetuals
    pub perpetuals: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// lpTokenMint
    pub lp_token_mint: &'a AccountView,
}

impl<'a> RefreshassetsundermanagementAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 4] {
        [
            InstructionAccount::readonly_signer(self.keeper.address()),
            InstructionAccount::readonly(self.perpetuals.address()),
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::readonly(self.lp_token_mint.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 4] {
        [
            self.keeper, self.perpetuals, self.pool, self.lp_token_mint
        ]
    }
}

/// Accounts for `setMaxGlobalSizes`
pub struct SetmaxglobalsizesAccounts<'a> {
    /// keeper
    pub keeper: &'a AccountView,
    /// custody
    pub custody: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
}

impl<'a> SetmaxglobalsizesAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 3] {
        [
            InstructionAccount::readonly_signer(self.keeper.address()),
            InstructionAccount::writable(self.custody.address()),
            InstructionAccount::readonly(self.pool.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 3] {
        [
            self.keeper, self.custody, self.pool
        ]
    }
}

/// Accounts for `instantCreateTpsl`
pub struct InstantcreatetpslAccounts<'a> {
    /// keeper
    pub keeper: &'a AccountView,
    /// apiKeeper
    pub api_keeper: &'a AccountView,
    /// owner
    pub owner: &'a AccountView,
    /// receivingAccount
    pub receiving_account: &'a AccountView,
    /// perpetuals
    pub perpetuals: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// position
    pub position: &'a AccountView,
    /// positionRequest
    pub position_request: &'a AccountView,
    /// positionRequestAta
    pub position_request_ata: &'a AccountView,
    /// custody
    pub custody: &'a AccountView,
    /// custodyDovesPriceAccount
    pub custody_doves_price_account: &'a AccountView,
    /// custodyPythnetPriceAccount
    pub custody_pythnet_price_account: &'a AccountView,
    /// collateralCustody
    pub collateral_custody: &'a AccountView,
    /// desiredMint
    pub desired_mint: &'a AccountView,
    /// referral
    pub referral: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// associatedTokenProgram
    pub associated_token_program: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
    /// eventAuthority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> InstantcreatetpslAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 20] {
        [
            InstructionAccount::readonly_signer(self.keeper.address()),
            InstructionAccount::readonly_signer(self.api_keeper.address()),
            InstructionAccount::writable_signer(self.owner.address()),
            InstructionAccount::writable(self.receiving_account.address()),
            InstructionAccount::readonly(self.perpetuals.address()),
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::readonly(self.position.address()),
            InstructionAccount::writable(self.position_request.address()),
            InstructionAccount::writable(self.position_request_ata.address()),
            InstructionAccount::readonly(self.custody.address()),
            InstructionAccount::readonly(self.custody_doves_price_account.address()),
            InstructionAccount::readonly(self.custody_pythnet_price_account.address()),
            InstructionAccount::readonly(self.collateral_custody.address()),
            InstructionAccount::readonly(self.desired_mint.address()),
            InstructionAccount::readonly(self.referral.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 20] {
        [
            self.keeper, self.api_keeper, self.owner, self.receiving_account, self.perpetuals, self.pool, self.position, self.position_request, self.position_request_ata, self.custody, self.custody_doves_price_account, self.custody_pythnet_price_account, self.collateral_custody, self.desired_mint, self.referral, self.token_program, self.associated_token_program, self.system_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `instantCreateLimitOrder`
pub struct InstantcreatelimitorderAccounts<'a> {
    /// keeper
    pub keeper: &'a AccountView,
    /// apiKeeper
    pub api_keeper: &'a AccountView,
    /// owner
    pub owner: &'a AccountView,
    /// fundingAccount
    pub funding_account: &'a AccountView,
    /// perpetuals
    pub perpetuals: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// position
    pub position: &'a AccountView,
    /// positionRequest
    pub position_request: &'a AccountView,
    /// positionRequestAta
    pub position_request_ata: &'a AccountView,
    /// custody
    pub custody: &'a AccountView,
    /// custodyDovesPriceAccount
    pub custody_doves_price_account: &'a AccountView,
    /// custodyPythnetPriceAccount
    pub custody_pythnet_price_account: &'a AccountView,
    /// collateralCustody
    pub collateral_custody: &'a AccountView,
    /// inputMint
    pub input_mint: &'a AccountView,
    /// referral
    pub referral: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// associatedTokenProgram
    pub associated_token_program: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
    /// eventAuthority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> InstantcreatelimitorderAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 20] {
        [
            InstructionAccount::readonly_signer(self.keeper.address()),
            InstructionAccount::readonly_signer(self.api_keeper.address()),
            InstructionAccount::writable_signer(self.owner.address()),
            InstructionAccount::writable(self.funding_account.address()),
            InstructionAccount::readonly(self.perpetuals.address()),
            InstructionAccount::readonly(self.pool.address()),
            InstructionAccount::writable(self.position.address()),
            InstructionAccount::writable(self.position_request.address()),
            InstructionAccount::writable(self.position_request_ata.address()),
            InstructionAccount::readonly(self.custody.address()),
            InstructionAccount::readonly(self.custody_doves_price_account.address()),
            InstructionAccount::readonly(self.custody_pythnet_price_account.address()),
            InstructionAccount::readonly(self.collateral_custody.address()),
            InstructionAccount::readonly(self.input_mint.address()),
            InstructionAccount::readonly(self.referral.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 20] {
        [
            self.keeper, self.api_keeper, self.owner, self.funding_account, self.perpetuals, self.pool, self.position, self.position_request, self.position_request_ata, self.custody, self.custody_doves_price_account, self.custody_pythnet_price_account, self.collateral_custody, self.input_mint, self.referral, self.token_program, self.associated_token_program, self.system_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `instantIncreasePosition`
pub struct InstantincreasepositionAccounts<'a> {
    /// keeper
    pub keeper: &'a AccountView,
    /// apiKeeper
    pub api_keeper: &'a AccountView,
    /// owner
    pub owner: &'a AccountView,
    /// fundingAccount
    pub funding_account: &'a AccountView,
    /// perpetuals
    pub perpetuals: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// position
    pub position: &'a AccountView,
    /// custody
    pub custody: &'a AccountView,
    /// custodyDovesPriceAccount
    pub custody_doves_price_account: &'a AccountView,
    /// custodyPythnetPriceAccount
    pub custody_pythnet_price_account: &'a AccountView,
    /// collateralCustody
    pub collateral_custody: &'a AccountView,
    /// collateralCustodyDovesPriceAccount
    pub collateral_custody_doves_price_account: &'a AccountView,
    /// collateralCustodyPythnetPriceAccount
    pub collateral_custody_pythnet_price_account: &'a AccountView,
    /// collateralCustodyTokenAccount
    pub collateral_custody_token_account: &'a AccountView,
    /// tokenLedger
    pub token_ledger: &'a AccountView,
    /// referral
    pub referral: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
    /// eventAuthority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> InstantincreasepositionAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 20] {
        [
            InstructionAccount::readonly_signer(self.keeper.address()),
            InstructionAccount::readonly_signer(self.api_keeper.address()),
            InstructionAccount::writable_signer(self.owner.address()),
            InstructionAccount::writable(self.funding_account.address()),
            InstructionAccount::readonly(self.perpetuals.address()),
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.position.address()),
            InstructionAccount::writable(self.custody.address()),
            InstructionAccount::readonly(self.custody_doves_price_account.address()),
            InstructionAccount::readonly(self.custody_pythnet_price_account.address()),
            InstructionAccount::writable(self.collateral_custody.address()),
            InstructionAccount::readonly(self.collateral_custody_doves_price_account.address()),
            InstructionAccount::readonly(self.collateral_custody_pythnet_price_account.address()),
            InstructionAccount::writable(self.collateral_custody_token_account.address()),
            InstructionAccount::readonly(self.token_ledger.address()),
            InstructionAccount::readonly(self.referral.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 20] {
        [
            self.keeper, self.api_keeper, self.owner, self.funding_account, self.perpetuals, self.pool, self.position, self.custody, self.custody_doves_price_account, self.custody_pythnet_price_account, self.collateral_custody, self.collateral_custody_doves_price_account, self.collateral_custody_pythnet_price_account, self.collateral_custody_token_account, self.token_ledger, self.referral, self.token_program, self.system_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `instantDecreasePosition`
pub struct InstantdecreasepositionAccounts<'a> {
    /// keeper
    pub keeper: &'a AccountView,
    /// apiKeeper
    pub api_keeper: &'a AccountView,
    /// owner
    pub owner: &'a AccountView,
    /// receivingAccount
    pub receiving_account: &'a AccountView,
    /// transferAuthority
    pub transfer_authority: &'a AccountView,
    /// perpetuals
    pub perpetuals: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// position
    pub position: &'a AccountView,
    /// custody
    pub custody: &'a AccountView,
    /// custodyDovesPriceAccount
    pub custody_doves_price_account: &'a AccountView,
    /// custodyPythnetPriceAccount
    pub custody_pythnet_price_account: &'a AccountView,
    /// collateralCustody
    pub collateral_custody: &'a AccountView,
    /// collateralCustodyDovesPriceAccount
    pub collateral_custody_doves_price_account: &'a AccountView,
    /// collateralCustodyPythnetPriceAccount
    pub collateral_custody_pythnet_price_account: &'a AccountView,
    /// collateralCustodyTokenAccount
    pub collateral_custody_token_account: &'a AccountView,
    /// desiredMint
    pub desired_mint: &'a AccountView,
    /// referral
    pub referral: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// associatedTokenProgram
    pub associated_token_program: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
    /// eventAuthority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> InstantdecreasepositionAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 22] {
        [
            InstructionAccount::readonly_signer(self.keeper.address()),
            InstructionAccount::readonly_signer(self.api_keeper.address()),
            InstructionAccount::writable_signer(self.owner.address()),
            InstructionAccount::writable(self.receiving_account.address()),
            InstructionAccount::readonly(self.transfer_authority.address()),
            InstructionAccount::readonly(self.perpetuals.address()),
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.position.address()),
            InstructionAccount::writable(self.custody.address()),
            InstructionAccount::readonly(self.custody_doves_price_account.address()),
            InstructionAccount::readonly(self.custody_pythnet_price_account.address()),
            InstructionAccount::writable(self.collateral_custody.address()),
            InstructionAccount::readonly(self.collateral_custody_doves_price_account.address()),
            InstructionAccount::readonly(self.collateral_custody_pythnet_price_account.address()),
            InstructionAccount::writable(self.collateral_custody_token_account.address()),
            InstructionAccount::readonly(self.desired_mint.address()),
            InstructionAccount::readonly(self.referral.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 22] {
        [
            self.keeper, self.api_keeper, self.owner, self.receiving_account, self.transfer_authority, self.perpetuals, self.pool, self.position, self.custody, self.custody_doves_price_account, self.custody_pythnet_price_account, self.collateral_custody, self.collateral_custody_doves_price_account, self.collateral_custody_pythnet_price_account, self.collateral_custody_token_account, self.desired_mint, self.referral, self.token_program, self.associated_token_program, self.system_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `instantDecreasePosition2`
pub struct Instantdecreaseposition2Accounts<'a> {
    /// keeper
    pub keeper: &'a AccountView,
    /// apiKeeper
    pub api_keeper: &'a AccountView,
    /// owner
    pub owner: &'a AccountView,
    /// receivingAccount
    pub receiving_account: &'a AccountView,
    /// transferAuthority
    pub transfer_authority: &'a AccountView,
    /// perpetuals
    pub perpetuals: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// position
    pub position: &'a AccountView,
    /// custody
    pub custody: &'a AccountView,
    /// custodyDovesPriceAccount
    pub custody_doves_price_account: &'a AccountView,
    /// collateralCustody
    pub collateral_custody: &'a AccountView,
    /// collateralCustodyDovesPriceAccount
    pub collateral_custody_doves_price_account: &'a AccountView,
    /// collateralCustodyTokenAccount
    pub collateral_custody_token_account: &'a AccountView,
    /// desiredMint
    pub desired_mint: &'a AccountView,
    /// referral
    pub referral: &'a AccountView,
    /// positionRequest
    pub position_request: &'a AccountView,
    /// positionRequestAta
    pub position_request_ata: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// associatedTokenProgram
    pub associated_token_program: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
    /// eventAuthority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> Instantdecreaseposition2Accounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 22] {
        [
            InstructionAccount::readonly_signer(self.keeper.address()),
            InstructionAccount::readonly_signer(self.api_keeper.address()),
            InstructionAccount::writable_signer(self.owner.address()),
            InstructionAccount::writable(self.receiving_account.address()),
            InstructionAccount::readonly(self.transfer_authority.address()),
            InstructionAccount::readonly(self.perpetuals.address()),
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.position.address()),
            InstructionAccount::writable(self.custody.address()),
            InstructionAccount::readonly(self.custody_doves_price_account.address()),
            InstructionAccount::writable(self.collateral_custody.address()),
            InstructionAccount::readonly(self.collateral_custody_doves_price_account.address()),
            InstructionAccount::writable(self.collateral_custody_token_account.address()),
            InstructionAccount::readonly(self.desired_mint.address()),
            InstructionAccount::readonly(self.referral.address()),
            InstructionAccount::writable(self.position_request.address()),
            InstructionAccount::writable(self.position_request_ata.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 22] {
        [
            self.keeper, self.api_keeper, self.owner, self.receiving_account, self.transfer_authority, self.perpetuals, self.pool, self.position, self.custody, self.custody_doves_price_account, self.collateral_custody, self.collateral_custody_doves_price_account, self.collateral_custody_token_account, self.desired_mint, self.referral, self.position_request, self.position_request_ata, self.token_program, self.associated_token_program, self.system_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `instantUpdateLimitOrder`
pub struct InstantupdatelimitorderAccounts<'a> {
    /// keeper
    pub keeper: &'a AccountView,
    /// apiKeeper
    pub api_keeper: &'a AccountView,
    /// owner
    pub owner: &'a AccountView,
    /// perpetuals
    pub perpetuals: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// position
    pub position: &'a AccountView,
    /// positionRequest
    pub position_request: &'a AccountView,
    /// custody
    pub custody: &'a AccountView,
    /// custodyDovesPriceAccount
    pub custody_doves_price_account: &'a AccountView,
    /// custodyPythnetPriceAccount
    pub custody_pythnet_price_account: &'a AccountView,
}

impl<'a> InstantupdatelimitorderAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 10] {
        [
            InstructionAccount::readonly_signer(self.keeper.address()),
            InstructionAccount::readonly_signer(self.api_keeper.address()),
            InstructionAccount::readonly_signer(self.owner.address()),
            InstructionAccount::readonly(self.perpetuals.address()),
            InstructionAccount::readonly(self.pool.address()),
            InstructionAccount::readonly(self.position.address()),
            InstructionAccount::writable(self.position_request.address()),
            InstructionAccount::readonly(self.custody.address()),
            InstructionAccount::readonly(self.custody_doves_price_account.address()),
            InstructionAccount::readonly(self.custody_pythnet_price_account.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 10] {
        [
            self.keeper, self.api_keeper, self.owner, self.perpetuals, self.pool, self.position, self.position_request, self.custody, self.custody_doves_price_account, self.custody_pythnet_price_account
        ]
    }
}

/// Accounts for `instantUpdateTpsl`
pub struct InstantupdatetpslAccounts<'a> {
    /// keeper
    pub keeper: &'a AccountView,
    /// apiKeeper
    pub api_keeper: &'a AccountView,
    /// owner
    pub owner: &'a AccountView,
    /// perpetuals
    pub perpetuals: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// position
    pub position: &'a AccountView,
    /// positionRequest
    pub position_request: &'a AccountView,
    /// custody
    pub custody: &'a AccountView,
    /// custodyDovesPriceAccount
    pub custody_doves_price_account: &'a AccountView,
    /// custodyPythnetPriceAccount
    pub custody_pythnet_price_account: &'a AccountView,
    /// eventAuthority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> InstantupdatetpslAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 12] {
        [
            InstructionAccount::readonly_signer(self.keeper.address()),
            InstructionAccount::readonly_signer(self.api_keeper.address()),
            InstructionAccount::readonly_signer(self.owner.address()),
            InstructionAccount::readonly(self.perpetuals.address()),
            InstructionAccount::readonly(self.pool.address()),
            InstructionAccount::readonly(self.position.address()),
            InstructionAccount::writable(self.position_request.address()),
            InstructionAccount::readonly(self.custody.address()),
            InstructionAccount::readonly(self.custody_doves_price_account.address()),
            InstructionAccount::readonly(self.custody_pythnet_price_account.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 12] {
        [
            self.keeper, self.api_keeper, self.owner, self.perpetuals, self.pool, self.position, self.position_request, self.custody, self.custody_doves_price_account, self.custody_pythnet_price_account, self.event_authority, self.program
        ]
    }
}

/// Accounts for `getAddLiquidityAmountAndFee2`
pub struct Getaddliquidityamountandfee2Accounts<'a> {
    /// perpetuals
    pub perpetuals: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// custody
    pub custody: &'a AccountView,
    /// custodyDovesPriceAccount
    pub custody_doves_price_account: &'a AccountView,
    /// custodyPythnetPriceAccount
    pub custody_pythnet_price_account: &'a AccountView,
    /// lpTokenMint
    pub lp_token_mint: &'a AccountView,
}

impl<'a> Getaddliquidityamountandfee2Accounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 6] {
        [
            InstructionAccount::readonly(self.perpetuals.address()),
            InstructionAccount::readonly(self.pool.address()),
            InstructionAccount::readonly(self.custody.address()),
            InstructionAccount::readonly(self.custody_doves_price_account.address()),
            InstructionAccount::readonly(self.custody_pythnet_price_account.address()),
            InstructionAccount::readonly(self.lp_token_mint.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 6] {
        [
            self.perpetuals, self.pool, self.custody, self.custody_doves_price_account, self.custody_pythnet_price_account, self.lp_token_mint
        ]
    }
}

/// Accounts for `getRemoveLiquidityAmountAndFee2`
pub struct Getremoveliquidityamountandfee2Accounts<'a> {
    /// perpetuals
    pub perpetuals: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// custody
    pub custody: &'a AccountView,
    /// custodyDovesPriceAccount
    pub custody_doves_price_account: &'a AccountView,
    /// custodyPythnetPriceAccount
    pub custody_pythnet_price_account: &'a AccountView,
    /// lpTokenMint
    pub lp_token_mint: &'a AccountView,
}

impl<'a> Getremoveliquidityamountandfee2Accounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 6] {
        [
            InstructionAccount::readonly(self.perpetuals.address()),
            InstructionAccount::readonly(self.pool.address()),
            InstructionAccount::readonly(self.custody.address()),
            InstructionAccount::readonly(self.custody_doves_price_account.address()),
            InstructionAccount::readonly(self.custody_pythnet_price_account.address()),
            InstructionAccount::readonly(self.lp_token_mint.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 6] {
        [
            self.perpetuals, self.pool, self.custody, self.custody_doves_price_account, self.custody_pythnet_price_account, self.lp_token_mint
        ]
    }
}

/// Accounts for `getAssetsUnderManagement2`
pub struct Getassetsundermanagement2Accounts<'a> {
    /// perpetuals
    pub perpetuals: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
}

impl<'a> Getassetsundermanagement2Accounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 2] {
        [
            InstructionAccount::readonly(self.perpetuals.address()),
            InstructionAccount::readonly(self.pool.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 2] {
        [
            self.perpetuals, self.pool
        ]
    }
}

/// Accounts for `borrowFromCustody`
pub struct BorrowfromcustodyAccounts<'a> {
    /// owner
    pub owner: &'a AccountView,
    /// perpetuals
    pub perpetuals: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// custody
    pub custody: &'a AccountView,
    /// transferAuthority
    pub transfer_authority: &'a AccountView,
    /// borrowPosition
    pub borrow_position: &'a AccountView,
    /// custodyTokenAccount
    pub custody_token_account: &'a AccountView,
    /// userTokenAccount
    pub user_token_account: &'a AccountView,
    /// lpTokenMint
    pub lp_token_mint: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// eventAuthority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> BorrowfromcustodyAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 12] {
        [
            InstructionAccount::writable_signer(self.owner.address()),
            InstructionAccount::readonly(self.perpetuals.address()),
            InstructionAccount::readonly(self.pool.address()),
            InstructionAccount::writable(self.custody.address()),
            InstructionAccount::readonly(self.transfer_authority.address()),
            InstructionAccount::writable(self.borrow_position.address()),
            InstructionAccount::writable(self.custody_token_account.address()),
            InstructionAccount::writable(self.user_token_account.address()),
            InstructionAccount::readonly(self.lp_token_mint.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 12] {
        [
            self.owner, self.perpetuals, self.pool, self.custody, self.transfer_authority, self.borrow_position, self.custody_token_account, self.user_token_account, self.lp_token_mint, self.token_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `repayToCustody`
pub struct RepaytocustodyAccounts<'a> {
    /// owner
    pub owner: &'a AccountView,
    /// perpetuals
    pub perpetuals: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// custody
    pub custody: &'a AccountView,
    /// borrowPosition
    pub borrow_position: &'a AccountView,
    /// custodyTokenAccount
    pub custody_token_account: &'a AccountView,
    /// userTokenAccount
    pub user_token_account: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// eventAuthority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> RepaytocustodyAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 10] {
        [
            InstructionAccount::writable_signer(self.owner.address()),
            InstructionAccount::readonly(self.perpetuals.address()),
            InstructionAccount::readonly(self.pool.address()),
            InstructionAccount::writable(self.custody.address()),
            InstructionAccount::writable(self.borrow_position.address()),
            InstructionAccount::writable(self.custody_token_account.address()),
            InstructionAccount::writable(self.user_token_account.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 10] {
        [
            self.owner, self.perpetuals, self.pool, self.custody, self.borrow_position, self.custody_token_account, self.user_token_account, self.token_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `depositCollateralForBorrows`
pub struct DepositcollateralforborrowsAccounts<'a> {
    /// owner
    pub owner: &'a AccountView,
    /// perpetuals
    pub perpetuals: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// custody
    pub custody: &'a AccountView,
    /// transferAuthority
    pub transfer_authority: &'a AccountView,
    /// borrowPosition
    pub borrow_position: &'a AccountView,
    /// collateralTokenAccount
    pub collateral_token_account: &'a AccountView,
    /// userTokenAccount
    pub user_token_account: &'a AccountView,
    /// lpTokenMint
    pub lp_token_mint: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
    /// eventAuthority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> DepositcollateralforborrowsAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 13] {
        [
            InstructionAccount::writable_signer(self.owner.address()),
            InstructionAccount::readonly(self.perpetuals.address()),
            InstructionAccount::readonly(self.pool.address()),
            InstructionAccount::readonly(self.custody.address()),
            InstructionAccount::readonly(self.transfer_authority.address()),
            InstructionAccount::writable(self.borrow_position.address()),
            InstructionAccount::writable(self.collateral_token_account.address()),
            InstructionAccount::writable(self.user_token_account.address()),
            InstructionAccount::readonly(self.lp_token_mint.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 13] {
        [
            self.owner, self.perpetuals, self.pool, self.custody, self.transfer_authority, self.borrow_position, self.collateral_token_account, self.user_token_account, self.lp_token_mint, self.token_program, self.system_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `withdrawCollateralForBorrows`
pub struct WithdrawcollateralforborrowsAccounts<'a> {
    /// owner
    pub owner: &'a AccountView,
    /// perpetuals
    pub perpetuals: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// custody
    pub custody: &'a AccountView,
    /// transferAuthority
    pub transfer_authority: &'a AccountView,
    /// borrowPosition
    pub borrow_position: &'a AccountView,
    /// collateralTokenAccount
    pub collateral_token_account: &'a AccountView,
    /// userTokenAccount
    pub user_token_account: &'a AccountView,
    /// lpTokenMint
    pub lp_token_mint: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// eventAuthority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> WithdrawcollateralforborrowsAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 12] {
        [
            InstructionAccount::writable_signer(self.owner.address()),
            InstructionAccount::readonly(self.perpetuals.address()),
            InstructionAccount::readonly(self.pool.address()),
            InstructionAccount::writable(self.custody.address()),
            InstructionAccount::readonly(self.transfer_authority.address()),
            InstructionAccount::writable(self.borrow_position.address()),
            InstructionAccount::writable(self.collateral_token_account.address()),
            InstructionAccount::writable(self.user_token_account.address()),
            InstructionAccount::readonly(self.lp_token_mint.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 12] {
        [
            self.owner, self.perpetuals, self.pool, self.custody, self.transfer_authority, self.borrow_position, self.collateral_token_account, self.user_token_account, self.lp_token_mint, self.token_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `liquidateBorrowPosition`
pub struct LiquidateborrowpositionAccounts<'a> {
    /// signer
    pub signer: &'a AccountView,
    /// perpetuals
    pub perpetuals: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// custody
    pub custody: &'a AccountView,
    /// transferAuthority
    pub transfer_authority: &'a AccountView,
    /// borrowPosition
    pub borrow_position: &'a AccountView,
    /// collateralTokenAccount
    pub collateral_token_account: &'a AccountView,
    /// lpTokenMint
    pub lp_token_mint: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// eventAuthority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> LiquidateborrowpositionAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 11] {
        [
            InstructionAccount::readonly_signer(self.signer.address()),
            InstructionAccount::readonly(self.perpetuals.address()),
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.custody.address()),
            InstructionAccount::readonly(self.transfer_authority.address()),
            InstructionAccount::writable(self.borrow_position.address()),
            InstructionAccount::writable(self.collateral_token_account.address()),
            InstructionAccount::writable(self.lp_token_mint.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 11] {
        [
            self.signer, self.perpetuals, self.pool, self.custody, self.transfer_authority, self.borrow_position, self.collateral_token_account, self.lp_token_mint, self.token_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `partialLiquidateBorrowPosition`
pub struct PartialliquidateborrowpositionAccounts<'a> {
    /// signer
    pub signer: &'a AccountView,
    /// perpetuals
    pub perpetuals: &'a AccountView,
    /// pool
    pub pool: &'a AccountView,
    /// custody
    pub custody: &'a AccountView,
    /// transferAuthority
    pub transfer_authority: &'a AccountView,
    /// borrowPosition
    pub borrow_position: &'a AccountView,
    /// collateralTokenAccount
    pub collateral_token_account: &'a AccountView,
    /// lpTokenMint
    pub lp_token_mint: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// eventAuthority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> PartialliquidateborrowpositionAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 11] {
        [
            InstructionAccount::readonly_signer(self.signer.address()),
            InstructionAccount::readonly(self.perpetuals.address()),
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.custody.address()),
            InstructionAccount::readonly(self.transfer_authority.address()),
            InstructionAccount::writable(self.borrow_position.address()),
            InstructionAccount::writable(self.collateral_token_account.address()),
            InstructionAccount::writable(self.lp_token_mint.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 11] {
        [
            self.signer, self.perpetuals, self.pool, self.custody, self.transfer_authority, self.borrow_position, self.collateral_token_account, self.lp_token_mint, self.token_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `closeBorrowPosition`
pub struct CloseborrowpositionAccounts<'a> {
    /// owner
    pub owner: &'a AccountView,
    /// borrowPosition
    pub borrow_position: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
    /// eventAuthority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> CloseborrowpositionAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 5] {
        [
            InstructionAccount::writable_signer(self.owner.address()),
            InstructionAccount::writable(self.borrow_position.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 5] {
        [
            self.owner, self.borrow_position, self.system_program, self.event_authority, self.program
        ]
    }
}


// ============================================
// CPI Functions
// ============================================
/// CPI: init
#[inline(always)]
pub fn init<'a>(
    accounts: &InitAccounts<'a>, args: &InitArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<InitArgs>()];
    data[0..8].copy_from_slice(&INIT);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const InitArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<InitArgs>(),
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

/// CPI: addPool
#[inline(always)]
pub fn add_pool<'a>(
    accounts: &AddpoolAccounts<'a>, args: &AddpoolArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<AddpoolArgs>()];
    data[0..8].copy_from_slice(&ADD_POOL);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const AddpoolArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<AddpoolArgs>(),
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

/// CPI: addCustody
#[inline(always)]
pub fn add_custody<'a>(
    accounts: &AddcustodyAccounts<'a>, args: &AddcustodyArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<AddcustodyArgs>()];
    data[0..8].copy_from_slice(&ADD_CUSTODY);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const AddcustodyArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<AddcustodyArgs>(),
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

/// CPI: setCustodyConfig
#[inline(always)]
pub fn set_custody_config<'a>(
    accounts: &SetcustodyconfigAccounts<'a>, args: &SetcustodyconfigArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SetcustodyconfigArgs>()];
    data[0..8].copy_from_slice(&SET_CUSTODY_CONFIG);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SetcustodyconfigArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SetcustodyconfigArgs>(),
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

/// CPI: setPoolConfig
#[inline(always)]
pub fn set_pool_config<'a>(
    accounts: &SetpoolconfigAccounts<'a>, args: &SetpoolconfigArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SetpoolconfigArgs>()];
    data[0..8].copy_from_slice(&SET_POOL_CONFIG);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SetpoolconfigArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SetpoolconfigArgs>(),
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

/// CPI: setPerpetualsConfig
#[inline(always)]
pub fn set_perpetuals_config<'a>(
    accounts: &SetperpetualsconfigAccounts<'a>, args: &SetperpetualsconfigArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SetperpetualsconfigArgs>()];
    data[0..8].copy_from_slice(&SET_PERPETUALS_CONFIG);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SetperpetualsconfigArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SetperpetualsconfigArgs>(),
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

/// CPI: transferAdmin
#[inline(always)]
pub fn transfer_admin<'a>(
    accounts: &TransferadminAccounts<'a>, args: &TransferadminArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<TransferadminArgs>()];
    data[0..8].copy_from_slice(&TRANSFER_ADMIN);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const TransferadminArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<TransferadminArgs>(),
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

/// CPI: withdrawFees2
#[inline(always)]
pub fn withdraw_fees2<'a>(
    accounts: &Withdrawfees2Accounts<'a>, args: &Withdrawfees2Args,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<Withdrawfees2Args>()];
    data[0..8].copy_from_slice(&WITHDRAW_FEES2);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const Withdrawfees2Args as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<Withdrawfees2Args>(),
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

/// CPI: createTokenMetadata
#[inline(always)]
pub fn create_token_metadata<'a>(
    accounts: &CreatetokenmetadataAccounts<'a>, args: &CreatetokenmetadataArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<CreatetokenmetadataArgs>()];
    data[0..8].copy_from_slice(&CREATE_TOKEN_METADATA);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const CreatetokenmetadataArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<CreatetokenmetadataArgs>(),
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

/// CPI: createTokenLedger
#[inline(always)]
pub fn create_token_ledger<'a>(
    accounts: &CreatetokenledgerAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&CREATE_TOKEN_LEDGER);
    
    
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

/// CPI: reallocCustody
#[inline(always)]
pub fn realloc_custody<'a>(
    accounts: &RealloccustodyAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&REALLOC_CUSTODY);
    
    
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

/// CPI: reallocPool
#[inline(always)]
pub fn realloc_pool<'a>(
    accounts: &ReallocpoolAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&REALLOC_POOL);
    
    
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

/// CPI: createAndDelegateStakeAccount
#[inline(always)]
pub fn create_and_delegate_stake_account<'a>(
    accounts: &CreateanddelegatestakeaccountAccounts<'a>, args: &CreateanddelegatestakeaccountArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<CreateanddelegatestakeaccountArgs>()];
    data[0..8].copy_from_slice(&CREATE_AND_DELEGATE_STAKE_ACCOUNT);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const CreateanddelegatestakeaccountArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<CreateanddelegatestakeaccountArgs>(),
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

/// CPI: unstake
#[inline(always)]
pub fn unstake<'a>(
    accounts: &UnstakeAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&UNSTAKE);
    
    
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

/// CPI: withdrawStake
#[inline(always)]
pub fn withdraw_stake<'a>(
    accounts: &WithdrawstakeAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&WITHDRAW_STAKE);
    
    
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

/// CPI: redeemStake
#[inline(always)]
pub fn redeem_stake<'a>(
    accounts: &RedeemstakeAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&REDEEM_STAKE);
    
    
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

/// CPI: operatorSetCustodyConfig
#[inline(always)]
pub fn operator_set_custody_config<'a>(
    accounts: &OperatorsetcustodyconfigAccounts<'a>, args: &OperatorsetcustodyconfigArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<OperatorsetcustodyconfigArgs>()];
    data[0..8].copy_from_slice(&OPERATOR_SET_CUSTODY_CONFIG);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const OperatorsetcustodyconfigArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<OperatorsetcustodyconfigArgs>(),
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

/// CPI: operatorSetPoolConfig
#[inline(always)]
pub fn operator_set_pool_config<'a>(
    accounts: &OperatorsetpoolconfigAccounts<'a>, args: &OperatorsetpoolconfigArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<OperatorsetpoolconfigArgs>()];
    data[0..8].copy_from_slice(&OPERATOR_SET_POOL_CONFIG);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const OperatorsetpoolconfigArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<OperatorsetpoolconfigArgs>(),
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

/// CPI: testInit
#[inline(always)]
pub fn test_init<'a>(
    accounts: &TestinitAccounts<'a>, args: &TestinitArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<TestinitArgs>()];
    data[0..8].copy_from_slice(&TEST_INIT);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const TestinitArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<TestinitArgs>(),
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

/// CPI: setTestTime
#[inline(always)]
pub fn set_test_time<'a>(
    accounts: &SettesttimeAccounts<'a>, args: &SettesttimeArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SettesttimeArgs>()];
    data[0..8].copy_from_slice(&SET_TEST_TIME);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SettesttimeArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SettesttimeArgs>(),
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

/// CPI: setTokenLedger
#[inline(always)]
pub fn set_token_ledger<'a>(
    accounts: &SettokenledgerAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&SET_TOKEN_LEDGER);
    
    
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

/// CPI: swap2
#[inline(always)]
pub fn swap2<'a>(
    accounts: &Swap2Accounts<'a>, args: &Swap2Args,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<Swap2Args>()];
    data[0..8].copy_from_slice(&SWAP2);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const Swap2Args as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<Swap2Args>(),
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

/// CPI: swapWithTokenLedger
#[inline(always)]
pub fn swap_with_token_ledger<'a>(
    accounts: &SwapwithtokenledgerAccounts<'a>, args: &SwapwithtokenledgerArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SwapwithtokenledgerArgs>()];
    data[0..8].copy_from_slice(&SWAP_WITH_TOKEN_LEDGER);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SwapwithtokenledgerArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SwapwithtokenledgerArgs>(),
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

/// CPI: instantIncreasePositionPreSwap
#[inline(always)]
pub fn instant_increase_position_pre_swap<'a>(
    accounts: &InstantincreasepositionpreswapAccounts<'a>, args: &InstantincreasepositionpreswapArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<InstantincreasepositionpreswapArgs>()];
    data[0..8].copy_from_slice(&INSTANT_INCREASE_POSITION_PRE_SWAP);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const InstantincreasepositionpreswapArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<InstantincreasepositionpreswapArgs>(),
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

/// CPI: addLiquidity2
#[inline(always)]
pub fn add_liquidity2<'a>(
    accounts: &Addliquidity2Accounts<'a>, args: &Addliquidity2Args,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<Addliquidity2Args>()];
    data[0..8].copy_from_slice(&ADD_LIQUIDITY2);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const Addliquidity2Args as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<Addliquidity2Args>(),
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

/// CPI: removeLiquidity2
#[inline(always)]
pub fn remove_liquidity2<'a>(
    accounts: &Removeliquidity2Accounts<'a>, args: &Removeliquidity2Args,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<Removeliquidity2Args>()];
    data[0..8].copy_from_slice(&REMOVE_LIQUIDITY2);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const Removeliquidity2Args as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<Removeliquidity2Args>(),
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

/// CPI: createIncreasePositionMarketRequest
#[inline(always)]
pub fn create_increase_position_market_request<'a>(
    accounts: &CreateincreasepositionmarketrequestAccounts<'a>, args: &CreateincreasepositionmarketrequestArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<CreateincreasepositionmarketrequestArgs>()];
    data[0..8].copy_from_slice(&CREATE_INCREASE_POSITION_MARKET_REQUEST);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const CreateincreasepositionmarketrequestArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<CreateincreasepositionmarketrequestArgs>(),
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

/// CPI: createDecreasePositionRequest2
#[inline(always)]
pub fn create_decrease_position_request2<'a>(
    accounts: &Createdecreasepositionrequest2Accounts<'a>, args: &Createdecreasepositionrequest2Args,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<Createdecreasepositionrequest2Args>()];
    data[0..8].copy_from_slice(&CREATE_DECREASE_POSITION_REQUEST2);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const Createdecreasepositionrequest2Args as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<Createdecreasepositionrequest2Args>(),
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

/// CPI: createDecreasePositionMarketRequest
#[inline(always)]
pub fn create_decrease_position_market_request<'a>(
    accounts: &CreatedecreasepositionmarketrequestAccounts<'a>, args: &CreatedecreasepositionmarketrequestArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<CreatedecreasepositionmarketrequestArgs>()];
    data[0..8].copy_from_slice(&CREATE_DECREASE_POSITION_MARKET_REQUEST);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const CreatedecreasepositionmarketrequestArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<CreatedecreasepositionmarketrequestArgs>(),
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

/// CPI: updateDecreasePositionRequest2
#[inline(always)]
pub fn update_decrease_position_request2<'a>(
    accounts: &Updatedecreasepositionrequest2Accounts<'a>, args: &Updatedecreasepositionrequest2Args,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<Updatedecreasepositionrequest2Args>()];
    data[0..8].copy_from_slice(&UPDATE_DECREASE_POSITION_REQUEST2);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const Updatedecreasepositionrequest2Args as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<Updatedecreasepositionrequest2Args>(),
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

/// CPI: closePositionRequest2
#[inline(always)]
pub fn close_position_request2<'a>(
    accounts: &Closepositionrequest2Accounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&CLOSE_POSITION_REQUEST2);
    
    
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

/// CPI: closePositionRequest3
#[inline(always)]
pub fn close_position_request3<'a>(
    accounts: &Closepositionrequest3Accounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&CLOSE_POSITION_REQUEST3);
    
    
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

/// CPI: increasePosition4
#[inline(always)]
pub fn increase_position4<'a>(
    accounts: &Increaseposition4Accounts<'a>, args: &Increaseposition4Args,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<Increaseposition4Args>()];
    data[0..8].copy_from_slice(&INCREASE_POSITION4);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const Increaseposition4Args as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<Increaseposition4Args>(),
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

/// CPI: increasePositionPreSwap
#[inline(always)]
pub fn increase_position_pre_swap<'a>(
    accounts: &IncreasepositionpreswapAccounts<'a>, args: &IncreasepositionpreswapArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<IncreasepositionpreswapArgs>()];
    data[0..8].copy_from_slice(&INCREASE_POSITION_PRE_SWAP);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const IncreasepositionpreswapArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<IncreasepositionpreswapArgs>(),
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

/// CPI: increasePositionWithInternalSwap
#[inline(always)]
pub fn increase_position_with_internal_swap<'a>(
    accounts: &IncreasepositionwithinternalswapAccounts<'a>, args: &IncreasepositionwithinternalswapArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<IncreasepositionwithinternalswapArgs>()];
    data[0..8].copy_from_slice(&INCREASE_POSITION_WITH_INTERNAL_SWAP);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const IncreasepositionwithinternalswapArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<IncreasepositionwithinternalswapArgs>(),
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
    invoke_signed::<20>(&instruction, &account_views, signers)
}

/// CPI: decreasePosition4
#[inline(always)]
pub fn decrease_position4<'a>(
    accounts: &Decreaseposition4Accounts<'a>, args: &Decreaseposition4Args,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<Decreaseposition4Args>()];
    data[0..8].copy_from_slice(&DECREASE_POSITION4);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const Decreaseposition4Args as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<Decreaseposition4Args>(),
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

/// CPI: decreasePositionWithInternalSwap
#[inline(always)]
pub fn decrease_position_with_internal_swap<'a>(
    accounts: &DecreasepositionwithinternalswapAccounts<'a>, args: &DecreasepositionwithinternalswapArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<DecreasepositionwithinternalswapArgs>()];
    data[0..8].copy_from_slice(&DECREASE_POSITION_WITH_INTERNAL_SWAP);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const DecreasepositionwithinternalswapArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<DecreasepositionwithinternalswapArgs>(),
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
    invoke_signed::<22>(&instruction, &account_views, signers)
}

/// CPI: decreasePositionWithTpsl
#[inline(always)]
pub fn decrease_position_with_tpsl<'a>(
    accounts: &DecreasepositionwithtpslAccounts<'a>, args: &DecreasepositionwithtpslArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<DecreasepositionwithtpslArgs>()];
    data[0..8].copy_from_slice(&DECREASE_POSITION_WITH_TPSL);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const DecreasepositionwithtpslArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<DecreasepositionwithtpslArgs>(),
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

/// CPI: decreasePositionWithTpslAndInternalSwap
#[inline(always)]
pub fn decrease_position_with_tpsl_and_internal_swap<'a>(
    accounts: &DecreasepositionwithtpslandinternalswapAccounts<'a>, args: &DecreasepositionwithtpslandinternalswapArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<DecreasepositionwithtpslandinternalswapArgs>()];
    data[0..8].copy_from_slice(&DECREASE_POSITION_WITH_TPSL_AND_INTERNAL_SWAP);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const DecreasepositionwithtpslandinternalswapArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<DecreasepositionwithtpslandinternalswapArgs>(),
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
    invoke_signed::<19>(&instruction, &account_views, signers)
}

/// CPI: liquidateFullPosition4
#[inline(always)]
pub fn liquidate_full_position4<'a>(
    accounts: &Liquidatefullposition4Accounts<'a>, args: &Liquidatefullposition4Args,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<Liquidatefullposition4Args>()];
    data[0..8].copy_from_slice(&LIQUIDATE_FULL_POSITION4);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const Liquidatefullposition4Args as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<Liquidatefullposition4Args>(),
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

/// CPI: refreshAssetsUnderManagement
#[inline(always)]
pub fn refresh_assets_under_management<'a>(
    accounts: &RefreshassetsundermanagementAccounts<'a>, args: &RefreshassetsundermanagementArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<RefreshassetsundermanagementArgs>()];
    data[0..8].copy_from_slice(&REFRESH_ASSETS_UNDER_MANAGEMENT);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const RefreshassetsundermanagementArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<RefreshassetsundermanagementArgs>(),
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

/// CPI: setMaxGlobalSizes
#[inline(always)]
pub fn set_max_global_sizes<'a>(
    accounts: &SetmaxglobalsizesAccounts<'a>, args: &SetmaxglobalsizesArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SetmaxglobalsizesArgs>()];
    data[0..8].copy_from_slice(&SET_MAX_GLOBAL_SIZES);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SetmaxglobalsizesArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SetmaxglobalsizesArgs>(),
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

/// CPI: instantCreateTpsl
#[inline(always)]
pub fn instant_create_tpsl<'a>(
    accounts: &InstantcreatetpslAccounts<'a>, args: &InstantcreatetpslArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<InstantcreatetpslArgs>()];
    data[0..8].copy_from_slice(&INSTANT_CREATE_TPSL);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const InstantcreatetpslArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<InstantcreatetpslArgs>(),
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
    invoke_signed::<20>(&instruction, &account_views, signers)
}

/// CPI: instantCreateLimitOrder
#[inline(always)]
pub fn instant_create_limit_order<'a>(
    accounts: &InstantcreatelimitorderAccounts<'a>, args: &InstantcreatelimitorderArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<InstantcreatelimitorderArgs>()];
    data[0..8].copy_from_slice(&INSTANT_CREATE_LIMIT_ORDER);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const InstantcreatelimitorderArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<InstantcreatelimitorderArgs>(),
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
    invoke_signed::<20>(&instruction, &account_views, signers)
}

/// CPI: instantIncreasePosition
#[inline(always)]
pub fn instant_increase_position<'a>(
    accounts: &InstantincreasepositionAccounts<'a>, args: &InstantincreasepositionArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<InstantincreasepositionArgs>()];
    data[0..8].copy_from_slice(&INSTANT_INCREASE_POSITION);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const InstantincreasepositionArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<InstantincreasepositionArgs>(),
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
    invoke_signed::<20>(&instruction, &account_views, signers)
}

/// CPI: instantDecreasePosition
#[inline(always)]
pub fn instant_decrease_position<'a>(
    accounts: &InstantdecreasepositionAccounts<'a>, args: &InstantdecreasepositionArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<InstantdecreasepositionArgs>()];
    data[0..8].copy_from_slice(&INSTANT_DECREASE_POSITION);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const InstantdecreasepositionArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<InstantdecreasepositionArgs>(),
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
    invoke_signed::<22>(&instruction, &account_views, signers)
}

/// CPI: instantDecreasePosition2
#[inline(always)]
pub fn instant_decrease_position2<'a>(
    accounts: &Instantdecreaseposition2Accounts<'a>, args: &Instantdecreaseposition2Args,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<Instantdecreaseposition2Args>()];
    data[0..8].copy_from_slice(&INSTANT_DECREASE_POSITION2);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const Instantdecreaseposition2Args as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<Instantdecreaseposition2Args>(),
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
    invoke_signed::<22>(&instruction, &account_views, signers)
}

/// CPI: instantUpdateLimitOrder
#[inline(always)]
pub fn instant_update_limit_order<'a>(
    accounts: &InstantupdatelimitorderAccounts<'a>, args: &InstantupdatelimitorderArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<InstantupdatelimitorderArgs>()];
    data[0..8].copy_from_slice(&INSTANT_UPDATE_LIMIT_ORDER);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const InstantupdatelimitorderArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<InstantupdatelimitorderArgs>(),
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

/// CPI: instantUpdateTpsl
#[inline(always)]
pub fn instant_update_tpsl<'a>(
    accounts: &InstantupdatetpslAccounts<'a>, args: &InstantupdatetpslArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<InstantupdatetpslArgs>()];
    data[0..8].copy_from_slice(&INSTANT_UPDATE_TPSL);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const InstantupdatetpslArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<InstantupdatetpslArgs>(),
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

/// CPI: getAddLiquidityAmountAndFee2
#[inline(always)]
pub fn get_add_liquidity_amount_and_fee2<'a>(
    accounts: &Getaddliquidityamountandfee2Accounts<'a>, args: &Getaddliquidityamountandfee2Args,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<Getaddliquidityamountandfee2Args>()];
    data[0..8].copy_from_slice(&GET_ADD_LIQUIDITY_AMOUNT_AND_FEE2);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const Getaddliquidityamountandfee2Args as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<Getaddliquidityamountandfee2Args>(),
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

/// CPI: getRemoveLiquidityAmountAndFee2
#[inline(always)]
pub fn get_remove_liquidity_amount_and_fee2<'a>(
    accounts: &Getremoveliquidityamountandfee2Accounts<'a>, args: &Getremoveliquidityamountandfee2Args,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<Getremoveliquidityamountandfee2Args>()];
    data[0..8].copy_from_slice(&GET_REMOVE_LIQUIDITY_AMOUNT_AND_FEE2);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const Getremoveliquidityamountandfee2Args as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<Getremoveliquidityamountandfee2Args>(),
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

/// CPI: getAssetsUnderManagement2
#[inline(always)]
pub fn get_assets_under_management2<'a>(
    accounts: &Getassetsundermanagement2Accounts<'a>, args: &Getassetsundermanagement2Args,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<Getassetsundermanagement2Args>()];
    data[0..8].copy_from_slice(&GET_ASSETS_UNDER_MANAGEMENT2);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const Getassetsundermanagement2Args as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<Getassetsundermanagement2Args>(),
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

/// CPI: borrowFromCustody
#[inline(always)]
pub fn borrow_from_custody<'a>(
    accounts: &BorrowfromcustodyAccounts<'a>, args: &BorrowfromcustodyArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<BorrowfromcustodyArgs>()];
    data[0..8].copy_from_slice(&BORROW_FROM_CUSTODY);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const BorrowfromcustodyArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<BorrowfromcustodyArgs>(),
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

/// CPI: repayToCustody
#[inline(always)]
pub fn repay_to_custody<'a>(
    accounts: &RepaytocustodyAccounts<'a>, args: &RepaytocustodyArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<RepaytocustodyArgs>()];
    data[0..8].copy_from_slice(&REPAY_TO_CUSTODY);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const RepaytocustodyArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<RepaytocustodyArgs>(),
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

/// CPI: depositCollateralForBorrows
#[inline(always)]
pub fn deposit_collateral_for_borrows<'a>(
    accounts: &DepositcollateralforborrowsAccounts<'a>, args: &DepositcollateralforborrowsArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<DepositcollateralforborrowsArgs>()];
    data[0..8].copy_from_slice(&DEPOSIT_COLLATERAL_FOR_BORROWS);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const DepositcollateralforborrowsArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<DepositcollateralforborrowsArgs>(),
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

/// CPI: withdrawCollateralForBorrows
#[inline(always)]
pub fn withdraw_collateral_for_borrows<'a>(
    accounts: &WithdrawcollateralforborrowsAccounts<'a>, args: &WithdrawcollateralforborrowsArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<WithdrawcollateralforborrowsArgs>()];
    data[0..8].copy_from_slice(&WITHDRAW_COLLATERAL_FOR_BORROWS);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const WithdrawcollateralforborrowsArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<WithdrawcollateralforborrowsArgs>(),
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

/// CPI: liquidateBorrowPosition
#[inline(always)]
pub fn liquidate_borrow_position<'a>(
    accounts: &LiquidateborrowpositionAccounts<'a>, args: &LiquidateborrowpositionArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<LiquidateborrowpositionArgs>()];
    data[0..8].copy_from_slice(&LIQUIDATE_BORROW_POSITION);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const LiquidateborrowpositionArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<LiquidateborrowpositionArgs>(),
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

/// CPI: partialLiquidateBorrowPosition
#[inline(always)]
pub fn partial_liquidate_borrow_position<'a>(
    accounts: &PartialliquidateborrowpositionAccounts<'a>, args: &PartialliquidateborrowpositionArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<PartialliquidateborrowpositionArgs>()];
    data[0..8].copy_from_slice(&PARTIAL_LIQUIDATE_BORROW_POSITION);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const PartialliquidateborrowpositionArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<PartialliquidateborrowpositionArgs>(),
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

/// CPI: closeBorrowPosition
#[inline(always)]
pub fn close_borrow_position<'a>(
    accounts: &CloseborrowpositionAccounts<'a>, args: &CloseborrowpositionArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<CloseborrowpositionArgs>()];
    data[0..8].copy_from_slice(&CLOSE_BORROW_POSITION);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const CloseborrowpositionArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<CloseborrowpositionArgs>(),
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

