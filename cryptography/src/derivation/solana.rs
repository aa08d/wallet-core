use ed25519_dalek::{Signer, SigningKey};

use super::{DerivationError, algorithms::slip10::derive_ed25519};

pub struct SolanaAccount {
    key: SigningKey,
    index: u32,
}

impl SolanaAccount {
    pub fn derive(seed: &[u8; 64], index: u32) -> Result<Self, DerivationError> {
        let secret = derive_ed25519(seed, &[44, 501, index, 0])?;
        let key = SigningKey::from_bytes(&secret);
        Ok(Self { key, index })
    }

    pub fn path(&self) -> String {
        format!("m/44'/501'/{}'/0'", self.index)
    }

    pub fn pubkey(&self) -> [u8; 32] {
        self.key.verifying_key().to_bytes()
    }

    pub fn address(&self) -> String {
        bs58::encode(self.pubkey()).into_string()
    }

    pub fn sign_bytes(&self, message: &[u8]) -> [u8; 64] {
        self.key.sign(message).to_bytes()
    }
}

#[cfg(test)]
mod tests {
    use ed25519_dalek::{Signature, VerifyingKey};

    use super::SolanaAccount;
    use crate::seed::to_seed;

    #[test]
    fn signature_verifies_only_for_the_original_message() {
        let account = SolanaAccount::derive(&[42; 64], 0).unwrap();
        let key = VerifyingKey::from_bytes(&account.pubkey()).unwrap();
        let message = b"Solana message";
        let signature = Signature::from_bytes(&account.sign_bytes(message));

        assert!(key.verify_strict(message, &signature).is_ok());
        assert!(key.verify_strict(b"Solana Message", &signature).is_err());
    }

    #[test]
    fn derives_the_first_solana_address_from_a_mnemonic() {
        let phrase = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        let seed = to_seed(phrase, "").unwrap();
        let account = SolanaAccount::derive(&seed, 0).unwrap();

        assert_eq!(account.path(), "m/44'/501'/0'/0'");
        assert_eq!(
            account.address(),
            "HAgk14JpMQLgt6rVgv7cBQFJWFto5Dqxi472uT3DKpqk"
        );
    }
}
