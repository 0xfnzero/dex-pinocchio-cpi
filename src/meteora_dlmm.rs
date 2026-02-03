//! CPI module for Meteora DLMM
//!
//! Program: lb_clmm
//! Program ID: LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo
//! Instructions: 74

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
pub const ID: Address = Address::new_from_array(five8_const::decode_32_const("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"));

// ============================================
// Instruction Discriminators
// ============================================
pub const ADD_LIQUIDITY: [u8; 8] = [181, 157, 89, 67, 143, 182, 52, 72];
pub const ADD_LIQUIDITY2: [u8; 8] = [228, 162, 78, 28, 70, 219, 116, 115];
pub const ADD_LIQUIDITY_BY_STRATEGY: [u8; 8] = [7, 3, 150, 127, 148, 40, 61, 200];
pub const ADD_LIQUIDITY_BY_STRATEGY2: [u8; 8] = [3, 221, 149, 218, 111, 141, 118, 213];
pub const ADD_LIQUIDITY_BY_STRATEGY_ONE_SIDE: [u8; 8] = [41, 5, 238, 175, 100, 225, 6, 205];
pub const ADD_LIQUIDITY_BY_WEIGHT: [u8; 8] = [28, 140, 238, 99, 231, 162, 21, 149];
pub const ADD_LIQUIDITY_ONE_SIDE: [u8; 8] = [94, 155, 103, 151, 70, 95, 220, 165];
pub const ADD_LIQUIDITY_ONE_SIDE_PRECISE: [u8; 8] = [161, 194, 103, 84, 171, 71, 250, 154];
pub const ADD_LIQUIDITY_ONE_SIDE_PRECISE2: [u8; 8] = [33, 51, 163, 201, 117, 98, 125, 231];
pub const CLAIM_FEE: [u8; 8] = [169, 32, 79, 137, 136, 232, 70, 137];
pub const CLAIM_FEE2: [u8; 8] = [112, 191, 101, 171, 28, 144, 127, 187];
pub const CLAIM_REWARD: [u8; 8] = [149, 95, 181, 242, 94, 90, 158, 162];
pub const CLAIM_REWARD2: [u8; 8] = [190, 3, 127, 119, 178, 87, 157, 183];
pub const CLOSE_CLAIM_FEE_OPERATOR_ACCOUNT: [u8; 8] = [184, 213, 88, 31, 179, 101, 130, 36];
pub const CLOSE_OPERATOR_ACCOUNT: [u8; 8] = [171, 9, 213, 74, 120, 23, 3, 29];
pub const CLOSE_POSITION: [u8; 8] = [123, 134, 81, 0, 49, 68, 98, 98];
pub const CLOSE_POSITION2: [u8; 8] = [174, 90, 35, 115, 186, 40, 147, 226];
pub const CLOSE_POSITION_IF_EMPTY: [u8; 8] = [59, 124, 212, 118, 91, 152, 110, 157];
pub const CLOSE_PRESET_PARAMETER: [u8; 8] = [4, 148, 145, 100, 134, 26, 181, 61];
pub const CLOSE_PRESET_PARAMETER2: [u8; 8] = [39, 25, 95, 107, 116, 17, 115, 28];
pub const CLOSE_TOKEN_BADGE: [u8; 8] = [108, 146, 86, 110, 179, 254, 10, 104];
pub const CREATE_OPERATOR_ACCOUNT: [u8; 8] = [221, 64, 246, 149, 240, 153, 229, 163];
pub const DECREASE_POSITION_LENGTH: [u8; 8] = [194, 219, 136, 32, 25, 96, 105, 37];
pub const FOR_IDL_TYPE_GENERATION_DO_NOT_CALL: [u8; 8] = [180, 105, 69, 80, 95, 50, 73, 108];
pub const FUND_REWARD: [u8; 8] = [188, 50, 249, 165, 93, 151, 38, 63];
pub const GO_TO_A_BIN: [u8; 8] = [146, 72, 174, 224, 40, 253, 84, 174];
pub const INCREASE_ORACLE_LENGTH: [u8; 8] = [190, 61, 125, 87, 103, 79, 158, 173];
pub const INCREASE_POSITION_LENGTH: [u8; 8] = [80, 83, 117, 211, 66, 13, 33, 149];
pub const INCREASE_POSITION_LENGTH2: [u8; 8] = [255, 210, 204, 71, 115, 137, 225, 113];
pub const INITIALIZE_BIN_ARRAY: [u8; 8] = [35, 86, 19, 185, 78, 212, 75, 211];
pub const INITIALIZE_BIN_ARRAY_BITMAP_EXTENSION: [u8; 8] = [47, 157, 226, 180, 12, 240, 33, 71];
pub const INITIALIZE_CUSTOMIZABLE_PERMISSIONLESS_LB_PAIR: [u8; 8] = [46, 39, 41, 135, 111, 183, 200, 64];
pub const INITIALIZE_CUSTOMIZABLE_PERMISSIONLESS_LB_PAIR2: [u8; 8] = [243, 73, 129, 126, 51, 19, 241, 107];
pub const INITIALIZE_LB_PAIR: [u8; 8] = [45, 154, 237, 210, 221, 15, 166, 92];
pub const INITIALIZE_LB_PAIR2: [u8; 8] = [73, 59, 36, 120, 237, 83, 108, 198];
pub const INITIALIZE_PERMISSION_LB_PAIR: [u8; 8] = [108, 102, 213, 85, 251, 3, 53, 21];
pub const INITIALIZE_POSITION: [u8; 8] = [219, 192, 234, 71, 190, 191, 102, 80];
pub const INITIALIZE_POSITION2: [u8; 8] = [143, 19, 242, 145, 213, 15, 104, 115];
pub const INITIALIZE_POSITION_BY_OPERATOR: [u8; 8] = [251, 189, 190, 244, 117, 254, 35, 148];
pub const INITIALIZE_POSITION_PDA: [u8; 8] = [46, 82, 125, 146, 85, 141, 228, 153];
pub const INITIALIZE_PRESET_PARAMETER: [u8; 8] = [66, 188, 71, 211, 98, 109, 14, 186];
pub const INITIALIZE_REWARD: [u8; 8] = [95, 135, 192, 196, 242, 129, 230, 68];
pub const INITIALIZE_TOKEN_BADGE: [u8; 8] = [253, 77, 205, 95, 27, 224, 89, 223];
pub const MIGRATE_POSITION: [u8; 8] = [15, 132, 59, 50, 199, 6, 251, 46];
pub const REBALANCE_LIQUIDITY: [u8; 8] = [92, 4, 176, 193, 119, 185, 83, 9];
pub const REMOVE_ALL_LIQUIDITY: [u8; 8] = [10, 51, 61, 35, 112, 105, 24, 85];
pub const REMOVE_LIQUIDITY: [u8; 8] = [80, 85, 209, 72, 24, 206, 177, 108];
pub const REMOVE_LIQUIDITY2: [u8; 8] = [230, 215, 82, 127, 241, 101, 227, 146];
pub const REMOVE_LIQUIDITY_BY_RANGE: [u8; 8] = [26, 82, 102, 152, 240, 74, 105, 26];
pub const REMOVE_LIQUIDITY_BY_RANGE2: [u8; 8] = [204, 2, 195, 145, 53, 145, 145, 205];
pub const RESET_BIN_ARRAY_TOMBSTONE_FIELDS: [u8; 8] = [54, 90, 252, 63, 41, 206, 63, 63];
pub const RESET_POOL_TOMBSTONE_FIELDS: [u8; 8] = [246, 109, 19, 120, 108, 113, 68, 252];
pub const RESET_POSITION_TOMBSTONE_FIELDS: [u8; 8] = [206, 6, 51, 218, 211, 30, 159, 84];
pub const SET_ACTIVATION_POINT: [u8; 8] = [91, 249, 15, 165, 26, 129, 254, 125];
pub const SET_PAIR_STATUS: [u8; 8] = [67, 248, 231, 137, 154, 149, 217, 174];
pub const SET_PAIR_STATUS_PERMISSIONLESS: [u8; 8] = [78, 59, 152, 211, 70, 183, 46, 208];
pub const SET_PRE_ACTIVATION_DURATION: [u8; 8] = [165, 61, 201, 244, 130, 159, 22, 100];
pub const SET_PRE_ACTIVATION_SWAP_ADDRESS: [u8; 8] = [57, 139, 47, 123, 216, 80, 223, 10];
pub const SWAP: [u8; 8] = [248, 198, 158, 145, 225, 117, 135, 200];
pub const SWAP2: [u8; 8] = [65, 75, 63, 76, 235, 91, 91, 136];
pub const SWAP_EXACT_OUT: [u8; 8] = [250, 73, 101, 33, 38, 207, 75, 184];
pub const SWAP_EXACT_OUT2: [u8; 8] = [43, 215, 247, 132, 137, 60, 243, 81];
pub const SWAP_WITH_PRICE_IMPACT: [u8; 8] = [56, 173, 230, 208, 173, 228, 156, 205];
pub const SWAP_WITH_PRICE_IMPACT2: [u8; 8] = [74, 98, 192, 214, 177, 51, 75, 51];
pub const UPDATE_BASE_FEE_PARAMETERS: [u8; 8] = [75, 168, 223, 161, 16, 195, 3, 47];
pub const UPDATE_DYNAMIC_FEE_PARAMETERS: [u8; 8] = [92, 161, 46, 246, 255, 189, 22, 22];
pub const UPDATE_FEES_AND_REWARD2: [u8; 8] = [32, 142, 184, 154, 103, 65, 184, 88];
pub const UPDATE_FEES_AND_REWARDS: [u8; 8] = [154, 230, 250, 13, 236, 209, 75, 223];
pub const UPDATE_POSITION_OPERATOR: [u8; 8] = [202, 184, 103, 143, 180, 191, 116, 217];
pub const UPDATE_REWARD_DURATION: [u8; 8] = [138, 174, 196, 169, 213, 235, 254, 107];
pub const UPDATE_REWARD_FUNDER: [u8; 8] = [211, 28, 48, 32, 215, 160, 35, 23];
pub const WITHDRAW_INELIGIBLE_REWARD: [u8; 8] = [148, 206, 42, 195, 247, 49, 103, 8];
pub const WITHDRAW_PROTOCOL_FEE: [u8; 8] = [158, 201, 158, 189, 33, 93, 162, 103];
pub const ZAP_PROTOCOL_FEE: [u8; 8] = [213, 155, 187, 34, 56, 182, 91, 240];

// ============================================
// Instruction Arguments
// ============================================
/// Arguments for `add_liquidity`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct AddLiquidityArgs {
    pub liquidity_parameter: [u8; 32],
}

/// Arguments for `add_liquidity2`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct AddLiquidity2Args {
    pub liquidity_parameter: [u8; 32],
    pub remaining_accounts_info: [u8; 32],
}

/// Arguments for `add_liquidity_by_strategy`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct AddLiquidityByStrategyArgs {
    pub liquidity_parameter: [u8; 32],
}

/// Arguments for `add_liquidity_by_strategy2`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct AddLiquidityByStrategy2Args {
    pub liquidity_parameter: [u8; 32],
    pub remaining_accounts_info: [u8; 32],
}

/// Arguments for `add_liquidity_by_strategy_one_side`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct AddLiquidityByStrategyOneSideArgs {
    pub liquidity_parameter: [u8; 32],
}

/// Arguments for `add_liquidity_by_weight`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct AddLiquidityByWeightArgs {
    pub liquidity_parameter: [u8; 32],
}

/// Arguments for `add_liquidity_one_side`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct AddLiquidityOneSideArgs {
    pub liquidity_parameter: [u8; 32],
}

/// Arguments for `add_liquidity_one_side_precise`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct AddLiquidityOneSidePreciseArgs {
    pub parameter: [u8; 32],
}

/// Arguments for `add_liquidity_one_side_precise2`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct AddLiquidityOneSidePrecise2Args {
    pub liquidity_parameter: [u8; 32],
    pub remaining_accounts_info: [u8; 32],
}

/// Arguments for `claim_fee2`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct ClaimFee2Args {
    pub min_bin_id: i32,
    pub max_bin_id: i32,
    pub remaining_accounts_info: [u8; 32],
}

/// Arguments for `claim_reward`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct ClaimRewardArgs {
    pub reward_index: u64,
}

/// Arguments for `claim_reward2`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct ClaimReward2Args {
    pub reward_index: u64,
    pub min_bin_id: i32,
    pub max_bin_id: i32,
    pub remaining_accounts_info: [u8; 32],
}

/// Arguments for `create_operator_account`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct CreateOperatorAccountArgs {
    pub permission: u128,
}

/// Arguments for `decrease_position_length`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct DecreasePositionLengthArgs {
    pub length_to_remove: u16,
    pub side: u8,
}

/// Arguments for `for_idl_type_generation_do_not_call`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct ForIdlTypeGenerationDoNotCallArgs {
    pub ix: [u8; 32],
}

/// Arguments for `fund_reward`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct FundRewardArgs {
    pub reward_index: u64,
    pub amount: u64,
    pub carry_forward: bool,
    pub remaining_accounts_info: [u8; 32],
}

/// Arguments for `go_to_a_bin`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct GoToABinArgs {
    pub bin_id: i32,
}

/// Arguments for `increase_oracle_length`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct IncreaseOracleLengthArgs {
    pub length_to_add: u64,
}

/// Arguments for `increase_position_length`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct IncreasePositionLengthArgs {
    pub length_to_add: u16,
    pub side: u8,
}

/// Arguments for `increase_position_length2`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct IncreasePositionLength2Args {
    pub minimum_upper_bin_id: i32,
}

/// Arguments for `initialize_bin_array`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct InitializeBinArrayArgs {
    pub index: i64,
}

/// Arguments for `initialize_customizable_permissionless_lb_pair`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct InitializeCustomizablePermissionlessLbPairArgs {
    pub params: [u8; 32],
}

/// Arguments for `initialize_customizable_permissionless_lb_pair2`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct InitializeCustomizablePermissionlessLbPair2Args {
    pub params: [u8; 32],
}

/// Arguments for `initialize_lb_pair`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct InitializeLbPairArgs {
    pub active_id: i32,
    pub bin_step: u16,
}

/// Arguments for `initialize_lb_pair2`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct InitializeLbPair2Args {
    pub params: [u8; 32],
}

/// Arguments for `initialize_permission_lb_pair`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct InitializePermissionLbPairArgs {
    pub ix_data: [u8; 32],
}

/// Arguments for `initialize_position`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct InitializePositionArgs {
    pub lower_bin_id: i32,
    pub width: i32,
}

/// Arguments for `initialize_position2`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct InitializePosition2Args {
    pub lower_bin_id: i32,
    pub width: i32,
}

/// Arguments for `initialize_position_by_operator`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct InitializePositionByOperatorArgs {
    pub lower_bin_id: i32,
    pub width: i32,
    pub fee_owner: [u8; 32],
    pub lock_release_point: u64,
}

/// Arguments for `initialize_position_pda`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct InitializePositionPdaArgs {
    pub lower_bin_id: i32,
    pub width: i32,
}

/// Arguments for `initialize_preset_parameter`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct InitializePresetParameterArgs {
    pub ix: [u8; 32],
}

/// Arguments for `initialize_reward`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct InitializeRewardArgs {
    pub reward_index: u64,
    pub reward_duration: u64,
    pub funder: [u8; 32],
}

/// Arguments for `rebalance_liquidity`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct RebalanceLiquidityArgs {
    pub params: [u8; 32],
    pub remaining_accounts_info: [u8; 32],
}

/// Arguments for `remove_liquidity`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct RemoveLiquidityArgs {
    pub bin_liquidity_removal: [u8; 32],
}

/// Arguments for `remove_liquidity2`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct RemoveLiquidity2Args {
    pub bin_liquidity_removal: [u8; 32],
    pub remaining_accounts_info: [u8; 32],
}

/// Arguments for `remove_liquidity_by_range`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct RemoveLiquidityByRangeArgs {
    pub from_bin_id: i32,
    pub to_bin_id: i32,
    pub bps_to_remove: u16,
}

/// Arguments for `remove_liquidity_by_range2`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct RemoveLiquidityByRange2Args {
    pub from_bin_id: i32,
    pub to_bin_id: i32,
    pub bps_to_remove: u16,
    pub remaining_accounts_info: [u8; 32],
}

/// Arguments for `set_activation_point`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SetActivationPointArgs {
    pub activation_point: u64,
}

/// Arguments for `set_pair_status`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SetPairStatusArgs {
    pub status: u8,
}

/// Arguments for `set_pair_status_permissionless`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SetPairStatusPermissionlessArgs {
    pub status: u8,
}

/// Arguments for `set_pre_activation_duration`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SetPreActivationDurationArgs {
    pub pre_activation_duration: u64,
}

/// Arguments for `set_pre_activation_swap_address`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SetPreActivationSwapAddressArgs {
    pub pre_activation_swap_address: [u8; 32],
}

/// Arguments for `swap`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SwapArgs {
    pub amount_in: u64,
    pub min_amount_out: u64,
}

