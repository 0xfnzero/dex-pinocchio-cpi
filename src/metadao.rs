//! CPI module for MetaDAO
//!
//! Program: futarchy
//! Program ID: FUTARELBfJfQ8RDGhg1wdhddq1odMAJUePHFuBYfUxKq
//! Instructions: 17

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
pub const ID: Address = Address::new_from_array(five8_const::decode_32_const("FUTARELBfJfQ8RDGhg1wdhddq1odMAJUePHFuBYfUxKq"));

// ============================================
// Instruction Discriminators
// ============================================
pub const INITIALIZE_DAO: [u8; 8] = [128, 226, 96, 90, 39, 56, 24, 196];
pub const INITIALIZE_PROPOSAL: [u8; 8] = [50, 73, 156, 98, 129, 149, 21, 158];
pub const STAKE_TO_PROPOSAL: [u8; 8] = [10, 169, 175, 238, 80, 221, 37, 16];
pub const UNSTAKE_FROM_PROPOSAL: [u8; 8] = [179, 220, 186, 86, 2, 96, 50, 161];
pub const LAUNCH_PROPOSAL: [u8; 8] = [16, 211, 189, 119, 245, 72, 0, 229];
pub const FINALIZE_PROPOSAL: [u8; 8] = [23, 68, 51, 167, 109, 173, 187, 164];
pub const UPDATE_DAO: [u8; 8] = [131, 72, 75, 25, 112, 210, 109, 2];
pub const SPOT_SWAP: [u8; 8] = [167, 97, 12, 231, 237, 78, 166, 251];
pub const CONDITIONAL_SWAP: [u8; 8] = [194, 136, 220, 89, 242, 169, 130, 157];
pub const PROVIDE_LIQUIDITY: [u8; 8] = [40, 110, 107, 116, 174, 127, 97, 204];
pub const WITHDRAW_LIQUIDITY: [u8; 8] = [149, 158, 33, 185, 47, 243, 253, 31];
pub const COLLECT_FEES: [u8; 8] = [164, 152, 207, 99, 30, 186, 19, 182];
pub const EXECUTE_SPENDING_LIMIT_CHANGE: [u8; 8] = [146, 175, 145, 31, 184, 129, 252, 79];
pub const SPONSOR_PROPOSAL: [u8; 8] = [193, 57, 170, 136, 101, 196, 58, 173];
pub const COLLECT_METEORA_DAMM_FEES: [u8; 8] = [139, 212, 105, 118, 126, 54, 214, 143];
pub const ADMIN_APPROVE_EXECUTE_MULTISIG_PROPOSAL: [u8; 8] = [99, 14, 66, 64, 95, 59, 11, 96];
pub const ADMIN_REMOVE_PROPOSAL: [u8; 8] = [242, 199, 27, 28, 7, 108, 122, 73];

// ============================================
// Instruction Arguments
// ============================================
/// Arguments for `initializeDao`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct InitializedaoArgs {
    pub params: [u8; 32],
}

/// Arguments for `stakeToProposal`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct StaketoproposalArgs {
    pub params: [u8; 32],
}

/// Arguments for `unstakeFromProposal`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct UnstakefromproposalArgs {
    pub params: [u8; 32],
}

/// Arguments for `updateDao`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct UpdatedaoArgs {
    pub dao_params: [u8; 32],
}

/// Arguments for `spotSwap`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SpotswapArgs {
    pub params: [u8; 32],
}

/// Arguments for `conditionalSwap`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct ConditionalswapArgs {
    pub params: [u8; 32],
}

/// Arguments for `provideLiquidity`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct ProvideliquidityArgs {
    pub params: [u8; 32],
}

/// Arguments for `withdrawLiquidity`
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct WithdrawliquidityArgs {
    pub params: [u8; 32],
}


