use crate::{EvmAccount, SolanaAccount};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Family {
    Solana,
    Evm,
}

pub enum Account {
    Solana(SolanaAccount),
    Evm(EvmAccount),
}

impl Account {
    pub fn address(&self) -> String {
        match self {
            Self::Solana(account) => account.address(),
            Self::Evm(account) => account.address(),
        }
    }
}
