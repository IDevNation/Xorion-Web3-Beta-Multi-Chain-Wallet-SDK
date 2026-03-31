//! Xorion DAO Governance Module
//!
//! Provides on-chain governance primitives for the Xorion Web3 OS:
//! - **Proposals** — create, vote, queue, execute with configurable parameters
//! - **Voting** — token-weighted voting (For / Against / Abstain) with quorum
//! - **Delegation** — delegate voting power to another address
//! - **Treasury** — deposit/withdraw funds via governance proposals
//! - **Timelock** — enforce delay between proposal success and execution
//! - **Governor ABI** — encode calls for on-chain Governor contracts

pub mod delegation;
pub mod governor;
pub mod proposal;
pub mod timelock;
pub mod treasury;
pub mod voting;

pub use delegation::DelegationRegistry;
pub use governor::GovernorAbi;
pub use proposal::{Proposal, ProposalParams, ProposalState};
pub use timelock::Timelock;
pub use treasury::Treasury;
pub use voting::{Vote, VoteType, VotingRecord};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum GovernanceError {
    #[error("proposal not found: {0}")]
    ProposalNotFound(u64),

    #[error("invalid state transition: {from:?} -> {to:?}")]
    InvalidTransition {
        from: ProposalState,
        to: ProposalState,
    },

    #[error("voting period not active for proposal {0}")]
    VotingNotActive(u64),

    #[error("already voted: address {0}")]
    AlreadyVoted(String),

    #[error("insufficient voting power: have {have}, need {need}")]
    InsufficientPower { have: u64, need: u64 },

    #[error("quorum not reached: {votes} / {quorum} required")]
    QuorumNotReached { votes: u64, quorum: u64 },

    #[error("timelock not expired: {remaining_secs}s remaining")]
    TimelockActive { remaining_secs: u64 },

    #[error("treasury error: {0}")]
    Treasury(String),

    #[error("delegation error: {0}")]
    Delegation(String),
}

pub type Result<T> = std::result::Result<T, GovernanceError>;