// ============================================
// Instruction Accounts
// ============================================
/// Accounts for `initializeDao`
pub struct InitializedaoAccounts<'a> {
    /// dao
    pub dao: &'a AccountView,
    /// daoCreator
    pub dao_creator: &'a AccountView,
    /// payer
    pub payer: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
    /// baseMint
    pub base_mint: &'a AccountView,
    /// quoteMint
    pub quote_mint: &'a AccountView,
    /// squadsMultisig
    pub squads_multisig: &'a AccountView,
    /// squadsMultisigVault
    pub squads_multisig_vault: &'a AccountView,
    /// squadsProgram
    pub squads_program: &'a AccountView,
    /// squadsProgramConfig
    pub squads_program_config: &'a AccountView,
    /// squadsProgramConfigTreasury
    pub squads_program_config_treasury: &'a AccountView,
    /// spendingLimit
    pub spending_limit: &'a AccountView,
    /// futarchyAmmBaseVault
    pub futarchy_amm_base_vault: &'a AccountView,
    /// futarchyAmmQuoteVault
    pub futarchy_amm_quote_vault: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// associatedTokenProgram
    pub associated_token_program: &'a AccountView,
    /// eventAuthority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> InitializedaoAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 18] {
        [
            InstructionAccount::writable(self.dao.address()),
            InstructionAccount::readonly_signer(self.dao_creator.address()),
            InstructionAccount::writable_signer(self.payer.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.base_mint.address()),
            InstructionAccount::readonly(self.quote_mint.address()),
            InstructionAccount::writable(self.squads_multisig.address()),
            InstructionAccount::readonly(self.squads_multisig_vault.address()),
            InstructionAccount::readonly(self.squads_program.address()),
            InstructionAccount::readonly(self.squads_program_config.address()),
            InstructionAccount::writable(self.squads_program_config_treasury.address()),
            InstructionAccount::writable(self.spending_limit.address()),
            InstructionAccount::writable(self.futarchy_amm_base_vault.address()),
            InstructionAccount::writable(self.futarchy_amm_quote_vault.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 18] {
        [
            self.dao, self.dao_creator, self.payer, self.system_program, self.base_mint, self.quote_mint, self.squads_multisig, self.squads_multisig_vault, self.squads_program, self.squads_program_config, self.squads_program_config_treasury, self.spending_limit, self.futarchy_amm_base_vault, self.futarchy_amm_quote_vault, self.token_program, self.associated_token_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `initializeProposal`
pub struct InitializeproposalAccounts<'a> {
    /// proposal
    pub proposal: &'a AccountView,
    /// squadsProposal
    pub squads_proposal: &'a AccountView,
    /// squadsMultisig
    pub squads_multisig: &'a AccountView,
    /// dao
    pub dao: &'a AccountView,
    /// question
    pub question: &'a AccountView,
    /// quoteVault
    pub quote_vault: &'a AccountView,
    /// baseVault
    pub base_vault: &'a AccountView,
    /// proposer
    pub proposer: &'a AccountView,
    /// payer
    pub payer: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
    /// eventAuthority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> InitializeproposalAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 12] {
        [
            InstructionAccount::writable(self.proposal.address()),
            InstructionAccount::readonly(self.squads_proposal.address()),
            InstructionAccount::readonly(self.squads_multisig.address()),
            InstructionAccount::writable(self.dao.address()),
            InstructionAccount::readonly(self.question.address()),
            InstructionAccount::readonly(self.quote_vault.address()),
            InstructionAccount::readonly(self.base_vault.address()),
            InstructionAccount::readonly_signer(self.proposer.address()),
            InstructionAccount::writable_signer(self.payer.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 12] {
        [
            self.proposal, self.squads_proposal, self.squads_multisig, self.dao, self.question, self.quote_vault, self.base_vault, self.proposer, self.payer, self.system_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `stakeToProposal`
pub struct StaketoproposalAccounts<'a> {
    /// proposal
    pub proposal: &'a AccountView,
    /// dao
    pub dao: &'a AccountView,
    /// stakerBaseAccount
    pub staker_base_account: &'a AccountView,
    /// proposalBaseAccount
    pub proposal_base_account: &'a AccountView,
    /// stakeAccount
    pub stake_account: &'a AccountView,
    /// staker
    pub staker: &'a AccountView,
    /// payer
    pub payer: &'a AccountView,
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

impl<'a> StaketoproposalAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 12] {
        [
            InstructionAccount::writable(self.proposal.address()),
            InstructionAccount::writable(self.dao.address()),
            InstructionAccount::writable(self.staker_base_account.address()),
            InstructionAccount::writable(self.proposal_base_account.address()),
            InstructionAccount::writable(self.stake_account.address()),
            InstructionAccount::readonly_signer(self.staker.address()),
            InstructionAccount::writable_signer(self.payer.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 12] {
        [
            self.proposal, self.dao, self.staker_base_account, self.proposal_base_account, self.stake_account, self.staker, self.payer, self.token_program, self.associated_token_program, self.system_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `unstakeFromProposal`
pub struct UnstakefromproposalAccounts<'a> {
    /// proposal
    pub proposal: &'a AccountView,
    /// dao
    pub dao: &'a AccountView,
    /// stakerBaseAccount
    pub staker_base_account: &'a AccountView,
    /// proposalBaseAccount
    pub proposal_base_account: &'a AccountView,
    /// stakeAccount
    pub stake_account: &'a AccountView,
    /// staker
    pub staker: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// associatedTokenProgram
    pub associated_token_program: &'a AccountView,
    /// eventAuthority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> UnstakefromproposalAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 10] {
        [
            InstructionAccount::writable(self.proposal.address()),
            InstructionAccount::writable(self.dao.address()),
            InstructionAccount::writable(self.staker_base_account.address()),
            InstructionAccount::writable(self.proposal_base_account.address()),
            InstructionAccount::writable(self.stake_account.address()),
            InstructionAccount::readonly_signer(self.staker.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 10] {
        [
            self.proposal, self.dao, self.staker_base_account, self.proposal_base_account, self.stake_account, self.staker, self.token_program, self.associated_token_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `launchProposal`
pub struct LaunchproposalAccounts<'a> {
    /// proposal
    pub proposal: &'a AccountView,
    /// baseVault
    pub base_vault: &'a AccountView,
    /// quoteVault
    pub quote_vault: &'a AccountView,
    /// passBaseMint
    pub pass_base_mint: &'a AccountView,
    /// passQuoteMint
    pub pass_quote_mint: &'a AccountView,
    /// failBaseMint
    pub fail_base_mint: &'a AccountView,
    /// failQuoteMint
    pub fail_quote_mint: &'a AccountView,
    /// dao
    pub dao: &'a AccountView,
    /// payer
    pub payer: &'a AccountView,
    /// ammPassBaseVault
    pub amm_pass_base_vault: &'a AccountView,
    /// ammPassQuoteVault
    pub amm_pass_quote_vault: &'a AccountView,
    /// ammFailBaseVault
    pub amm_fail_base_vault: &'a AccountView,
    /// ammFailQuoteVault
    pub amm_fail_quote_vault: &'a AccountView,
    /// squadsMultisig
    pub squads_multisig: &'a AccountView,
    /// squadsProposal
    pub squads_proposal: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// associatedTokenProgram
    pub associated_token_program: &'a AccountView,
    /// eventAuthority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> LaunchproposalAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 20] {
        [
            InstructionAccount::writable(self.proposal.address()),
            InstructionAccount::readonly(self.base_vault.address()),
            InstructionAccount::readonly(self.quote_vault.address()),
            InstructionAccount::readonly(self.pass_base_mint.address()),
            InstructionAccount::readonly(self.pass_quote_mint.address()),
            InstructionAccount::readonly(self.fail_base_mint.address()),
            InstructionAccount::readonly(self.fail_quote_mint.address()),
            InstructionAccount::writable(self.dao.address()),
            InstructionAccount::writable_signer(self.payer.address()),
            InstructionAccount::writable(self.amm_pass_base_vault.address()),
            InstructionAccount::writable(self.amm_pass_quote_vault.address()),
            InstructionAccount::writable(self.amm_fail_base_vault.address()),
            InstructionAccount::writable(self.amm_fail_quote_vault.address()),
            InstructionAccount::readonly(self.squads_multisig.address()),
            InstructionAccount::readonly(self.squads_proposal.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.associated_token_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 20] {
        [
            self.proposal, self.base_vault, self.quote_vault, self.pass_base_mint, self.pass_quote_mint, self.fail_base_mint, self.fail_quote_mint, self.dao, self.payer, self.amm_pass_base_vault, self.amm_pass_quote_vault, self.amm_fail_base_vault, self.amm_fail_quote_vault, self.squads_multisig, self.squads_proposal, self.system_program, self.token_program, self.associated_token_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `finalizeProposal`
pub struct FinalizeproposalAccounts<'a> {
    /// proposal
    pub proposal: &'a AccountView,
    /// dao
    pub dao: &'a AccountView,
    /// question
    pub question: &'a AccountView,
    /// squadsProposal
    pub squads_proposal: &'a AccountView,
    /// squadsMultisig
    pub squads_multisig: &'a AccountView,
    /// squadsMultisigProgram
    pub squads_multisig_program: &'a AccountView,
    /// ammPassBaseVault
    pub amm_pass_base_vault: &'a AccountView,
    /// ammPassQuoteVault
    pub amm_pass_quote_vault: &'a AccountView,
    /// ammFailBaseVault
    pub amm_fail_base_vault: &'a AccountView,
    /// ammFailQuoteVault
    pub amm_fail_quote_vault: &'a AccountView,
    /// ammBaseVault
    pub amm_base_vault: &'a AccountView,
    /// ammQuoteVault
    pub amm_quote_vault: &'a AccountView,
    /// vaultProgram
    pub vault_program: &'a AccountView,
    /// vaultEventAuthority
    pub vault_event_authority: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// quoteVault
    pub quote_vault: &'a AccountView,
    /// quoteVaultUnderlyingTokenAccount
    pub quote_vault_underlying_token_account: &'a AccountView,
    /// passQuoteMint
    pub pass_quote_mint: &'a AccountView,
    /// failQuoteMint
    pub fail_quote_mint: &'a AccountView,
    /// passBaseMint
    pub pass_base_mint: &'a AccountView,
    /// failBaseMint
    pub fail_base_mint: &'a AccountView,
    /// baseVault
    pub base_vault: &'a AccountView,
    /// baseVaultUnderlyingTokenAccount
    pub base_vault_underlying_token_account: &'a AccountView,
    /// eventAuthority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> FinalizeproposalAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 25] {
        [
            InstructionAccount::writable(self.proposal.address()),
            InstructionAccount::writable(self.dao.address()),
            InstructionAccount::writable(self.question.address()),
            InstructionAccount::writable(self.squads_proposal.address()),
            InstructionAccount::readonly(self.squads_multisig.address()),
            InstructionAccount::readonly(self.squads_multisig_program.address()),
            InstructionAccount::writable(self.amm_pass_base_vault.address()),
            InstructionAccount::writable(self.amm_pass_quote_vault.address()),
            InstructionAccount::writable(self.amm_fail_base_vault.address()),
            InstructionAccount::writable(self.amm_fail_quote_vault.address()),
            InstructionAccount::writable(self.amm_base_vault.address()),
            InstructionAccount::writable(self.amm_quote_vault.address()),
            InstructionAccount::readonly(self.vault_program.address()),
            InstructionAccount::readonly(self.vault_event_authority.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::writable(self.quote_vault.address()),
            InstructionAccount::writable(self.quote_vault_underlying_token_account.address()),
            InstructionAccount::writable(self.pass_quote_mint.address()),
            InstructionAccount::writable(self.fail_quote_mint.address()),
            InstructionAccount::writable(self.pass_base_mint.address()),
            InstructionAccount::writable(self.fail_base_mint.address()),
            InstructionAccount::writable(self.base_vault.address()),
            InstructionAccount::writable(self.base_vault_underlying_token_account.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 25] {
        [
            self.proposal, self.dao, self.question, self.squads_proposal, self.squads_multisig, self.squads_multisig_program, self.amm_pass_base_vault, self.amm_pass_quote_vault, self.amm_fail_base_vault, self.amm_fail_quote_vault, self.amm_base_vault, self.amm_quote_vault, self.vault_program, self.vault_event_authority, self.token_program, self.quote_vault, self.quote_vault_underlying_token_account, self.pass_quote_mint, self.fail_quote_mint, self.pass_base_mint, self.fail_base_mint, self.base_vault, self.base_vault_underlying_token_account, self.event_authority, self.program
        ]
    }
}

/// Accounts for `updateDao`
pub struct UpdatedaoAccounts<'a> {
    /// dao
    pub dao: &'a AccountView,
    /// squadsMultisigVault
    pub squads_multisig_vault: &'a AccountView,
    /// eventAuthority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> UpdatedaoAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 4] {
        [
            InstructionAccount::writable(self.dao.address()),
            InstructionAccount::readonly_signer(self.squads_multisig_vault.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 4] {
        [
            self.dao, self.squads_multisig_vault, self.event_authority, self.program
        ]
    }
}

/// Accounts for `spotSwap`
pub struct SpotswapAccounts<'a> {
    /// dao
    pub dao: &'a AccountView,
    /// userBaseAccount
    pub user_base_account: &'a AccountView,
    /// userQuoteAccount
    pub user_quote_account: &'a AccountView,
    /// ammBaseVault
    pub amm_base_vault: &'a AccountView,
    /// ammQuoteVault
    pub amm_quote_vault: &'a AccountView,
    /// user
    pub user: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// eventAuthority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> SpotswapAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 9] {
        [
            InstructionAccount::writable(self.dao.address()),
            InstructionAccount::writable(self.user_base_account.address()),
            InstructionAccount::writable(self.user_quote_account.address()),
            InstructionAccount::writable(self.amm_base_vault.address()),
            InstructionAccount::writable(self.amm_quote_vault.address()),
            InstructionAccount::readonly_signer(self.user.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 9] {
        [
            self.dao, self.user_base_account, self.user_quote_account, self.amm_base_vault, self.amm_quote_vault, self.user, self.token_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `conditionalSwap`
pub struct ConditionalswapAccounts<'a> {
    /// dao
    pub dao: &'a AccountView,
    /// ammBaseVault
    pub amm_base_vault: &'a AccountView,
    /// ammQuoteVault
    pub amm_quote_vault: &'a AccountView,
    /// proposal
    pub proposal: &'a AccountView,
    /// ammPassBaseVault
    pub amm_pass_base_vault: &'a AccountView,
    /// ammPassQuoteVault
    pub amm_pass_quote_vault: &'a AccountView,
    /// ammFailBaseVault
    pub amm_fail_base_vault: &'a AccountView,
    /// ammFailQuoteVault
    pub amm_fail_quote_vault: &'a AccountView,
    /// trader
    pub trader: &'a AccountView,
    /// userInputAccount
    pub user_input_account: &'a AccountView,
    /// userOutputAccount
    pub user_output_account: &'a AccountView,
    /// baseVault
    pub base_vault: &'a AccountView,
    /// baseVaultUnderlyingTokenAccount
    pub base_vault_underlying_token_account: &'a AccountView,
    /// quoteVault
    pub quote_vault: &'a AccountView,
    /// quoteVaultUnderlyingTokenAccount
    pub quote_vault_underlying_token_account: &'a AccountView,
    /// passBaseMint
    pub pass_base_mint: &'a AccountView,
    /// failBaseMint
    pub fail_base_mint: &'a AccountView,
    /// passQuoteMint
    pub pass_quote_mint: &'a AccountView,
    /// failQuoteMint
    pub fail_quote_mint: &'a AccountView,
    /// conditionalVaultProgram
    pub conditional_vault_program: &'a AccountView,
    /// vaultEventAuthority
    pub vault_event_authority: &'a AccountView,
    /// question
    pub question: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// eventAuthority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> ConditionalswapAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 25] {
        [
            InstructionAccount::writable(self.dao.address()),
            InstructionAccount::writable(self.amm_base_vault.address()),
            InstructionAccount::writable(self.amm_quote_vault.address()),
            InstructionAccount::readonly(self.proposal.address()),
            InstructionAccount::writable(self.amm_pass_base_vault.address()),
            InstructionAccount::writable(self.amm_pass_quote_vault.address()),
            InstructionAccount::writable(self.amm_fail_base_vault.address()),
            InstructionAccount::writable(self.amm_fail_quote_vault.address()),
            InstructionAccount::readonly_signer(self.trader.address()),
            InstructionAccount::writable(self.user_input_account.address()),
            InstructionAccount::writable(self.user_output_account.address()),
            InstructionAccount::writable(self.base_vault.address()),
            InstructionAccount::writable(self.base_vault_underlying_token_account.address()),
            InstructionAccount::writable(self.quote_vault.address()),
            InstructionAccount::writable(self.quote_vault_underlying_token_account.address()),
            InstructionAccount::writable(self.pass_base_mint.address()),
            InstructionAccount::writable(self.fail_base_mint.address()),
            InstructionAccount::writable(self.pass_quote_mint.address()),
            InstructionAccount::writable(self.fail_quote_mint.address()),
            InstructionAccount::readonly(self.conditional_vault_program.address()),
            InstructionAccount::readonly(self.vault_event_authority.address()),
            InstructionAccount::readonly(self.question.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 25] {
        [
            self.dao, self.amm_base_vault, self.amm_quote_vault, self.proposal, self.amm_pass_base_vault, self.amm_pass_quote_vault, self.amm_fail_base_vault, self.amm_fail_quote_vault, self.trader, self.user_input_account, self.user_output_account, self.base_vault, self.base_vault_underlying_token_account, self.quote_vault, self.quote_vault_underlying_token_account, self.pass_base_mint, self.fail_base_mint, self.pass_quote_mint, self.fail_quote_mint, self.conditional_vault_program, self.vault_event_authority, self.question, self.token_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `provideLiquidity`
pub struct ProvideliquidityAccounts<'a> {
    /// dao
    pub dao: &'a AccountView,
    /// liquidityProvider
    pub liquidity_provider: &'a AccountView,
    /// liquidityProviderBaseAccount
    pub liquidity_provider_base_account: &'a AccountView,
    /// liquidityProviderQuoteAccount
    pub liquidity_provider_quote_account: &'a AccountView,
    /// payer
    pub payer: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
    /// ammBaseVault
    pub amm_base_vault: &'a AccountView,
    /// ammQuoteVault
    pub amm_quote_vault: &'a AccountView,
    /// ammPosition
    pub amm_position: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// eventAuthority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> ProvideliquidityAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 12] {
        [
            InstructionAccount::writable(self.dao.address()),
            InstructionAccount::readonly_signer(self.liquidity_provider.address()),
            InstructionAccount::writable(self.liquidity_provider_base_account.address()),
            InstructionAccount::writable(self.liquidity_provider_quote_account.address()),
            InstructionAccount::writable_signer(self.payer.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::writable(self.amm_base_vault.address()),
            InstructionAccount::writable(self.amm_quote_vault.address()),
            InstructionAccount::writable(self.amm_position.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 12] {
        [
            self.dao, self.liquidity_provider, self.liquidity_provider_base_account, self.liquidity_provider_quote_account, self.payer, self.system_program, self.amm_base_vault, self.amm_quote_vault, self.amm_position, self.token_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `withdrawLiquidity`
pub struct WithdrawliquidityAccounts<'a> {
    /// dao
    pub dao: &'a AccountView,
    /// positionAuthority
    pub position_authority: &'a AccountView,
    /// liquidityProviderBaseAccount
    pub liquidity_provider_base_account: &'a AccountView,
    /// liquidityProviderQuoteAccount
    pub liquidity_provider_quote_account: &'a AccountView,
    /// ammBaseVault
    pub amm_base_vault: &'a AccountView,
    /// ammQuoteVault
    pub amm_quote_vault: &'a AccountView,
    /// ammPosition
    pub amm_position: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// eventAuthority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> WithdrawliquidityAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 10] {
        [
            InstructionAccount::writable(self.dao.address()),
            InstructionAccount::readonly_signer(self.position_authority.address()),
            InstructionAccount::writable(self.liquidity_provider_base_account.address()),
            InstructionAccount::writable(self.liquidity_provider_quote_account.address()),
            InstructionAccount::writable(self.amm_base_vault.address()),
            InstructionAccount::writable(self.amm_quote_vault.address()),
            InstructionAccount::writable(self.amm_position.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 10] {
        [
            self.dao, self.position_authority, self.liquidity_provider_base_account, self.liquidity_provider_quote_account, self.amm_base_vault, self.amm_quote_vault, self.amm_position, self.token_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `collectFees`
pub struct CollectfeesAccounts<'a> {
    /// dao
    pub dao: &'a AccountView,
    /// admin
    pub admin: &'a AccountView,
    /// baseTokenAccount
    pub base_token_account: &'a AccountView,
    /// quoteTokenAccount
    pub quote_token_account: &'a AccountView,
    /// ammBaseVault
    pub amm_base_vault: &'a AccountView,
    /// ammQuoteVault
    pub amm_quote_vault: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// eventAuthority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> CollectfeesAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 9] {
        [
            InstructionAccount::writable(self.dao.address()),
            InstructionAccount::readonly_signer(self.admin.address()),
            InstructionAccount::writable(self.base_token_account.address()),
            InstructionAccount::writable(self.quote_token_account.address()),
            InstructionAccount::writable(self.amm_base_vault.address()),
            InstructionAccount::writable(self.amm_quote_vault.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 9] {
        [
            self.dao, self.admin, self.base_token_account, self.quote_token_account, self.amm_base_vault, self.amm_quote_vault, self.token_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `executeSpendingLimitChange`
pub struct ExecutespendinglimitchangeAccounts<'a> {
    /// proposal
    pub proposal: &'a AccountView,
    /// dao
    pub dao: &'a AccountView,
    /// squadsProposal
    pub squads_proposal: &'a AccountView,
    /// squadsMultisig
    pub squads_multisig: &'a AccountView,
    /// squadsMultisigProgram
    pub squads_multisig_program: &'a AccountView,
    /// vaultTransaction
    pub vault_transaction: &'a AccountView,
    /// eventAuthority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> ExecutespendinglimitchangeAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 8] {
        [
            InstructionAccount::writable(self.proposal.address()),
            InstructionAccount::writable(self.dao.address()),
            InstructionAccount::writable(self.squads_proposal.address()),
            InstructionAccount::readonly(self.squads_multisig.address()),
            InstructionAccount::readonly(self.squads_multisig_program.address()),
            InstructionAccount::readonly(self.vault_transaction.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 8] {
        [
            self.proposal, self.dao, self.squads_proposal, self.squads_multisig, self.squads_multisig_program, self.vault_transaction, self.event_authority, self.program
        ]
    }
}

/// Accounts for `sponsorProposal`
pub struct SponsorproposalAccounts<'a> {
    /// proposal
    pub proposal: &'a AccountView,
    /// dao
    pub dao: &'a AccountView,
    /// teamAddress
    pub team_address: &'a AccountView,
    /// eventAuthority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> SponsorproposalAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 5] {
        [
            InstructionAccount::writable(self.proposal.address()),
            InstructionAccount::writable(self.dao.address()),
            InstructionAccount::readonly_signer(self.team_address.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 5] {
        [
            self.proposal, self.dao, self.team_address, self.event_authority, self.program
        ]
    }
}

/// Accounts for `collectMeteoraDammFees`
pub struct CollectmeteoradammfeesAccounts<'a> {
    /// dao
    pub dao: &'a AccountView,
    /// admin
    pub admin: &'a AccountView,
    /// squadsMultisig
    pub squads_multisig: &'a AccountView,
    /// squadsMultisigVault
    pub squads_multisig_vault: &'a AccountView,
    /// squadsMultisigVaultTransaction
    pub squads_multisig_vault_transaction: &'a AccountView,
    /// squadsMultisigProposal
    pub squads_multisig_proposal: &'a AccountView,
    /// squadsMultisigPermissionlessAccount
    pub squads_multisig_permissionless_account: &'a AccountView,
    /// meteoraClaimPositionFeesAccounts
    pub meteora_claim_position_fees_accounts: &'a AccountView,
    /// systemProgram
    pub system_program: &'a AccountView,
    /// tokenProgram
    pub token_program: &'a AccountView,
    /// squadsProgram
    pub squads_program: &'a AccountView,
    /// eventAuthority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> CollectmeteoradammfeesAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 13] {
        [
            InstructionAccount::writable(self.dao.address()),
            InstructionAccount::writable_signer(self.admin.address()),
            InstructionAccount::writable(self.squads_multisig.address()),
            InstructionAccount::readonly(self.squads_multisig_vault.address()),
            InstructionAccount::writable(self.squads_multisig_vault_transaction.address()),
            InstructionAccount::writable(self.squads_multisig_proposal.address()),
            InstructionAccount::readonly_signer(self.squads_multisig_permissionless_account.address()),
            InstructionAccount::readonly(self.meteora_claim_position_fees_accounts.address()),
            InstructionAccount::readonly(self.system_program.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.squads_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 13] {
        [
            self.dao, self.admin, self.squads_multisig, self.squads_multisig_vault, self.squads_multisig_vault_transaction, self.squads_multisig_proposal, self.squads_multisig_permissionless_account, self.meteora_claim_position_fees_accounts, self.system_program, self.token_program, self.squads_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `adminApproveExecuteMultisigProposal`
pub struct AdminapproveexecutemultisigproposalAccounts<'a> {
    /// dao
    pub dao: &'a AccountView,
    /// admin
    pub admin: &'a AccountView,
    /// squadsMultisig
    pub squads_multisig: &'a AccountView,
    /// squadsMultisigProposal
    pub squads_multisig_proposal: &'a AccountView,
    /// squadsMultisigVaultTransaction
    pub squads_multisig_vault_transaction: &'a AccountView,
    /// squadsMultisigProgram
    pub squads_multisig_program: &'a AccountView,
    /// eventAuthority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> AdminapproveexecutemultisigproposalAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 8] {
        [
            InstructionAccount::writable(self.dao.address()),
            InstructionAccount::writable_signer(self.admin.address()),
            InstructionAccount::writable(self.squads_multisig.address()),
            InstructionAccount::writable(self.squads_multisig_proposal.address()),
            InstructionAccount::writable(self.squads_multisig_vault_transaction.address()),
            InstructionAccount::readonly(self.squads_multisig_program.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 8] {
        [
            self.dao, self.admin, self.squads_multisig, self.squads_multisig_proposal, self.squads_multisig_vault_transaction, self.squads_multisig_program, self.event_authority, self.program
        ]
    }
}

/// Accounts for `adminRemoveProposal`
pub struct AdminremoveproposalAccounts<'a> {
    /// proposal
    pub proposal: &'a AccountView,
    /// dao
    pub dao: &'a AccountView,
    /// admin
    pub admin: &'a AccountView,
    /// eventAuthority
    pub event_authority: &'a AccountView,
    /// program
    pub program: &'a AccountView,
}

impl<'a> AdminremoveproposalAccounts<'a> {
    /// Convert to InstructionAccount array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; 5] {
        [
            InstructionAccount::writable(self.proposal.address()),
            InstructionAccount::writable(self.dao.address()),
            InstructionAccount::writable_signer(self.admin.address()),
            InstructionAccount::readonly(self.event_authority.address()),
            InstructionAccount::readonly(self.program.address()),
        ]
    }
    
    /// Convert to AccountView slice
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; 5] {
        [
            self.proposal, self.dao, self.admin, self.event_authority, self.program
        ]
    }
}


// ============================================
// CPI Functions
// ============================================
/// CPI: initializeDao
#[inline(always)]
pub fn initialize_dao<'a>(
    accounts: &InitializedaoAccounts<'a>, args: &InitializedaoArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<InitializedaoArgs>()];
    data[0..8].copy_from_slice(&INITIALIZE_DAO);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const InitializedaoArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<InitializedaoArgs>(),
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

/// CPI: initializeProposal
#[inline(always)]
pub fn initialize_proposal<'a>(
    accounts: &InitializeproposalAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&INITIALIZE_PROPOSAL);
    
    
    // Build InstructionAccount array
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

/// CPI: stakeToProposal
#[inline(always)]
pub fn stake_to_proposal<'a>(
    accounts: &StaketoproposalAccounts<'a>, args: &StaketoproposalArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<StaketoproposalArgs>()];
    data[0..8].copy_from_slice(&STAKE_TO_PROPOSAL);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const StaketoproposalArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<StaketoproposalArgs>(),
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

/// CPI: unstakeFromProposal
#[inline(always)]
pub fn unstake_from_proposal<'a>(
    accounts: &UnstakefromproposalAccounts<'a>, args: &UnstakefromproposalArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<UnstakefromproposalArgs>()];
    data[0..8].copy_from_slice(&UNSTAKE_FROM_PROPOSAL);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const UnstakefromproposalArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<UnstakefromproposalArgs>(),
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

/// CPI: launchProposal
#[inline(always)]
pub fn launch_proposal<'a>(
    accounts: &LaunchproposalAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&LAUNCH_PROPOSAL);
    
    
    // Build InstructionAccount array
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

/// CPI: finalizeProposal
#[inline(always)]
pub fn finalize_proposal<'a>(
    accounts: &FinalizeproposalAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&FINALIZE_PROPOSAL);
    
    
    // Build InstructionAccount array
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

/// CPI: updateDao
#[inline(always)]
pub fn update_dao<'a>(
    accounts: &UpdatedaoAccounts<'a>, args: &UpdatedaoArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<UpdatedaoArgs>()];
    data[0..8].copy_from_slice(&UPDATE_DAO);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const UpdatedaoArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<UpdatedaoArgs>(),
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

/// CPI: spotSwap
#[inline(always)]
pub fn spot_swap<'a>(
    accounts: &SpotswapAccounts<'a>, args: &SpotswapArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<SpotswapArgs>()];
    data[0..8].copy_from_slice(&SPOT_SWAP);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const SpotswapArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<SpotswapArgs>(),
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

/// CPI: conditionalSwap
#[inline(always)]
pub fn conditional_swap<'a>(
    accounts: &ConditionalswapAccounts<'a>, args: &ConditionalswapArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<ConditionalswapArgs>()];
    data[0..8].copy_from_slice(&CONDITIONAL_SWAP);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const ConditionalswapArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<ConditionalswapArgs>(),
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

/// CPI: provideLiquidity
#[inline(always)]
pub fn provide_liquidity<'a>(
    accounts: &ProvideliquidityAccounts<'a>, args: &ProvideliquidityArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<ProvideliquidityArgs>()];
    data[0..8].copy_from_slice(&PROVIDE_LIQUIDITY);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const ProvideliquidityArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<ProvideliquidityArgs>(),
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

/// CPI: withdrawLiquidity
#[inline(always)]
pub fn withdraw_liquidity<'a>(
    accounts: &WithdrawliquidityAccounts<'a>, args: &WithdrawliquidityArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + core::mem::size_of::<WithdrawliquidityArgs>()];
    data[0..8].copy_from_slice(&WITHDRAW_LIQUIDITY);
    
        // Copy arguments (unsafe due to packed struct)
        unsafe {
            core::ptr::copy_nonoverlapping(
                args as *const WithdrawliquidityArgs as *const u8,
                data.as_mut_ptr().add(8),
                core::mem::size_of::<WithdrawliquidityArgs>(),
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

/// CPI: collectFees
#[inline(always)]
pub fn collect_fees<'a>(
    accounts: &CollectfeesAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&COLLECT_FEES);
    
    
    // Build InstructionAccount array
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

/// CPI: executeSpendingLimitChange
#[inline(always)]
pub fn execute_spending_limit_change<'a>(
    accounts: &ExecutespendinglimitchangeAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&EXECUTE_SPENDING_LIMIT_CHANGE);
    
    
    // Build InstructionAccount array
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

/// CPI: sponsorProposal
#[inline(always)]
pub fn sponsor_proposal<'a>(
    accounts: &SponsorproposalAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&SPONSOR_PROPOSAL);
    
    
    // Build InstructionAccount array
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

/// CPI: collectMeteoraDammFees
#[inline(always)]
pub fn collect_meteora_damm_fees<'a>(
    accounts: &CollectmeteoradammfeesAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&COLLECT_METEORA_DAMM_FEES);
    
    
    // Build InstructionAccount array
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

/// CPI: adminApproveExecuteMultisigProposal
#[inline(always)]
pub fn admin_approve_execute_multisig_proposal<'a>(
    accounts: &AdminapproveexecutemultisigproposalAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&ADMIN_APPROVE_EXECUTE_MULTISIG_PROPOSAL);
    
    
    // Build InstructionAccount array
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

/// CPI: adminRemoveProposal
#[inline(always)]
pub fn admin_remove_proposal<'a>(
    accounts: &AdminremoveproposalAccounts<'a>,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Build instruction data
    let mut data = [0u8; 8 + 0];
    data[0..8].copy_from_slice(&ADMIN_REMOVE_PROPOSAL);
    
    
    // Build InstructionAccount array
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