/// Arguments for `swap2`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct Swap2Args {
    pub amount_in: u64,
    pub min_amount_out: u64,
    pub remaining_accounts_info: [u8; 32],
}

/// Arguments for `swap_exact_out`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SwapExactOutArgs {
    pub max_in_amount: u64,
    pub out_amount: u64,
}

/// Arguments for `swap_exact_out2`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SwapExactOut2Args {
    pub max_in_amount: u64,
    pub out_amount: u64,
    pub remaining_accounts_info: [u8; 32],
}

/// Arguments for `swap_with_price_impact`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SwapWithPriceImpactArgs {
    pub amount_in: u64,
    pub active_id: Option<i32>,
    pub max_price_impact_bps: u16,
}

/// Arguments for `swap_with_price_impact2`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SwapWithPriceImpact2Args {
    pub amount_in: u64,
    pub active_id: Option<i32>,
    pub max_price_impact_bps: u16,
    pub remaining_accounts_info: [u8; 32],
}

/// Arguments for `update_base_fee_parameters`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct UpdateBaseFeeParametersArgs {
    pub fee_parameter: [u8; 32],
}

/// Arguments for `update_dynamic_fee_parameters`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct UpdateDynamicFeeParametersArgs {
    pub fee_parameter: [u8; 32],
}

/// Arguments for `update_fees_and_reward2`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct UpdateFeesAndReward2Args {
    pub min_bin_id: i32,
    pub max_bin_id: i32,
}

/// Arguments for `update_position_operator`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct UpdatePositionOperatorArgs {
    pub operator: [u8; 32],
}

/// Arguments for `update_reward_duration`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct UpdateRewardDurationArgs {
    pub reward_index: u64,
    pub new_duration: u64,
}

/// Arguments for `update_reward_funder`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct UpdateRewardFunderArgs {
    pub reward_index: u64,
    pub new_funder: [u8; 32],
}

/// Arguments for `withdraw_ineligible_reward`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct WithdrawIneligibleRewardArgs {
    pub reward_index: u64,
    pub remaining_accounts_info: [u8; 32],
}

/// Arguments for `withdraw_protocol_fee`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct WithdrawProtocolFeeArgs {
    pub max_amount_x: u64,
    pub max_amount_y: u64,
    pub remaining_accounts_info: [u8; 32],
}

/// Arguments for `zap_protocol_fee`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct ZapProtocolFeeArgs {
    pub max_amount: u64,
    pub remaining_accounts_info: [u8; 32],
}


