use base64::{self, Engine, prelude::BASE64_STANDARD_NO_PAD};
use hex;

pub fn hex_to_base64(input: &str) -> String {
    let hex_bytes = hex::decode(input).expect("Invalid hex string");
    BASE64_STANDARD_NO_PAD.encode(&hex_bytes)
}

pub fn test() {
    assert_eq!(
        hex_to_base64(
            "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"
        ),
        "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
    );
}
