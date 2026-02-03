//! CPI module for Byreal
//!
//! Program: byreal_clmm
//! Program ID: REALQqNEomY6cQGZJUGwywTBD2UmDT32rZcNnfxQ5N2
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
pub const ID: Address = Address::new_from_array(five8_const::decode_32_const("REALQqNEomY6cQGZJUGwywTBD2UmDT32rZcNnfxQ5N2"));

// ============================================
// Instruction Discriminators
// ============================================
pub const CLAIM_OFFCHAIN_REWARD: [u8; 8] = [195, 87, 221, 149, 141, 195, 146, 19];
pub const CLOSE_POSITION: [u8; 8] = [123, 134, 81, 0, 49, 68, 98, 98];
pub const COLLECT_FUND_FEE: [u8; 8] = [167, 138, 78, 149, 223, 194, 6, 126];
pub const COLLECT_PROTOCOL_FEE: [u8; 8] = [136, 136, 252, 221, 194, 66, 126, 89];
pub const COLLECT_REMAINING_REWARDS: [u8; 8] = [18, 237, 166, 197, 34, 16, 213, 144];
pub const CREATE_AMM_CONFIG: [u8; 8] = [137, 52, 237, 212, 215, 117, 108, 104];
pub const CREATE_OPERATION_ACCOUNT: [u8; 8] = [63, 87, 148, 33, 109, 35, 8, 104];
pub const CREATE_POOL: [u8; 8] = [233, 146, 209, 142, 207, 104, 64, 188];
pub const CREATE_SUPPORT_MINT_ASSOCIATED: [u8; 8] = [17, 251, 65, 92, 136, 242, 14, 169];
pub const DECREASE_LIQUIDITY: [u8; 8] = [160, 38, 208, 111, 104, 91, 44, 1];
pub const DECREASE_LIQUIDITY_V2: [u8; 8] = [58, 127, 188, 62, 79, 82, 196, 96];
pub const DEPOSIT_OFFCHAIN_REWARD: [u8; 8] = [97, 125, 48, 169, 92, 241, 44, 142];
pub const INCREASE_LIQUIDITY: [u8; 8] = [46, 156, 243, 118, 13, 205, 251, 178];
pub const INCREASE_LIQUIDITY_V2: [u8; 8] = [133, 29, 89, 223, 69, 238, 176, 10];
pub const INIT_AMM_ADMIN_GROUP: [u8; 8] = [209, 108, 32, 246, 157, 214, 237, 86];
pub const INITIALIZE_REWARD: [u8; 8] = [95, 135, 192, 196, 242, 129, 230, 68];
pub const OPEN_POSITION: [u8; 8] = [135, 128, 47, 77, 15, 152, 240, 49];
pub const OPEN_POSITION_V2: [u8; 8] = [77, 184, 74, 214, 112, 86, 241, 199];
pub const OPEN_POSITION_WITH_TOKEN22_NFT: [u8; 8] = [77, 255, 174, 82, 125, 29, 201, 46];
pub const SET_REWARD_PARAMS: [u8; 8] = [112, 52, 167, 75, 32, 201, 211, 137];
pub const SWAP: [u8; 8] = [248, 198, 158, 145, 225, 117, 135, 200];
pub const SWAP_V2: [u8; 8] = [43, 4, 237, 11, 26, 201, 30, 98];
pub const TRANSFER_REWARD_OWNER: [u8; 8] = [7, 22, 12, 83, 242, 43, 48, 121];
pub const UPDATE_AMM_ADMIN_GROUP: [u8; 8] = [61, 183, 185, 188, 82, 81, 141, 197];
pub const UPDATE_AMM_CONFIG: [u8; 8] = [49, 60, 174, 136, 154, 28, 116, 200];
pub const UPDATE_OPERATION_ACCOUNT: [u8; 8] = [127, 70, 119, 40, 188, 227, 61, 7];
pub const UPDATE_POOL_STATUS: [u8; 8] = [130, 87, 108, 6, 46, 224, 117, 123];
pub const UPDATE_REWARD_INFOS: [u8; 8] = [163, 172, 224, 52, 11, 154, 106, 223];
pub const WITHDRAW_OFFCHAIN_REWARD: [u8; 8] = [86, 53, 59, 76, 217, 38, 71, 213];

// ============================================
// Instruction Arguments
// ============================================
/// Arguments for `claim_offchain_reward`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct ClaimOffchainRewardArgs {
    pub amount: u64,
}

/// Arguments for `collect_fund_fee`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct CollectFundFeeArgs {
    pub amount_0_requested: u64,
    pub amount_1_requested: u64,
}

/// Arguments for `collect_protocol_fee`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct CollectProtocolFeeArgs {
    pub amount_0_requested: u64,
    pub amount_1_requested: u64,
}

/// Arguments for `collect_remaining_rewards`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct CollectRemainingRewardsArgs {
    pub reward_index: u8,
}

/// Arguments for `create_amm_config`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct CreateAmmConfigArgs {
    pub index: u16,
    pub tick_spacing: u16,
    pub trade_fee_rate: u32,
    pub protocol_fee_rate: u32,
    pub fund_fee_rate: u32,
}

/// Arguments for `create_pool`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct CreatePoolArgs {
    pub sqrt_price_x64: u128,
    pub open_time: u64,
}

/// Arguments for `decrease_liquidity`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct DecreaseLiquidityArgs {
    pub liquidity: u128,
    pub amount_0_min: u64,
    pub amount_1_min: u64,
}

/// Arguments for `decrease_liquidity_v2`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct DecreaseLiquidityV2Args {
    pub liquidity: u128,
    pub amount_0_min: u64,
    pub amount_1_min: u64,
}

/// Arguments for `deposit_offchain_reward`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct DepositOffchainRewardArgs {
    pub amount: u64,
}

/// Arguments for `increase_liquidity`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct IncreaseLiquidityArgs {
    pub liquidity: u128,
    pub amount_0_max: u64,
    pub amount_1_max: u64,
}

/// Arguments for `increase_liquidity_v2`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct IncreaseLiquidityV2Args {
    pub liquidity: u128,
    pub amount_0_max: u64,
    pub amount_1_max: u64,
    pub base_flag: Option<bool>,
}

/// Arguments for `init_amm_admin_group`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct InitAmmAdminGroupArgs {
    pub params: [u8; 32],
}

/// Arguments for `initialize_reward`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct InitializeRewardArgs {
    pub param: [u8; 32],
}

/// Arguments for `open_position`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct OpenPositionArgs {
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
    pub tick_array_lower_start_index: i32,
    pub tick_array_upper_start_index: i32,
    pub liquidity: u128,
    pub amount_0_max: u64,
    pub amount_1_max: u64,
}

/// Arguments for `open_position_v2`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct OpenPositionV2Args {
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
    pub tick_array_lower_start_index: i32,
    pub tick_array_upper_start_index: i32,
    pub liquidity: u128,
    pub amount_0_max: u64,
    pub amount_1_max: u64,
    pub with_metadata: bool,
    pub base_flag: Option<bool>,
}

/// Arguments for `open_position_with_token22_nft`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct OpenPositionWithToken22NftArgs {
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
    pub tick_array_lower_start_index: i32,
    pub tick_array_upper_start_index: i32,
    pub liquidity: u128,
    pub amount_0_max: u64,
    pub amount_1_max: u64,
    pub with_metadata: bool,
    pub base_flag: Option<bool>,
}

/// Arguments for `set_reward_params`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SetRewardParamsArgs {
    pub reward_index: u8,
    pub emissions_per_second_x64: u128,
    pub open_time: u64,
    pub end_time: u64,
}

/// Arguments for `swap`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SwapArgs {
    pub amount: u64,
    pub other_amount_threshold: u64,
    pub sqrt_price_limit_x64: u128,
    pub is_base_input: bool,
}

/// Arguments for `swap_v2`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SwapV2Args {
    pub amount: u64,
    pub other_amount_threshold: u64,
    pub sqrt_price_limit_x64: u128,
    pub is_base_input: bool,
}

/// Arguments for `transfer_reward_owner`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct TransferRewardOwnerArgs {
    pub new_owner: [u8; 32],
}

/// Arguments for `update_amm_admin_group`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct UpdateAmmAdminGroupArgs {
    pub params: [u8; 32],
}

