mod derivation;
mod error;
mod keys;
pub mod mnemonic;
pub mod seed;

pub use derivation::algorithms::slip10::{DerivationError, derive_ed25519};
pub use derivation::solana::SolanaAccount;
pub use error::{KeyError, Result};
pub use keys::{Account, Family, KeyRing};
