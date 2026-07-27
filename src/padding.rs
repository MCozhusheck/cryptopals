use std::iter;

pub fn pkcs7(input: &[u8], block_size: usize) -> Vec<u8> {
    if block_size == 0 {
        return input.to_vec();
    }

    let mut missing_pads = block_size - (input.len() % block_size);
    if missing_pads == 0 {
        missing_pads = block_size
    }
    let mut pad: Vec<u8> = iter::repeat_n(missing_pads as u8, missing_pads).collect();
    let mut padded = input.to_vec();
    padded.append(&mut pad);
    padded
}

mod tests {
    use super::*;

    #[test]
    fn pkcs7_padding() {
        let input = "YELLOW SUBMARINE";
        let out = pkcs7(input.as_bytes(), 20);
        let expected_len: usize = 20;
        assert!(out.len().eq(&expected_len))
    }
    #[test]
    fn pkcs7_exact_padding() {
        let input = "YELLOW SUBMARINE";
        let out = pkcs7(input.as_bytes(), 16);
        let expected_len: usize = 32;
        assert!(out.len().eq(&expected_len))
    }
}