/// Arguments for `update_amm_config`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct UpdateAmmConfigArgs {
    pub param: u8,
    pub value: u32,
}

/// Arguments for `update_operation_account`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct UpdateOperationAccountArgs {
    pub param: u8,
    pub keys: [u8; 32],
}

/// Arguments for `update_pool_status`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct UpdatePoolStatusArgs {
    pub status: u8,
}

/// Arguments for `withdraw_offchain_reward`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct WithdrawOffchainRewardArgs {
    pub amount: u64,
}


// ============================================
// Instruction Accounts
// ============================================
/// Accounts for `claim_offchain_reward`
pub struct ClaimOffchainRewardAccounts<'a> {
    /// the address who claim the offchain reward.
    pub claimer: &'a AccountView,
    /// The authority make decision that who can claim the offchain reward.
    pub authority: &'a AccountView,
    /// Initialize amm admin group account to store admin permissions.
    pub admin_group: &'a AccountView,
    /// the pool id, which is the pool state account.
    pub pool_id: &'a AccountView,
    /// token_mint
    pub token_mint: &'a AccountView,
    /// 
    pub claimer_token_account: &'a AccountView,
    /// reward_vault_token_account
    pub reward_vault_token_account: &'a AccountView,
    /// The offchain reward config account, it also is the reward vault account.
    pub reward_config: &'a AccountView,
    /// Spl token program or token program 2022
    pub token_program: &'a AccountView,
    /// associated_token_program
    pub associated_token_program: &'a AccountView,
}

impl<'a> ClaimOffchainRewardAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 10] {
        [
            InstructionAccount::writable_signer(self.claimer.address()),
            InstructionAccount::readonly_signer(self.authority.address()),
            InstructionAccount::readonly(self.admin_group.address()),
            InstructionAccount::readonly(self.pool_id.address()),
            InstructionAccount::readonly(self.token_mint.address()),
            InstructionAccount::writable(self.claimer_token_account.address()),
            InstructionAccount::writable(self.reward_vault_token_account.address()),
            InstructionAccount::readonly(self.reward_config.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 10] {
        [
            self.claimer, self.authority, self.admin_group, self.pool_id, self.token_mint, self.claimer_token_account, self.reward_vault_token_account, self.reward_config, self.token_program, self.associated_token_program
        ]
    }
}

/// Accounts for `close_position`
pub struct ClosePositionAccounts<'a> {
    /// The position nft owner
    pub nft_owner: &'a AccountView,
    /// Mint address bound to the personal position.
    pub position_nft_mint: &'a AccountView,
    /// User token account where position NFT be minted to
    pub position_nft_account: &'a AccountView,
    /// personal_position
    pub personal_position: &'a AccountView,
    /// System program to close the position state account
    pub system_program: &'a AccountView,
    /// Token/Token2022 program to close token/mint account
    pub token_program: &'a AccountView,
}

impl<'a> ClosePositionAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 6] {
        [
            InstructionAccount::writable_signer(self.nft_owner.address()),
            InstructionAccount::writable(self.position_nft_mint.address()),
            InstructionAccount::writable(self.position_nft_account.address()),
            InstructionAccount::writable(self.personal_position.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 6] {
        [
            self.nft_owner, self.position_nft_mint, self.position_nft_account, self.personal_position, self.system_program, self.token_program
        ]
    }
}

/// Accounts for `collect_fund_fee`
pub struct CollectFundFeeAccounts<'a> {
    /// amm admin group account to store admin permissions.
    pub admin_group: &'a AccountView,
    /// Pool state stores accumulated protocol fee amount
    pub pool_state: &'a AccountView,
    /// The address that holds pool tokens for token_0
    pub token_vault_0: &'a AccountView,
    /// The address that holds pool tokens for token_1
    pub token_vault_1: &'a AccountView,
    /// The mint of token vault 0
    pub vault_0_mint: &'a AccountView,
    /// The mint of token vault 1
    pub vault_1_mint: &'a AccountView,
    /// The address that receives the collected token_0 protocol fees
    pub recipient_token_account_0: &'a AccountView,
    /// The address that receives the collected token_1 protocol fees
    pub recipient_token_account_1: &'a AccountView,
    /// The SPL program to perform token transfers
    pub token_program: &'a AccountView,
    /// The SPL program 2022 to perform token transfers
    pub token_program_2022: &'a AccountView,
    /// associated_token_program
    pub associated_token_program: &'a AccountView,
}

impl<'a> CollectFundFeeAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 11] {
        [
            InstructionAccount::readonly(self.admin_group.address()),
            InstructionAccount::writable(self.pool_state.address()),
            InstructionAccount::writable(self.token_vault_0.address()),
            InstructionAccount::writable(self.token_vault_1.address()),
            InstructionAccount::readonly(self.vault_0_mint.address()),
            InstructionAccount::readonly(self.vault_1_mint.address()),
            InstructionAccount::writable(self.recipient_token_account_0.address()),
            InstructionAccount::writable(self.recipient_token_account_1.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.token_program_2022.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 11] {
        [
            self.admin_group, self.pool_state, self.token_vault_0, self.token_vault_1, self.vault_0_mint, self.vault_1_mint, self.recipient_token_account_0, self.recipient_token_account_1, self.token_program, self.token_program_2022, self.associated_token_program
        ]
    }
}

/// Accounts for `collect_protocol_fee`
pub struct CollectProtocolFeeAccounts<'a> {
    /// amm admin group account to store admin permissions.
    pub admin_group: &'a AccountView,
    /// Pool state stores accumulated protocol fee amount
    pub pool_state: &'a AccountView,
    /// The address that holds pool tokens for token_0
    pub token_vault_0: &'a AccountView,
    /// The address that holds pool tokens for token_1
    pub token_vault_1: &'a AccountView,
    /// The mint of token vault 0
    pub vault_0_mint: &'a AccountView,
    /// The mint of token vault 1
    pub vault_1_mint: &'a AccountView,
    /// The address that receives the collected token_0 protocol fees
    pub recipient_token_account_0: &'a AccountView,
    /// The address that receives the collected token_1 protocol fees
    pub recipient_token_account_1: &'a AccountView,
    /// The SPL program to perform token transfers
    pub token_program: &'a AccountView,
    /// The SPL program 2022 to perform token transfers
    pub token_program_2022: &'a AccountView,
    /// associated_token_program
    pub associated_token_program: &'a AccountView,
}

impl<'a> CollectProtocolFeeAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 11] {
        [
            InstructionAccount::readonly(self.admin_group.address()),
            InstructionAccount::writable(self.pool_state.address()),
            InstructionAccount::writable(self.token_vault_0.address()),
            InstructionAccount::writable(self.token_vault_1.address()),
            InstructionAccount::readonly(self.vault_0_mint.address()),
            InstructionAccount::readonly(self.vault_1_mint.address()),
            InstructionAccount::writable(self.recipient_token_account_0.address()),
            InstructionAccount::writable(self.recipient_token_account_1.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.token_program_2022.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 11] {
        [
            self.admin_group, self.pool_state, self.token_vault_0, self.token_vault_1, self.vault_0_mint, self.vault_1_mint, self.recipient_token_account_0, self.recipient_token_account_1, self.token_program, self.token_program_2022, self.associated_token_program
        ]
    }
}

/// Accounts for `collect_remaining_rewards`
pub struct CollectRemainingRewardsAccounts<'a> {
    /// The founder who init reward info previously
    pub reward_funder: &'a AccountView,
    /// amm admin group account to store admin permissions.
    pub admin_group: &'a AccountView,
    /// The funder's reward token account
    pub funder_token_account: &'a AccountView,
    /// Set reward for this pool
    pub pool_state: &'a AccountView,
    /// Reward vault transfer remaining token to founder token account
    pub reward_token_vault: &'a AccountView,
    /// The mint of reward token vault
    pub reward_vault_mint: &'a AccountView,
    /// token_program
    pub token_program: &'a AccountView,
    /// Token program 2022
    pub token_program_2022: &'a AccountView,
    /// memo program
    pub memo_program: &'a AccountView,
}

impl<'a> CollectRemainingRewardsAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 9] {
        [
            InstructionAccount::readonly_signer(self.reward_funder.address()),
            InstructionAccount::readonly(self.admin_group.address()),
            InstructionAccount::writable(self.funder_token_account.address()),
            InstructionAccount::writable(self.pool_state.address()),
            InstructionAccount::readonly(self.reward_token_vault.address()),
            InstructionAccount::readonly(self.reward_vault_mint.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.token_program_2022.address()),
            InstructionAccount::readonly(self.memo_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 9] {
        [
            self.reward_funder, self.admin_group, self.funder_token_account, self.pool_state, self.reward_token_vault, self.reward_vault_mint, self.token_program, self.token_program_2022, self.memo_program
        ]
    }
}

/// Accounts for `create_amm_config`
pub struct CreateAmmConfigAccounts<'a> {
    /// Address to be set as normal manager in admin group.
    pub owner: &'a AccountView,
    /// amm admin group account to store admin permissions.
    pub admin_group: &'a AccountView,
    /// Initialize config state account to store protocol owner address and fee rates.
    pub amm_config: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
}

impl<'a> CreateAmmConfigAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 4] {
        [
            InstructionAccount::writable_signer(self.owner.address()),
            InstructionAccount::readonly(self.admin_group.address()),
            InstructionAccount::writable(self.amm_config.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 4] {
        [
            self.owner, self.admin_group, self.amm_config, self.system_program
        ]
    }
}

/// Accounts for `create_operation_account`
pub struct CreateOperationAccountAccounts<'a> {
    /// Address to be set as operation account owner.
    pub owner: &'a AccountView,
    /// amm admin group account to store admin permissions.
    pub admin_group: &'a AccountView,
    /// Initialize operation state account to store operation owner address and white list mint.
    pub operation_state: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
}

impl<'a> CreateOperationAccountAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 4] {
        [
            InstructionAccount::writable_signer(self.owner.address()),
            InstructionAccount::readonly(self.admin_group.address()),
            InstructionAccount::writable(self.operation_state.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 4] {
        [
            self.owner, self.admin_group, self.operation_state, self.system_program
        ]
    }
}

/// Accounts for `create_pool`
pub struct CreatePoolAccounts<'a> {
    /// Address paying to create the pool. Can be anyone
    pub pool_creator: &'a AccountView,
    /// with pool_manager permission, the pool creator can create a pool.
    pub pool_manager: &'a AccountView,
    /// amm admin group account to store admin permissions.
    pub admin_group: &'a AccountView,
    /// Which config the pool belongs to.
    pub amm_config: &'a AccountView,
    /// Initialize an account to store the pool state
    pub pool_state: &'a AccountView,
    /// Initialize an account to store the off-chain reward config
    pub offchain_reward_config: &'a AccountView,
    /// Token_0 mint, the key must be smaller then token_1 mint.
    pub token_mint_0: &'a AccountView,
    /// Token_1 mint
    pub token_mint_1: &'a AccountView,
    /// Token_0 vault for the pool
    pub token_vault_0: &'a AccountView,
    /// Token_1 vault for the pool
    pub token_vault_1: &'a AccountView,
    /// Initialize an account to store oracle observations
    pub observation_state: &'a AccountView,
    /// Initialize an account to store if a tick array is initialized.
    pub tick_array_bitmap: &'a AccountView,
    /// Spl token program or token program 2022
    pub token_program_0: &'a AccountView,
    /// Spl token program or token program 2022
    pub token_program_1: &'a AccountView,
    /// To create a new program account
    pub system_program: &'a AccountView,
    /// Sysvar for program account
    pub rent: &'a AccountView,
}

impl<'a> CreatePoolAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 16] {
        [
            InstructionAccount::writable_signer(self.pool_creator.address()),
            InstructionAccount::readonly_signer(self.pool_manager.address()),
            InstructionAccount::readonly(self.admin_group.address()),
            InstructionAccount::readonly(self.amm_config.address()),
            InstructionAccount::writable(self.pool_state.address()),
            InstructionAccount::writable(self.offchain_reward_config.address()),
            InstructionAccount::readonly(self.token_mint_0.address()),
            InstructionAccount::readonly(self.token_mint_1.address()),
            InstructionAccount::writable(self.token_vault_0.address()),
            InstructionAccount::writable(self.token_vault_1.address()),
            InstructionAccount::writable(self.observation_state.address()),
            InstructionAccount::writable(self.tick_array_bitmap.address()),
            InstructionAccount::readonly(self.token_program_0.address()),
            InstructionAccount::readonly(self.token_program_1.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.rent.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 16] {
        [
            self.pool_creator, self.pool_manager, self.admin_group, self.amm_config, self.pool_state, self.offchain_reward_config, self.token_mint_0, self.token_mint_1, self.token_vault_0, self.token_vault_1, self.observation_state, self.tick_array_bitmap, self.token_program_0, self.token_program_1, self.system_program, self.rent
        ]
    }
}

/// Accounts for `create_support_mint_associated`
pub struct CreateSupportMintAssociatedAccounts<'a> {
    /// Address to be set as protocol owner.
    pub owner: &'a AccountView,
    /// amm admin group account to store admin permissions.
    pub admin_group: &'a AccountView,
    /// Support token mint
    pub token_mint: &'a AccountView,
    /// Initialize support mint state account to store support mint address and bump.
    pub support_mint_associated: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
}

impl<'a> CreateSupportMintAssociatedAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 5] {
        [
            InstructionAccount::writable_signer(self.owner.address()),
            InstructionAccount::readonly(self.admin_group.address()),
            InstructionAccount::readonly(self.token_mint.address()),
            InstructionAccount::writable(self.support_mint_associated.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 5] {
        [
            self.owner, self.admin_group, self.token_mint, self.support_mint_associated, self.system_program
        ]
    }
}

/// Accounts for `decrease_liquidity`
pub struct DecreaseLiquidityAccounts<'a> {
    /// The position owner or delegated authority
    pub nft_owner: &'a AccountView,
    /// The token account for the tokenized position
    pub nft_account: &'a AccountView,
    /// Decrease liquidity for this position
    pub personal_position: &'a AccountView,
    /// pool_state
    pub pool_state: &'a AccountView,
    /// protocol_position
    pub protocol_position: &'a AccountView,
    /// Token_0 vault
    pub token_vault_0: &'a AccountView,
    /// Token_1 vault
    pub token_vault_1: &'a AccountView,
    /// Stores init state for the lower tick
    pub tick_array_lower: &'a AccountView,
    /// Stores init state for the upper tick
    pub tick_array_upper: &'a AccountView,
    /// The destination token account for receive amount_0
    pub recipient_token_account_0: &'a AccountView,
    /// The destination token account for receive amount_1
    pub recipient_token_account_1: &'a AccountView,
    /// SPL program to transfer out tokens
    pub token_program: &'a AccountView,
}

impl<'a> DecreaseLiquidityAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 12] {
        [
            InstructionAccount::readonly_signer(self.nft_owner.address()),
            InstructionAccount::readonly(self.nft_account.address()),
            InstructionAccount::writable(self.personal_position.address()),
            InstructionAccount::writable(self.pool_state.address()),
            InstructionAccount::writable(self.protocol_position.address()),
            InstructionAccount::writable(self.token_vault_0.address()),
            InstructionAccount::writable(self.token_vault_1.address()),
            InstructionAccount::writable(self.tick_array_lower.address()),
            InstructionAccount::writable(self.tick_array_upper.address()),
            InstructionAccount::writable(self.recipient_token_account_0.address()),
            InstructionAccount::writable(self.recipient_token_account_1.address()),
            InstructionAccount::readonly(self.token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 12] {
        [
            self.nft_owner, self.nft_account, self.personal_position, self.pool_state, self.protocol_position, self.token_vault_0, self.token_vault_1, self.tick_array_lower, self.tick_array_upper, self.recipient_token_account_0, self.recipient_token_account_1, self.token_program
        ]
    }
}

