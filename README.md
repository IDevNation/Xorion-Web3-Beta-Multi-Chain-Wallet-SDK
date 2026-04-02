# X-OS (XOS) — The Web3-Native Operating System

[![Crates.io](https://img.shields.io/crates/v/xos-sdk.svg)](https://crates.io/crates/xos-sdk)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![Tests](https://img.shields.io/badge/tests-229%20passing-brightgreen.svg)](#)
[![Version](https://img.shields.io/badge/version-v1.0.0--beta.1-blue.svg)](https://github.com/IDevNation/xorion-web3-os/releases)

> **The Web3-Native Operating System** — Built in Rust. Web3 at the kernel level.

X-OS (formerly Xorion Web3 OS) is a modular, multi-chain Web3 SDK and decentralized operating system where blockchain primitives live at the kernel level. One SDK to rule all chains. Part of the XORION Ecosystem.

---

## Getting Started

### Installation

Add Xorion to your project:

```toml
[dependencies]
xorion-sdk = "0.4.0"          # Core wallet + RPC + contracts
xorion-core = "0.1.0"         # WASM dApp runtime
xorion-governance = "0.1.0"   # DAO governance
xorion-ipfs = "0.1.0"         # IPFS storage + encryption
xorion-zk = "0.1.0"           # zk-SNARKs privacy layer
```

Or build from source:

```bash
git clone https://github.com/IDevNation/xorion-web3-os.git
cd xorion-web3-os
cargo build --workspace
```

### Quick Start

```rust
use xorion_sdk::Wallet;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new HD wallet from a mnemonic
    let mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
    let wallet = Wallet::from_mnemonic(mnemonic)?;

    // Derive addresses for multiple chains
    println!("ETH: {}", wallet.derive_eth_address()?);
    println!("SOL: {}", wallet.derive_solana_address()?);

    Ok(())
}
```

See [examples/](examples/) for more: wallet creation, DAO voting, IPFS uploads, private transfers.

---

## Features

| Crate | Feature | Description |
|-------|---------|-------------|
| `xorion-sdk` | Multi-Chain Wallet | BIP-39/BIP-44 HD wallet (Ethereum + Solana) |
| `xorion-sdk` | RPC Providers | Async JSON-RPC for ETH and SOL |
| `xorion-sdk` | Smart Contracts | ABI encoding, ERC-20, ERC-721, Uniswap V2 |
| `xorion-core` | WASM Runtime | Wasmtime sandbox with wallet bridge |
| `xorion-zk` | ZK Privacy | Groth16 zk-SNARKs (private tx, age verify, balance proofs) |
| `xorion-ipfs` | IPFS Storage | Encrypted file storage with AES-256-GCM + Argon2 |
| `xorion-governance` | DAO Governance | Proposals, token-weighted voting, delegation, treasury |

Additional workspace crates (not published to crates.io):
- `xorion-scheme` — Redox OS `wallet:/` filesystem daemon
- `xorion-gui` — Dioxus desktop wallet interface

---

## Examples

```bash
# Wallet creation
cargo run --example simple_wallet

# DAO governance voting
cargo run --example dao_voting

# IPFS encrypted upload
cargo run --example ipfs_upload

# ZK private transfer proof
cargo run --example private_transfer

# Legacy demos
cargo run --example demo              # Basic wallet
cargo run --example rpc_demo          # RPC integration
cargo run --example contract_demo     # Smart contracts
cargo run --example kernel_demo       # Full kernel demo
```

---

## Architecture

```
xorion-web3-os/
├── src/                        # xorion-sdk: Core SDK (Phases 1-3)
│   ├── wallet.rs               # BIP39/BIP44 HD wallet
│   ├── rpc/                    # ETH + SOL JSON-RPC providers
│   ├── contract/               # ABI, ERC20, Uniswap
│   └── kernel.rs               # WalletClient unified API
│
├── xorion-runtime/             # xorion-core: WASM Runtime (Phase 6)
│   └── src/
│       ├── sandbox.rs          # Permission-based sandbox
│       ├── api.rs              # WalletBridge host functions
│       └── ipfs_loader.rs      # IPFS dApp fetcher
│
├── xorion-privacy/             # xorion-zk: ZK Privacy (Phase 7)
│   └── src/
│       ├── circuits/           # Groth16 R1CS circuits
│       ├── proof.rs            # ProofSystem (setup, prove, verify)
│       └── cache.rs            # Proof caching with TTL
│
├── xorion-storage/             # xorion-ipfs: IPFS Storage (Phase 8)
│   └── src/
│       ├── ipfs.rs             # IPFS HTTP API client
│       ├── encryption.rs       # AES-256-GCM + Argon2 key derivation
│       ├── vfs.rs              # Virtual filesystem over IPFS
│       └── cache.rs            # Disk-backed LRU cache
│
├── xorion-governance/          # xorion-governance: DAO (Phase 9)
│   └── src/
│       ├── proposal.rs         # Proposal state machine
│       ├── voting.rs           # Token-weighted voting + quorum
│       ├── delegation.rs       # Voting power delegation
│       └── treasury.rs         # DAO treasury management
│
├── xorion-scheme/              # Redox wallet:/ daemon (Phase 4)
├── xorion-gui/                 # Dioxus desktop GUI (Phase 5)
├── examples/                   # Example applications
├── tests/integration.rs        # Cross-crate integration tests
└── .github/workflows/ci.yml    # CI/CD pipeline
```

---

## Development

```bash
# Build
cargo build --workspace

# Test (229 tests)
cargo test --workspace

# Lint
cargo clippy --workspace -- -D warnings

# Docs
cargo doc --workspace --no-deps --exclude xorion-gui --exclude xorion-scheme --open

# GUI (requires GTK)
sudo apt install libgtk-3-dev libwebkit2gtk-4.1-dev libxdo-dev
cargo run -p xorion-gui
```

---

## Roadmap

| Phase | Description | Status |
|-------|-------------|--------|
| Phase 1 | Multi-Chain Wallet SDK | ✅ Complete |
| Phase 2 | RPC Integration | ✅ Complete |
| Phase 3 | Smart Contracts (ERC20, Uniswap) | ✅ Complete |
| Phase 4 | Redox Scheme Daemon | ✅ Complete |
| Phase 5 | Desktop GUI (Dioxus) | ✅ Complete |
| Phase 6 | WASM dApp Runtime | ✅ Complete |
| Phase 7 | zk-SNARKs Privacy | ✅ Complete |
| Phase 8 | IPFS Filesystem | ✅ Complete |
| Phase 9 | DAO Governance | ✅ Complete |
| Phase 10 | Beta Release | ✅ Complete |

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for setup, testing, and PR guidelines.

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for release history.

## License

MIT License — see [LICENSE](LICENSE)

---

**Made with ❤️ for the decentralized web** 🌌
