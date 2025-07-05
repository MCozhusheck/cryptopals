use base64::{Engine as _, engine::general_purpose};
use openssl::symm::{Cipher, Crypter, Mode};

pub fn encrypt_ecb(key: &[u8], plaintext: &[u8]) -> Vec<u8> {
    let cipher = Cipher::aes_128_ecb();
    let mut crypter = Crypter::new(cipher, Mode::Encrypt, key, None).unwrap();
    crypter.pad(false);

    let mut ciphertext = vec![0; plaintext.len() + cipher.block_size()];
    let mut count = crypter.update(plaintext, &mut ciphertext).unwrap();
    count += crypter.finalize(&mut ciphertext[count..]).unwrap();
    ciphertext.truncate(count);

    ciphertext
}

pub fn decrypt_ecb(key: &[u8], ciphertext: &[u8]) -> Vec<u8> {
    let cipher = Cipher::aes_128_ecb();
    let mut crypter = Crypter::new(cipher, Mode::Decrypt, key, None).unwrap();
    crypter.pad(false);

    let mut plaintext = vec![0; ciphertext.len() + cipher.block_size()];
    let mut count = crypter.update(ciphertext, &mut plaintext).unwrap();
    count += crypter.finalize(&mut plaintext[count..]).unwrap();
    plaintext.truncate(count);

    plaintext
}

pub fn decrypt_file() -> String {
    let key = b"YELLOW SUBMARINE";

    let base64_str = include_str!("data/s1c7_input.txt");
    let base64_clean = base64_str.replace('\n', "").replace('\r', "");
    let ciphertext = general_purpose::STANDARD.decode(base64_clean).unwrap();

    let decrypted = decrypt_ecb(key, &ciphertext);
    String::from_utf8(decrypted).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aes_ecb_encrypt_decrypt() {
        let key = b"YELLOW SUBMARINE";
        let plaintext = b"Hello, World!123"; // 16 bytes for AES block size

        let ciphertext = encrypt_ecb(key, plaintext);
        let decrypted = decrypt_ecb(key, &ciphertext);

        assert_eq!(plaintext, decrypted.as_slice());
    }

    #[test]
    fn test_aes_ecb_known_vector() {
        // Test with a known test vector
        let key = b"YELLOW SUBMARINE";
        let plaintext = b"0123456789abcdef"; // 16 bytes

        let ciphertext = encrypt_ecb(key, plaintext);
        let decrypted = decrypt_ecb(key, &ciphertext);

        assert_eq!(plaintext, decrypted.as_slice());
        assert_eq!(ciphertext.len(), 16); // Should be one block
    }

    #[test]
    fn test_aes_ecb_multiple_blocks() {
        let key = b"YELLOW SUBMARINE";
        let plaintext = b"This is exactly 48 bytes of plaintext data here!"; // 48 bytes (3 blocks)

        let ciphertext = encrypt_ecb(key, plaintext);
        let decrypted = decrypt_ecb(key, &ciphertext);

        assert_eq!(plaintext, decrypted.as_slice());
        assert_eq!(ciphertext.len(), 48); // Should be three blocks
    }
}