/// Accounts for `decrease_liquidity_v2`
pub struct DecreaseLiquidityV2Accounts<'a> {
    /// The position owner or delegated authority
    pub nft_owner: &'a AccountView,
    /// The token account for the tokenized position
    pub nft_account: &'a AccountView,
    /// Decrease liquidity for this position
    pub personal_position: &'a AccountView,
    /// pool_state
    pub pool_state: &'a AccountView,
    /// protocol_position
    pub protocol_position: &'a AccountView,
    /// Token_0 vault
    pub token_vault_0: &'a AccountView,
    /// Token_1 vault
    pub token_vault_1: &'a AccountView,
    /// Stores init state for the lower tick
    pub tick_array_lower: &'a AccountView,
    /// Stores init state for the upper tick
    pub tick_array_upper: &'a AccountView,
    /// The destination token account for receive amount_0
    pub recipient_token_account_0: &'a AccountView,
    /// The destination token account for receive amount_1
    pub recipient_token_account_1: &'a AccountView,
    /// SPL program to transfer out tokens
    pub token_program: &'a AccountView,
    /// Token program 2022
    pub token_program_2022: &'a AccountView,
    /// memo program
    pub memo_program: &'a AccountView,
    /// The mint of token vault 0
    pub vault_0_mint: &'a AccountView,
    /// The mint of token vault 1
    pub vault_1_mint: &'a AccountView,
}

impl<'a> DecreaseLiquidityV2Accounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 16] {
        [
            InstructionAccount::readonly_signer(self.nft_owner.address()),
            InstructionAccount::readonly(self.nft_account.address()),
            InstructionAccount::writable(self.personal_position.address()),
            InstructionAccount::writable(self.pool_state.address()),
            InstructionAccount::writable(self.protocol_position.address()),
            InstructionAccount::writable(self.token_vault_0.address()),
            InstructionAccount::writable(self.token_vault_1.address()),
            InstructionAccount::writable(self.tick_array_lower.address()),
            InstructionAccount::writable(self.tick_array_upper.address()),
            InstructionAccount::writable(self.recipient_token_account_0.address()),
            InstructionAccount::writable(self.recipient_token_account_1.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.token_program_2022.address()),
            InstructionAccount::readonly(self.memo_program.address()),
            InstructionAccount::readonly(self.vault_0_mint.address()),
            InstructionAccount::readonly(self.vault_1_mint.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 16] {
        [
            self.nft_owner, self.nft_account, self.personal_position, self.pool_state, self.protocol_position, self.token_vault_0, self.token_vault_1, self.tick_array_lower, self.tick_array_upper, self.recipient_token_account_0, self.recipient_token_account_1, self.token_program, self.token_program_2022, self.memo_program, self.vault_0_mint, self.vault_1_mint
        ]
    }
}

/// Accounts for `deposit_offchain_reward`
pub struct DepositOffchainRewardAccounts<'a> {
    /// the address paying to deposit the offchain reward.
    pub payer: &'a AccountView,
    /// The authority make decision that who can deposit the offchain reward.
    pub authority: &'a AccountView,
    /// Initialize amm admin group account to store admin permissions.
    pub admin_group: &'a AccountView,
    /// the pool id, which is the pool state account.
    pub pool_id: &'a AccountView,
    /// token_mint
    pub token_mint: &'a AccountView,
    /// 
    pub payer_token_account: &'a AccountView,
    /// reward_vault_token_account
    pub reward_vault_token_account: &'a AccountView,
    /// The offchain reward config account, it also is the reward vault account.
    pub reward_config: &'a AccountView,
    /// Spl token program or token program 2022
    pub token_program: &'a AccountView,
    /// associated_token_program
    pub associated_token_program: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
}

impl<'a> DepositOffchainRewardAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 11] {
        [
            InstructionAccount::writable_signer(self.payer.address()),
            InstructionAccount::readonly_signer(self.authority.address()),
            InstructionAccount::readonly(self.admin_group.address()),
            InstructionAccount::readonly(self.pool_id.address()),
            InstructionAccount::readonly(self.token_mint.address()),
            InstructionAccount::writable(self.payer_token_account.address()),
            InstructionAccount::writable(self.reward_vault_token_account.address()),
            InstructionAccount::writable(self.reward_config.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 11] {
        [
            self.payer, self.authority, self.admin_group, self.pool_id, self.token_mint, self.payer_token_account, self.reward_vault_token_account, self.reward_config, self.token_program, self.associated_token_program, self.system_program
        ]
    }
}

/// Accounts for `increase_liquidity`
pub struct IncreaseLiquidityAccounts<'a> {
    /// Pays to mint the position
    pub nft_owner: &'a AccountView,
    /// The token account for nft
    pub nft_account: &'a AccountView,
    /// pool_state
    pub pool_state: &'a AccountView,
    /// protocol_position
    pub protocol_position: &'a AccountView,
    /// Increase liquidity for this position
    pub personal_position: &'a AccountView,
    /// Stores init state for the lower tick
    pub tick_array_lower: &'a AccountView,
    /// Stores init state for the upper tick
    pub tick_array_upper: &'a AccountView,
    /// The payer's token account for token_0
    pub token_account_0: &'a AccountView,
    /// The token account spending token_1 to mint the position
    pub token_account_1: &'a AccountView,
    /// The address that holds pool tokens for token_0
    pub token_vault_0: &'a AccountView,
    /// The address that holds pool tokens for token_1
    pub token_vault_1: &'a AccountView,
    /// Program to create mint account and mint tokens
    pub token_program: &'a AccountView,
}

impl<'a> IncreaseLiquidityAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 12] {
        [
            InstructionAccount::readonly_signer(self.nft_owner.address()),
            InstructionAccount::readonly(self.nft_account.address()),
            InstructionAccount::writable(self.pool_state.address()),
            InstructionAccount::writable(self.protocol_position.address()),
            InstructionAccount::writable(self.personal_position.address()),
            InstructionAccount::writable(self.tick_array_lower.address()),
            InstructionAccount::writable(self.tick_array_upper.address()),
            InstructionAccount::writable(self.token_account_0.address()),
            InstructionAccount::writable(self.token_account_1.address()),
            InstructionAccount::writable(self.token_vault_0.address()),
            InstructionAccount::writable(self.token_vault_1.address()),
            InstructionAccount::readonly(self.token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 12] {
        [
            self.nft_owner, self.nft_account, self.pool_state, self.protocol_position, self.personal_position, self.tick_array_lower, self.tick_array_upper, self.token_account_0, self.token_account_1, self.token_vault_0, self.token_vault_1, self.token_program
        ]
    }
}

/// Accounts for `increase_liquidity_v2`
pub struct IncreaseLiquidityV2Accounts<'a> {
    /// Pays to mint the position
    pub nft_owner: &'a AccountView,
    /// The token account for nft
    pub nft_account: &'a AccountView,
    /// pool_state
    pub pool_state: &'a AccountView,
    /// protocol_position
    pub protocol_position: &'a AccountView,
    /// Increase liquidity for this position
    pub personal_position: &'a AccountView,
    /// Stores init state for the lower tick
    pub tick_array_lower: &'a AccountView,
    /// Stores init state for the upper tick
    pub tick_array_upper: &'a AccountView,
    /// The payer's token account for token_0
    pub token_account_0: &'a AccountView,
    /// The token account spending token_1 to mint the position
    pub token_account_1: &'a AccountView,
    /// The address that holds pool tokens for token_0
    pub token_vault_0: &'a AccountView,
    /// The address that holds pool tokens for token_1
    pub token_vault_1: &'a AccountView,
    /// Program to create mint account and mint tokens
    pub token_program: &'a AccountView,
    /// Token program 2022
    pub token_program_2022: &'a AccountView,
    /// The mint of token vault 0
    pub vault_0_mint: &'a AccountView,
    /// The mint of token vault 1
    pub vault_1_mint: &'a AccountView,
}

impl<'a> IncreaseLiquidityV2Accounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 15] {
        [
            InstructionAccount::readonly_signer(self.nft_owner.address()),
            InstructionAccount::readonly(self.nft_account.address()),
            InstructionAccount::writable(self.pool_state.address()),
            InstructionAccount::writable(self.protocol_position.address()),
            InstructionAccount::writable(self.personal_position.address()),
            InstructionAccount::writable(self.tick_array_lower.address()),
            InstructionAccount::writable(self.tick_array_upper.address()),
            InstructionAccount::writable(self.token_account_0.address()),
            InstructionAccount::writable(self.token_account_1.address()),
            InstructionAccount::writable(self.token_vault_0.address()),
            InstructionAccount::writable(self.token_vault_1.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.token_program_2022.address()),
            InstructionAccount::readonly(self.vault_0_mint.address()),
            InstructionAccount::readonly(self.vault_1_mint.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 15] {
        [
            self.nft_owner, self.nft_account, self.pool_state, self.protocol_position, self.personal_position, self.tick_array_lower, self.tick_array_upper, self.token_account_0, self.token_account_1, self.token_vault_0, self.token_vault_1, self.token_program, self.token_program_2022, self.vault_0_mint, self.vault_1_mint
        ]
    }
}

