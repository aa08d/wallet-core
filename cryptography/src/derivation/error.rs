use thiserror::Error;

#[derive(Debug, Error)]
pub enum DerivationError {
    #[error("SLIP-0010 seed must be 16 to 64 bytes, got {0}")]
    InvalidSeedLength(usize),

    #[error("child index must be below 2^31, got {0}")]
    InvalidChildIndex(u32),

    #[error("BIP32 derivation failed: {0}")]
    Bip32(#[from] bitcoin::bip32::Error),

    #[error("derived secp256k1 signing key is invalid")]
    InvalidSigningKey,
}
