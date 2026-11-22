use bitcoin::bip32::ChildNumber;
use k256::ecdsa::{RecoveryId, Signature, SigningKey};
use sha3::{Digest, Keccak256};

use super::{DerivationError, algorithms::bip32::derive_secp256k1};

pub struct EvmAccount {
    key: SigningKey,
    index: u32,
}

impl EvmAccount {
    pub fn derive(seed: &[u8; 64], index: u32) -> Result<Self, DerivationError> {
        let address_index = ChildNumber::from_normal_idx(index)
            .map_err(|_| DerivationError::InvalidChildIndex(index))?;
        let path = [
            ChildNumber::Hardened { index: 44 },
            ChildNumber::Hardened { index: 60 },
            ChildNumber::Hardened { index: 0 },
            ChildNumber::Normal { index: 0 },
            address_index,
        ];
        let secret = derive_secp256k1(seed, &path)?;
        let key =
            SigningKey::from_slice(&secret[..]).map_err(|_| DerivationError::InvalidSigningKey)?;
        Ok(Self { key, index })
    }

    pub fn path(&self) -> String {
        format!("m/44'/60'/0'/0/{}", self.index)
    }

    pub fn address(&self) -> String {
        let public_key = self.key.verifying_key().to_sec1_point(false);
        let hash = Keccak256::digest(&public_key.as_bytes()[1..]);
        let lower = hex::encode(&hash[12..]);
        let checksum = Keccak256::digest(lower.as_bytes());
        let mut address = String::with_capacity(42);
        address.push_str("0x");

        address.extend(
            lower
                .bytes()
                .zip(checksum.iter().flat_map(|byte| [byte >> 4, byte & 0x0f]))
                .map(|(digit, nibble)| {
                    char::from(if nibble >= 8 {
                        digit.to_ascii_uppercase()
                    } else {
                        digit
                    })
                }),
        );

        address
    }

    pub fn sign_hash(&self, hash: &[u8; 32]) -> (Signature, RecoveryId) {
        self.key.sign_prehash_recoverable(hash)
    }
}

#[cfg(test)]
mod tests {
    use k256::ecdsa::VerifyingKey;
    use sha3::{Digest, Keccak256};

    use super::EvmAccount;
    use crate::DerivationError;

    #[test]
    fn recovers_the_signer_from_the_signature_and_hash() {
        let account = EvmAccount::derive(&[42; 64], 0).unwrap();
        let hash = Keccak256::digest(b"EVM signing test").into();
        let (signature, recovery_id) = account.sign_hash(&hash);
        let signer = VerifyingKey::recover_from_prehash(&hash, &signature, recovery_id).unwrap();

        assert_eq!(&signer, account.key.verifying_key());
    }

    #[test]
    fn rejects_a_hardened_address_index() {
        let index = 1 << 31;
        assert!(matches!(
            EvmAccount::derive(&[42; 64], index),
            Err(DerivationError::InvalidChildIndex(value)) if value == index
        ));
    }
}