/// Accounts for `init_amm_admin_group`
pub struct InitAmmAdminGroupAccounts<'a> {
    /// only super admin can create admin group
    pub payer: &'a AccountView,
    /// Initialize amm admin group account to store admin permissions.
    pub admin_group: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
}

impl<'a> InitAmmAdminGroupAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 3] {
        [
            InstructionAccount::writable_signer(self.payer.address()),
            InstructionAccount::writable(self.admin_group.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 3] {
        [
            self.payer, self.admin_group, self.system_program
        ]
    }
}

/// Accounts for `initialize_reward`
pub struct InitializeRewardAccounts<'a> {
    /// The founder deposit reward token to vault
    pub reward_funder: &'a AccountView,
    /// amm admin group account to store admin permissions.
    pub admin_group: &'a AccountView,
    /// funder_token_account
    pub funder_token_account: &'a AccountView,
    /// For check the reward_funder authority
    pub amm_config: &'a AccountView,
    /// Set reward for this pool
    pub pool_state: &'a AccountView,
    /// load info from the account to judge reward permission
    pub operation_state: &'a AccountView,
    /// Reward mint
    pub reward_token_mint: &'a AccountView,
    /// A pda, reward vault
    pub reward_token_vault: &'a AccountView,
    /// reward_token_program
    pub reward_token_program: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
    /// rent
    pub rent: &'a AccountView,
}

impl<'a> InitializeRewardAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 11] {
        [
            InstructionAccount::writable_signer(self.reward_funder.address()),
            InstructionAccount::readonly(self.admin_group.address()),
            InstructionAccount::writable(self.funder_token_account.address()),
            InstructionAccount::readonly(self.amm_config.address()),
            InstructionAccount::writable(self.pool_state.address()),
            InstructionAccount::readonly(self.operation_state.address()),
            InstructionAccount::readonly(self.reward_token_mint.address()),
            InstructionAccount::writable(self.reward_token_vault.address()),
            InstructionAccount::readonly(self.reward_token_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.rent.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 11] {
        [
            self.reward_funder, self.admin_group, self.funder_token_account, self.amm_config, self.pool_state, self.operation_state, self.reward_token_mint, self.reward_token_vault, self.reward_token_program, self.system_program, self.rent
        ]
    }
}

/// Accounts for `open_position`
pub struct OpenPositionAccounts<'a> {
    /// Pays to mint the position
    pub payer: &'a AccountView,
    /// position_nft_owner
    pub position_nft_owner: &'a AccountView,
    /// Unique token mint address
    pub position_nft_mint: &'a AccountView,
    /// Token account where position NFT will be minted
    pub position_nft_account: &'a AccountView,
    /// To store metaplex metadata
    pub metadata_account: &'a AccountView,
    /// Add liquidity for this pool
    pub pool_state: &'a AccountView,
    /// Store the information of market marking in range
    pub protocol_position: &'a AccountView,
    /// tick_array_lower
    pub tick_array_lower: &'a AccountView,
    /// tick_array_upper
    pub tick_array_upper: &'a AccountView,
    /// personal position state
    pub personal_position: &'a AccountView,
    /// The token_0 account deposit token to the pool
    pub token_account_0: &'a AccountView,
    /// The token_1 account deposit token to the pool
    pub token_account_1: &'a AccountView,
    /// The address that holds pool tokens for token_0
    pub token_vault_0: &'a AccountView,
    /// The address that holds pool tokens for token_1
    pub token_vault_1: &'a AccountView,
    /// Sysvar for token mint and ATA creation
    pub rent: &'a AccountView,
    /// Program to create the position manager state account
    pub system_program: &'a AccountView,
    /// Program to create mint account and mint tokens
    pub token_program: &'a AccountView,
    /// Program to create an ATA for receiving position NFT
    pub associated_token_program: &'a AccountView,
    /// Program to create NFT metadata
    pub metadata_program: &'a AccountView,
}

impl<'a> OpenPositionAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 19] {
        [
            InstructionAccount::writable_signer(self.payer.address()),
            InstructionAccount::readonly(self.position_nft_owner.address()),
            InstructionAccount::writable_signer(self.position_nft_mint.address()),
            InstructionAccount::writable(self.position_nft_account.address()),
            InstructionAccount::writable(self.metadata_account.address()),
            InstructionAccount::writable(self.pool_state.address()),
            InstructionAccount::writable(self.protocol_position.address()),
            InstructionAccount::writable(self.tick_array_lower.address()),
            InstructionAccount::writable(self.tick_array_upper.address()),
            InstructionAccount::writable(self.personal_position.address()),
            InstructionAccount::writable(self.token_account_0.address()),
            InstructionAccount::writable(self.token_account_1.address()),
            InstructionAccount::writable(self.token_vault_0.address()),
            InstructionAccount::writable(self.token_vault_1.address()),
            InstructionAccount::readonly(self.rent.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
            InstructionAccount::readonly(self.metadata_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 19] {
        [
            self.payer, self.position_nft_owner, self.position_nft_mint, self.position_nft_account, self.metadata_account, self.pool_state, self.protocol_position, self.tick_array_lower, self.tick_array_upper, self.personal_position, self.token_account_0, self.token_account_1, self.token_vault_0, self.token_vault_1, self.rent, self.system_program, self.token_program, self.associated_token_program, self.metadata_program
        ]
    }
}

/// Accounts for `open_position_v2`
pub struct OpenPositionV2Accounts<'a> {
    /// Pays to mint the position
    pub payer: &'a AccountView,
    /// position_nft_owner
    pub position_nft_owner: &'a AccountView,
    /// Unique token mint address
    pub position_nft_mint: &'a AccountView,
    /// Token account where position NFT will be minted
    pub position_nft_account: &'a AccountView,
    /// To store metaplex metadata
    pub metadata_account: &'a AccountView,
    /// Add liquidity for this pool
    pub pool_state: &'a AccountView,
    /// Store the information of market marking in range
    pub protocol_position: &'a AccountView,
    /// tick_array_lower
    pub tick_array_lower: &'a AccountView,
    /// tick_array_upper
    pub tick_array_upper: &'a AccountView,
    /// personal position state
    pub personal_position: &'a AccountView,
    /// The token_0 account deposit token to the pool
    pub token_account_0: &'a AccountView,
    /// The token_1 account deposit token to the pool
    pub token_account_1: &'a AccountView,
    /// The address that holds pool tokens for token_0
    pub token_vault_0: &'a AccountView,
    /// The address that holds pool tokens for token_1
    pub token_vault_1: &'a AccountView,
    /// Sysvar for token mint and ATA creation
    pub rent: &'a AccountView,
    /// Program to create the position manager state account
    pub system_program: &'a AccountView,
    /// Program to create mint account and mint tokens
    pub token_program: &'a AccountView,
    /// Program to create an ATA for receiving position NFT
    pub associated_token_program: &'a AccountView,
    /// Program to create NFT metadata
    pub metadata_program: &'a AccountView,
    /// Program to create mint account and mint tokens
    pub token_program_2022: &'a AccountView,
    /// The mint of token vault 0
    pub vault_0_mint: &'a AccountView,
    /// The mint of token vault 1
    pub vault_1_mint: &'a AccountView,
}

