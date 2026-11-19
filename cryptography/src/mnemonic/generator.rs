use bip39::{Language, Mnemonic, WordCount};
use zeroize::Zeroizing;

pub fn generate_mnemonic(word_count: WordCount) -> Zeroizing<String> {
    let mnemonic = Mnemonic::generate_in(Language::English, word_count)
        .expect("WordCount always maps to a valid BIP39 length");

    Zeroizing::new(mnemonic.to_string())
}

#[cfg(test)]
mod tests {
    use super::generate_mnemonic;
    use bip39::{Language, Mnemonic, WordCount};

    #[test]
    fn generates_valid_mnemonic_for_each_word_count() {
        let cases = [
            (WordCount::Words12, 12),
            (WordCount::Words15, 15),
            (WordCount::Words18, 18),
            (WordCount::Words21, 21),
            (WordCount::Words24, 24),
        ];

        for (word_count, expected) in cases {
            let phrase = generate_mnemonic(word_count);

            assert_eq!(phrase.split_whitespace().count(), expected);
            assert!(
                Mnemonic::parse_in(Language::English, phrase.as_str()).is_ok(),
                "Expected a valid English BIP39 phrase for {word_count:?}",
            );
        }
    }
}
