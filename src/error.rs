use thiserror::Error;

/// Errors that can occur in the Xorion wallet SDK.
#[derive(Debug, Error)]
pub enum WalletError {
    #[error("invalid mnemonic phrase: {0}")]
    InvalidMnemonic(String),

    #[error("invalid seed: {0}")]
    InvalidSeed(String),

    #[error("key derivation failed: {0}")]
    DerivationError(String),

    #[error("cryptographic operation failed: {0}")]
    CryptoError(String),
}

pub type Result<T> = std::result::Result<T, WalletError>;