impl<'a> OpenPositionV2Accounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 22] {
        [
            InstructionAccount::writable_signer(self.payer.address()),
            InstructionAccount::readonly(self.position_nft_owner.address()),
            InstructionAccount::writable_signer(self.position_nft_mint.address()),
            InstructionAccount::writable(self.position_nft_account.address()),
            InstructionAccount::writable(self.metadata_account.address()),
            InstructionAccount::writable(self.pool_state.address()),
            InstructionAccount::writable(self.protocol_position.address()),
            InstructionAccount::writable(self.tick_array_lower.address()),
            InstructionAccount::writable(self.tick_array_upper.address()),
            InstructionAccount::writable(self.personal_position.address()),
            InstructionAccount::writable(self.token_account_0.address()),
            InstructionAccount::writable(self.token_account_1.address()),
            InstructionAccount::writable(self.token_vault_0.address()),
            InstructionAccount::writable(self.token_vault_1.address()),
            InstructionAccount::readonly(self.rent.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
            InstructionAccount::readonly(self.metadata_program.address()),
            InstructionAccount::readonly(self.token_program_2022.address()),
            InstructionAccount::readonly(self.vault_0_mint.address()),
            InstructionAccount::readonly(self.vault_1_mint.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 22] {
        [
            self.payer, self.position_nft_owner, self.position_nft_mint, self.position_nft_account, self.metadata_account, self.pool_state, self.protocol_position, self.tick_array_lower, self.tick_array_upper, self.personal_position, self.token_account_0, self.token_account_1, self.token_vault_0, self.token_vault_1, self.rent, self.system_program, self.token_program, self.associated_token_program, self.metadata_program, self.token_program_2022, self.vault_0_mint, self.vault_1_mint
        ]
    }
}

/// Accounts for `open_position_with_token22_nft`
pub struct OpenPositionWithToken22NftAccounts<'a> {
    /// Pays to mint the position
    pub payer: &'a AccountView,
    /// position_nft_owner
    pub position_nft_owner: &'a AccountView,
    /// Unique token mint address, initialize in contract
    pub position_nft_mint: &'a AccountView,
    /// position_nft_account
    pub position_nft_account: &'a AccountView,
    /// Add liquidity for this pool
    pub pool_state: &'a AccountView,
    /// Store the information of market marking in range
    pub protocol_position: &'a AccountView,
    /// tick_array_lower
    pub tick_array_lower: &'a AccountView,
    /// tick_array_upper
    pub tick_array_upper: &'a AccountView,
    /// personal position state
    pub personal_position: &'a AccountView,
    /// The token_0 account deposit token to the pool
    pub token_account_0: &'a AccountView,
    /// The token_1 account deposit token to the pool
    pub token_account_1: &'a AccountView,
    /// The address that holds pool tokens for token_0
    pub token_vault_0: &'a AccountView,
    /// The address that holds pool tokens for token_1
    pub token_vault_1: &'a AccountView,
    /// Sysvar for token mint and ATA creation
    pub rent: &'a AccountView,
    /// Program to create the position manager state account
    pub system_program: &'a AccountView,
    /// Program to transfer for token account
    pub token_program: &'a AccountView,
    /// Program to create an ATA for receiving position NFT
    pub associated_token_program: &'a AccountView,
    /// Program to create NFT mint/token account and transfer for token22 account
    pub token_program_2022: &'a AccountView,
    /// The mint of token vault 0
    pub vault_0_mint: &'a AccountView,
    /// The mint of token vault 1
    pub vault_1_mint: &'a AccountView,
}

impl<'a> OpenPositionWithToken22NftAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 20] {
        [
            InstructionAccount::writable_signer(self.payer.address()),
            InstructionAccount::readonly(self.position_nft_owner.address()),
            InstructionAccount::writable_signer(self.position_nft_mint.address()),
            InstructionAccount::writable(self.position_nft_account.address()),
            InstructionAccount::writable(self.pool_state.address()),
            InstructionAccount::writable(self.protocol_position.address()),
            InstructionAccount::writable(self.tick_array_lower.address()),
            InstructionAccount::writable(self.tick_array_upper.address()),
            InstructionAccount::writable(self.personal_position.address()),
            InstructionAccount::writable(self.token_account_0.address()),
            InstructionAccount::writable(self.token_account_1.address()),
            InstructionAccount::writable(self.token_vault_0.address()),
            InstructionAccount::writable(self.token_vault_1.address()),
            InstructionAccount::readonly(self.rent.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
            InstructionAccount::readonly(self.token_program_2022.address()),
            InstructionAccount::readonly(self.vault_0_mint.address()),
            InstructionAccount::readonly(self.vault_1_mint.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 20] {
        [
            self.payer, self.position_nft_owner, self.position_nft_mint, self.position_nft_account, self.pool_state, self.protocol_position, self.tick_array_lower, self.tick_array_upper, self.personal_position, self.token_account_0, self.token_account_1, self.token_vault_0, self.token_vault_1, self.rent, self.system_program, self.token_program, self.associated_token_program, self.token_program_2022, self.vault_0_mint, self.vault_1_mint
        ]
    }
}

/// Accounts for `set_reward_params`
pub struct SetRewardParamsAccounts<'a> {
    /// Address to be set as protocol owner. It pays to create factory state account.
    pub authority: &'a AccountView,
    /// amm admin group account to store admin permissions.
    pub admin_group: &'a AccountView,
    /// amm_config
    pub amm_config: &'a AccountView,
    /// pool_state
    pub pool_state: &'a AccountView,
    /// load info from the account to judge reward permission
    pub operation_state: &'a AccountView,
    /// Token program
    pub token_program: &'a AccountView,
    /// Token program 2022
    pub token_program_2022: &'a AccountView,
}

impl<'a> SetRewardParamsAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 7] {
        [
            InstructionAccount::readonly_signer(self.authority.address()),
            InstructionAccount::readonly(self.admin_group.address()),
            InstructionAccount::readonly(self.amm_config.address()),
            InstructionAccount::writable(self.pool_state.address()),
            InstructionAccount::readonly(self.operation_state.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.token_program_2022.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 7] {
        [
            self.authority, self.admin_group, self.amm_config, self.pool_state, self.operation_state, self.token_program, self.token_program_2022
        ]
    }
}

/// Accounts for `swap`
pub struct SwapAccounts<'a> {
    /// The user performing the swap
    pub payer: &'a AccountView,
    /// The factory state to read protocol fees
    pub amm_config: &'a AccountView,
    /// The program account of the pool in which the swap will be performed
    pub pool_state: &'a AccountView,
    /// The user token account for input token
    pub input_token_account: &'a AccountView,
    /// The user token account for output token
    pub output_token_account: &'a AccountView,
    /// The vault token account for input token
    pub input_vault: &'a AccountView,
    /// The vault token account for output token
    pub output_vault: &'a AccountView,
    /// The program account for the most recent oracle observation
    pub observation_state: &'a AccountView,
    /// SPL program for token transfers
    pub token_program: &'a AccountView,
    /// tick_array
    pub tick_array: &'a AccountView,
}

impl<'a> SwapAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 10] {
        [
            InstructionAccount::readonly_signer(self.payer.address()),
            InstructionAccount::readonly(self.amm_config.address()),
            InstructionAccount::writable(self.pool_state.address()),
            InstructionAccount::writable(self.input_token_account.address()),
            InstructionAccount::writable(self.output_token_account.address()),
            InstructionAccount::writable(self.input_vault.address()),
            InstructionAccount::writable(self.output_vault.address()),
            InstructionAccount::writable(self.observation_state.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::writable(self.tick_array.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 10] {
        [
            self.payer, self.amm_config, self.pool_state, self.input_token_account, self.output_token_account, self.input_vault, self.output_vault, self.observation_state, self.token_program, self.tick_array
        ]
    }
}

/// Accounts for `swap_v2`
pub struct SwapV2Accounts<'a> {
    /// The user performing the swap
    pub payer: &'a AccountView,
    /// The factory state to read protocol fees
    pub amm_config: &'a AccountView,
    /// The program account of the pool in which the swap will be performed
    pub pool_state: &'a AccountView,
    /// The user token account for input token
    pub input_token_account: &'a AccountView,
    /// The user token account for output token
    pub output_token_account: &'a AccountView,
    /// The vault token account for input token
    pub input_vault: &'a AccountView,
    /// The vault token account for output token
    pub output_vault: &'a AccountView,
    /// The program account for the most recent oracle observation
    pub observation_state: &'a AccountView,
    /// SPL program for token transfers
    pub token_program: &'a AccountView,
    /// SPL program 2022 for token transfers
    pub token_program_2022: &'a AccountView,
    /// Memo program
    pub memo_program: &'a AccountView,
    /// The mint of token vault 0
    pub input_vault_mint: &'a AccountView,
    /// The mint of token vault 1
    pub output_vault_mint: &'a AccountView,
}

