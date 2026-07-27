use crate::{aes_ecb::detect_ecb, padding::pkcs7};

mod aes_ecb;
mod fixed_xor;
mod hex_to_base64;
mod padding;
mod repeating_key_xor;
mod single_byte_xor;
mod utils;

fn main() {
    hex_to_base64::test();
    fixed_xor::test();
    single_byte_xor::solve_challenge_3().unwrap();
    single_byte_xor::solve_challenge_4().unwrap();
    println!("Decrypted file: {}", aes_ecb::decrypt_file());
    detect_ecb();
    let input = "YELLOW SUBMARINE";
    let out = pkcs7(input.as_bytes(), 20);
    println!("{:?}", out)
}
