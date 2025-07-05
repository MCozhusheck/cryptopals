use crate::utils::{Result, XOR, from_hex, from_hex_lines};
use std::{collections::HashMap, path::Path};

pub fn single_byte_xor_decode(hex_input: &str, key: u8) -> Vec<u8> {
    let bytes = hex::decode(hex_input).expect("Invalid hex input");
    bytes.iter().map(|&b| b ^ key).collect()
}

static EXPECTED_FREQUENCIES: [(u8, f32); 28] = [
    (b' ', 12.17), // Whitespace
    (b'.', 6.57),  // Others
    (b'a', 6.09),
    (b'b', 1.05),
    (b'c', 2.84),
    (b'd', 2.92),
    (b'e', 11.36),
    (b'f', 1.79),
    (b'g', 1.38),
    (b'h', 3.41),
    (b'i', 5.44),
    (b'j', 0.24),
    (b'k', 0.41),
    (b'l', 2.92),
    (b'm', 2.76),
    (b'n', 5.44),
    (b'o', 6.00),
    (b'p', 1.95),
    (b'q', 0.24),
    (b'r', 4.95),
    (b's', 5.68),
    (b't', 8.03),
    (b'u', 2.43),
    (b'v', 0.97),
    (b'w', 1.38),
    (b'x', 0.24),
    (b'y', 1.30),
    (b'z', 0.03),
];

fn is_control(u: u8) -> bool {
    u < 0x20 || u == 0x7F
}

fn is_alphabetic(u: u8) -> bool {
    (u >= 0x41 && u <= 0x5A) || (u >= 0x61 && u <= 0x7A)
}

fn get_character_counts(v: &[u8]) -> HashMap<u8, f32> {
    let mut counts: HashMap<u8, f32> = HashMap::new();
    for &c in v.iter() {
        if is_control(c) {
            continue;
        }
        let key = if is_alphabetic(c) {
            c.to_ascii_lowercase()
        } else if c == b' ' || c == b'\t' {
            b' '
        } else {
            b'.'
        };

        let count = counts.entry(key).or_insert(0f32);
        *count += 1f32;
    }
    counts
}

pub fn compute_score(v: &[u8]) -> u32 {
    if !v.is_ascii() {
        return std::u32::MAX;
    }

    if v.iter().any(|&c| is_control(c) && c != b'\n') {
        return std::u32::MAX;
    }

    let counts = get_character_counts(v);
    let length = v.len() as f32;

    EXPECTED_FREQUENCIES.iter().fold(0f32, |a, &(c, score)| {
        let expected_count = score / 100f32 * length;
        let &actual_count = counts.get(&c).unwrap_or(&0f32);
        a + (expected_count - actual_count).powi(2)
    }) as u32
}

pub fn break_single_byte_xor(input: &[u8]) -> u8 {
    // We consider arbitrary bytes here because of challenges 19 and 20.
    (0u8..=255)
        .min_by_key(|&u| compute_score(&input.xor(&[u])))
        .unwrap() // unwrap is ok
}

pub fn solve_challenge_3() -> Result<()> {
    let input = from_hex("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736")?;
    let key = break_single_byte_xor(&input);
    let decoded = input.xor(&[key]);
    println!("Key: {}", key);
    println!("Decoded: {}", String::from_utf8_lossy(&decoded));
    Ok(())
}

pub fn solve_challenge_4() -> Result<()> {
    let path = Path::new("src/data/s1c4_input.txt");
    let lines = from_hex_lines(path)?;
    let result = lines
        .into_iter()
        .flat_map(|line: Vec<u8>| (0u8..128).map(move |u| line.xor(&[u])))
        .min_by_key(|cand| compute_score(cand))
        .unwrap(); // unwrap is ok
    println!("Decoded: {}", String::from_utf8_lossy(&result));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_byte_xor_decode() {
        let result = single_byte_xor_decode("1c0111001f010100061a024b53535009181c", 42);
        assert!(!result.is_empty());
    }
}
