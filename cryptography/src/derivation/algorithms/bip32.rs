use bitcoin::{
    NetworkKind,
    bip32::{ChildNumber, Xpriv},
    secp256k1::Secp256k1,
};
use zeroize::Zeroizing;

use crate::derivation::DerivationError;

pub(crate) fn derive_secp256k1(
    seed: &[u8; 64],
    path: &[ChildNumber],
) -> Result<Zeroizing<[u8; 32]>, DerivationError> {
    let mut master = Xpriv::new_master(NetworkKind::Main, seed)?;
    let child = master.derive_priv(&Secp256k1::signing_only(), &path);
    master.private_key.non_secure_erase();

    let mut child = child?;
    let secret = Zeroizing::new(child.private_key.secret_bytes());
    child.private_key.non_secure_erase();
    Ok(secret)
}
