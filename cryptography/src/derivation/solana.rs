use ed25519_dalek::SigningKey;

use super::algorithms::slip10::{DerivationError, derive_ed25519};

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
}

#[cfg(test)]
mod tests {
    use super::SolanaAccount;
    use crate::seed::to_seed;

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
