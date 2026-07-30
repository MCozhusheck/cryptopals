pub fn fixed_xor<const N: usize>(left: &[u8; N], right: &[u8; N]) -> [u8; N] {
    std::array::from_fn(|index| left[index] ^ right[index])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_xor_known_vector() {
        const LEN: usize = 18;

        let left: [u8; LEN] = hex::decode("1c0111001f010100061a024b53535009181c")
            .unwrap()
            .try_into()
            .unwrap();

        let right: [u8; LEN] = hex::decode("686974207468652062756c6c277320657965")
            .unwrap()
            .try_into()
            .unwrap();

        let expected: [u8; LEN] = hex::decode("746865206b696420646f6e277420706c6179")
            .unwrap()
            .try_into()
            .unwrap();

        assert_eq!(fixed_xor(&left, &right), expected);
    }
}
