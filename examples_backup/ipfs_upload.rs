//! IPFS encrypted upload example using the virtual filesystem.
//!
//! Run: cargo run --example ipfs_upload
//!
//! Note: This example demonstrates the encryption and VFS layers.
//! Actual IPFS upload requires a running IPFS daemon (localhost:5001).

use xorion_ipfs::encryption::Encryption;
use xorion_ipfs::vfs::VirtualFs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Xorion IPFS Upload ===\n");

    // 1. Encrypt some data
    let password = "my-secure-password";
    let salt = b"xorion_example_salt_16b";
    let enc = Encryption::from_password(password, salt)?;

    let plaintext = b"Hello from Xorion! This file is encrypted with AES-256-GCM.";
    let encrypted = enc.encrypt(plaintext)?;
    println!("Plaintext size: {} bytes", plaintext.len());
    println!("Encrypted size: {} bytes (12 nonce + data + 16 tag)", encrypted.len());

    // 2. Verify decryption roundtrip
    let decrypted = enc.decrypt(&encrypted)?;
    assert_eq!(&decrypted, plaintext);
    println!("Decryption: OK\n");

    // 3. Use the virtual filesystem
    let mut vfs = VirtualFs::new();
    vfs.mkdir("/documents")?;
    vfs.mkdir("/documents/private")?;

    // Add files (using fake CIDs since we're not connected to IPFS)
    vfs.add_file("/documents/readme.txt", "QmFakeCid1234567890abcdef", false)?;
    vfs.add_file(
        "/documents/private/secrets.enc",
        "QmFakeCid0987654321fedcba",
        true, // encrypted
    )?;

    // List directory
    println!("Files in /documents:");
    for entry in vfs.ls("/documents")? {
        println!("  {}", entry.path());
    }

    println!("\nFiles in /documents/private:");
    for entry in vfs.ls("/documents/private")? {
        let suffix = if entry.encrypted() { " [encrypted]" } else { "" };
        println!("  {}{suffix}", entry.path());
    }

    // Look up a file
    let entry = vfs.get("/documents/readme.txt")?;
    println!("\nLookup: {} -> CID: {}", entry.path(), entry.cid());

    println!("\nDone.");
    Ok(())
}
