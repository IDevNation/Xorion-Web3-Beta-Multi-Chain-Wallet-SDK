# Xorion Web3 OS

![Status](https://img.shields.io/badge/status-production--ready-brightgreen)
![Tests](https://img.shields.io/badge/tests-36%2F36_passing-brightgreen)
![Rust](https://img.shields.io/badge/rust-1.70%2B-orange)
![License: MIT](https://img.shields.io/badge/license-MIT-blue)

> **The Web3-Native Operating System** — Built in Rust for the decentralized future. Blockchain primitives at the kernel level.

---

## Progress

```
Phase 1-5 Complete  [##████████████████████░░░░░░░░]  50%  (5/10 phases)
Tests Passing       [████████████████████████████████] 36/36
```

---

## Core Features

| Feature | Description | Status |
|---------|-------------|--------|
| Multi-Chain Wallet | BIP-39/BIP-44 HD wallet (Ethereum + Solana), EIP-55 checksums, zeroize-on-drop | Completed |
| RPC Integration | Async JSON-RPC providers for ETH + SOL, balance queries, tx broadcasting | Completed |
| Smart Contracts | ABI encoding/decoding, ERC-20 interface, Uniswap V2 Router/Pair | Completed |
| Kernel Integration | Redox OS `wallet:` scheme daemon, per-process isolation, syscall-style API | Completed |
| Desktop GUI | Dioxus dark-themed wallet — Dashboard, Send, Receive (QR), Settings | Completed |
| DeFi Protocols | Uniswap V2 swaps, liquidity, reserves, price quotes | Completed |
| ZK Privacy | zk-SNARKs at OS level | Planned |
| IPFS Storage | Decentralized filesystem | Planned |
| WASM Runtime | In-OS dApp execution | Planned |
| DAO Governance | On-chain governance module | Planned |

---

## Architecture

```
┌─────────────────────────────────────────────┐
│            Desktop GUI (Dioxus)             │  Phase 5
│    Dashboard │ Send │ Receive │ Settings    │
├─────────────────────────────────────────────┤
│          Wallet Scheme Daemon               │  Phase 4
│   wallet: scheme  │  WalletClient API       │
│   Per-process isolation  │  JSON protocol   │
├─────────────────────────────────────────────┤
│           Smart Contract Layer              │  Phase 3
│   ABI Encoder │ ERC-20 │ Uniswap V2        │
├─────────────────────────────────────────────┤
│            RPC Integration                  │  Phase 2
│   EthereumProvider │ SolanaProvider         │
│   ChainProvider trait │ JSON-RPC            │
├─────────────────────────────────────────────┤
│          Multi-Chain Wallet SDK             │  Phase 1
│   BIP-39 │ BIP-32 │ secp256k1 │ Keccak256  │
│   HD derivation │ Address generation        │
├─────────────────────────────────────────────┤
│           Blockchain Networks               │
│       Ethereum  │  Solana  │  More...       │
└─────────────────────────────────────────────┘
```

---

## Roadmap

| Phase | Focus | Tests | Status |
|-------|-------|-------|--------|
| **Phase 1** | Multi-Chain Wallet SDK | 5 | Completed |
| **Phase 2** | RPC Integration & Broadcasting | 3 | Completed |
| **Phase 3** | Smart Contract Interaction | 15 | Completed |
| **Phase 4** | Kernel Integration (Redox OS) | 10 | Completed |
| **Phase 5** | Desktop GUI (Dioxus) | 3 | Completed |
| **Phase 6** | WASM dApp Runtime | — | Planned |
| **Phase 7** | zk-SNARKs Privacy Layer | — | Planned |
| **Phase 8** | IPFS Native Filesystem | — | Planned |
| **Phase 9** | DAO Governance Module | — | Planned |
| **Phase 10** | Beta Release | — | Planned |
| | **Total** | **36** | |

---

## Quick Start

```bash
# Clone
git clone https://github.com/IDevNation/xorion-web3-os.git
cd xorion-web3-os

# Build entire workspace
cargo build --workspace

# Run all tests (36/36 passing)
cargo test --workspace

# Run examples
cargo run --example demo              # Wallet creation
cargo run --example rpc_demo          # Live RPC queries
cargo run --example contract_demo     # ERC-20 + Uniswap

# Start kernel scheme daemon + client
cargo run -p xorion-scheme            # Terminal 1
cargo run --example kernel_demo       # Terminal 2

# Launch desktop GUI (requires GTK3 + WebKit2GTK)
cargo run -p xorion-gui
```

### Basic Usage

```rust
use xorion_wallet_sdk::{Wallet, ChainProvider};
use xorion_wallet_sdk::rpc::ethereum::EthereumProvider;
use xorion_wallet_sdk::contract::erc20::Erc20;

// Create wallet from mnemonic
let wallet = Wallet::from_mnemonic("your 12/24 word mnemonic")?;
println!("ETH: {}", wallet.derive_eth_address()?);
println!("SOL: {}", wallet.derive_solana_address()?);

// Query blockchain
let provider = EthereumProvider::new("https://eth.llamarpc.com");
let block = provider.get_block_number().await?;
let balance = provider.get_balance("0x...").await?;

// Interact with ERC-20 tokens
let usdc = Erc20::new("0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48", provider);
let name = usdc.name().await?;
let bal = usdc.balance_of("0x...").await?;
```

---

## Project Structure

```
xorion-web3-os/
├── src/                        # xorion-wallet-sdk (core library)
│   ├── wallet.rs               #   HD wallet — Phase 1
│   ├── error.rs                #   Error types
│   ├── kernel.rs               #   WalletClient API — Phase 4
│   ├── rpc/                    #   RPC providers — Phase 2
│   │   ├── ethereum.rs         #     Ethereum JSON-RPC
│   │   ├── solana.rs           #     Solana JSON-RPC
│   │   └── types.rs            #     Shared types
│   └── contract/               #   Smart contracts — Phase 3
│       ├── abi.rs              #     ABI encoding/decoding
│       ├── erc20.rs            #     ERC-20 interface
│       └── defi.rs             #     Uniswap V2 Router/Pair
├── xorion-scheme/              # Redox OS scheme daemon — Phase 4
│   └── src/
│       ├── main.rs             #   Daemon entry point
│       ├── handler.rs          #   Request handler (10 tests)
│       └── protocol.rs         #   JSON wire protocol
├── xorion-gui/                 # Desktop GUI — Phase 5
│   └── src/
│       ├── main.rs             #   App shell + dark theme CSS
│       └── components/
│           ├── dashboard.rs    #     Balances, portfolio, tx list
│           ├── send.rs         #     Send ETH/SOL form
│           ├── receive.rs      #     QR code + address display
│           └── settings.rs     #     RPC, theme, security
├── examples/                   # Runnable demos
│   ├── demo.rs                 #   Wallet creation
│   ├── rpc_demo.rs             #   Live RPC queries
│   ├── contract_demo.rs        #   ERC-20 + Uniswap
│   └── kernel_demo.rs          #   Scheme daemon client
└── docs/
    └── redox-setup-guide.md    # Redox OS build instructions
```

---

## System Requirements

| Component | Requirement |
|-----------|-------------|
| Rust | 1.70+ (stable) |
| OS | Linux, macOS, Windows (GUI: Linux/macOS only) |
| GUI deps | GTK3, WebKit2GTK 4.1, libxdo (Linux only) |
| Redox OS | See `docs/redox-setup-guide.md` |

### Install GUI dependencies (Ubuntu/Debian)

```bash
sudo apt install libgtk-3-dev libwebkit2gtk-4.1-dev libxdo-dev
```

---

## License

MIT License — see [LICENSE](LICENSE)
