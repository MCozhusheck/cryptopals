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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_base64() {
        // Test case 1: Simple hex string
        let hex_input = "48656c6c6f";
        let expected = "SGVsbG8";
        assert_eq!(hex_to_base64(hex_input), expected);

        // Test case 2: Empty string
        assert_eq!(hex_to_base64(""), "");

        // Test case 3: Longer hex string
        let hex_input = "4d616e";
        let expected = "TWFu";
        assert_eq!(hex_to_base64(hex_input), expected);

        // Test case 4: Mixed case hex
        let hex_input = "48656C6C6F";
        let expected = "SGVsbG8";
        assert_eq!(hex_to_base64(hex_input), expected);
    }

    #[test]
    #[should_panic(expected = "Invalid hex string")]
    fn test_hex_to_base64_invalid_hex() {
        hex_to_base64("xyz");
    }
}
