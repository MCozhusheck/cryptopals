use crate::padding::pkcs7;

mod aes_cbc;
mod aes_cbc_ecb_detection;
mod aes_ecb;
mod fixed_xor;
mod hex_to_base64;
mod padding;
mod repeating_key_xor;
mod single_byte_xor;
mod utils;

fn main() {
    let input = "YELLOW SUBMARINE";
    let out = pkcs7(input.as_bytes(), 20);
    println!("{:?}", out)
}
