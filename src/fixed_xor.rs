pub fn fixed_xor(hex_input: &str, hex_key: &str) -> String {
    let bytes = hex::decode(hex_input).unwrap();
    let key_bytes = hex::decode(hex_key).unwrap();

    let result: Vec<u8> = bytes.iter().zip(key_bytes.iter().cycle()).map(|(&byte, &key_byte)| {
        byte ^ key_byte
    }).collect();

    hex::encode(result)
}

pub fn test() {
    assert!(fixed_xor("1c0111001f010100061a024b53535009181c", "686974207468652062756c6c277320657965") == "746865206b696420646f6e277420706c6179");
}
