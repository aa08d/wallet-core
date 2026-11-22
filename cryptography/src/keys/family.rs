use crate::SolanaAccount;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Family {
    Solana,
}

pub enum Account {
    Solana(SolanaAccount),
}

impl Account {
    pub fn address(&self) -> String {
        match self {
            Self::Solana(account) => account.address(),
        }
    }
}
