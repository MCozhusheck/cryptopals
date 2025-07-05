#[allow(dead_code)]
pub fn repeating_key_xor(input: &[u8], key: &[u8]) -> Vec<u8> {
    let mut output = Vec::with_capacity(input.len());
    let key_len = key.len();
    let mut key_index = 0;

    for byte in input {
        output.push(byte ^ key[key_index]);
        key_index = (key_index + 1) % key_len;
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repeating_key_xor() {
        let input = b"Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
        let key = b"ICE";
        let expected_hex = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";
        let result_bytes = repeating_key_xor(input, key);
        let result_hex = hex::encode(result_bytes);

        assert_eq!(result_hex, expected_hex);
    }
}