impl<'a> SwapV2Accounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 13] {
        [
            InstructionAccount::readonly_signer(self.payer.address()),
            InstructionAccount::readonly(self.amm_config.address()),
            InstructionAccount::writable(self.pool_state.address()),
            InstructionAccount::writable(self.input_token_account.address()),
            InstructionAccount::writable(self.output_token_account.address()),
            InstructionAccount::writable(self.input_vault.address()),
            InstructionAccount::writable(self.output_vault.address()),
            InstructionAccount::writable(self.observation_state.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.token_program_2022.address()),
            InstructionAccount::readonly(self.memo_program.address()),
            InstructionAccount::readonly(self.input_vault_mint.address()),
            InstructionAccount::readonly(self.output_vault_mint.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 13] {
        [
            self.payer, self.amm_config, self.pool_state, self.input_token_account, self.output_token_account, self.input_vault, self.output_vault, self.observation_state, self.token_program, self.token_program_2022, self.memo_program, self.input_vault_mint, self.output_vault_mint
        ]
    }
}

/// Accounts for `transfer_reward_owner`
pub struct TransferRewardOwnerAccounts<'a> {
    /// Address to be set as operation account owner.
    pub authority: &'a AccountView,
    /// amm admin group account to store admin permissions.
    pub admin_group: &'a AccountView,
    /// pool_state
    pub pool_state: &'a AccountView,
}

impl<'a> TransferRewardOwnerAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 3] {
        [
            InstructionAccount::readonly_signer(self.authority.address()),
            InstructionAccount::readonly(self.admin_group.address()),
            InstructionAccount::writable(self.pool_state.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 3] {
        [
            self.authority, self.admin_group, self.pool_state
        ]
    }
}

/// Accounts for `update_amm_admin_group`
pub struct UpdateAmmAdminGroupAccounts<'a> {
    /// only super admin can create admin group
    pub payer: &'a AccountView,
    /// update amm admin group account to store admin permissions.
    pub admin_group: &'a AccountView,
}

impl<'a> UpdateAmmAdminGroupAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 2] {
        [
            InstructionAccount::writable_signer(self.payer.address()),
            InstructionAccount::writable(self.admin_group.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 2] {
        [
            self.payer, self.admin_group
        ]
    }
}

/// Accounts for `update_amm_config`
pub struct UpdateAmmConfigAccounts<'a> {
    /// The amm config owner or admin
    pub owner: &'a AccountView,
    /// amm admin group account to store admin permissions.
    pub admin_group: &'a AccountView,
    /// Amm config account to be changed
    pub amm_config: &'a AccountView,
}

impl<'a> UpdateAmmConfigAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 3] {
        [
            InstructionAccount::readonly_signer(self.owner.address()),
            InstructionAccount::readonly(self.admin_group.address()),
            InstructionAccount::writable(self.amm_config.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 3] {
        [
            self.owner, self.admin_group, self.amm_config
        ]
    }
}

/// Accounts for `update_operation_account`
pub struct UpdateOperationAccountAccounts<'a> {
    /// Address to be set as operation account owner.
    pub owner: &'a AccountView,
    /// amm admin group account to store admin permissions.
    pub admin_group: &'a AccountView,
    /// Initialize operation state account to store operation owner address and white list mint.
    pub operation_state: &'a AccountView,
    /// system_program
    pub system_program: &'a AccountView,
}

impl<'a> UpdateOperationAccountAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 4] {
        [
            InstructionAccount::readonly_signer(self.owner.address()),
            InstructionAccount::readonly(self.admin_group.address()),
            InstructionAccount::writable(self.operation_state.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 4] {
        [
            self.owner, self.admin_group, self.operation_state, self.system_program
        ]
    }
}

/// Accounts for `update_pool_status`
pub struct UpdatePoolStatusAccounts<'a> {
    /// authority
    pub authority: &'a AccountView,
    /// amm admin group account to store admin permissions.
    pub admin_group: &'a AccountView,
    /// pool_state
    pub pool_state: &'a AccountView,
}

impl<'a> UpdatePoolStatusAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 3] {
        [
            InstructionAccount::readonly_signer(self.authority.address()),
            InstructionAccount::readonly(self.admin_group.address()),
            InstructionAccount::writable(self.pool_state.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 3] {
        [
            self.authority, self.admin_group, self.pool_state
        ]
    }
}

/// Accounts for `update_reward_infos`
pub struct UpdateRewardInfosAccounts<'a> {
    /// The liquidity pool for which reward info to update
    pub pool_state: &'a AccountView,
}

impl<'a> UpdateRewardInfosAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 1] {
        [
            InstructionAccount::writable(self.pool_state.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 1] {
        [
            self.pool_state
        ]
    }
}

/// Accounts for `withdraw_offchain_reward`
pub struct WithdrawOffchainRewardAccounts<'a> {
    /// The authority make decision that who can withdraw the offchain reward.
    pub authority: &'a AccountView,
    /// Initialize amm admin group account to store admin permissions.
    pub admin_group: &'a AccountView,
    /// the pool id, which is the pool state account.
    pub pool_id: &'a AccountView,
    /// token_mint
    pub token_mint: &'a AccountView,
    /// the address who receive the withdrawn offchain reward.
    pub receiver_token_account: &'a AccountView,
    /// reward_vault_token_account
    pub reward_vault_token_account: &'a AccountView,
    /// The offchain reward config account, it also is the reward vault account.
    pub reward_config: &'a AccountView,
    /// Spl token program or token program 2022
    pub token_program: &'a AccountView,
    /// associated_token_program
    pub associated_token_program: &'a AccountView,
}

impl<'a> WithdrawOffchainRewardAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 9] {
        [
            InstructionAccount::readonly_signer(self.authority.address()),
            InstructionAccount::readonly(self.admin_group.address()),
            InstructionAccount::readonly(self.pool_id.address()),
            InstructionAccount::readonly(self.token_mint.address()),
            InstructionAccount::writable(self.receiver_token_account.address()),
            InstructionAccount::writable(self.reward_vault_token_account.address()),
            InstructionAccount::writable(self.reward_config.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 9] {
        [
            self.authority, self.admin_group, self.pool_id, self.token_mint, self.receiver_token_account, self.reward_vault_token_account, self.reward_config, self.token_program, self.associated_token_program
        ]
    }
}


// ============================================
// CPI Functions
// ============================================
/// CPI: claim_offchain_reward
#[inline(always)]
pub fn claim_offchain_reward<'a>(
    accounts: &ClaimOffchainRewardAccounts<'a>, args: &ClaimOffchainRewardArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<ClaimOffchainRewardArgs>()];
    data[0..8].copy_from_slice(&CLAIM_OFFCHAIN_REWARD);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const ClaimOffchainRewardArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<ClaimOffchainRewardArgs>(),
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
    invoke_signed::<6>(&instruction, &account_views, signers)
}

/// CPI: collect_fund_fee
#[inline(always)]
pub fn collect_fund_fee<'a>(
    accounts: &CollectFundFeeAccounts<'a>, args: &CollectFundFeeArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<CollectFundFeeArgs>()];
    data[0..8].copy_from_slice(&COLLECT_FUND_FEE);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const CollectFundFeeArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<CollectFundFeeArgs>(),
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

/// CPI: collect_protocol_fee
#[inline(always)]
pub fn collect_protocol_fee<'a>(
    accounts: &CollectProtocolFeeAccounts<'a>, args: &CollectProtocolFeeArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<CollectProtocolFeeArgs>()];
    data[0..8].copy_from_slice(&COLLECT_PROTOCOL_FEE);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const CollectProtocolFeeArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<CollectProtocolFeeArgs>(),
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

/// CPI: collect_remaining_rewards
#[inline(always)]
pub fn collect_remaining_rewards<'a>(
    accounts: &CollectRemainingRewardsAccounts<'a>, args: &CollectRemainingRewardsArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<CollectRemainingRewardsArgs>()];
    data[0..8].copy_from_slice(&COLLECT_REMAINING_REWARDS);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const CollectRemainingRewardsArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<CollectRemainingRewardsArgs>(),
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

