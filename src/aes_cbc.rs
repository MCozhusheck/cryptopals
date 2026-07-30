use crate::{
    aes_ecb::{decrypt_ecb, encrypt_ecb},
    fixed_xor::fixed_xor,
};

pub const AES_BLOCK_SIZE: usize = 16;

pub fn aes_cbc_encrypt(key: &[u8], iv: &[u8; AES_BLOCK_SIZE], plaintext: &[u8]) -> Vec<u8> {
    assert_eq!(plaintext.len() % AES_BLOCK_SIZE, 0);

    let mut ciphertext = Vec::with_capacity(plaintext.len());
    let mut prev_block = *iv;

    for plaintext_block in plaintext.chunks_exact(AES_BLOCK_SIZE) {
        let plaintext_block: &[u8; AES_BLOCK_SIZE] = plaintext_block
            .try_into()
            .expect("chunks_exact produced an incorrectly sized block");

        let combined = fixed_xor(&prev_block, plaintext_block);

        let encrypted: [u8; AES_BLOCK_SIZE] = encrypt_ecb(key, &combined)
            .try_into()
            .expect("ECB encryption did not produce one complete block");

        ciphertext.extend_from_slice(&encrypted);
        prev_block = encrypted;
    }

    ciphertext
}

pub fn aes_cbc_decrypt(key: &[u8], iv: &[u8; AES_BLOCK_SIZE], ciphertext: &[u8]) -> Vec<u8> {
    let mut plaintext = Vec::with_capacity(ciphertext.len());
    let mut prev_block = *iv;

    for ciphertext_block in ciphertext.chunks_exact(AES_BLOCK_SIZE) {
        let ciphertext_block: &[u8; AES_BLOCK_SIZE] = ciphertext_block
            .try_into()
            .expect("chunks_exact produced an incorrectly sized block");
        let decrypted: [u8; AES_BLOCK_SIZE] = decrypt_ecb(key, ciphertext_block)
            .try_into()
            .expect("ECB decryption did not produce one complete block");

        let combined = fixed_xor(&prev_block, &decrypted);

        plaintext.extend_from_slice(&combined);
        prev_block = *ciphertext_block;
    }

    plaintext
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::padding::pkcs7;
    use base64::{Engine as _, engine::general_purpose};

    #[test]
    fn challenge_10_decrypts_and_reencrypts() {
        let encoded: String = include_str!("data/s2c10_input.txt").lines().collect();
        let ciphertext = general_purpose::STANDARD
            .decode(encoded)
            .expect("challenge input should be valid Base64");

        let key = b"YELLOW SUBMARINE";
        let iv = [0u8; AES_BLOCK_SIZE];
        let plaintext = aes_cbc_decrypt(key, &iv, &ciphertext);
        let expected_plaintext = pkcs7(include_bytes!("data/s2c10_output.txt"), AES_BLOCK_SIZE);

        assert_eq!(
            plaintext, expected_plaintext,
            "CBC decryption did not produce the expected padded plaintext"
        );

        let reencrypted = aes_cbc_encrypt(key, &iv, &plaintext);
        assert_eq!(
            reencrypted, ciphertext,
            "CBC encryption did not reproduce the challenge ciphertext"
        );
    }
}
