use zeroize::Zeroizing;

use super::{Account, Family};
use crate::{DerivationError, EvmAccount, SolanaAccount, seed::to_seed};

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
            Family::Evm => EvmAccount::derive(&self.seed, index).map(Account::Evm),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Account, Family, KeyRing};

    #[test]
    fn derives_both_account_families_from_one_mnemonic() {
        let phrase = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        let keys = KeyRing::from_mnemonic(phrase, "").unwrap();

        let solana = keys.derive_account(Family::Solana, 0).unwrap();
        let evm = keys.derive_account(Family::Evm, 0).unwrap();

        assert_eq!(
            solana.address(),
            "HAgk14JpMQLgt6rVgv7cBQFJWFto5Dqxi472uT3DKpqk"
        );
        assert_eq!(evm.address(), "0x9858EfFD232B4033E47d90003D41EC34EcaEda94");
        assert!(matches!(evm, Account::Evm(account) if account.path() == "m/44'/60'/0'/0/0"));
    }

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
