//! Simple wallet creation and address derivation example.
//!
//! Run: cargo run --example simple_wallet

use xorion_sdk::Wallet;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Xorion Simple Wallet ===\n");

    // Create wallet from a known mnemonic
    let mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
    let wallet = Wallet::from_mnemonic(mnemonic)?;

    // Derive addresses for both chains
    let eth_address = wallet.derive_eth_address()?;
    let sol_address = wallet.derive_solana_address()?;

    println!("Mnemonic: {}...", &mnemonic[..30]);
    println!("ETH Address: {eth_address}");
    println!("SOL Address: {sol_address}");

    // Generate a fresh wallet with a random mnemonic
    let fresh = Wallet::new()?;
    println!("\n--- Fresh Wallet ---");
    println!("ETH Address: {}", fresh.derive_eth_address()?);
    println!("SOL Address: {}", fresh.derive_solana_address()?);

    println!("\nDone.");
    Ok(())
}
