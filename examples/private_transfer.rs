//! ZK private transfer example: generate and verify a Groth16 proof
//! that a sender has sufficient balance without revealing the amounts.
//!
//! Run: cargo run --example private_transfer
//!
//! Note: First run generates proving keys (~2-3 seconds).

use xorion_zk::proof::ProofSystem;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Xorion Private Transfer ===\n");

    // 1. Setup: generate proving and verifying keys
    println!("Generating Groth16 proving keys (BN254)...");
    let (pk, vk) = ProofSystem::setup_private_tx()?;
    println!("Setup complete.\n");

    // 2. Prove: sender has balance >= transfer amount
    let sender_balance: u64 = 1000;
    let transfer_amount: u64 = 250;
    let sender_secret = b"sender_private_key_bytes_here!!!"; // 31 bytes for nullifier

    println!("Proving: balance({sender_balance}) >= amount({transfer_amount})");
    let proof = ProofSystem::prove_private_tx(
        &pk,
        sender_balance,
        transfer_amount,
        sender_secret,
    )?;
    println!("Proof generated ({} bytes serialized)", ProofSystem::serialize_proof(&proof)?.len());

    // 3. Verify: anyone can check the proof without knowing balance/amount
    let valid = ProofSystem::verify(&vk, &proof)?;
    println!("Verification: {}\n", if valid { "VALID" } else { "INVALID" });

    // 4. Try an invalid transfer (amount > balance)
    println!("Proving: balance(100) >= amount(500) [should fail verification]");
    let bad_proof = ProofSystem::prove_private_tx(&pk, 100, 500, sender_secret)?;
    let bad_valid = ProofSystem::verify(&vk, &bad_proof)?;
    println!("Verification: {}", if bad_valid { "VALID" } else { "INVALID" });

    // 5. Proof serialization roundtrip
    let bytes = ProofSystem::serialize_proof(&proof)?;
    let restored = ProofSystem::deserialize_proof(&bytes)?;
    let roundtrip_valid = ProofSystem::verify(&vk, &restored)?;
    println!("\nSerialization roundtrip: {}", if roundtrip_valid { "OK" } else { "FAILED" });

    println!("\nDone.");
    Ok(())
}