// ============================================
// Instruction Accounts
// ============================================
/// Accounts for `add_liquidity`
pub struct AddLiquidityAccounts<'a> {
    /// position
    pub position: &'a AccountView,
    /// lb_pair
    pub lb_pair: &'a AccountView,
    /// bin_array_bitmap_extension
    pub bin_array_bitmap_extension: &'a AccountView,
    /// user_token_x
    pub user_token_x: &'a AccountView,
    /// user_token_y
    pub user_token_y: &'a AccountView,
    /// reserve_x
    pub reserve_x: &'a AccountView,
    /// reserve_y
    pub reserve_y: &'a AccountView,
    /// token_x_mint
    pub token_x_mint: &'a AccountView,
    /// token_y_mint
    pub token_y_mint: &'a AccountView,
    /// bin_array_lower
    pub bin_array_lower: &'a AccountView,
    /// bin_array_upper
    pub bin_array_upper: &'a AccountView,
    /// sender
    pub sender: &'a AccountView,
    /// token_x_program
    pub token_x_program: &'a AccountView,
    /// token_y_program
    pub token_y_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> AddLiquidityAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 16] {
        [
            InstructionAccount::writable(self.position.address()),
            InstructionAccount::writable(self.lb_pair.address()),
            InstructionAccount::writable(self.bin_array_bitmap_extension.address()),
            InstructionAccount::writable(self.user_token_x.address()),
            InstructionAccount::writable(self.user_token_y.address()),
            InstructionAccount::writable(self.reserve_x.address()),
            InstructionAccount::writable(self.reserve_y.address()),
            InstructionAccount::readonly(self.token_x_mint.address()),
            InstructionAccount::readonly(self.token_y_mint.address()),
            InstructionAccount::writable(self.bin_array_lower.address()),
            InstructionAccount::writable(self.bin_array_upper.address()),
            InstructionAccount::readonly_signer(self.sender.address()),
            InstructionAccount::readonly(self.token_x_program.address()),
            InstructionAccount::readonly(self.token_y_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 16] {
        [
            self.position, self.lb_pair, self.bin_array_bitmap_extension, self.user_token_x, self.user_token_y, self.reserve_x, self.reserve_y, self.token_x_mint, self.token_y_mint, self.bin_array_lower, self.bin_array_upper, self.sender, self.token_x_program, self.token_y_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `add_liquidity2`
pub struct AddLiquidity2Accounts<'a> {
    /// position
    pub position: &'a AccountView,
    /// lb_pair
    pub lb_pair: &'a AccountView,
    /// bin_array_bitmap_extension
    pub bin_array_bitmap_extension: &'a AccountView,
    /// user_token_x
    pub user_token_x: &'a AccountView,
    /// user_token_y
    pub user_token_y: &'a AccountView,
    /// reserve_x
    pub reserve_x: &'a AccountView,
    /// reserve_y
    pub reserve_y: &'a AccountView,
    /// token_x_mint
    pub token_x_mint: &'a AccountView,
    /// token_y_mint
    pub token_y_mint: &'a AccountView,
    /// sender
    pub sender: &'a AccountView,
    /// token_x_program
    pub token_x_program: &'a AccountView,
    /// token_y_program
    pub token_y_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> AddLiquidity2Accounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 14] {
        [
            InstructionAccount::writable(self.position.address()),
            InstructionAccount::writable(self.lb_pair.address()),
            InstructionAccount::writable(self.bin_array_bitmap_extension.address()),
            InstructionAccount::writable(self.user_token_x.address()),
            InstructionAccount::writable(self.user_token_y.address()),
            InstructionAccount::writable(self.reserve_x.address()),
            InstructionAccount::writable(self.reserve_y.address()),
            InstructionAccount::readonly(self.token_x_mint.address()),
            InstructionAccount::readonly(self.token_y_mint.address()),
            InstructionAccount::readonly_signer(self.sender.address()),
            InstructionAccount::readonly(self.token_x_program.address()),
            InstructionAccount::readonly(self.token_y_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 14] {
        [
            self.position, self.lb_pair, self.bin_array_bitmap_extension, self.user_token_x, self.user_token_y, self.reserve_x, self.reserve_y, self.token_x_mint, self.token_y_mint, self.sender, self.token_x_program, self.token_y_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `add_liquidity_by_strategy`
pub struct AddLiquidityByStrategyAccounts<'a> {
    /// position
    pub position: &'a AccountView,
    /// lb_pair
    pub lb_pair: &'a AccountView,
    /// bin_array_bitmap_extension
    pub bin_array_bitmap_extension: &'a AccountView,
    /// user_token_x
    pub user_token_x: &'a AccountView,
    /// user_token_y
    pub user_token_y: &'a AccountView,
    /// reserve_x
    pub reserve_x: &'a AccountView,
    /// reserve_y
    pub reserve_y: &'a AccountView,
    /// token_x_mint
    pub token_x_mint: &'a AccountView,
    /// token_y_mint
    pub token_y_mint: &'a AccountView,
    /// bin_array_lower
    pub bin_array_lower: &'a AccountView,
    /// bin_array_upper
    pub bin_array_upper: &'a AccountView,
    /// sender
    pub sender: &'a AccountView,
    /// token_x_program
    pub token_x_program: &'a AccountView,
    /// token_y_program
    pub token_y_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> AddLiquidityByStrategyAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 16] {
        [
            InstructionAccount::writable(self.position.address()),
            InstructionAccount::writable(self.lb_pair.address()),
            InstructionAccount::writable(self.bin_array_bitmap_extension.address()),
            InstructionAccount::writable(self.user_token_x.address()),
            InstructionAccount::writable(self.user_token_y.address()),
            InstructionAccount::writable(self.reserve_x.address()),
            InstructionAccount::writable(self.reserve_y.address()),
            InstructionAccount::readonly(self.token_x_mint.address()),
            InstructionAccount::readonly(self.token_y_mint.address()),
            InstructionAccount::writable(self.bin_array_lower.address()),
            InstructionAccount::writable(self.bin_array_upper.address()),
            InstructionAccount::readonly_signer(self.sender.address()),
            InstructionAccount::readonly(self.token_x_program.address()),
            InstructionAccount::readonly(self.token_y_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 16] {
        [
            self.position, self.lb_pair, self.bin_array_bitmap_extension, self.user_token_x, self.user_token_y, self.reserve_x, self.reserve_y, self.token_x_mint, self.token_y_mint, self.bin_array_lower, self.bin_array_upper, self.sender, self.token_x_program, self.token_y_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `add_liquidity_by_strategy2`
pub struct AddLiquidityByStrategy2Accounts<'a> {
    /// position
    pub position: &'a AccountView,
    /// lb_pair
    pub lb_pair: &'a AccountView,
    /// bin_array_bitmap_extension
    pub bin_array_bitmap_extension: &'a AccountView,
    /// user_token_x
    pub user_token_x: &'a AccountView,
    /// user_token_y
    pub user_token_y: &'a AccountView,
    /// reserve_x
    pub reserve_x: &'a AccountView,
    /// reserve_y
    pub reserve_y: &'a AccountView,
    /// token_x_mint
    pub token_x_mint: &'a AccountView,
    /// token_y_mint
    pub token_y_mint: &'a AccountView,
    /// sender
    pub sender: &'a AccountView,
    /// token_x_program
    pub token_x_program: &'a AccountView,
    /// token_y_program
    pub token_y_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> AddLiquidityByStrategy2Accounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 14] {
        [
            InstructionAccount::writable(self.position.address()),
            InstructionAccount::writable(self.lb_pair.address()),
            InstructionAccount::writable(self.bin_array_bitmap_extension.address()),
            InstructionAccount::writable(self.user_token_x.address()),
            InstructionAccount::writable(self.user_token_y.address()),
            InstructionAccount::writable(self.reserve_x.address()),
            InstructionAccount::writable(self.reserve_y.address()),
            InstructionAccount::readonly(self.token_x_mint.address()),
            InstructionAccount::readonly(self.token_y_mint.address()),
            InstructionAccount::readonly_signer(self.sender.address()),
            InstructionAccount::readonly(self.token_x_program.address()),
            InstructionAccount::readonly(self.token_y_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 14] {
        [
            self.position, self.lb_pair, self.bin_array_bitmap_extension, self.user_token_x, self.user_token_y, self.reserve_x, self.reserve_y, self.token_x_mint, self.token_y_mint, self.sender, self.token_x_program, self.token_y_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `add_liquidity_by_strategy_one_side`
pub struct AddLiquidityByStrategyOneSideAccounts<'a> {
    /// position
    pub position: &'a AccountView,
    /// lb_pair
    pub lb_pair: &'a AccountView,
    /// bin_array_bitmap_extension
    pub bin_array_bitmap_extension: &'a AccountView,
    /// user_token
    pub user_token: &'a AccountView,
    /// reserve
    pub reserve: &'a AccountView,
    /// token_mint
    pub token_mint: &'a AccountView,
    /// bin_array_lower
    pub bin_array_lower: &'a AccountView,
    /// bin_array_upper
    pub bin_array_upper: &'a AccountView,
    /// sender
    pub sender: &'a AccountView,
    /// token_program
    pub token_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> AddLiquidityByStrategyOneSideAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 12] {
        [
            InstructionAccount::writable(self.position.address()),
            InstructionAccount::writable(self.lb_pair.address()),
            InstructionAccount::writable(self.bin_array_bitmap_extension.address()),
            InstructionAccount::writable(self.user_token.address()),
            InstructionAccount::writable(self.reserve.address()),
            InstructionAccount::readonly(self.token_mint.address()),
            InstructionAccount::writable(self.bin_array_lower.address()),
            InstructionAccount::writable(self.bin_array_upper.address()),
            InstructionAccount::readonly_signer(self.sender.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 12] {
        [
            self.position, self.lb_pair, self.bin_array_bitmap_extension, self.user_token, self.reserve, self.token_mint, self.bin_array_lower, self.bin_array_upper, self.sender, self.token_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `add_liquidity_by_weight`
pub struct AddLiquidityByWeightAccounts<'a> {
    /// position
    pub position: &'a AccountView,
    /// lb_pair
    pub lb_pair: &'a AccountView,
    /// bin_array_bitmap_extension
    pub bin_array_bitmap_extension: &'a AccountView,
    /// user_token_x
    pub user_token_x: &'a AccountView,
    /// user_token_y
    pub user_token_y: &'a AccountView,
    /// reserve_x
    pub reserve_x: &'a AccountView,
    /// reserve_y
    pub reserve_y: &'a AccountView,
    /// token_x_mint
    pub token_x_mint: &'a AccountView,
    /// token_y_mint
    pub token_y_mint: &'a AccountView,
    /// bin_array_lower
    pub bin_array_lower: &'a AccountView,
    /// bin_array_upper
    pub bin_array_upper: &'a AccountView,
    /// sender
    pub sender: &'a AccountView,
    /// token_x_program
    pub token_x_program: &'a AccountView,
    /// token_y_program
    pub token_y_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> AddLiquidityByWeightAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 16] {
        [
            InstructionAccount::writable(self.position.address()),
            InstructionAccount::writable(self.lb_pair.address()),
            InstructionAccount::writable(self.bin_array_bitmap_extension.address()),
            InstructionAccount::writable(self.user_token_x.address()),
            InstructionAccount::writable(self.user_token_y.address()),
            InstructionAccount::writable(self.reserve_x.address()),
            InstructionAccount::writable(self.reserve_y.address()),
            InstructionAccount::readonly(self.token_x_mint.address()),
            InstructionAccount::readonly(self.token_y_mint.address()),
            InstructionAccount::writable(self.bin_array_lower.address()),
            InstructionAccount::writable(self.bin_array_upper.address()),
            InstructionAccount::readonly_signer(self.sender.address()),
            InstructionAccount::readonly(self.token_x_program.address()),
            InstructionAccount::readonly(self.token_y_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 16] {
        [
            self.position, self.lb_pair, self.bin_array_bitmap_extension, self.user_token_x, self.user_token_y, self.reserve_x, self.reserve_y, self.token_x_mint, self.token_y_mint, self.bin_array_lower, self.bin_array_upper, self.sender, self.token_x_program, self.token_y_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `add_liquidity_one_side`
pub struct AddLiquidityOneSideAccounts<'a> {
    /// position
    pub position: &'a AccountView,
    /// lb_pair
    pub lb_pair: &'a AccountView,
    /// bin_array_bitmap_extension
    pub bin_array_bitmap_extension: &'a AccountView,
    /// user_token
    pub user_token: &'a AccountView,
    /// reserve
    pub reserve: &'a AccountView,
    /// token_mint
    pub token_mint: &'a AccountView,
    /// bin_array_lower
    pub bin_array_lower: &'a AccountView,
    /// bin_array_upper
    pub bin_array_upper: &'a AccountView,
    /// sender
    pub sender: &'a AccountView,
    /// token_program
    pub token_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> AddLiquidityOneSideAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 12] {
        [
            InstructionAccount::writable(self.position.address()),
            InstructionAccount::writable(self.lb_pair.address()),
            InstructionAccount::writable(self.bin_array_bitmap_extension.address()),
            InstructionAccount::writable(self.user_token.address()),
            InstructionAccount::writable(self.reserve.address()),
            InstructionAccount::readonly(self.token_mint.address()),
            InstructionAccount::writable(self.bin_array_lower.address()),
            InstructionAccount::writable(self.bin_array_upper.address()),
            InstructionAccount::readonly_signer(self.sender.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 12] {
        [
            self.position, self.lb_pair, self.bin_array_bitmap_extension, self.user_token, self.reserve, self.token_mint, self.bin_array_lower, self.bin_array_upper, self.sender, self.token_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `add_liquidity_one_side_precise`
pub struct AddLiquidityOneSidePreciseAccounts<'a> {
    /// position
    pub position: &'a AccountView,
    /// lb_pair
    pub lb_pair: &'a AccountView,
    /// bin_array_bitmap_extension
    pub bin_array_bitmap_extension: &'a AccountView,
    /// user_token
    pub user_token: &'a AccountView,
    /// reserve
    pub reserve: &'a AccountView,
    /// token_mint
    pub token_mint: &'a AccountView,
    /// bin_array_lower
    pub bin_array_lower: &'a AccountView,
    /// bin_array_upper
    pub bin_array_upper: &'a AccountView,
    /// sender
    pub sender: &'a AccountView,
    /// token_program
    pub token_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> AddLiquidityOneSidePreciseAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 12] {
        [
            InstructionAccount::writable(self.position.address()),
            InstructionAccount::writable(self.lb_pair.address()),
            InstructionAccount::writable(self.bin_array_bitmap_extension.address()),
            InstructionAccount::writable(self.user_token.address()),
            InstructionAccount::writable(self.reserve.address()),
            InstructionAccount::readonly(self.token_mint.address()),
            InstructionAccount::writable(self.bin_array_lower.address()),
            InstructionAccount::writable(self.bin_array_upper.address()),
            InstructionAccount::readonly_signer(self.sender.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 12] {
        [
            self.position, self.lb_pair, self.bin_array_bitmap_extension, self.user_token, self.reserve, self.token_mint, self.bin_array_lower, self.bin_array_upper, self.sender, self.token_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `add_liquidity_one_side_precise2`
pub struct AddLiquidityOneSidePrecise2Accounts<'a> {
    /// position
    pub position: &'a AccountView,
    /// lb_pair
    pub lb_pair: &'a AccountView,
    /// bin_array_bitmap_extension
    pub bin_array_bitmap_extension: &'a AccountView,
    /// user_token
    pub user_token: &'a AccountView,
    /// reserve
    pub reserve: &'a AccountView,
    /// token_mint
    pub token_mint: &'a AccountView,
    /// sender
    pub sender: &'a AccountView,
    /// token_program
    pub token_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> AddLiquidityOneSidePrecise2Accounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 10] {
        [
            InstructionAccount::writable(self.position.address()),
            InstructionAccount::writable(self.lb_pair.address()),
            InstructionAccount::writable(self.bin_array_bitmap_extension.address()),
            InstructionAccount::writable(self.user_token.address()),
            InstructionAccount::writable(self.reserve.address()),
            InstructionAccount::readonly(self.token_mint.address()),
            InstructionAccount::readonly_signer(self.sender.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 10] {
        [
            self.position, self.lb_pair, self.bin_array_bitmap_extension, self.user_token, self.reserve, self.token_mint, self.sender, self.token_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `claim_fee`
pub struct ClaimFeeAccounts<'a> {
    /// lb_pair
    pub lb_pair: &'a AccountView,
    /// position
    pub position: &'a AccountView,
    /// bin_array_lower
    pub bin_array_lower: &'a AccountView,
    /// bin_array_upper
    pub bin_array_upper: &'a AccountView,
    /// sender
    pub sender: &'a AccountView,
    /// reserve_x
    pub reserve_x: &'a AccountView,
    /// reserve_y
    pub reserve_y: &'a AccountView,
    /// user_token_x
    pub user_token_x: &'a AccountView,
    /// user_token_y
    pub user_token_y: &'a AccountView,
    /// token_x_mint
    pub token_x_mint: &'a AccountView,
    /// token_y_mint
    pub token_y_mint: &'a AccountView,
    /// token_program
    pub token_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> ClaimFeeAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 14] {
        [
            InstructionAccount::writable(self.lb_pair.address()),
            InstructionAccount::writable(self.position.address()),
            InstructionAccount::writable(self.bin_array_lower.address()),
            InstructionAccount::writable(self.bin_array_upper.address()),
            InstructionAccount::readonly_signer(self.sender.address()),
            InstructionAccount::writable(self.reserve_x.address()),
            InstructionAccount::writable(self.reserve_y.address()),
            InstructionAccount::writable(self.user_token_x.address()),
            InstructionAccount::writable(self.user_token_y.address()),
            InstructionAccount::readonly(self.token_x_mint.address()),
            InstructionAccount::readonly(self.token_y_mint.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 14] {
        [
            self.lb_pair, self.position, self.bin_array_lower, self.bin_array_upper, self.sender, self.reserve_x, self.reserve_y, self.user_token_x, self.user_token_y, self.token_x_mint, self.token_y_mint, self.token_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `claim_fee2`
pub struct ClaimFee2Accounts<'a> {
    /// lb_pair
    pub lb_pair: &'a AccountView,
    /// position
    pub position: &'a AccountView,
    /// sender
    pub sender: &'a AccountView,
    /// reserve_x
    pub reserve_x: &'a AccountView,
    /// reserve_y
    pub reserve_y: &'a AccountView,
    /// user_token_x
    pub user_token_x: &'a AccountView,
    /// user_token_y
    pub user_token_y: &'a AccountView,
    /// token_x_mint
    pub token_x_mint: &'a AccountView,
    /// token_y_mint
    pub token_y_mint: &'a AccountView,
    /// token_program_x
    pub token_program_x: &'a AccountView,
    /// token_program_y
    pub token_program_y: &'a AccountView,
    /// memo_program
    pub memo_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> ClaimFee2Accounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 14] {
        [
            InstructionAccount::writable(self.lb_pair.address()),
            InstructionAccount::writable(self.position.address()),
            InstructionAccount::readonly_signer(self.sender.address()),
            InstructionAccount::writable(self.reserve_x.address()),
            InstructionAccount::writable(self.reserve_y.address()),
            InstructionAccount::writable(self.user_token_x.address()),
            InstructionAccount::writable(self.user_token_y.address()),
            InstructionAccount::readonly(self.token_x_mint.address()),
            InstructionAccount::readonly(self.token_y_mint.address()),
            InstructionAccount::readonly(self.token_program_x.address()),
            InstructionAccount::readonly(self.token_program_y.address()),
            InstructionAccount::readonly(self.memo_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 14] {
        [
            self.lb_pair, self.position, self.sender, self.reserve_x, self.reserve_y, self.user_token_x, self.user_token_y, self.token_x_mint, self.token_y_mint, self.token_program_x, self.token_program_y, self.memo_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `claim_reward`
pub struct ClaimRewardAccounts<'a> {
    /// lb_pair
    pub lb_pair: &'a AccountView,
    /// position
    pub position: &'a AccountView,
    /// bin_array_lower
    pub bin_array_lower: &'a AccountView,
    /// bin_array_upper
    pub bin_array_upper: &'a AccountView,
    /// sender
    pub sender: &'a AccountView,
    /// reward_vault
    pub reward_vault: &'a AccountView,
    /// reward_mint
    pub reward_mint: &'a AccountView,
    /// user_token_account
    pub user_token_account: &'a AccountView,
    /// token_program
    pub token_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> ClaimRewardAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 11] {
        [
            InstructionAccount::writable(self.lb_pair.address()),
            InstructionAccount::writable(self.position.address()),
            InstructionAccount::writable(self.bin_array_lower.address()),
            InstructionAccount::writable(self.bin_array_upper.address()),
            InstructionAccount::readonly_signer(self.sender.address()),
            InstructionAccount::writable(self.reward_vault.address()),
            InstructionAccount::readonly(self.reward_mint.address()),
            InstructionAccount::writable(self.user_token_account.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 11] {
        [
            self.lb_pair, self.position, self.bin_array_lower, self.bin_array_upper, self.sender, self.reward_vault, self.reward_mint, self.user_token_account, self.token_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `claim_reward2`
pub struct ClaimReward2Accounts<'a> {
    /// lb_pair
    pub lb_pair: &'a AccountView,
    /// position
    pub position: &'a AccountView,
    /// sender
    pub sender: &'a AccountView,
    /// reward_vault
    pub reward_vault: &'a AccountView,
    /// reward_mint
    pub reward_mint: &'a AccountView,
    /// user_token_account
    pub user_token_account: &'a AccountView,
    /// token_program
    pub token_program: &'a AccountView,
    /// memo_program
    pub memo_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> ClaimReward2Accounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 10] {
        [
            InstructionAccount::writable(self.lb_pair.address()),
            InstructionAccount::writable(self.position.address()),
            InstructionAccount::readonly_signer(self.sender.address()),
            InstructionAccount::writable(self.reward_vault.address()),
            InstructionAccount::readonly(self.reward_mint.address()),
            InstructionAccount::writable(self.user_token_account.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.memo_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 10] {
        [
            self.lb_pair, self.position, self.sender, self.reward_vault, self.reward_mint, self.user_token_account, self.token_program, self.memo_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `close_claim_fee_operator_account`
pub struct CloseClaimFeeOperatorAccountAccounts<'a> {
    /// claim_fee_operator
    pub claim_fee_operator: &'a AccountView,
    /// rent_receiver
    pub rent_receiver: &'a AccountView,
    /// signer
    pub signer: &'a AccountView,
}

impl<'a> CloseClaimFeeOperatorAccountAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 3] {
        [
            InstructionAccount::writable(self.claim_fee_operator.address()),
            InstructionAccount::writable(self.rent_receiver.address()),
            InstructionAccount::readonly_signer(self.signer.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 3] {
        [
            self.claim_fee_operator, self.rent_receiver, self.signer
        ]
    }
}

/// Accounts for `close_operator_account`
pub struct CloseOperatorAccountAccounts<'a> {
    /// operator
    pub operator: &'a AccountView,
    /// signer
    pub signer: &'a AccountView,
    /// rent_receiver
    pub rent_receiver: &'a AccountView,
}

impl<'a> CloseOperatorAccountAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 3] {
        [
            InstructionAccount::writable(self.operator.address()),
            InstructionAccount::readonly_signer(self.signer.address()),
            InstructionAccount::writable(self.rent_receiver.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 3] {
        [
            self.operator, self.signer, self.rent_receiver
        ]
    }
}

/// Accounts for `close_position`
pub struct ClosePositionAccounts<'a> {
    /// position
    pub position: &'a AccountView,
    /// lb_pair
    pub lb_pair: &'a AccountView,
    /// bin_array_lower
    pub bin_array_lower: &'a AccountView,
    /// bin_array_upper
    pub bin_array_upper: &'a AccountView,
    /// sender
    pub sender: &'a AccountView,
    /// rent_receiver
    pub rent_receiver: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> ClosePositionAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 8] {
        [
            InstructionAccount::writable(self.position.address()),
            InstructionAccount::writable(self.lb_pair.address()),
            InstructionAccount::writable(self.bin_array_lower.address()),
            InstructionAccount::writable(self.bin_array_upper.address()),
            InstructionAccount::readonly_signer(self.sender.address()),
            InstructionAccount::writable(self.rent_receiver.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 8] {
        [
            self.position, self.lb_pair, self.bin_array_lower, self.bin_array_upper, self.sender, self.rent_receiver, self.event_authority, self.program
        ]
    }
}

/// Accounts for `close_position2`
pub struct ClosePosition2Accounts<'a> {
    /// position
    pub position: &'a AccountView,
    /// sender
    pub sender: &'a AccountView,
    /// rent_receiver
    pub rent_receiver: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> ClosePosition2Accounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 5] {
        [
            InstructionAccount::writable(self.position.address()),
            InstructionAccount::readonly_signer(self.sender.address()),
            InstructionAccount::writable(self.rent_receiver.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 5] {
        [
            self.position, self.sender, self.rent_receiver, self.event_authority, self.program
        ]
    }
}

/// Accounts for `close_position_if_empty`
pub struct ClosePositionIfEmptyAccounts<'a> {
    /// position
    pub position: &'a AccountView,
    /// sender
    pub sender: &'a AccountView,
    /// rent_receiver
    pub rent_receiver: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> ClosePositionIfEmptyAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 5] {
        [
            InstructionAccount::writable(self.position.address()),
            InstructionAccount::readonly_signer(self.sender.address()),
            InstructionAccount::writable(self.rent_receiver.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 5] {
        [
            self.position, self.sender, self.rent_receiver, self.event_authority, self.program
        ]
    }
}

/// Accounts for `close_preset_parameter`
pub struct ClosePresetParameterAccounts<'a> {
    /// preset_parameter
    pub preset_parameter: &'a AccountView,
    /// operator
    pub operator: &'a AccountView,
    /// signer
    pub signer: &'a AccountView,
    /// rent_receiver
    pub rent_receiver: &'a AccountView,
}

impl<'a> ClosePresetParameterAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 4] {
        [
            InstructionAccount::writable(self.preset_parameter.address()),
            InstructionAccount::readonly(self.operator.address()),
            InstructionAccount::readonly_signer(self.signer.address()),
            InstructionAccount::writable(self.rent_receiver.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 4] {
        [
            self.preset_parameter, self.operator, self.signer, self.rent_receiver
        ]
    }
}

/// Accounts for `close_preset_parameter2`
pub struct ClosePresetParameter2Accounts<'a> {
    /// preset_parameter
    pub preset_parameter: &'a AccountView,
    /// operator
    pub operator: &'a AccountView,
    /// signer
    pub signer: &'a AccountView,
    /// rent_receiver
    pub rent_receiver: &'a AccountView,
}

impl<'a> ClosePresetParameter2Accounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 4] {
        [
            InstructionAccount::writable(self.preset_parameter.address()),
            InstructionAccount::readonly(self.operator.address()),
            InstructionAccount::readonly_signer(self.signer.address()),
            InstructionAccount::writable(self.rent_receiver.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 4] {
        [
            self.preset_parameter, self.operator, self.signer, self.rent_receiver
        ]
    }
}

/// Accounts for `close_token_badge`
pub struct CloseTokenBadgeAccounts<'a> {
    /// token_badge
    pub token_badge: &'a AccountView,
    /// rent_receiver
    pub rent_receiver: &'a AccountView,
    /// operator
    pub operator: &'a AccountView,
    /// signer
    pub signer: &'a AccountView,
}

impl<'a> CloseTokenBadgeAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 4] {
        [
            InstructionAccount::writable(self.token_badge.address()),
            InstructionAccount::writable(self.rent_receiver.address()),
            InstructionAccount::readonly(self.operator.address()),
            InstructionAccount::readonly_signer(self.signer.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 4] {
        [
            self.token_badge, self.rent_receiver, self.operator, self.signer
        ]
    }
}

/// Accounts for `create_operator_account`
pub struct CreateOperatorAccountAccounts<'a> {
    /// operator
    pub operator: &'a AccountView,
    /// whitelisted_signer
    pub whitelisted_signer: &'a AccountView,
    /// signer
    pub signer: &'a AccountView,
    /// payer
    pub payer: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
}

impl<'a> CreateOperatorAccountAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 5] {
        [
            InstructionAccount::writable(self.operator.address()),
            InstructionAccount::readonly(self.whitelisted_signer.address()),
            InstructionAccount::readonly_signer(self.signer.address()),
            InstructionAccount::writable_signer(self.payer.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 5] {
        [
            self.operator, self.whitelisted_signer, self.signer, self.payer, self.system_program
        ]
    }
}

/// Accounts for `decrease_position_length`
pub struct DecreasePositionLengthAccounts<'a> {
    /// rent_receiver
    pub rent_receiver: &'a AccountView,
    /// position
    pub position: &'a AccountView,
    /// owner
    pub owner: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> DecreasePositionLengthAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 6] {
        [
            InstructionAccount::writable(self.rent_receiver.address()),
            InstructionAccount::writable(self.position.address()),
            InstructionAccount::readonly_signer(self.owner.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 6] {
        [
            self.rent_receiver, self.position, self.owner, self.system_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `for_idl_type_generation_do_not_call`
pub struct ForIdlTypeGenerationDoNotCallAccounts<'a> {
    /// dummy_zc_account
    pub dummy_zc_account: &'a AccountView,
}

impl<'a> ForIdlTypeGenerationDoNotCallAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 1] {
        [
            InstructionAccount::readonly(self.dummy_zc_account.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 1] {
        [
            self.dummy_zc_account
        ]
    }
}

/// Accounts for `fund_reward`
pub struct FundRewardAccounts<'a> {
    /// lb_pair
    pub lb_pair: &'a AccountView,
    /// reward_vault
    pub reward_vault: &'a AccountView,
    /// reward_mint
    pub reward_mint: &'a AccountView,
    /// funder_token_account
    pub funder_token_account: &'a AccountView,
    /// funder
    pub funder: &'a AccountView,
    /// bin_array
    pub bin_array: &'a AccountView,
    /// token_program
    pub token_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> FundRewardAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 9] {
        [
            InstructionAccount::writable(self.lb_pair.address()),
            InstructionAccount::writable(self.reward_vault.address()),
            InstructionAccount::readonly(self.reward_mint.address()),
            InstructionAccount::writable(self.funder_token_account.address()),
            InstructionAccount::readonly_signer(self.funder.address()),
            InstructionAccount::writable(self.bin_array.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 9] {
        [
            self.lb_pair, self.reward_vault, self.reward_mint, self.funder_token_account, self.funder, self.bin_array, self.token_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `go_to_a_bin`
pub struct GoToABinAccounts<'a> {
    /// lb_pair
    pub lb_pair: &'a AccountView,
    /// bin_array_bitmap_extension
    pub bin_array_bitmap_extension: &'a AccountView,
    /// from_bin_array
    pub from_bin_array: &'a AccountView,
    /// to_bin_array
    pub to_bin_array: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> GoToABinAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 6] {
        [
            InstructionAccount::writable(self.lb_pair.address()),
            InstructionAccount::readonly(self.bin_array_bitmap_extension.address()),
            InstructionAccount::readonly(self.from_bin_array.address()),
            InstructionAccount::readonly(self.to_bin_array.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 6] {
        [
            self.lb_pair, self.bin_array_bitmap_extension, self.from_bin_array, self.to_bin_array, self.event_authority, self.program
        ]
    }
}

/// Accounts for `increase_oracle_length`
pub struct IncreaseOracleLengthAccounts<'a> {
    /// oracle
    pub oracle: &'a AccountView,
    /// funder
    pub funder: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> IncreaseOracleLengthAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 5] {
        [
            InstructionAccount::writable(self.oracle.address()),
            InstructionAccount::writable_signer(self.funder.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 5] {
        [
            self.oracle, self.funder, self.system_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `increase_position_length`
pub struct IncreasePositionLengthAccounts<'a> {
    /// funder
    pub funder: &'a AccountView,
    /// lb_pair
    pub lb_pair: &'a AccountView,
    /// position
    pub position: &'a AccountView,
    /// owner
    pub owner: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> IncreasePositionLengthAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 7] {
        [
            InstructionAccount::writable_signer(self.funder.address()),
            InstructionAccount::readonly(self.lb_pair.address()),
            InstructionAccount::writable(self.position.address()),
            InstructionAccount::readonly_signer(self.owner.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 7] {
        [
            self.funder, self.lb_pair, self.position, self.owner, self.system_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `increase_position_length2`
pub struct IncreasePositionLength2Accounts<'a> {
    /// funder
    pub funder: &'a AccountView,
    /// lb_pair
    pub lb_pair: &'a AccountView,
    /// position
    pub position: &'a AccountView,
    /// owner
    pub owner: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> IncreasePositionLength2Accounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 7] {
        [
            InstructionAccount::writable_signer(self.funder.address()),
            InstructionAccount::readonly(self.lb_pair.address()),
            InstructionAccount::writable(self.position.address()),
            InstructionAccount::readonly_signer(self.owner.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 7] {
        [
            self.funder, self.lb_pair, self.position, self.owner, self.system_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `initialize_bin_array`
pub struct InitializeBinArrayAccounts<'a> {
    /// lb_pair
    pub lb_pair: &'a AccountView,
    /// bin_array
    pub bin_array: &'a AccountView,
    /// funder
    pub funder: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
}

impl<'a> InitializeBinArrayAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 4] {
        [
            InstructionAccount::readonly(self.lb_pair.address()),
            InstructionAccount::writable(self.bin_array.address()),
            InstructionAccount::writable_signer(self.funder.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 4] {
        [
            self.lb_pair, self.bin_array, self.funder, self.system_program
        ]
    }
}

/// Accounts for `initialize_bin_array_bitmap_extension`
pub struct InitializeBinArrayBitmapExtensionAccounts<'a> {
    /// lb_pair
    pub lb_pair: &'a AccountView,
    /// Initialize an account to store if a bin array is initialized.
    pub bin_array_bitmap_extension: &'a AccountView,
    /// funder
    pub funder: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
    /// rent
    pub rent: &'a AccountView,
}

impl<'a> InitializeBinArrayBitmapExtensionAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 5] {
        [
            InstructionAccount::readonly(self.lb_pair.address()),
            InstructionAccount::writable(self.bin_array_bitmap_extension.address()),
            InstructionAccount::writable_signer(self.funder.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.rent.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 5] {
        [
            self.lb_pair, self.bin_array_bitmap_extension, self.funder, self.system_program, self.rent
        ]
    }
}

/// Accounts for `initialize_customizable_permissionless_lb_pair`
pub struct InitializeCustomizablePermissionlessLbPairAccounts<'a> {
    /// lb_pair
    pub lb_pair: &'a AccountView,
    /// bin_array_bitmap_extension
    pub bin_array_bitmap_extension: &'a AccountView,
    /// token_mint_x
    pub token_mint_x: &'a AccountView,
    /// token_mint_y
    pub token_mint_y: &'a AccountView,
    /// reserve_x
    pub reserve_x: &'a AccountView,
    /// reserve_y
    pub reserve_y: &'a AccountView,
    /// oracle
    pub oracle: &'a AccountView,
    /// user_token_x
    pub user_token_x: &'a AccountView,
    /// funder
    pub funder: &'a AccountView,
    /// token_program
    pub token_program: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
    /// user_token_y
    pub user_token_y: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> InitializeCustomizablePermissionlessLbPairAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 14] {
        [
            InstructionAccount::writable(self.lb_pair.address()),
            InstructionAccount::writable(self.bin_array_bitmap_extension.address()),
            InstructionAccount::readonly(self.token_mint_x.address()),
            InstructionAccount::readonly(self.token_mint_y.address()),
            InstructionAccount::writable(self.reserve_x.address()),
            InstructionAccount::writable(self.reserve_y.address()),
            InstructionAccount::writable(self.oracle.address()),
            InstructionAccount::readonly(self.user_token_x.address()),
            InstructionAccount::writable_signer(self.funder.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.user_token_y.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 14] {
        [
            self.lb_pair, self.bin_array_bitmap_extension, self.token_mint_x, self.token_mint_y, self.reserve_x, self.reserve_y, self.oracle, self.user_token_x, self.funder, self.token_program, self.system_program, self.user_token_y, self.event_authority, self.program
        ]
    }
}

/// Accounts for `initialize_customizable_permissionless_lb_pair2`
pub struct InitializeCustomizablePermissionlessLbPair2Accounts<'a> {
    /// lb_pair
    pub lb_pair: &'a AccountView,
    /// bin_array_bitmap_extension
    pub bin_array_bitmap_extension: &'a AccountView,
    /// token_mint_x
    pub token_mint_x: &'a AccountView,
    /// token_mint_y
    pub token_mint_y: &'a AccountView,
    /// reserve_x
    pub reserve_x: &'a AccountView,
    /// reserve_y
    pub reserve_y: &'a AccountView,
    /// oracle
    pub oracle: &'a AccountView,
    /// user_token_x
    pub user_token_x: &'a AccountView,
    /// funder
    pub funder: &'a AccountView,
    /// token_badge_x
    pub token_badge_x: &'a AccountView,
    /// token_badge_y
    pub token_badge_y: &'a AccountView,
    /// token_program_x
    pub token_program_x: &'a AccountView,
    /// token_program_y
    pub token_program_y: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
    /// user_token_y
    pub user_token_y: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> InitializeCustomizablePermissionlessLbPair2Accounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 17] {
        [
            InstructionAccount::writable(self.lb_pair.address()),
            InstructionAccount::writable(self.bin_array_bitmap_extension.address()),
            InstructionAccount::readonly(self.token_mint_x.address()),
            InstructionAccount::readonly(self.token_mint_y.address()),
            InstructionAccount::writable(self.reserve_x.address()),
            InstructionAccount::writable(self.reserve_y.address()),
            InstructionAccount::writable(self.oracle.address()),
            InstructionAccount::readonly(self.user_token_x.address()),
            InstructionAccount::writable_signer(self.funder.address()),
            InstructionAccount::readonly(self.token_badge_x.address()),
            InstructionAccount::readonly(self.token_badge_y.address()),
            InstructionAccount::readonly(self.token_program_x.address()),
            InstructionAccount::readonly(self.token_program_y.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.user_token_y.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 17] {
        [
            self.lb_pair, self.bin_array_bitmap_extension, self.token_mint_x, self.token_mint_y, self.reserve_x, self.reserve_y, self.oracle, self.user_token_x, self.funder, self.token_badge_x, self.token_badge_y, self.token_program_x, self.token_program_y, self.system_program, self.user_token_y, self.event_authority, self.program
        ]
    }
}

/// Accounts for `initialize_lb_pair`
pub struct InitializeLbPairAccounts<'a> {
    /// lb_pair
    pub lb_pair: &'a AccountView,
    /// bin_array_bitmap_extension
    pub bin_array_bitmap_extension: &'a AccountView,
    /// token_mint_x
    pub token_mint_x: &'a AccountView,
    /// token_mint_y
    pub token_mint_y: &'a AccountView,
    /// reserve_x
    pub reserve_x: &'a AccountView,
    /// reserve_y
    pub reserve_y: &'a AccountView,
    /// oracle
    pub oracle: &'a AccountView,
    /// preset_parameter
    pub preset_parameter: &'a AccountView,
    /// funder
    pub funder: &'a AccountView,
    /// token_program
    pub token_program: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
    /// rent
    pub rent: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> InitializeLbPairAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 14] {
        [
            InstructionAccount::writable(self.lb_pair.address()),
            InstructionAccount::writable(self.bin_array_bitmap_extension.address()),
            InstructionAccount::readonly(self.token_mint_x.address()),
            InstructionAccount::readonly(self.token_mint_y.address()),
            InstructionAccount::writable(self.reserve_x.address()),
            InstructionAccount::writable(self.reserve_y.address()),
            InstructionAccount::writable(self.oracle.address()),
            InstructionAccount::readonly(self.preset_parameter.address()),
            InstructionAccount::writable_signer(self.funder.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.rent.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 14] {
        [
            self.lb_pair, self.bin_array_bitmap_extension, self.token_mint_x, self.token_mint_y, self.reserve_x, self.reserve_y, self.oracle, self.preset_parameter, self.funder, self.token_program, self.system_program, self.rent, self.event_authority, self.program
        ]
    }
}

/// Accounts for `initialize_lb_pair2`
pub struct InitializeLbPair2Accounts<'a> {
    /// lb_pair
    pub lb_pair: &'a AccountView,
    /// bin_array_bitmap_extension
    pub bin_array_bitmap_extension: &'a AccountView,
    /// token_mint_x
    pub token_mint_x: &'a AccountView,
    /// token_mint_y
    pub token_mint_y: &'a AccountView,
    /// reserve_x
    pub reserve_x: &'a AccountView,
    /// reserve_y
    pub reserve_y: &'a AccountView,
    /// oracle
    pub oracle: &'a AccountView,
    /// preset_parameter
    pub preset_parameter: &'a AccountView,
    /// funder
    pub funder: &'a AccountView,
    /// token_badge_x
    pub token_badge_x: &'a AccountView,
    /// token_badge_y
    pub token_badge_y: &'a AccountView,
    /// token_program_x
    pub token_program_x: &'a AccountView,
    /// token_program_y
    pub token_program_y: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> InitializeLbPair2Accounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 16] {
        [
            InstructionAccount::writable(self.lb_pair.address()),
            InstructionAccount::writable(self.bin_array_bitmap_extension.address()),
            InstructionAccount::readonly(self.token_mint_x.address()),
            InstructionAccount::readonly(self.token_mint_y.address()),
            InstructionAccount::writable(self.reserve_x.address()),
            InstructionAccount::writable(self.reserve_y.address()),
            InstructionAccount::writable(self.oracle.address()),
            InstructionAccount::readonly(self.preset_parameter.address()),
            InstructionAccount::writable_signer(self.funder.address()),
            InstructionAccount::readonly(self.token_badge_x.address()),
            InstructionAccount::readonly(self.token_badge_y.address()),
            InstructionAccount::readonly(self.token_program_x.address()),
            InstructionAccount::readonly(self.token_program_y.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 16] {
        [
            self.lb_pair, self.bin_array_bitmap_extension, self.token_mint_x, self.token_mint_y, self.reserve_x, self.reserve_y, self.oracle, self.preset_parameter, self.funder, self.token_badge_x, self.token_badge_y, self.token_program_x, self.token_program_y, self.system_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `initialize_permission_lb_pair`
pub struct InitializePermissionLbPairAccounts<'a> {
    /// base
    pub base: &'a AccountView,
    /// lb_pair
    pub lb_pair: &'a AccountView,
    /// bin_array_bitmap_extension
    pub bin_array_bitmap_extension: &'a AccountView,
    /// token_mint_x
    pub token_mint_x: &'a AccountView,
    /// token_mint_y
    pub token_mint_y: &'a AccountView,
    /// reserve_x
    pub reserve_x: &'a AccountView,
    /// reserve_y
    pub reserve_y: &'a AccountView,
    /// oracle
    pub oracle: &'a AccountView,
    /// payer
    pub payer: &'a AccountView,
    /// operator
    pub operator: &'a AccountView,
    /// signer
    pub signer: &'a AccountView,
    /// token_badge_x
    pub token_badge_x: &'a AccountView,
    /// token_badge_y
    pub token_badge_y: &'a AccountView,
    /// token_program_x
    pub token_program_x: &'a AccountView,
    /// token_program_y
    pub token_program_y: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> InitializePermissionLbPairAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 18] {
        [
            InstructionAccount::readonly_signer(self.base.address()),
            InstructionAccount::writable(self.lb_pair.address()),
            InstructionAccount::writable(self.bin_array_bitmap_extension.address()),
            InstructionAccount::readonly(self.token_mint_x.address()),
            InstructionAccount::readonly(self.token_mint_y.address()),
            InstructionAccount::writable(self.reserve_x.address()),
            InstructionAccount::writable(self.reserve_y.address()),
            InstructionAccount::writable(self.oracle.address()),
            InstructionAccount::writable_signer(self.payer.address()),
            InstructionAccount::readonly(self.operator.address()),
            InstructionAccount::readonly_signer(self.signer.address()),
            InstructionAccount::readonly(self.token_badge_x.address()),
            InstructionAccount::readonly(self.token_badge_y.address()),
            InstructionAccount::readonly(self.token_program_x.address()),
            InstructionAccount::readonly(self.token_program_y.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 18] {
        [
            self.base, self.lb_pair, self.bin_array_bitmap_extension, self.token_mint_x, self.token_mint_y, self.reserve_x, self.reserve_y, self.oracle, self.payer, self.operator, self.signer, self.token_badge_x, self.token_badge_y, self.token_program_x, self.token_program_y, self.system_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `initialize_position`
pub struct InitializePositionAccounts<'a> {
    /// payer
    pub payer: &'a AccountView,
    /// position
    pub position: &'a AccountView,
    /// lb_pair
    pub lb_pair: &'a AccountView,
    /// owner
    pub owner: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
    /// rent
    pub rent: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> InitializePositionAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 8] {
        [
            InstructionAccount::writable_signer(self.payer.address()),
            InstructionAccount::writable_signer(self.position.address()),
            InstructionAccount::readonly(self.lb_pair.address()),
            InstructionAccount::readonly_signer(self.owner.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.rent.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 8] {
        [
            self.payer, self.position, self.lb_pair, self.owner, self.system_program, self.rent, self.event_authority, self.program
        ]
    }
}

/// Accounts for `initialize_position2`
pub struct InitializePosition2Accounts<'a> {
    /// payer
    pub payer: &'a AccountView,
    /// position
    pub position: &'a AccountView,
    /// lb_pair
    pub lb_pair: &'a AccountView,
    /// owner
    pub owner: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> InitializePosition2Accounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 7] {
        [
            InstructionAccount::writable_signer(self.payer.address()),
            InstructionAccount::writable_signer(self.position.address()),
            InstructionAccount::readonly(self.lb_pair.address()),
            InstructionAccount::readonly_signer(self.owner.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 7] {
        [
            self.payer, self.position, self.lb_pair, self.owner, self.system_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `initialize_position_by_operator`
pub struct InitializePositionByOperatorAccounts<'a> {
    /// payer
    pub payer: &'a AccountView,
    /// base
    pub base: &'a AccountView,
    /// position
    pub position: &'a AccountView,
    /// lb_pair
    pub lb_pair: &'a AccountView,
    /// owner
    pub owner: &'a AccountView,
    /// operator
    pub operator: &'a AccountView,
    /// operator_token_x
    pub operator_token_x: &'a AccountView,
    /// owner_token_x
    pub owner_token_x: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> InitializePositionByOperatorAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 11] {
        [
            InstructionAccount::writable_signer(self.payer.address()),
            InstructionAccount::readonly_signer(self.base.address()),
            InstructionAccount::writable(self.position.address()),
            InstructionAccount::readonly(self.lb_pair.address()),
            InstructionAccount::readonly(self.owner.address()),
            InstructionAccount::readonly_signer(self.operator.address()),
            InstructionAccount::readonly(self.operator_token_x.address()),
            InstructionAccount::readonly(self.owner_token_x.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 11] {
        [
            self.payer, self.base, self.position, self.lb_pair, self.owner, self.operator, self.operator_token_x, self.owner_token_x, self.system_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `initialize_position_pda`
pub struct InitializePositionPdaAccounts<'a> {
    /// payer
    pub payer: &'a AccountView,
    /// base
    pub base: &'a AccountView,
    /// position
    pub position: &'a AccountView,
    /// lb_pair
    pub lb_pair: &'a AccountView,
    /// owner
    pub owner: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
    /// rent
    pub rent: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> InitializePositionPdaAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 9] {
        [
            InstructionAccount::writable_signer(self.payer.address()),
            InstructionAccount::readonly_signer(self.base.address()),
            InstructionAccount::writable(self.position.address()),
            InstructionAccount::readonly(self.lb_pair.address()),
            InstructionAccount::readonly_signer(self.owner.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.rent.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 9] {
        [
            self.payer, self.base, self.position, self.lb_pair, self.owner, self.system_program, self.rent, self.event_authority, self.program
        ]
    }
}

/// Accounts for `initialize_preset_parameter`
pub struct InitializePresetParameterAccounts<'a> {
    /// preset_parameter
    pub preset_parameter: &'a AccountView,
    /// operator
    pub operator: &'a AccountView,
    /// signer
    pub signer: &'a AccountView,
    /// payer
    pub payer: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
}

impl<'a> InitializePresetParameterAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 5] {
        [
            InstructionAccount::writable(self.preset_parameter.address()),
            InstructionAccount::readonly(self.operator.address()),
            InstructionAccount::readonly_signer(self.signer.address()),
            InstructionAccount::writable_signer(self.payer.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 5] {
        [
            self.preset_parameter, self.operator, self.signer, self.payer, self.system_program
        ]
    }
}

/// Accounts for `initialize_reward`
pub struct InitializeRewardAccounts<'a> {
    /// lb_pair
    pub lb_pair: &'a AccountView,
    /// reward_vault
    pub reward_vault: &'a AccountView,
    /// reward_mint
    pub reward_mint: &'a AccountView,
    /// token_badge
    pub token_badge: &'a AccountView,
    /// operator
    pub operator: &'a AccountView,
    /// signer
    pub signer: &'a AccountView,
    /// payer
    pub payer: &'a AccountView,
    /// token_program
    pub token_program: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> InitializeRewardAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 11] {
        [
            InstructionAccount::writable(self.lb_pair.address()),
            InstructionAccount::writable(self.reward_vault.address()),
            InstructionAccount::readonly(self.reward_mint.address()),
            InstructionAccount::readonly(self.token_badge.address()),
            InstructionAccount::readonly(self.operator.address()),
            InstructionAccount::readonly_signer(self.signer.address()),
            InstructionAccount::writable_signer(self.payer.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 11] {
        [
            self.lb_pair, self.reward_vault, self.reward_mint, self.token_badge, self.operator, self.signer, self.payer, self.token_program, self.system_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `initialize_token_badge`
pub struct InitializeTokenBadgeAccounts<'a> {
    /// token_mint
    pub token_mint: &'a AccountView,
    /// token_badge
    pub token_badge: &'a AccountView,
    /// operator
    pub operator: &'a AccountView,
    /// signer
    pub signer: &'a AccountView,
    /// payer
    pub payer: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
}

impl<'a> InitializeTokenBadgeAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 6] {
        [
            InstructionAccount::readonly(self.token_mint.address()),
            InstructionAccount::writable(self.token_badge.address()),
            InstructionAccount::readonly(self.operator.address()),
            InstructionAccount::readonly_signer(self.signer.address()),
            InstructionAccount::writable_signer(self.payer.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 6] {
        [
            self.token_mint, self.token_badge, self.operator, self.signer, self.payer, self.system_program
        ]
    }
}

/// Accounts for `migrate_position`
pub struct MigratePositionAccounts<'a> {
    /// position_v2
    pub position_v2: &'a AccountView,
    /// position_v1
    pub position_v1: &'a AccountView,
    /// lb_pair
    pub lb_pair: &'a AccountView,
    /// bin_array_lower
    pub bin_array_lower: &'a AccountView,
    /// bin_array_upper
    pub bin_array_upper: &'a AccountView,
    /// signer_and_payer
    pub signer_and_payer: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
    /// rent_receiver
    pub rent_receiver: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> MigratePositionAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 10] {
        [
            InstructionAccount::writable_signer(self.position_v2.address()),
            InstructionAccount::writable(self.position_v1.address()),
            InstructionAccount::readonly(self.lb_pair.address()),
            InstructionAccount::writable(self.bin_array_lower.address()),
            InstructionAccount::writable(self.bin_array_upper.address()),
            InstructionAccount::writable_signer(self.signer_and_payer.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::writable(self.rent_receiver.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 10] {
        [
            self.position_v2, self.position_v1, self.lb_pair, self.bin_array_lower, self.bin_array_upper, self.signer_and_payer, self.system_program, self.rent_receiver, self.event_authority, self.program
        ]
    }
}

/// Accounts for `rebalance_liquidity`
pub struct RebalanceLiquidityAccounts<'a> {
    /// position
    pub position: &'a AccountView,
    /// lb_pair
    pub lb_pair: &'a AccountView,
    /// bin_array_bitmap_extension
    pub bin_array_bitmap_extension: &'a AccountView,
    /// user_token_x
    pub user_token_x: &'a AccountView,
    /// user_token_y
    pub user_token_y: &'a AccountView,
    /// reserve_x
    pub reserve_x: &'a AccountView,
    /// reserve_y
    pub reserve_y: &'a AccountView,
    /// token_x_mint
    pub token_x_mint: &'a AccountView,
    /// token_y_mint
    pub token_y_mint: &'a AccountView,
    /// owner
    pub owner: &'a AccountView,
    /// rent_payer
    pub rent_payer: &'a AccountView,
    /// token_x_program
    pub token_x_program: &'a AccountView,
    /// token_y_program
    pub token_y_program: &'a AccountView,
    /// memo_program
    pub memo_program: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> RebalanceLiquidityAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 17] {
        [
            InstructionAccount::writable(self.position.address()),
            InstructionAccount::writable(self.lb_pair.address()),
            InstructionAccount::writable(self.bin_array_bitmap_extension.address()),
            InstructionAccount::writable(self.user_token_x.address()),
            InstructionAccount::writable(self.user_token_y.address()),
            InstructionAccount::writable(self.reserve_x.address()),
            InstructionAccount::writable(self.reserve_y.address()),
            InstructionAccount::readonly(self.token_x_mint.address()),
            InstructionAccount::readonly(self.token_y_mint.address()),
            InstructionAccount::readonly_signer(self.owner.address()),
            InstructionAccount::writable_signer(self.rent_payer.address()),
            InstructionAccount::readonly(self.token_x_program.address()),
            InstructionAccount::readonly(self.token_y_program.address()),
            InstructionAccount::readonly(self.memo_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 17] {
        [
            self.position, self.lb_pair, self.bin_array_bitmap_extension, self.user_token_x, self.user_token_y, self.reserve_x, self.reserve_y, self.token_x_mint, self.token_y_mint, self.owner, self.rent_payer, self.token_x_program, self.token_y_program, self.memo_program, self.system_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `remove_all_liquidity`
pub struct RemoveAllLiquidityAccounts<'a> {
    /// position
    pub position: &'a AccountView,
    /// lb_pair
    pub lb_pair: &'a AccountView,
    /// bin_array_bitmap_extension
    pub bin_array_bitmap_extension: &'a AccountView,
    /// user_token_x
    pub user_token_x: &'a AccountView,
    /// user_token_y
    pub user_token_y: &'a AccountView,
    /// reserve_x
    pub reserve_x: &'a AccountView,
    /// reserve_y
    pub reserve_y: &'a AccountView,
    /// token_x_mint
    pub token_x_mint: &'a AccountView,
    /// token_y_mint
    pub token_y_mint: &'a AccountView,
    /// bin_array_lower
    pub bin_array_lower: &'a AccountView,
    /// bin_array_upper
    pub bin_array_upper: &'a AccountView,
    /// sender
    pub sender: &'a AccountView,
    /// token_x_program
    pub token_x_program: &'a AccountView,
    /// token_y_program
    pub token_y_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> RemoveAllLiquidityAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 16] {
        [
            InstructionAccount::writable(self.position.address()),
            InstructionAccount::writable(self.lb_pair.address()),
            InstructionAccount::writable(self.bin_array_bitmap_extension.address()),
            InstructionAccount::writable(self.user_token_x.address()),
            InstructionAccount::writable(self.user_token_y.address()),
            InstructionAccount::writable(self.reserve_x.address()),
            InstructionAccount::writable(self.reserve_y.address()),
            InstructionAccount::readonly(self.token_x_mint.address()),
            InstructionAccount::readonly(self.token_y_mint.address()),
            InstructionAccount::writable(self.bin_array_lower.address()),
            InstructionAccount::writable(self.bin_array_upper.address()),
            InstructionAccount::readonly_signer(self.sender.address()),
            InstructionAccount::readonly(self.token_x_program.address()),
            InstructionAccount::readonly(self.token_y_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 16] {
        [
            self.position, self.lb_pair, self.bin_array_bitmap_extension, self.user_token_x, self.user_token_y, self.reserve_x, self.reserve_y, self.token_x_mint, self.token_y_mint, self.bin_array_lower, self.bin_array_upper, self.sender, self.token_x_program, self.token_y_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `remove_liquidity`
pub struct RemoveLiquidityAccounts<'a> {
    /// position
    pub position: &'a AccountView,
    /// lb_pair
    pub lb_pair: &'a AccountView,
    /// bin_array_bitmap_extension
    pub bin_array_bitmap_extension: &'a AccountView,
    /// user_token_x
    pub user_token_x: &'a AccountView,
    /// user_token_y
    pub user_token_y: &'a AccountView,
    /// reserve_x
    pub reserve_x: &'a AccountView,
    /// reserve_y
    pub reserve_y: &'a AccountView,
    /// token_x_mint
    pub token_x_mint: &'a AccountView,
    /// token_y_mint
    pub token_y_mint: &'a AccountView,
    /// bin_array_lower
    pub bin_array_lower: &'a AccountView,
    /// bin_array_upper
    pub bin_array_upper: &'a AccountView,
    /// sender
    pub sender: &'a AccountView,
    /// token_x_program
    pub token_x_program: &'a AccountView,
    /// token_y_program
    pub token_y_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> RemoveLiquidityAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 16] {
        [
            InstructionAccount::writable(self.position.address()),
            InstructionAccount::writable(self.lb_pair.address()),
            InstructionAccount::writable(self.bin_array_bitmap_extension.address()),
            InstructionAccount::writable(self.user_token_x.address()),
            InstructionAccount::writable(self.user_token_y.address()),
            InstructionAccount::writable(self.reserve_x.address()),
            InstructionAccount::writable(self.reserve_y.address()),
            InstructionAccount::readonly(self.token_x_mint.address()),
            InstructionAccount::readonly(self.token_y_mint.address()),
            InstructionAccount::writable(self.bin_array_lower.address()),
            InstructionAccount::writable(self.bin_array_upper.address()),
            InstructionAccount::readonly_signer(self.sender.address()),
            InstructionAccount::readonly(self.token_x_program.address()),
            InstructionAccount::readonly(self.token_y_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 16] {
        [
            self.position, self.lb_pair, self.bin_array_bitmap_extension, self.user_token_x, self.user_token_y, self.reserve_x, self.reserve_y, self.token_x_mint, self.token_y_mint, self.bin_array_lower, self.bin_array_upper, self.sender, self.token_x_program, self.token_y_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `remove_liquidity2`
pub struct RemoveLiquidity2Accounts<'a> {
    /// position
    pub position: &'a AccountView,
    /// lb_pair
    pub lb_pair: &'a AccountView,
    /// bin_array_bitmap_extension
    pub bin_array_bitmap_extension: &'a AccountView,
    /// user_token_x
    pub user_token_x: &'a AccountView,
    /// user_token_y
    pub user_token_y: &'a AccountView,
    /// reserve_x
    pub reserve_x: &'a AccountView,
    /// reserve_y
    pub reserve_y: &'a AccountView,
    /// token_x_mint
    pub token_x_mint: &'a AccountView,
    /// token_y_mint
    pub token_y_mint: &'a AccountView,
    /// sender
    pub sender: &'a AccountView,
    /// token_x_program
    pub token_x_program: &'a AccountView,
    /// token_y_program
    pub token_y_program: &'a AccountView,
    /// memo_program
    pub memo_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> RemoveLiquidity2Accounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 15] {
        [
            InstructionAccount::writable(self.position.address()),
            InstructionAccount::writable(self.lb_pair.address()),
            InstructionAccount::writable(self.bin_array_bitmap_extension.address()),
            InstructionAccount::writable(self.user_token_x.address()),
            InstructionAccount::writable(self.user_token_y.address()),
            InstructionAccount::writable(self.reserve_x.address()),
            InstructionAccount::writable(self.reserve_y.address()),
            InstructionAccount::readonly(self.token_x_mint.address()),
            InstructionAccount::readonly(self.token_y_mint.address()),
            InstructionAccount::readonly_signer(self.sender.address()),
            InstructionAccount::readonly(self.token_x_program.address()),
            InstructionAccount::readonly(self.token_y_program.address()),
            InstructionAccount::readonly(self.memo_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 15] {
        [
            self.position, self.lb_pair, self.bin_array_bitmap_extension, self.user_token_x, self.user_token_y, self.reserve_x, self.reserve_y, self.token_x_mint, self.token_y_mint, self.sender, self.token_x_program, self.token_y_program, self.memo_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `remove_liquidity_by_range`
pub struct RemoveLiquidityByRangeAccounts<'a> {
    /// position
    pub position: &'a AccountView,
    /// lb_pair
    pub lb_pair: &'a AccountView,
    /// bin_array_bitmap_extension
    pub bin_array_bitmap_extension: &'a AccountView,
    /// user_token_x
    pub user_token_x: &'a AccountView,
    /// user_token_y
    pub user_token_y: &'a AccountView,
    /// reserve_x
    pub reserve_x: &'a AccountView,
    /// reserve_y
    pub reserve_y: &'a AccountView,
    /// token_x_mint
    pub token_x_mint: &'a AccountView,
    /// token_y_mint
    pub token_y_mint: &'a AccountView,
    /// bin_array_lower
    pub bin_array_lower: &'a AccountView,
    /// bin_array_upper
    pub bin_array_upper: &'a AccountView,
    /// sender
    pub sender: &'a AccountView,
    /// token_x_program
    pub token_x_program: &'a AccountView,
    /// token_y_program
    pub token_y_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> RemoveLiquidityByRangeAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 16] {
        [
            InstructionAccount::writable(self.position.address()),
            InstructionAccount::writable(self.lb_pair.address()),
            InstructionAccount::writable(self.bin_array_bitmap_extension.address()),
            InstructionAccount::writable(self.user_token_x.address()),
            InstructionAccount::writable(self.user_token_y.address()),
            InstructionAccount::writable(self.reserve_x.address()),
            InstructionAccount::writable(self.reserve_y.address()),
            InstructionAccount::readonly(self.token_x_mint.address()),
            InstructionAccount::readonly(self.token_y_mint.address()),
            InstructionAccount::writable(self.bin_array_lower.address()),
            InstructionAccount::writable(self.bin_array_upper.address()),
            InstructionAccount::readonly_signer(self.sender.address()),
            InstructionAccount::readonly(self.token_x_program.address()),
            InstructionAccount::readonly(self.token_y_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 16] {
        [
            self.position, self.lb_pair, self.bin_array_bitmap_extension, self.user_token_x, self.user_token_y, self.reserve_x, self.reserve_y, self.token_x_mint, self.token_y_mint, self.bin_array_lower, self.bin_array_upper, self.sender, self.token_x_program, self.token_y_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `remove_liquidity_by_range2`
pub struct RemoveLiquidityByRange2Accounts<'a> {
    /// position
    pub position: &'a AccountView,
    /// lb_pair
    pub lb_pair: &'a AccountView,
    /// bin_array_bitmap_extension
    pub bin_array_bitmap_extension: &'a AccountView,
    /// user_token_x
    pub user_token_x: &'a AccountView,
    /// user_token_y
    pub user_token_y: &'a AccountView,
    /// reserve_x
    pub reserve_x: &'a AccountView,
    /// reserve_y
    pub reserve_y: &'a AccountView,
    /// token_x_mint
    pub token_x_mint: &'a AccountView,
    /// token_y_mint
    pub token_y_mint: &'a AccountView,
    /// sender
    pub sender: &'a AccountView,
    /// token_x_program
    pub token_x_program: &'a AccountView,
    /// token_y_program
    pub token_y_program: &'a AccountView,
    /// memo_program
    pub memo_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> RemoveLiquidityByRange2Accounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 15] {
        [
            InstructionAccount::writable(self.position.address()),
            InstructionAccount::writable(self.lb_pair.address()),
            InstructionAccount::writable(self.bin_array_bitmap_extension.address()),
            InstructionAccount::writable(self.user_token_x.address()),
            InstructionAccount::writable(self.user_token_y.address()),
            InstructionAccount::writable(self.reserve_x.address()),
            InstructionAccount::writable(self.reserve_y.address()),
            InstructionAccount::readonly(self.token_x_mint.address()),
            InstructionAccount::readonly(self.token_y_mint.address()),
            InstructionAccount::readonly_signer(self.sender.address()),
            InstructionAccount::readonly(self.token_x_program.address()),
            InstructionAccount::readonly(self.token_y_program.address()),
            InstructionAccount::readonly(self.memo_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 15] {
        [
            self.position, self.lb_pair, self.bin_array_bitmap_extension, self.user_token_x, self.user_token_y, self.reserve_x, self.reserve_y, self.token_x_mint, self.token_y_mint, self.sender, self.token_x_program, self.token_y_program, self.memo_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `reset_bin_array_tombstone_fields`
pub struct ResetBinArrayTombstoneFieldsAccounts<'a> {
    /// lb_pair
    pub lb_pair: &'a AccountView,
    /// bin_array
    pub bin_array: &'a AccountView,
    /// operator
    pub operator: &'a AccountView,
    /// signer
    pub signer: &'a AccountView,
}

impl<'a> ResetBinArrayTombstoneFieldsAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 4] {
        [
            InstructionAccount::readonly(self.lb_pair.address()),
            InstructionAccount::writable(self.bin_array.address()),
            InstructionAccount::readonly(self.operator.address()),
            InstructionAccount::readonly_signer(self.signer.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 4] {
        [
            self.lb_pair, self.bin_array, self.operator, self.signer
        ]
    }
}

/// Accounts for `reset_pool_tombstone_fields`
pub struct ResetPoolTombstoneFieldsAccounts<'a> {
    /// lb_pair
    pub lb_pair: &'a AccountView,
    /// operator
    pub operator: &'a AccountView,
    /// signer
    pub signer: &'a AccountView,
}

impl<'a> ResetPoolTombstoneFieldsAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 3] {
        [
            InstructionAccount::writable(self.lb_pair.address()),
            InstructionAccount::readonly(self.operator.address()),
            InstructionAccount::readonly_signer(self.signer.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 3] {
        [
            self.lb_pair, self.operator, self.signer
        ]
    }
}

/// Accounts for `reset_position_tombstone_fields`
pub struct ResetPositionTombstoneFieldsAccounts<'a> {
    /// position
    pub position: &'a AccountView,
    /// operator
    pub operator: &'a AccountView,
    /// signer
    pub signer: &'a AccountView,
}

impl<'a> ResetPositionTombstoneFieldsAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 3] {
        [
            InstructionAccount::writable(self.position.address()),
            InstructionAccount::readonly(self.operator.address()),
            InstructionAccount::readonly_signer(self.signer.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 3] {
        [
            self.position, self.operator, self.signer
        ]
    }
}

/// Accounts for `set_activation_point`
pub struct SetActivationPointAccounts<'a> {
    /// lb_pair
    pub lb_pair: &'a AccountView,
    /// signer
    pub signer: &'a AccountView,
}

impl<'a> SetActivationPointAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 2] {
        [
            InstructionAccount::writable(self.lb_pair.address()),
            InstructionAccount::readonly_signer(self.signer.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 2] {
        [
            self.lb_pair, self.signer
        ]
    }
}

/// Accounts for `set_pair_status`
pub struct SetPairStatusAccounts<'a> {
    /// lb_pair
    pub lb_pair: &'a AccountView,
    /// operator
    pub operator: &'a AccountView,
    /// signer
    pub signer: &'a AccountView,
}

impl<'a> SetPairStatusAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 3] {
        [
            InstructionAccount::writable(self.lb_pair.address()),
            InstructionAccount::readonly(self.operator.address()),
            InstructionAccount::readonly_signer(self.signer.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 3] {
        [
            self.lb_pair, self.operator, self.signer
        ]
    }
}

/// Accounts for `set_pair_status_permissionless`
pub struct SetPairStatusPermissionlessAccounts<'a> {
    /// lb_pair
    pub lb_pair: &'a AccountView,
    /// signer
    pub signer: &'a AccountView,
}

impl<'a> SetPairStatusPermissionlessAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 2] {
        [
            InstructionAccount::writable(self.lb_pair.address()),
            InstructionAccount::readonly_signer(self.signer.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 2] {
        [
            self.lb_pair, self.signer
        ]
    }
}

/// Accounts for `set_pre_activation_duration`
pub struct SetPreActivationDurationAccounts<'a> {
    /// lb_pair
    pub lb_pair: &'a AccountView,
    /// signer
    pub signer: &'a AccountView,
}

impl<'a> SetPreActivationDurationAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 2] {
        [
            InstructionAccount::writable(self.lb_pair.address()),
            InstructionAccount::readonly_signer(self.signer.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 2] {
        [
            self.lb_pair, self.signer
        ]
    }
}

/// Accounts for `set_pre_activation_swap_address`
pub struct SetPreActivationSwapAddressAccounts<'a> {
    /// lb_pair
    pub lb_pair: &'a AccountView,
    /// signer
    pub signer: &'a AccountView,
}

impl<'a> SetPreActivationSwapAddressAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 2] {
        [
            InstructionAccount::writable(self.lb_pair.address()),
            InstructionAccount::readonly_signer(self.signer.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 2] {
        [
            self.lb_pair, self.signer
        ]
    }
}

/// Accounts for `swap`
pub struct SwapAccounts<'a> {
    /// lb_pair
    pub lb_pair: &'a AccountView,
    /// bin_array_bitmap_extension
    pub bin_array_bitmap_extension: &'a AccountView,
    /// reserve_x
    pub reserve_x: &'a AccountView,
    /// reserve_y
    pub reserve_y: &'a AccountView,
    /// user_token_in
    pub user_token_in: &'a AccountView,
    /// user_token_out
    pub user_token_out: &'a AccountView,
    /// token_x_mint
    pub token_x_mint: &'a AccountView,
    /// token_y_mint
    pub token_y_mint: &'a AccountView,
    /// oracle
    pub oracle: &'a AccountView,
    /// host_fee_in
    pub host_fee_in: &'a AccountView,
    /// user
    pub user: &'a AccountView,
    /// token_x_program
    pub token_x_program: &'a AccountView,
    /// token_y_program
    pub token_y_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> SwapAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 15] {
        [
            InstructionAccount::writable(self.lb_pair.address()),
            InstructionAccount::readonly(self.bin_array_bitmap_extension.address()),
            InstructionAccount::writable(self.reserve_x.address()),
            InstructionAccount::writable(self.reserve_y.address()),
            InstructionAccount::writable(self.user_token_in.address()),
            InstructionAccount::writable(self.user_token_out.address()),
            InstructionAccount::readonly(self.token_x_mint.address()),
            InstructionAccount::readonly(self.token_y_mint.address()),
            InstructionAccount::writable(self.oracle.address()),
            InstructionAccount::writable(self.host_fee_in.address()),
            InstructionAccount::readonly_signer(self.user.address()),
            InstructionAccount::readonly(self.token_x_program.address()),
            InstructionAccount::readonly(self.token_y_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 15] {
        [
            self.lb_pair, self.bin_array_bitmap_extension, self.reserve_x, self.reserve_y, self.user_token_in, self.user_token_out, self.token_x_mint, self.token_y_mint, self.oracle, self.host_fee_in, self.user, self.token_x_program, self.token_y_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `swap2`
pub struct Swap2Accounts<'a> {
    /// lb_pair
    pub lb_pair: &'a AccountView,
    /// bin_array_bitmap_extension
    pub bin_array_bitmap_extension: &'a AccountView,
    /// reserve_x
    pub reserve_x: &'a AccountView,
    /// reserve_y
    pub reserve_y: &'a AccountView,
    /// user_token_in
    pub user_token_in: &'a AccountView,
    /// user_token_out
    pub user_token_out: &'a AccountView,
    /// token_x_mint
    pub token_x_mint: &'a AccountView,
    /// token_y_mint
    pub token_y_mint: &'a AccountView,
    /// oracle
    pub oracle: &'a AccountView,
    /// host_fee_in
    pub host_fee_in: &'a AccountView,
    /// user
    pub user: &'a AccountView,
    /// token_x_program
    pub token_x_program: &'a AccountView,
    /// token_y_program
    pub token_y_program: &'a AccountView,
    /// memo_program
    pub memo_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> Swap2Accounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 16] {
        [
            InstructionAccount::writable(self.lb_pair.address()),
            InstructionAccount::readonly(self.bin_array_bitmap_extension.address()),
            InstructionAccount::writable(self.reserve_x.address()),
            InstructionAccount::writable(self.reserve_y.address()),
            InstructionAccount::writable(self.user_token_in.address()),
            InstructionAccount::writable(self.user_token_out.address()),
            InstructionAccount::readonly(self.token_x_mint.address()),
            InstructionAccount::readonly(self.token_y_mint.address()),
            InstructionAccount::writable(self.oracle.address()),
            InstructionAccount::writable(self.host_fee_in.address()),
            InstructionAccount::readonly_signer(self.user.address()),
            InstructionAccount::readonly(self.token_x_program.address()),
            InstructionAccount::readonly(self.token_y_program.address()),
            InstructionAccount::readonly(self.memo_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 16] {
        [
            self.lb_pair, self.bin_array_bitmap_extension, self.reserve_x, self.reserve_y, self.user_token_in, self.user_token_out, self.token_x_mint, self.token_y_mint, self.oracle, self.host_fee_in, self.user, self.token_x_program, self.token_y_program, self.memo_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `swap_exact_out`
pub struct SwapExactOutAccounts<'a> {
    /// lb_pair
    pub lb_pair: &'a AccountView,
    /// bin_array_bitmap_extension
    pub bin_array_bitmap_extension: &'a AccountView,
    /// reserve_x
    pub reserve_x: &'a AccountView,
    /// reserve_y
    pub reserve_y: &'a AccountView,
    /// user_token_in
    pub user_token_in: &'a AccountView,
    /// user_token_out
    pub user_token_out: &'a AccountView,
    /// token_x_mint
    pub token_x_mint: &'a AccountView,
    /// token_y_mint
    pub token_y_mint: &'a AccountView,
    /// oracle
    pub oracle: &'a AccountView,
    /// host_fee_in
    pub host_fee_in: &'a AccountView,
    /// user
    pub user: &'a AccountView,
    /// token_x_program
    pub token_x_program: &'a AccountView,
    /// token_y_program
    pub token_y_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> SwapExactOutAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 15] {
        [
            InstructionAccount::writable(self.lb_pair.address()),
            InstructionAccount::readonly(self.bin_array_bitmap_extension.address()),
            InstructionAccount::writable(self.reserve_x.address()),
            InstructionAccount::writable(self.reserve_y.address()),
            InstructionAccount::writable(self.user_token_in.address()),
            InstructionAccount::writable(self.user_token_out.address()),
            InstructionAccount::readonly(self.token_x_mint.address()),
            InstructionAccount::readonly(self.token_y_mint.address()),
            InstructionAccount::writable(self.oracle.address()),
            InstructionAccount::writable(self.host_fee_in.address()),
            InstructionAccount::readonly_signer(self.user.address()),
            InstructionAccount::readonly(self.token_x_program.address()),
            InstructionAccount::readonly(self.token_y_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 15] {
        [
            self.lb_pair, self.bin_array_bitmap_extension, self.reserve_x, self.reserve_y, self.user_token_in, self.user_token_out, self.token_x_mint, self.token_y_mint, self.oracle, self.host_fee_in, self.user, self.token_x_program, self.token_y_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `swap_exact_out2`
pub struct SwapExactOut2Accounts<'a> {
    /// lb_pair
    pub lb_pair: &'a AccountView,
    /// bin_array_bitmap_extension
    pub bin_array_bitmap_extension: &'a AccountView,
    /// reserve_x
    pub reserve_x: &'a AccountView,
    /// reserve_y
    pub reserve_y: &'a AccountView,
    /// user_token_in
    pub user_token_in: &'a AccountView,
    /// user_token_out
    pub user_token_out: &'a AccountView,
    /// token_x_mint
    pub token_x_mint: &'a AccountView,
    /// token_y_mint
    pub token_y_mint: &'a AccountView,
    /// oracle
    pub oracle: &'a AccountView,
    /// host_fee_in
    pub host_fee_in: &'a AccountView,
    /// user
    pub user: &'a AccountView,
    /// token_x_program
    pub token_x_program: &'a AccountView,
    /// token_y_program
    pub token_y_program: &'a AccountView,
    /// memo_program
    pub memo_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> SwapExactOut2Accounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 16] {
        [
            InstructionAccount::writable(self.lb_pair.address()),
            InstructionAccount::readonly(self.bin_array_bitmap_extension.address()),
            InstructionAccount::writable(self.reserve_x.address()),
            InstructionAccount::writable(self.reserve_y.address()),
            InstructionAccount::writable(self.user_token_in.address()),
            InstructionAccount::writable(self.user_token_out.address()),
            InstructionAccount::readonly(self.token_x_mint.address()),
            InstructionAccount::readonly(self.token_y_mint.address()),
            InstructionAccount::writable(self.oracle.address()),
            InstructionAccount::writable(self.host_fee_in.address()),
            InstructionAccount::readonly_signer(self.user.address()),
            InstructionAccount::readonly(self.token_x_program.address()),
            InstructionAccount::readonly(self.token_y_program.address()),
            InstructionAccount::readonly(self.memo_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 16] {
        [
            self.lb_pair, self.bin_array_bitmap_extension, self.reserve_x, self.reserve_y, self.user_token_in, self.user_token_out, self.token_x_mint, self.token_y_mint, self.oracle, self.host_fee_in, self.user, self.token_x_program, self.token_y_program, self.memo_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `swap_with_price_impact`
pub struct SwapWithPriceImpactAccounts<'a> {
    /// lb_pair
    pub lb_pair: &'a AccountView,
    /// bin_array_bitmap_extension
    pub bin_array_bitmap_extension: &'a AccountView,
    /// reserve_x
    pub reserve_x: &'a AccountView,
    /// reserve_y
    pub reserve_y: &'a AccountView,
    /// user_token_in
    pub user_token_in: &'a AccountView,
    /// user_token_out
    pub user_token_out: &'a AccountView,
    /// token_x_mint
    pub token_x_mint: &'a AccountView,
    /// token_y_mint
    pub token_y_mint: &'a AccountView,
    /// oracle
    pub oracle: &'a AccountView,
    /// host_fee_in
    pub host_fee_in: &'a AccountView,
    /// user
    pub user: &'a AccountView,
    /// token_x_program
    pub token_x_program: &'a AccountView,
    /// token_y_program
    pub token_y_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> SwapWithPriceImpactAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 15] {
        [
            InstructionAccount::writable(self.lb_pair.address()),
            InstructionAccount::readonly(self.bin_array_bitmap_extension.address()),
            InstructionAccount::writable(self.reserve_x.address()),
            InstructionAccount::writable(self.reserve_y.address()),
            InstructionAccount::writable(self.user_token_in.address()),
            InstructionAccount::writable(self.user_token_out.address()),
            InstructionAccount::readonly(self.token_x_mint.address()),
            InstructionAccount::readonly(self.token_y_mint.address()),
            InstructionAccount::writable(self.oracle.address()),
            InstructionAccount::writable(self.host_fee_in.address()),
            InstructionAccount::readonly_signer(self.user.address()),
            InstructionAccount::readonly(self.token_x_program.address()),
            InstructionAccount::readonly(self.token_y_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 15] {
        [
            self.lb_pair, self.bin_array_bitmap_extension, self.reserve_x, self.reserve_y, self.user_token_in, self.user_token_out, self.token_x_mint, self.token_y_mint, self.oracle, self.host_fee_in, self.user, self.token_x_program, self.token_y_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `swap_with_price_impact2`
pub struct SwapWithPriceImpact2Accounts<'a> {
    /// lb_pair
    pub lb_pair: &'a AccountView,
    /// bin_array_bitmap_extension
    pub bin_array_bitmap_extension: &'a AccountView,
    /// reserve_x
    pub reserve_x: &'a AccountView,
    /// reserve_y
    pub reserve_y: &'a AccountView,
    /// user_token_in
    pub user_token_in: &'a AccountView,
    /// user_token_out
    pub user_token_out: &'a AccountView,
    /// token_x_mint
    pub token_x_mint: &'a AccountView,
    /// token_y_mint
    pub token_y_mint: &'a AccountView,
    /// oracle
    pub oracle: &'a AccountView,
    /// host_fee_in
    pub host_fee_in: &'a AccountView,
    /// user
    pub user: &'a AccountView,
    /// token_x_program
    pub token_x_program: &'a AccountView,
    /// token_y_program
    pub token_y_program: &'a AccountView,
    /// memo_program
    pub memo_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> SwapWithPriceImpact2Accounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 16] {
        [
            InstructionAccount::writable(self.lb_pair.address()),
            InstructionAccount::readonly(self.bin_array_bitmap_extension.address()),
            InstructionAccount::writable(self.reserve_x.address()),
            InstructionAccount::writable(self.reserve_y.address()),
            InstructionAccount::writable(self.user_token_in.address()),
            InstructionAccount::writable(self.user_token_out.address()),
            InstructionAccount::readonly(self.token_x_mint.address()),
            InstructionAccount::readonly(self.token_y_mint.address()),
            InstructionAccount::writable(self.oracle.address()),
            InstructionAccount::writable(self.host_fee_in.address()),
            InstructionAccount::readonly_signer(self.user.address()),
            InstructionAccount::readonly(self.token_x_program.address()),
            InstructionAccount::readonly(self.token_y_program.address()),
            InstructionAccount::readonly(self.memo_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 16] {
        [
            self.lb_pair, self.bin_array_bitmap_extension, self.reserve_x, self.reserve_y, self.user_token_in, self.user_token_out, self.token_x_mint, self.token_y_mint, self.oracle, self.host_fee_in, self.user, self.token_x_program, self.token_y_program, self.memo_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `update_base_fee_parameters`
pub struct UpdateBaseFeeParametersAccounts<'a> {
    /// lb_pair
    pub lb_pair: &'a AccountView,
    /// operator
    pub operator: &'a AccountView,
    /// signer
    pub signer: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> UpdateBaseFeeParametersAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 5] {
        [
            InstructionAccount::writable(self.lb_pair.address()),
            InstructionAccount::readonly(self.operator.address()),
            InstructionAccount::readonly_signer(self.signer.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 5] {
        [
            self.lb_pair, self.operator, self.signer, self.event_authority, self.program
        ]
    }
}

/// Accounts for `update_dynamic_fee_parameters`
pub struct UpdateDynamicFeeParametersAccounts<'a> {
    /// lb_pair
    pub lb_pair: &'a AccountView,
    /// operator
    pub operator: &'a AccountView,
    /// signer
    pub signer: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> UpdateDynamicFeeParametersAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 5] {
        [
            InstructionAccount::writable(self.lb_pair.address()),
            InstructionAccount::readonly(self.operator.address()),
            InstructionAccount::readonly_signer(self.signer.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 5] {
        [
            self.lb_pair, self.operator, self.signer, self.event_authority, self.program
        ]
    }
}

/// Accounts for `update_fees_and_reward2`
pub struct UpdateFeesAndReward2Accounts<'a> {
    /// position
    pub position: &'a AccountView,
    /// lb_pair
    pub lb_pair: &'a AccountView,
    /// owner
    pub owner: &'a AccountView,
}

impl<'a> UpdateFeesAndReward2Accounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 3] {
        [
            InstructionAccount::writable(self.position.address()),
            InstructionAccount::writable(self.lb_pair.address()),
            InstructionAccount::readonly_signer(self.owner.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 3] {
        [
            self.position, self.lb_pair, self.owner
        ]
    }
}

/// Accounts for `update_fees_and_rewards`
pub struct UpdateFeesAndRewardsAccounts<'a> {
    /// position
    pub position: &'a AccountView,
    /// lb_pair
    pub lb_pair: &'a AccountView,
    /// bin_array_lower
    pub bin_array_lower: &'a AccountView,
    /// bin_array_upper
    pub bin_array_upper: &'a AccountView,
    /// owner
    pub owner: &'a AccountView,
}

impl<'a> UpdateFeesAndRewardsAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 5] {
        [
            InstructionAccount::writable(self.position.address()),
            InstructionAccount::writable(self.lb_pair.address()),
            InstructionAccount::writable(self.bin_array_lower.address()),
            InstructionAccount::writable(self.bin_array_upper.address()),
            InstructionAccount::readonly_signer(self.owner.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 5] {
        [
            self.position, self.lb_pair, self.bin_array_lower, self.bin_array_upper, self.owner
        ]
    }
}

/// Accounts for `update_position_operator`
pub struct UpdatePositionOperatorAccounts<'a> {
    /// position
    pub position: &'a AccountView,
    /// owner
    pub owner: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> UpdatePositionOperatorAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 4] {
        [
            InstructionAccount::writable(self.position.address()),
            InstructionAccount::readonly_signer(self.owner.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 4] {
        [
            self.position, self.owner, self.event_authority, self.program
        ]
    }
}

/// Accounts for `update_reward_duration`
pub struct UpdateRewardDurationAccounts<'a> {
    /// lb_pair
    pub lb_pair: &'a AccountView,
    /// operator
    pub operator: &'a AccountView,
    /// signer
    pub signer: &'a AccountView,
    /// bin_array
    pub bin_array: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> UpdateRewardDurationAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 6] {
        [
            InstructionAccount::writable(self.lb_pair.address()),
            InstructionAccount::readonly(self.operator.address()),
            InstructionAccount::readonly_signer(self.signer.address()),
            InstructionAccount::writable(self.bin_array.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 6] {
        [
            self.lb_pair, self.operator, self.signer, self.bin_array, self.event_authority, self.program
        ]
    }
}

/// Accounts for `update_reward_funder`
pub struct UpdateRewardFunderAccounts<'a> {
    /// lb_pair
    pub lb_pair: &'a AccountView,
    /// operator
    pub operator: &'a AccountView,
    /// signer
    pub signer: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> UpdateRewardFunderAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 5] {
        [
            InstructionAccount::writable(self.lb_pair.address()),
            InstructionAccount::readonly(self.operator.address()),
            InstructionAccount::readonly_signer(self.signer.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 5] {
        [
            self.lb_pair, self.operator, self.signer, self.event_authority, self.program
        ]
    }
}

/// Accounts for `withdraw_ineligible_reward`
pub struct WithdrawIneligibleRewardAccounts<'a> {
    /// lb_pair
    pub lb_pair: &'a AccountView,
    /// reward_vault
    pub reward_vault: &'a AccountView,
    /// reward_mint
    pub reward_mint: &'a AccountView,
    /// funder_token_account
    pub funder_token_account: &'a AccountView,
    /// funder
    pub funder: &'a AccountView,
    /// bin_array
    pub bin_array: &'a AccountView,
    /// token_program
    pub token_program: &'a AccountView,
    /// memo_program
    pub memo_program: &'a AccountView,
    /// event_authority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> WithdrawIneligibleRewardAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 10] {
        [
            InstructionAccount::writable(self.lb_pair.address()),
            InstructionAccount::writable(self.reward_vault.address()),
            InstructionAccount::readonly(self.reward_mint.address()),
            InstructionAccount::writable(self.funder_token_account.address()),
            InstructionAccount::readonly_signer(self.funder.address()),
            InstructionAccount::writable(self.bin_array.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.memo_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 10] {
        [
            self.lb_pair, self.reward_vault, self.reward_mint, self.funder_token_account, self.funder, self.bin_array, self.token_program, self.memo_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `withdraw_protocol_fee`
pub struct WithdrawProtocolFeeAccounts<'a> {
    /// lb_pair
    pub lb_pair: &'a AccountView,
    /// reserve_x
    pub reserve_x: &'a AccountView,
    /// reserve_y
    pub reserve_y: &'a AccountView,
    /// token_x_mint
    pub token_x_mint: &'a AccountView,
    /// token_y_mint
    pub token_y_mint: &'a AccountView,
    /// receiver_token_x
    pub receiver_token_x: &'a AccountView,
    /// receiver_token_y
    pub receiver_token_y: &'a AccountView,
    /// operator
    pub operator: &'a AccountView,
    /// operator
    pub signer: &'a AccountView,
    /// token_x_program
    pub token_x_program: &'a AccountView,
    /// token_y_program
    pub token_y_program: &'a AccountView,
}

impl<'a> WithdrawProtocolFeeAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 11] {
        [
            InstructionAccount::writable(self.lb_pair.address()),
            InstructionAccount::writable(self.reserve_x.address()),
            InstructionAccount::writable(self.reserve_y.address()),
            InstructionAccount::readonly(self.token_x_mint.address()),
            InstructionAccount::readonly(self.token_y_mint.address()),
            InstructionAccount::writable(self.receiver_token_x.address()),
            InstructionAccount::writable(self.receiver_token_y.address()),
            InstructionAccount::readonly(self.operator.address()),
            InstructionAccount::readonly_signer(self.signer.address()),
            InstructionAccount::readonly(self.token_x_program.address()),
            InstructionAccount::readonly(self.token_y_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 11] {
        [
            self.lb_pair, self.reserve_x, self.reserve_y, self.token_x_mint, self.token_y_mint, self.receiver_token_x, self.receiver_token_y, self.operator, self.signer, self.token_x_program, self.token_y_program
        ]
    }
}

/// Accounts for `zap_protocol_fee`
pub struct ZapProtocolFeeAccounts<'a> {
    /// lb_pair
    pub lb_pair: &'a AccountView,
    /// reserve
    pub reserve: &'a AccountView,
    /// token_mint
    pub token_mint: &'a AccountView,
    /// receiver_token
    pub receiver_token: &'a AccountView,
    /// operator
    pub operator: &'a AccountView,
    /// operator
    pub signer: &'a AccountView,
    /// token_program
    pub token_program: &'a AccountView,
    /// sysvar_instructions
    pub sysvar_instructions: &'a AccountView,
}

impl<'a> ZapProtocolFeeAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 8] {
        [
            InstructionAccount::writable(self.lb_pair.address()),
            InstructionAccount::writable(self.reserve.address()),
            InstructionAccount::readonly(self.token_mint.address()),
            InstructionAccount::writable(self.receiver_token.address()),
            InstructionAccount::readonly(self.operator.address()),
            InstructionAccount::readonly_signer(self.signer.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.sysvar_instructions.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 8] {
        [
            self.lb_pair, self.reserve, self.token_mint, self.receiver_token, self.operator, self.signer, self.token_program, self.sysvar_instructions
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
    invoke_signed::<16>(&instruction, &account_views, signers)
}

/// CPI: add_liquidity2
#[inline(always)]
pub fn add_liquidity2<'a>(
    accounts: &AddLiquidity2Accounts<'a>, args: &AddLiquidity2Args,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<AddLiquidity2Args>()];
    data[0..8].copy_from_slice(&ADD_LIQUIDITY2);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const AddLiquidity2Args as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<AddLiquidity2Args>(),
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

/// CPI: add_liquidity_by_strategy
#[inline(always)]
pub fn add_liquidity_by_strategy<'a>(
    accounts: &AddLiquidityByStrategyAccounts<'a>, args: &AddLiquidityByStrategyArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<AddLiquidityByStrategyArgs>()];
    data[0..8].copy_from_slice(&ADD_LIQUIDITY_BY_STRATEGY);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const AddLiquidityByStrategyArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<AddLiquidityByStrategyArgs>(),
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

/// CPI: add_liquidity_by_strategy2
#[inline(always)]
pub fn add_liquidity_by_strategy2<'a>(
    accounts: &AddLiquidityByStrategy2Accounts<'a>, args: &AddLiquidityByStrategy2Args,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<AddLiquidityByStrategy2Args>()];
    data[0..8].copy_from_slice(&ADD_LIQUIDITY_BY_STRATEGY2);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const AddLiquidityByStrategy2Args as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<AddLiquidityByStrategy2Args>(),
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

/// CPI: add_liquidity_by_strategy_one_side
#[inline(always)]
pub fn add_liquidity_by_strategy_one_side<'a>(
    accounts: &AddLiquidityByStrategyOneSideAccounts<'a>, args: &AddLiquidityByStrategyOneSideArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<AddLiquidityByStrategyOneSideArgs>()];
    data[0..8].copy_from_slice(&ADD_LIQUIDITY_BY_STRATEGY_ONE_SIDE);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const AddLiquidityByStrategyOneSideArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<AddLiquidityByStrategyOneSideArgs>(),
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

/// CPI: add_liquidity_by_weight
#[inline(always)]
pub fn add_liquidity_by_weight<'a>(
    accounts: &AddLiquidityByWeightAccounts<'a>, args: &AddLiquidityByWeightArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<AddLiquidityByWeightArgs>()];
    data[0..8].copy_from_slice(&ADD_LIQUIDITY_BY_WEIGHT);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const AddLiquidityByWeightArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<AddLiquidityByWeightArgs>(),
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

/// CPI: add_liquidity_one_side
#[inline(always)]
pub fn add_liquidity_one_side<'a>(
    accounts: &AddLiquidityOneSideAccounts<'a>, args: &AddLiquidityOneSideArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<AddLiquidityOneSideArgs>()];
    data[0..8].copy_from_slice(&ADD_LIQUIDITY_ONE_SIDE);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const AddLiquidityOneSideArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<AddLiquidityOneSideArgs>(),
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

/// CPI: add_liquidity_one_side_precise
#[inline(always)]
pub fn add_liquidity_one_side_precise<'a>(
    accounts: &AddLiquidityOneSidePreciseAccounts<'a>, args: &AddLiquidityOneSidePreciseArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<AddLiquidityOneSidePreciseArgs>()];
    data[0..8].copy_from_slice(&ADD_LIQUIDITY_ONE_SIDE_PRECISE);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const AddLiquidityOneSidePreciseArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<AddLiquidityOneSidePreciseArgs>(),
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

/// CPI: add_liquidity_one_side_precise2
#[inline(always)]
pub fn add_liquidity_one_side_precise2<'a>(
    accounts: &AddLiquidityOneSidePrecise2Accounts<'a>, args: &AddLiquidityOneSidePrecise2Args,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<AddLiquidityOneSidePrecise2Args>()];
    data[0..8].copy_from_slice(&ADD_LIQUIDITY_ONE_SIDE_PRECISE2);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const AddLiquidityOneSidePrecise2Args as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<AddLiquidityOneSidePrecise2Args>(),
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

/// CPI: claim_fee
#[inline(always)]
pub fn claim_fee<'a>(
    accounts: &ClaimFeeAccounts<'a>,
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
    invoke_signed::<14>(&instruction, &account_views, signers)
}

/// CPI: claim_fee2
#[inline(always)]
pub fn claim_fee2<'a>(
    accounts: &ClaimFee2Accounts<'a>, args: &ClaimFee2Args,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<ClaimFee2Args>()];
    data[0..8].copy_from_slice(&CLAIM_FEE2);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const ClaimFee2Args as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<ClaimFee2Args>(),
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

/// CPI: claim_reward
#[inline(always)]
pub fn claim_reward<'a>(
    accounts: &ClaimRewardAccounts<'a>, args: &ClaimRewardArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<ClaimRewardArgs>()];
    data[0..8].copy_from_slice(&CLAIM_REWARD);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const ClaimRewardArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<ClaimRewardArgs>(),
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

/// CPI: claim_reward2
#[inline(always)]
pub fn claim_reward2<'a>(
    accounts: &ClaimReward2Accounts<'a>, args: &ClaimReward2Args,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<ClaimReward2Args>()];
    data[0..8].copy_from_slice(&CLAIM_REWARD2);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const ClaimReward2Args as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<ClaimReward2Args>(),
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

/// CPI: close_claim_fee_operator_account
#[inline(always)]
pub fn close_claim_fee_operator_account<'a>(
    accounts: &CloseClaimFeeOperatorAccountAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&CLOSE_CLAIM_FEE_OPERATOR_ACCOUNT);
    
    
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

/// CPI: close_operator_account
#[inline(always)]
pub fn close_operator_account<'a>(
    accounts: &CloseOperatorAccountAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&CLOSE_OPERATOR_ACCOUNT);
    
    
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

/// CPI: close_position
#[inline(always)]
pub fn close_position<'a>(
    accounts: &ClosePositionAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&CLOSE_POSITION);
    
    
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

/// CPI: close_position2
#[inline(always)]
pub fn close_position2<'a>(
    accounts: &ClosePosition2Accounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&CLOSE_POSITION2);
    
    
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

/// CPI: close_position_if_empty
#[inline(always)]
pub fn close_position_if_empty<'a>(
    accounts: &ClosePositionIfEmptyAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&CLOSE_POSITION_IF_EMPTY);
    
    
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

/// CPI: close_preset_parameter
#[inline(always)]
pub fn close_preset_parameter<'a>(
    accounts: &ClosePresetParameterAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&CLOSE_PRESET_PARAMETER);
    
    
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

/// CPI: close_preset_parameter2
#[inline(always)]
pub fn close_preset_parameter2<'a>(
    accounts: &ClosePresetParameter2Accounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&CLOSE_PRESET_PARAMETER2);
    
    
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

/// CPI: close_token_badge
#[inline(always)]
pub fn close_token_badge<'a>(
    accounts: &CloseTokenBadgeAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&CLOSE_TOKEN_BADGE);
    
    
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

/// CPI: create_operator_account
#[inline(always)]
pub fn create_operator_account<'a>(
    accounts: &CreateOperatorAccountAccounts<'a>, args: &CreateOperatorAccountArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<CreateOperatorAccountArgs>()];
    data[0..8].copy_from_slice(&CREATE_OPERATOR_ACCOUNT);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const CreateOperatorAccountArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<CreateOperatorAccountArgs>(),
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

/// CPI: decrease_position_length
#[inline(always)]
pub fn decrease_position_length<'a>(
    accounts: &DecreasePositionLengthAccounts<'a>, args: &DecreasePositionLengthArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<DecreasePositionLengthArgs>()];
    data[0..8].copy_from_slice(&DECREASE_POSITION_LENGTH);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const DecreasePositionLengthArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<DecreasePositionLengthArgs>(),
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

/// CPI: for_idl_type_generation_do_not_call
#[inline(always)]
pub fn for_idl_type_generation_do_not_call<'a>(
    accounts: &ForIdlTypeGenerationDoNotCallAccounts<'a>, args: &ForIdlTypeGenerationDoNotCallArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<ForIdlTypeGenerationDoNotCallArgs>()];
    data[0..8].copy_from_slice(&FOR_IDL_TYPE_GENERATION_DO_NOT_CALL);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const ForIdlTypeGenerationDoNotCallArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<ForIdlTypeGenerationDoNotCallArgs>(),
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
    invoke_signed::<1>(&instruction, &account_views, signers)
}

/// CPI: fund_reward
#[inline(always)]
pub fn fund_reward<'a>(
    accounts: &FundRewardAccounts<'a>, args: &FundRewardArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<FundRewardArgs>()];
    data[0..8].copy_from_slice(&FUND_REWARD);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const FundRewardArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<FundRewardArgs>(),
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

/// CPI: go_to_a_bin
#[inline(always)]
pub fn go_to_a_bin<'a>(
    accounts: &GoToABinAccounts<'a>, args: &GoToABinArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<GoToABinArgs>()];
    data[0..8].copy_from_slice(&GO_TO_A_BIN);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const GoToABinArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<GoToABinArgs>(),
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

/// CPI: increase_oracle_length
#[inline(always)]
pub fn increase_oracle_length<'a>(
    accounts: &IncreaseOracleLengthAccounts<'a>, args: &IncreaseOracleLengthArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<IncreaseOracleLengthArgs>()];
    data[0..8].copy_from_slice(&INCREASE_ORACLE_LENGTH);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const IncreaseOracleLengthArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<IncreaseOracleLengthArgs>(),
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

/// CPI: increase_position_length
#[inline(always)]
pub fn increase_position_length<'a>(
    accounts: &IncreasePositionLengthAccounts<'a>, args: &IncreasePositionLengthArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<IncreasePositionLengthArgs>()];
    data[0..8].copy_from_slice(&INCREASE_POSITION_LENGTH);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const IncreasePositionLengthArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<IncreasePositionLengthArgs>(),
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

/// CPI: increase_position_length2
#[inline(always)]
pub fn increase_position_length2<'a>(
    accounts: &IncreasePositionLength2Accounts<'a>, args: &IncreasePositionLength2Args,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<IncreasePositionLength2Args>()];
    data[0..8].copy_from_slice(&INCREASE_POSITION_LENGTH2);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const IncreasePositionLength2Args as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<IncreasePositionLength2Args>(),
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

/// CPI: initialize_bin_array
#[inline(always)]
pub fn initialize_bin_array<'a>(
    accounts: &InitializeBinArrayAccounts<'a>, args: &InitializeBinArrayArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<InitializeBinArrayArgs>()];
    data[0..8].copy_from_slice(&INITIALIZE_BIN_ARRAY);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const InitializeBinArrayArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<InitializeBinArrayArgs>(),
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

/// CPI: initialize_bin_array_bitmap_extension
#[inline(always)]
pub fn initialize_bin_array_bitmap_extension<'a>(
    accounts: &InitializeBinArrayBitmapExtensionAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&INITIALIZE_BIN_ARRAY_BITMAP_EXTENSION);
    
    
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

/// CPI: initialize_customizable_permissionless_lb_pair
#[inline(always)]
pub fn initialize_customizable_permissionless_lb_pair<'a>(
    accounts: &InitializeCustomizablePermissionlessLbPairAccounts<'a>, args: &InitializeCustomizablePermissionlessLbPairArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<InitializeCustomizablePermissionlessLbPairArgs>()];
    data[0..8].copy_from_slice(&INITIALIZE_CUSTOMIZABLE_PERMISSIONLESS_LB_PAIR);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const InitializeCustomizablePermissionlessLbPairArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<InitializeCustomizablePermissionlessLbPairArgs>(),
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

/// CPI: initialize_customizable_permissionless_lb_pair2
#[inline(always)]
pub fn initialize_customizable_permissionless_lb_pair2<'a>(
    accounts: &InitializeCustomizablePermissionlessLbPair2Accounts<'a>, args: &InitializeCustomizablePermissionlessLbPair2Args,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<InitializeCustomizablePermissionlessLbPair2Args>()];
    data[0..8].copy_from_slice(&INITIALIZE_CUSTOMIZABLE_PERMISSIONLESS_LB_PAIR2);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const InitializeCustomizablePermissionlessLbPair2Args as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<InitializeCustomizablePermissionlessLbPair2Args>(),
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

/// CPI: initialize_lb_pair
#[inline(always)]
pub fn initialize_lb_pair<'a>(
    accounts: &InitializeLbPairAccounts<'a>, args: &InitializeLbPairArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<InitializeLbPairArgs>()];
    data[0..8].copy_from_slice(&INITIALIZE_LB_PAIR);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const InitializeLbPairArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<InitializeLbPairArgs>(),
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

/// CPI: initialize_lb_pair2
#[inline(always)]
pub fn initialize_lb_pair2<'a>(
    accounts: &InitializeLbPair2Accounts<'a>, args: &InitializeLbPair2Args,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<InitializeLbPair2Args>()];
    data[0..8].copy_from_slice(&INITIALIZE_LB_PAIR2);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const InitializeLbPair2Args as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<InitializeLbPair2Args>(),
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

/// CPI: initialize_permission_lb_pair
#[inline(always)]
pub fn initialize_permission_lb_pair<'a>(
    accounts: &InitializePermissionLbPairAccounts<'a>, args: &InitializePermissionLbPairArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<InitializePermissionLbPairArgs>()];
    data[0..8].copy_from_slice(&INITIALIZE_PERMISSION_LB_PAIR);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const InitializePermissionLbPairArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<InitializePermissionLbPairArgs>(),
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

/// CPI: initialize_position
#[inline(always)]
pub fn initialize_position<'a>(
    accounts: &InitializePositionAccounts<'a>, args: &InitializePositionArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<InitializePositionArgs>()];
    data[0..8].copy_from_slice(&INITIALIZE_POSITION);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const InitializePositionArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<InitializePositionArgs>(),
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

/// CPI: initialize_position2
#[inline(always)]
pub fn initialize_position2<'a>(
    accounts: &InitializePosition2Accounts<'a>, args: &InitializePosition2Args,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<InitializePosition2Args>()];
    data[0..8].copy_from_slice(&INITIALIZE_POSITION2);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const InitializePosition2Args as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<InitializePosition2Args>(),
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

/// CPI: initialize_position_by_operator
#[inline(always)]
pub fn initialize_position_by_operator<'a>(
    accounts: &InitializePositionByOperatorAccounts<'a>, args: &InitializePositionByOperatorArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<InitializePositionByOperatorArgs>()];
    data[0..8].copy_from_slice(&INITIALIZE_POSITION_BY_OPERATOR);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const InitializePositionByOperatorArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<InitializePositionByOperatorArgs>(),
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

/// CPI: initialize_position_pda
#[inline(always)]
pub fn initialize_position_pda<'a>(
    accounts: &InitializePositionPdaAccounts<'a>, args: &InitializePositionPdaArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<InitializePositionPdaArgs>()];
    data[0..8].copy_from_slice(&INITIALIZE_POSITION_PDA);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const InitializePositionPdaArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<InitializePositionPdaArgs>(),
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

/// CPI: initialize_preset_parameter
#[inline(always)]
pub fn initialize_preset_parameter<'a>(
    accounts: &InitializePresetParameterAccounts<'a>, args: &InitializePresetParameterArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<InitializePresetParameterArgs>()];
    data[0..8].copy_from_slice(&INITIALIZE_PRESET_PARAMETER);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const InitializePresetParameterArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<InitializePresetParameterArgs>(),
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

/// CPI: initialize_reward
#[inline(always)]
pub fn initialize_reward<'a>(
    accounts: &InitializeRewardAccounts<'a>, args: &InitializeRewardArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<InitializeRewardArgs>()];
    data[0..8].copy_from_slice(&INITIALIZE_REWARD);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const InitializeRewardArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<InitializeRewardArgs>(),
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

/// CPI: initialize_token_badge
#[inline(always)]
pub fn initialize_token_badge<'a>(
    accounts: &InitializeTokenBadgeAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&INITIALIZE_TOKEN_BADGE);
    
    
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

/// CPI: migrate_position
#[inline(always)]
pub fn migrate_position<'a>(
    accounts: &MigratePositionAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&MIGRATE_POSITION);
    
    
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

/// CPI: rebalance_liquidity
#[inline(always)]
pub fn rebalance_liquidity<'a>(
    accounts: &RebalanceLiquidityAccounts<'a>, args: &RebalanceLiquidityArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<RebalanceLiquidityArgs>()];
    data[0..8].copy_from_slice(&REBALANCE_LIQUIDITY);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const RebalanceLiquidityArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<RebalanceLiquidityArgs>(),
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
    invoke_signed::<16>(&instruction, &account_views, signers)
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
    invoke_signed::<16>(&instruction, &account_views, signers)
}

/// CPI: remove_liquidity2
#[inline(always)]
pub fn remove_liquidity2<'a>(
    accounts: &RemoveLiquidity2Accounts<'a>, args: &RemoveLiquidity2Args,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<RemoveLiquidity2Args>()];
    data[0..8].copy_from_slice(&REMOVE_LIQUIDITY2);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const RemoveLiquidity2Args as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<RemoveLiquidity2Args>(),
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

/// CPI: remove_liquidity_by_range
#[inline(always)]
pub fn remove_liquidity_by_range<'a>(
    accounts: &RemoveLiquidityByRangeAccounts<'a>, args: &RemoveLiquidityByRangeArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<RemoveLiquidityByRangeArgs>()];
    data[0..8].copy_from_slice(&REMOVE_LIQUIDITY_BY_RANGE);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const RemoveLiquidityByRangeArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<RemoveLiquidityByRangeArgs>(),
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

/// CPI: remove_liquidity_by_range2
#[inline(always)]
pub fn remove_liquidity_by_range2<'a>(
    accounts: &RemoveLiquidityByRange2Accounts<'a>, args: &RemoveLiquidityByRange2Args,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<RemoveLiquidityByRange2Args>()];
    data[0..8].copy_from_slice(&REMOVE_LIQUIDITY_BY_RANGE2);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const RemoveLiquidityByRange2Args as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<RemoveLiquidityByRange2Args>(),
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

/// CPI: reset_bin_array_tombstone_fields
#[inline(always)]
pub fn reset_bin_array_tombstone_fields<'a>(
    accounts: &ResetBinArrayTombstoneFieldsAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&RESET_BIN_ARRAY_TOMBSTONE_FIELDS);
    
    
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

/// CPI: reset_pool_tombstone_fields
#[inline(always)]
pub fn reset_pool_tombstone_fields<'a>(
    accounts: &ResetPoolTombstoneFieldsAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&RESET_POOL_TOMBSTONE_FIELDS);
    
    
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

/// CPI: reset_position_tombstone_fields
#[inline(always)]
pub fn reset_position_tombstone_fields<'a>(
    accounts: &ResetPositionTombstoneFieldsAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&RESET_POSITION_TOMBSTONE_FIELDS);
    
    
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

/// CPI: set_activation_point
#[inline(always)]
pub fn set_activation_point<'a>(
    accounts: &SetActivationPointAccounts<'a>, args: &SetActivationPointArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SetActivationPointArgs>()];
    data[0..8].copy_from_slice(&SET_ACTIVATION_POINT);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SetActivationPointArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SetActivationPointArgs>(),
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

/// CPI: set_pair_status
#[inline(always)]
pub fn set_pair_status<'a>(
    accounts: &SetPairStatusAccounts<'a>, args: &SetPairStatusArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SetPairStatusArgs>()];
    data[0..8].copy_from_slice(&SET_PAIR_STATUS);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SetPairStatusArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SetPairStatusArgs>(),
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

/// CPI: set_pair_status_permissionless
#[inline(always)]
pub fn set_pair_status_permissionless<'a>(
    accounts: &SetPairStatusPermissionlessAccounts<'a>, args: &SetPairStatusPermissionlessArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SetPairStatusPermissionlessArgs>()];
    data[0..8].copy_from_slice(&SET_PAIR_STATUS_PERMISSIONLESS);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SetPairStatusPermissionlessArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SetPairStatusPermissionlessArgs>(),
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

/// CPI: set_pre_activation_duration
#[inline(always)]
pub fn set_pre_activation_duration<'a>(
    accounts: &SetPreActivationDurationAccounts<'a>, args: &SetPreActivationDurationArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SetPreActivationDurationArgs>()];
    data[0..8].copy_from_slice(&SET_PRE_ACTIVATION_DURATION);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SetPreActivationDurationArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SetPreActivationDurationArgs>(),
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

/// CPI: set_pre_activation_swap_address
#[inline(always)]
pub fn set_pre_activation_swap_address<'a>(
    accounts: &SetPreActivationSwapAddressAccounts<'a>, args: &SetPreActivationSwapAddressArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SetPreActivationSwapAddressArgs>()];
    data[0..8].copy_from_slice(&SET_PRE_ACTIVATION_SWAP_ADDRESS);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SetPreActivationSwapAddressArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SetPreActivationSwapAddressArgs>(),
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
    invoke_signed::<16>(&instruction, &account_views, signers)
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
    invoke_signed::<15>(&instruction, &account_views, signers)
}

/// CPI: swap_exact_out2
#[inline(always)]
pub fn swap_exact_out2<'a>(
    accounts: &SwapExactOut2Accounts<'a>, args: &SwapExactOut2Args,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SwapExactOut2Args>()];
    data[0..8].copy_from_slice(&SWAP_EXACT_OUT2);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SwapExactOut2Args as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SwapExactOut2Args>(),
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

/// CPI: swap_with_price_impact
#[inline(always)]
pub fn swap_with_price_impact<'a>(
    accounts: &SwapWithPriceImpactAccounts<'a>, args: &SwapWithPriceImpactArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SwapWithPriceImpactArgs>()];
    data[0..8].copy_from_slice(&SWAP_WITH_PRICE_IMPACT);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SwapWithPriceImpactArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SwapWithPriceImpactArgs>(),
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

/// CPI: swap_with_price_impact2
#[inline(always)]
pub fn swap_with_price_impact2<'a>(
    accounts: &SwapWithPriceImpact2Accounts<'a>, args: &SwapWithPriceImpact2Args,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SwapWithPriceImpact2Args>()];
    data[0..8].copy_from_slice(&SWAP_WITH_PRICE_IMPACT2);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SwapWithPriceImpact2Args as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SwapWithPriceImpact2Args>(),
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

/// CPI: update_base_fee_parameters
#[inline(always)]
pub fn update_base_fee_parameters<'a>(
    accounts: &UpdateBaseFeeParametersAccounts<'a>, args: &UpdateBaseFeeParametersArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<UpdateBaseFeeParametersArgs>()];
    data[0..8].copy_from_slice(&UPDATE_BASE_FEE_PARAMETERS);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const UpdateBaseFeeParametersArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<UpdateBaseFeeParametersArgs>(),
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

/// CPI: update_dynamic_fee_parameters
#[inline(always)]
pub fn update_dynamic_fee_parameters<'a>(
    accounts: &UpdateDynamicFeeParametersAccounts<'a>, args: &UpdateDynamicFeeParametersArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<UpdateDynamicFeeParametersArgs>()];
    data[0..8].copy_from_slice(&UPDATE_DYNAMIC_FEE_PARAMETERS);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const UpdateDynamicFeeParametersArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<UpdateDynamicFeeParametersArgs>(),
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

/// CPI: update_fees_and_reward2
#[inline(always)]
pub fn update_fees_and_reward2<'a>(
    accounts: &UpdateFeesAndReward2Accounts<'a>, args: &UpdateFeesAndReward2Args,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<UpdateFeesAndReward2Args>()];
    data[0..8].copy_from_slice(&UPDATE_FEES_AND_REWARD2);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const UpdateFeesAndReward2Args as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<UpdateFeesAndReward2Args>(),
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

/// CPI: update_fees_and_rewards
#[inline(always)]
pub fn update_fees_and_rewards<'a>(
    accounts: &UpdateFeesAndRewardsAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&UPDATE_FEES_AND_REWARDS);
    
    
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

/// CPI: update_position_operator
#[inline(always)]
pub fn update_position_operator<'a>(
    accounts: &UpdatePositionOperatorAccounts<'a>, args: &UpdatePositionOperatorArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<UpdatePositionOperatorArgs>()];
    data[0..8].copy_from_slice(&UPDATE_POSITION_OPERATOR);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const UpdatePositionOperatorArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<UpdatePositionOperatorArgs>(),
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

/// CPI: update_reward_duration
#[inline(always)]
pub fn update_reward_duration<'a>(
    accounts: &UpdateRewardDurationAccounts<'a>, args: &UpdateRewardDurationArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<UpdateRewardDurationArgs>()];
    data[0..8].copy_from_slice(&UPDATE_REWARD_DURATION);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const UpdateRewardDurationArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<UpdateRewardDurationArgs>(),
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

/// CPI: update_reward_funder
#[inline(always)]
pub fn update_reward_funder<'a>(
    accounts: &UpdateRewardFunderAccounts<'a>, args: &UpdateRewardFunderArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<UpdateRewardFunderArgs>()];
    data[0..8].copy_from_slice(&UPDATE_REWARD_FUNDER);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const UpdateRewardFunderArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<UpdateRewardFunderArgs>(),
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

/// CPI: withdraw_ineligible_reward
#[inline(always)]
pub fn withdraw_ineligible_reward<'a>(
    accounts: &WithdrawIneligibleRewardAccounts<'a>, args: &WithdrawIneligibleRewardArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<WithdrawIneligibleRewardArgs>()];
    data[0..8].copy_from_slice(&WITHDRAW_INELIGIBLE_REWARD);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const WithdrawIneligibleRewardArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<WithdrawIneligibleRewardArgs>(),
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

/// CPI: withdraw_protocol_fee
#[inline(always)]
pub fn withdraw_protocol_fee<'a>(
    accounts: &WithdrawProtocolFeeAccounts<'a>, args: &WithdrawProtocolFeeArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<WithdrawProtocolFeeArgs>()];
    data[0..8].copy_from_slice(&WITHDRAW_PROTOCOL_FEE);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const WithdrawProtocolFeeArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<WithdrawProtocolFeeArgs>(),
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

/// CPI: zap_protocol_fee
#[inline(always)]
pub fn zap_protocol_fee<'a>(
    accounts: &ZapProtocolFeeAccounts<'a>, args: &ZapProtocolFeeArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<ZapProtocolFeeArgs>()];
    data[0..8].copy_from_slice(&ZAP_PROTOCOL_FEE);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const ZapProtocolFeeArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<ZapProtocolFeeArgs>(),
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

