mod derivation;
mod error;
pub mod mnemonic;
pub mod seed;

pub use derivation::algorithms::slip10::{DerivationError, derive_ed25519};
pub use error::{KeyError, Result};
