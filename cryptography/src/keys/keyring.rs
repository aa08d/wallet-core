use zeroize::Zeroizing;

use super::{Account, Family};
use crate::{DerivationError, SolanaAccount, seed::to_seed};

pub struct KeyRing {
    seed: Zeroizing<[u8; 64]>,
}

impl KeyRing {
    pub fn from_mnemonic(phrase: &str, passphrase: &str) -> crate::Result<Self> {
        let seed = to_seed(phrase, passphrase)?;
        Ok(Self { seed })
    }

    pub fn derive_account(&self, family: Family, index: u32) -> Result<Account, DerivationError> {
        match family {
            Family::Solana => SolanaAccount::derive(&self.seed, index).map(Account::Solana),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Family, KeyRing};

    #[test]
    fn restores_solana_addresses_by_account_index() {
        let phrase = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        let original = KeyRing::from_mnemonic(phrase, "").unwrap();
        let restored = KeyRing::from_mnemonic(phrase, "").unwrap();
        let addresses = [0, 1].map(|index| {
            let address = original
                .derive_account(Family::Solana, index)
                .unwrap()
                .address();
            let restored_address = restored
                .derive_account(Family::Solana, index)
                .unwrap()
                .address();

            assert_eq!(address, restored_address);
            address
        });

        assert_ne!(addresses[0], addresses[1]);
    }
}
