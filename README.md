# 🌌 Xorion Web3 OS

![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)
![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)
![Status](https://img.shields.io/badge/status-production%20ready-brightgreen.svg)
![Tests](https://img.shields.io/badge/tests-36%20passing-brightgreen.svg)

> **The Web3-Native Operating System** — Built in Rust. Web3 at the kernel level.

---

## 🎯 Vision

Xorion is a decentralized operating system where blockchain primitives live at the kernel level — not browser extensions. One OS to rule all chains (Ethereum + Solana).

---

## ✨ Core Features

| Feature | Description | Status |
|---------|-------------|--------|
| 🔐 **Multi-Chain Wallet** | BIP39/BIP44 HD wallet (Ethereum + Solana) | ✅ Complete |
| ⛓️ **Native Blockchain SDK** | Direct RPC integration (ETH + SOL) | ✅ Complete |
| 📋 **Smart Contracts** | ABI parsing, function calls, events | ✅ Complete |
| 💰 **Token Standards** | ERC20, ERC721 (NFTs) support | ✅ Complete |
| 🔄 **DeFi Protocols** | Uniswap, Aave integration | ✅ Complete |
| 🏗️ **Redox Scheme** | wallet:/ filesystem daemon | ✅ Complete |
| 🎨 **Desktop GUI** | Dioxus-based wallet interface | ✅ Complete |
| 🧩 **WASM Runtime** | Native dApp execution | 📋 Planned |
| 🤫 **ZK Privacy** | zk-SNARKs at OS level | 📋 Planned |
| 📦 **IPFS Storage** | Decentralized filesystem | 📋 Planned |

---

## 🗺️ Development Roadmap

| Phase | Description | Status | Tests |
|-------|-------------|--------|-------|
| **Phase 1** | Multi-Chain SDK (Wallet, ETH/SOL addresses) | ✅ Complete | 5 |
| **Phase 2** | RPC Integration & Broadcasting | ✅ Complete | 3 |
| **Phase 3** | Smart Contract Interaction (ERC20, Uniswap) | ✅ Complete | 15 |
| **Phase 4** | Redox Scheme Daemon (wallet:/) | ✅ Complete | 10 |
| **Phase 5** | Desktop GUI (Dioxus) | ✅ Complete | 3 |
| **Phase 6** | WASM dApp Runtime | 📋 Planned | - |
| **Phase 7** | zk-SNARKs Privacy Layer | 📋 Planned | - |
| **Phase 8** | IPFS Native Filesystem | 📋 Planned | - |
| **Phase 9** | DAO Governance Module | 📋 Planned | - |
| **Phase 10** | Beta Release | 📋 Planned | - |

**✅ Total Tests: 36/36 Passing**

---

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────────┐
│              XORION WEB3 OS - FULL STACK                    │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  ┌────────────────────────────────────────────────────┐    │
│  │         GUI (Dioxus Desktop) — Phase 5             │    │
│  │    Dashboard │ Send │ Receive │ Settings           │    │
│  └────────────────────────────────────────────────────┘    │
│                           ⬇️                                │
│  ┌────────────────────────────────────────────────────┐    │
│  │         Redox Scheme (wallet:/) — Phase 4          │    │
│  │    Daemon │ Protocol │ Keyring │ Handler           │    │
│  └────────────────────────────────────────────────────┘    │
│                           ⬇️                                │
│  ┌────────────────────────────────────────────────────┐    │
│  │         Xorion SDK — Phases 1-3                    │    │
│  │    Wallet │ RPC │ Contract │ Tokens │ DeFi         │    │
│  └────────────────────────────────────────────────────┘    │
│                           ⬇️                                │
│  ┌────────────────────────────────────────────────────┐    │
│  │         Blockchain Layer                           │    │
│  │    Ethereum (JSON-RPC) │ Solana (JSON-RPC)        │    │
│  └────────────────────────────────────────────────────┘    │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

---

## 🚀 Quick Start

### Prerequisites
- Rust 1.70 or later
- For GUI: `sudo apt install libgtk-3-dev libwebkit2gtk-4.0-dev`

### Commands

```bash
# Clone
git clone https://github.com/IDevNation/xorion-web3-os.git
cd xorion-web3-os

# Build
cargo build --release

# Test (36 tests)
cargo test

# Run GUI
cargo run -p xorion-gui

# Run examples
cargo run --example demo              # Wallet creation
cargo run --example rpc_demo          # RPC integration
cargo run --example contract_demo     # Smart contracts
cargo run --example kernel_demo       # Full demo
```

### Code Example

```rust
use xorion_sdk::Wallet;

let wallet = Wallet::from_mnemonic("your 12/24 word mnemonic")?;
println!("ETH: {}", wallet.eth_address());
println!("SOL: {}", wallet.solana_address());
```

---

## 📁 Project Structure

```
xorion-web3-os/
├── xorion-sdk/           # Phases 1-3: Core SDK
│   ├── src/
│   │   ├── wallet.rs     # BIP39, ETH/SOL addresses
│   │   ├── rpc/          # ETH/SOL RPC clients
│   │   ├── contract/     # ABI, ERC20, Uniswap
│   │   └── signing/      # Transaction signing
│   └── Cargo.toml
│
├── xorion-scheme/        # Phase 4: Redox Scheme Daemon
│   ├── src/
│   │   ├── main.rs       # wallet:/ scheme
│   │   ├── protocol.rs   # JSON wire protocol
│   │   ├── handler.rs    # Request handler
│   │   └── keyring.rs    # Encrypted keys
│   └── Cargo.toml
│
├── xorion-gui/           # Phase 5: Desktop GUI
│   ├── src/
│   │   ├── main.rs       # App shell
│   │   └── components/
│   │       ├── dashboard.rs
│   │       ├── send.rs
│   │       ├── receive.rs
│   │       └── settings.rs
│   └── Cargo.toml
│
├── examples/             # Demo applications
├── tests/                # Integration tests
└── Cargo.toml           # Workspace
```

---

## 📊 Progress

```
Phase 1: ████████████████████ 100% ✅
Phase 2: ████████████████████ 100% ✅
Phase 3: ████████████████████ 100% ✅
Phase 4: ████████████████████ 100% ✅
Phase 5: ████████████████████ 100% ✅
Phase 6: ░░░░░░░░░░░░░░░░░░░░ 0% 📋
Phase 7: ░░░░░░░░░░░░░░░░░░░░ 0% 📋
Phase 8: ░░░░░░░░░░░░░░░░░░░░ 0% 📋
Phase 9: ░░░░░░░░░░░░░░░░░░░░ 0% 📋
Phase 10:░░░░░░░░░░░░░░░░░░░░ 0% 📋
```

---

## 📄 License

MIT License

---

**Made with ❤️ for the decentralized web** 🌌

*5 phases complete | 36 tests passing | Production ready*
```


