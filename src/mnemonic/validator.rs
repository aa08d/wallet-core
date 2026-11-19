use bip39::{Language, Mnemonic};
use zeroize::Zeroizing;

use crate::error::{KeyError, Result};

pub fn validate_mnemonic(phrase: &str) -> Result<Zeroizing<String>> {
    let normalized = Zeroizing::new(
        phrase
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
            .to_lowercase(),
    );
    let mnemonic = Mnemonic::parse_in(Language::English, normalized.as_str())
        .map_err(|error| KeyError::InvalidMnemonic(error.to_string()))?;

    Ok(Zeroizing::new(mnemonic.to_string()))
}

#[cfg(test)]
mod tests {
    use bip39::WordCount;

    use super::validate_mnemonic;
    use crate::error::KeyError;
    use crate::mnemonic::generate_mnemonic;

    const VALID_PHRASE: &str = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";

    #[test]
    fn accepts_a_valid_bip39_phrase() {
        let normalized = validate_mnemonic(VALID_PHRASE).unwrap();

        assert_eq!(normalized.as_str(), VALID_PHRASE);
    }

    #[test]
    fn normalizes_whitespace_and_case() {
        let input = format!(" \n{}\t ", VALID_PHRASE.to_uppercase().replace(' ', "  \t"));
        let normalized = validate_mnemonic(&input).unwrap();

        assert_eq!(normalized.as_str(), VALID_PHRASE);
    }

    #[test]
    fn accepts_generated_phrases_for_every_supported_length() {
        for word_count in [
            WordCount::Words12,
            WordCount::Words15,
            WordCount::Words18,
            WordCount::Words21,
            WordCount::Words24,
        ] {
            let phrase = generate_mnemonic(word_count);
            let validated = validate_mnemonic(phrase.as_str());

            assert!(
                validated.is_ok_and(|normalized| normalized.as_str() == phrase.as_str()),
                "Failed to validate a generated phrase for {word_count:?}",
            );
        }
    }

    #[test]
    fn rejects_an_unsupported_word_count() {
        let input = format!("{VALID_PHRASE} abandon");

        assert!(matches!(
            validate_mnemonic(&input),
            Err(KeyError::InvalidMnemonic(_))
        ));
    }

    #[test]
    fn rejects_a_word_outside_the_english_wordlist() {
        let input = VALID_PHRASE.replace("about", "notaword");

        assert!(matches!(
            validate_mnemonic(&input),
            Err(KeyError::InvalidMnemonic(_))
        ));
    }

    #[test]
    fn rejects_an_invalid_checksum() {
        let input = VALID_PHRASE.replace("about", "abandon");

        assert!(matches!(
            validate_mnemonic(&input),
            Err(KeyError::InvalidMnemonic(_))
        ));
    }
}