/// CPI: create_amm_config
#[inline(always)]
pub fn create_amm_config<'a>(
    accounts: &CreateAmmConfigAccounts<'a>, args: &CreateAmmConfigArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<CreateAmmConfigArgs>()];
    data[0..8].copy_from_slice(&CREATE_AMM_CONFIG);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const CreateAmmConfigArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<CreateAmmConfigArgs>(),
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

/// CPI: create_operation_account
#[inline(always)]
pub fn create_operation_account<'a>(
    accounts: &CreateOperationAccountAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&CREATE_OPERATION_ACCOUNT);
    
    
    // Build InstructionAccount array
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
    invoke_signed::<16>(&instruction, &account_views, signers)
}

/// CPI: create_support_mint_associated
#[inline(always)]
pub fn create_support_mint_associated<'a>(
    accounts: &CreateSupportMintAssociatedAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&CREATE_SUPPORT_MINT_ASSOCIATED);
    
    
    // Build InstructionAccount array
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

/// CPI: decrease_liquidity
#[inline(always)]
pub fn decrease_liquidity<'a>(
    accounts: &DecreaseLiquidityAccounts<'a>, args: &DecreaseLiquidityArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<DecreaseLiquidityArgs>()];
    data[0..8].copy_from_slice(&DECREASE_LIQUIDITY);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const DecreaseLiquidityArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<DecreaseLiquidityArgs>(),
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

/// CPI: decrease_liquidity_v2
#[inline(always)]
pub fn decrease_liquidity_v2<'a>(
    accounts: &DecreaseLiquidityV2Accounts<'a>, args: &DecreaseLiquidityV2Args,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<DecreaseLiquidityV2Args>()];
    data[0..8].copy_from_slice(&DECREASE_LIQUIDITY_V2);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const DecreaseLiquidityV2Args as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<DecreaseLiquidityV2Args>(),
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

/// CPI: deposit_offchain_reward
#[inline(always)]
pub fn deposit_offchain_reward<'a>(
    accounts: &DepositOffchainRewardAccounts<'a>, args: &DepositOffchainRewardArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<DepositOffchainRewardArgs>()];
    data[0..8].copy_from_slice(&DEPOSIT_OFFCHAIN_REWARD);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const DepositOffchainRewardArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<DepositOffchainRewardArgs>(),
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

/// CPI: increase_liquidity
#[inline(always)]
pub fn increase_liquidity<'a>(
    accounts: &IncreaseLiquidityAccounts<'a>, args: &IncreaseLiquidityArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<IncreaseLiquidityArgs>()];
    data[0..8].copy_from_slice(&INCREASE_LIQUIDITY);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const IncreaseLiquidityArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<IncreaseLiquidityArgs>(),
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

/// CPI: increase_liquidity_v2
#[inline(always)]
pub fn increase_liquidity_v2<'a>(
    accounts: &IncreaseLiquidityV2Accounts<'a>, args: &IncreaseLiquidityV2Args,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<IncreaseLiquidityV2Args>()];
    data[0..8].copy_from_slice(&INCREASE_LIQUIDITY_V2);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const IncreaseLiquidityV2Args as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<IncreaseLiquidityV2Args>(),
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

/// CPI: init_amm_admin_group
#[inline(always)]
pub fn init_amm_admin_group<'a>(
    accounts: &InitAmmAdminGroupAccounts<'a>, args: &InitAmmAdminGroupArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<InitAmmAdminGroupArgs>()];
    data[0..8].copy_from_slice(&INIT_AMM_ADMIN_GROUP);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const InitAmmAdminGroupArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<InitAmmAdminGroupArgs>(),
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

/// CPI: open_position
#[inline(always)]
pub fn open_position<'a>(
    accounts: &OpenPositionAccounts<'a>, args: &OpenPositionArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<OpenPositionArgs>()];
    data[0..8].copy_from_slice(&OPEN_POSITION);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const OpenPositionArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<OpenPositionArgs>(),
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

/// CPI: open_position_v2
#[inline(always)]
pub fn open_position_v2<'a>(
    accounts: &OpenPositionV2Accounts<'a>, args: &OpenPositionV2Args,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<OpenPositionV2Args>()];
    data[0..8].copy_from_slice(&OPEN_POSITION_V2);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const OpenPositionV2Args as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<OpenPositionV2Args>(),
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

/// CPI: open_position_with_token22_nft
#[inline(always)]
pub fn open_position_with_token22_nft<'a>(
    accounts: &OpenPositionWithToken22NftAccounts<'a>, args: &OpenPositionWithToken22NftArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<OpenPositionWithToken22NftArgs>()];
    data[0..8].copy_from_slice(&OPEN_POSITION_WITH_TOKEN22_NFT);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const OpenPositionWithToken22NftArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<OpenPositionWithToken22NftArgs>(),
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

/// CPI: set_reward_params
#[inline(always)]
pub fn set_reward_params<'a>(
    accounts: &SetRewardParamsAccounts<'a>, args: &SetRewardParamsArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SetRewardParamsArgs>()];
    data[0..8].copy_from_slice(&SET_REWARD_PARAMS);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SetRewardParamsArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SetRewardParamsArgs>(),
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
    invoke_signed::<10>(&instruction, &account_views, signers)
}

/// CPI: swap_v2
#[inline(always)]
pub fn swap_v2<'a>(
    accounts: &SwapV2Accounts<'a>, args: &SwapV2Args,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SwapV2Args>()];
    data[0..8].copy_from_slice(&SWAP_V2);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SwapV2Args as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SwapV2Args>(),
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

/// CPI: transfer_reward_owner
#[inline(always)]
pub fn transfer_reward_owner<'a>(
    accounts: &TransferRewardOwnerAccounts<'a>, args: &TransferRewardOwnerArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<TransferRewardOwnerArgs>()];
    data[0..8].copy_from_slice(&TRANSFER_REWARD_OWNER);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const TransferRewardOwnerArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<TransferRewardOwnerArgs>(),
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

/// CPI: update_amm_admin_group
#[inline(always)]
pub fn update_amm_admin_group<'a>(
    accounts: &UpdateAmmAdminGroupAccounts<'a>, args: &UpdateAmmAdminGroupArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<UpdateAmmAdminGroupArgs>()];
    data[0..8].copy_from_slice(&UPDATE_AMM_ADMIN_GROUP);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const UpdateAmmAdminGroupArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<UpdateAmmAdminGroupArgs>(),
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

/// CPI: update_amm_config
#[inline(always)]
pub fn update_amm_config<'a>(
    accounts: &UpdateAmmConfigAccounts<'a>, args: &UpdateAmmConfigArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<UpdateAmmConfigArgs>()];
    data[0..8].copy_from_slice(&UPDATE_AMM_CONFIG);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const UpdateAmmConfigArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<UpdateAmmConfigArgs>(),
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

/// CPI: update_operation_account
#[inline(always)]
pub fn update_operation_account<'a>(
    accounts: &UpdateOperationAccountAccounts<'a>, args: &UpdateOperationAccountArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<UpdateOperationAccountArgs>()];
    data[0..8].copy_from_slice(&UPDATE_OPERATION_ACCOUNT);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const UpdateOperationAccountArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<UpdateOperationAccountArgs>(),
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

/// CPI: update_pool_status
#[inline(always)]
pub fn update_pool_status<'a>(
    accounts: &UpdatePoolStatusAccounts<'a>, args: &UpdatePoolStatusArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<UpdatePoolStatusArgs>()];
    data[0..8].copy_from_slice(&UPDATE_POOL_STATUS);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const UpdatePoolStatusArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<UpdatePoolStatusArgs>(),
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

/// CPI: update_reward_infos
#[inline(always)]
pub fn update_reward_infos<'a>(
    accounts: &UpdateRewardInfosAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&UPDATE_REWARD_INFOS);
    
    
    // Build InstructionAccount array
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

/// CPI: withdraw_offchain_reward
#[inline(always)]
pub fn withdraw_offchain_reward<'a>(
    accounts: &WithdrawOffchainRewardAccounts<'a>, args: &WithdrawOffchainRewardArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<WithdrawOffchainRewardArgs>()];
    data[0..8].copy_from_slice(&WITHDRAW_OFFCHAIN_REWARD);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const WithdrawOffchainRewardArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<WithdrawOffchainRewardArgs>(),
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

