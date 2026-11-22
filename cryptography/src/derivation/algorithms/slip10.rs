use hmac::{Hmac, Mac};
use sha2::Sha512;
use zeroize::Zeroizing;

use crate::derivation::DerivationError;

type HmacSha512 = Hmac<Sha512>;

const HARDENED_BIT: u32 = 1 << 31;

pub fn derive_ed25519(seed: &[u8], path: &[u32]) -> Result<Zeroizing<[u8; 32]>, DerivationError> {
    if !(16..=64).contains(&seed.len()) {
        return Err(DerivationError::InvalidSeedLength(seed.len()));
    }

    let mut node = hmac_sha512(b"ed25519 seed", seed);
    let mut input = Zeroizing::new([0u8; 37]);

    for &index in path {
        if index >= HARDENED_BIT {
            return Err(DerivationError::InvalidChildIndex(index));
        }

        input[1..33].copy_from_slice(&node[..32]);
        input[33..].copy_from_slice(&(index | HARDENED_BIT).to_be_bytes());

        node = hmac_sha512(&node[32..], &input[..]);
    }

    let mut key = Zeroizing::new([0u8; 32]);
    key.copy_from_slice(&node[..32]);
    Ok(key)
}

fn hmac_sha512(key: &[u8], data: &[u8]) -> Zeroizing<[u8; 64]> {
    let mut mac = HmacSha512::new_from_slice(key).expect("HMAC accepts any key length");
    mac.update(data);
    Zeroizing::new(mac.finalize().into_bytes().into())
}

#[cfg(test)]
mod tests {
    use super::{DerivationError, HARDENED_BIT, derive_ed25519};

    const SEED: [u8; 16] = [
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
        0x0f,
    ];

    #[test]
    fn matches_slip10_ed25519_vector() {
        let cases: [(&[u32], [u8; 32]); 3] = [
            (
                &[],
                [
                    0x2b, 0x4b, 0xe7, 0xf1, 0x9e, 0xe2, 0x7b, 0xbf, 0x30, 0xc6, 0x67, 0xb6, 0x42,
                    0xd5, 0xf4, 0xaa, 0x69, 0xfd, 0x16, 0x98, 0x72, 0xf8, 0xfc, 0x30, 0x59, 0xc0,
                    0x8e, 0xba, 0xe2, 0xeb, 0x19, 0xe7,
                ],
            ),
            (
                &[0],
                [
                    0x68, 0xe0, 0xfe, 0x46, 0xdf, 0xb6, 0x7e, 0x36, 0x8c, 0x75, 0x37, 0x9a, 0xce,
                    0xc5, 0x91, 0xda, 0xd1, 0x9d, 0xf3, 0xcd, 0xe2, 0x6e, 0x63, 0xb9, 0x3a, 0x8e,
                    0x70, 0x4f, 0x1d, 0xad, 0xe7, 0xa3,
                ],
            ),
            (
                &[0, 1],
                [
                    0xb1, 0xd0, 0xba, 0xd4, 0x04, 0xbf, 0x35, 0xda, 0x78, 0x5a, 0x64, 0xca, 0x1a,
                    0xc5, 0x4b, 0x26, 0x17, 0x21, 0x1d, 0x27, 0x77, 0x69, 0x6f, 0xbf, 0xfa, 0xf2,
                    0x08, 0xf7, 0x46, 0xae, 0x84, 0xf2,
                ],
            ),
        ];

        for (path, expected) in cases {
            assert_eq!(
                *derive_ed25519(&SEED, path).unwrap(),
                expected,
                "path: {path:?}"
            );
        }
    }

    #[test]
    fn rejects_an_index_with_the_hardened_bit_already_set() {
        assert!(matches!(
            derive_ed25519(&SEED, &[HARDENED_BIT]),
            Err(DerivationError::InvalidChildIndex(HARDENED_BIT))
        ));
    }
}
