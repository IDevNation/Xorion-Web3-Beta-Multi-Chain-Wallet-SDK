//! DAO governance example: create a proposal, cast votes, finalize.
//!
//! Run: cargo run --example dao_voting

use xorion_governance::{
    DelegationRegistry, Proposal, ProposalParams, ProposalState, VoteType, VotingRecord,
};

fn main() {
    println!("=== Xorion DAO Voting ===\n");

    // 1. Create a proposal
    let params = ProposalParams {
        title: "Fund community grants program".into(),
        description: "Allocate 10,000 tokens to fund open-source developer grants.".into(),
        proposer: "0xAlice".into(),
        quorum: 100,
        voting_period_blocks: 1000,
    };

    let mut proposal = Proposal::new(params);
    println!("Proposal: {}", proposal.title());
    println!("State: {:?}", proposal.state());

    // 2. Activate the proposal
    proposal.activate().expect("should activate");
    println!("State: {:?}", proposal.state());

    // 3. Set up delegation
    let mut registry = DelegationRegistry::new();
    registry.delegate("0xCharlie", "0xAlice"); // Charlie delegates to Alice
    println!("\nCharlie delegated to Alice");

    // Alice's voting power = her own (500) + Charlie's delegation (200)
    let alice_power = registry.voting_power("0xAlice", 500, &|addr| {
        if addr == "0xCharlie" { 200 } else { 0 }
    });
    println!("Alice voting power: {alice_power}");

    // 4. Cast votes
    let mut record = VotingRecord::new();
    record
        .cast_vote("0xAlice", VoteType::For, alice_power)
        .expect("alice votes");
    record
        .cast_vote("0xBob", VoteType::Against, 150)
        .expect("bob votes");

    println!("\nVotes cast:");
    println!("  For: {}", record.votes_for());
    println!("  Against: {}", record.votes_against());

    // 5. Finalize
    proposal
        .finalize(record.votes_for(), record.votes_against(), record.total_votes())
        .expect("should finalize");

    match proposal.state() {
        ProposalState::Succeeded => println!("\nProposal PASSED!"),
        ProposalState::Defeated => println!("\nProposal DEFEATED."),
        other => println!("\nUnexpected state: {other:?}"),
    }

    println!("\nDone.");
}
