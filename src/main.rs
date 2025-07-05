mod aes_ecb;
mod fixed_xor;
mod hex_to_base64;
mod repeating_key_xor;
mod single_byte_xor;
mod utils;

fn main() {
    hex_to_base64::test();
    fixed_xor::test();
    single_byte_xor::solve_challenge_3().unwrap();
    single_byte_xor::solve_challenge_4().unwrap();
    println!("Decrypted file: {}", aes_ecb::decrypt_file());
}
