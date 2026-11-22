use bip39::{Language, Mnemonic};
use zeroize::Zeroizing;

use crate::{KeyError, Result};

pub(crate) fn parse_mnemonic(phrase: &str) -> Result<Mnemonic> {
    let mut normalized = Zeroizing::new(String::with_capacity(phrase.len()));

    for word in phrase.split_whitespace() {
        if !normalized.is_empty() {
            normalized.push(' ');
        }
        normalized.push_str(word);
    }
    normalized.make_ascii_lowercase();

    Mnemonic::parse_in(Language::English, normalized.as_str())
        .map_err(|error| KeyError::InvalidMnemonic(error.to_string()))
}

#[cfg(test)]
mod tests {
    use super::parse_mnemonic;

    #[test]
    fn normalizes_whitespace_and_case() {
        let phrase = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        let input = format!(" \n{}\t ", phrase.to_ascii_uppercase().replace(' ', "  \t"));

        assert_eq!(parse_mnemonic(&input).unwrap().to_string(), phrase);
    }
}
