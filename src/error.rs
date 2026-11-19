use thiserror::Error;

pub type Result<T> = std::result::Result<T, KeyError>;

#[derive(Debug, Error)]
pub enum KeyError {
    #[error("неверный мнемоник: {0}")]
    InvalidMnemonic(String),

    #[error("ошибка хранилища: {0}")]
    Keystore(String),
}

impl From<serde_json::Error> for KeyError {
    fn from(error: serde_json::Error) -> Self {
        KeyError::Keystore(error.to_string())
    }
}
