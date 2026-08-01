use std::fmt;

use crate::{
    aes_cbc::{AES_BLOCK_SIZE, aes_cbc_encrypt},
    aes_ecb::{detect_ecb, encrypt_ecb},
    padding::pkcs7,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EncryptionMode {
    Ecb,
    Cbc,
}

impl fmt::Display for EncryptionMode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ecb => write!(formatter, "AES in ECB mode"),
            Self::Cbc => write!(formatter, "AES in CBC mode"),
        }
    }
}

pub fn encryption_oracle(input: &str) -> Vec<u8> {
    encryption_oracle_with_mode(input).0
}

fn encryption_oracle_with_mode(input: &str) -> (Vec<u8>, EncryptionMode) {
    let prefix_size = rand::random_range(5..=10);
    let postfix_size = rand::random_range(5..=10);
    let mut prefix = vec![0; prefix_size];
    let mut postfix = vec![0; postfix_size];
    rand::fill(&mut prefix);
    rand::fill(&mut postfix);

    let mut key = [0u8; AES_BLOCK_SIZE];
    rand::fill(&mut key);

    let mut plaintext = Vec::with_capacity(prefix.len() + input.len() + postfix.len());
    plaintext.extend_from_slice(&prefix);
    plaintext.extend_from_slice(input.as_bytes());
    plaintext.extend_from_slice(&postfix);
    let plaintext = pkcs7(&plaintext, AES_BLOCK_SIZE);

    let mode = if rand::random_bool(0.5) {
        EncryptionMode::Cbc
    } else {
        EncryptionMode::Ecb
    };

    let ciphertext = match mode {
        EncryptionMode::Cbc => {
            let mut iv = [0u8; AES_BLOCK_SIZE];
            rand::fill(&mut iv);
            aes_cbc_encrypt(&key, &iv, &plaintext)
        }
        EncryptionMode::Ecb => encrypt_ecb(&key, &plaintext),
    };

    (ciphertext, mode)
}

pub fn find_mode(input: &[u8]) -> EncryptionMode {
    if detect_ecb(input) {
        EncryptionMode::Ecb
    } else {
        EncryptionMode::Cbc
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn oracle_returns_complete_aes_blocks() {
        let input = include_str!("data/s2c11_input.txt").trim_end();

        for _ in 0..100 {
            let ciphertext = encryption_oracle(input);
            assert!(!ciphertext.is_empty());
            assert_eq!(ciphertext.len() % AES_BLOCK_SIZE, 0);
        }
    }

    #[test]
    fn finds_the_mode_selected_by_the_oracle() {
        let input = include_str!("data/s2c11_input.txt").trim_end();

        for _ in 0..100 {
            let (ciphertext, actual_mode) = encryption_oracle_with_mode(input);
            let detected_mode = find_mode(&ciphertext);

            assert_eq!(detected_mode, actual_mode);
        }
    }

    #[test]
    fn displays_encryption_modes() {
        assert_eq!(EncryptionMode::Ecb.to_string(), "AES in ECB mode");
        assert_eq!(EncryptionMode::Cbc.to_string(), "AES in CBC mode");
    }
}
