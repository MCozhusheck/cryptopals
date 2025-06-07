# Cryptopals Set 1 Challenge 3: Single-byte XOR cipher

## Problem Summary
You have a hex-encoded string that has been XORed with a single character. Your task is to find the key and decrypt the message.

## The Challenge
```
1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736
```

## Solution Approach

### 1. Understanding XOR
XOR (exclusive or) has a key property: if `A XOR B = C`, then `C XOR B = A`. This means if we XOR the ciphertext with the correct key, we get back the original plaintext.

### 2. Brute Force Strategy
Since it's a single-byte key, there are only 256 possible values (0-255). We can try each one and score the resulting text to find the most English-like result.

### 3. Text Scoring Methods

#### Method 1: Character Frequency Analysis
- Compare letter frequencies in the decrypted text with standard English frequencies
- Use chi-squared test to measure how well the frequencies match
- English letter frequencies (approximate):
  - E: 12.7%, T: 9.1%, A: 8.1%, O: 7.5%, I: 7.0%, N: 6.7%, S: 6.3%, H: 6.1%, R: 6.0%

#### Method 2: Common Character Scoring
- Award points for common English characters (space, e, t, a, o, i, n, s, h, r)
- Penalize heavily for non-printable ASCII characters
- Simple but effective for this challenge

### 4. Handling Encoding Issues

#### Printable ASCII Filter
```rust
pub fn is_printable_ascii(byte: u8) -> bool {
    byte >= 32 && byte <= 126
}
```
- ASCII 32-126 covers printable characters (space through tilde ~)
- Non-printable characters indicate wrong key

#### Invalid Character Handling
- If decryption produces non-printable characters, heavily penalize the score
- Only consider keys that produce entirely printable output

## Implementation Details

### Core Functions

1. **single_byte_xor_decode**: XORs hex input with a single byte key
2. **score_english_text**: Scores text based on English characteristics
3. **find_single_byte_xor_key**: Tries all 256 keys and returns the best one

### Scoring Algorithm
```rust
// Combine multiple scoring methods:
// 1. Chi-squared test on letter frequencies (lower is better)
// 2. Common character ratio (higher is better)
// Final score = -chi_squared + (common_ratio * weight)
```

## Solution
- **Key**: 88 (ASCII 'X')
- **Decrypted text**: "Cooking MC's like a pound of bacon"
- **Method**: The key 'X' when XORed with the ciphertext produces readable English

## Key Insights

1. **Statistical Analysis Works**: Even with limited text, frequency analysis can identify English
2. **Printable ASCII Filter**: Eliminates most wrong keys immediately
3. **Multiple Scoring Methods**: Combining different approaches improves reliability
4. **Common Characters**: Space and frequent letters are strong indicators

## Extensions for Future Challenges

This foundation supports:
- Multi-byte XOR (repeating key)
- Detecting XOR in multiple ciphertexts
- Breaking Vigenère ciphers
- General cryptanalysis techniques

The scoring functions and text analysis methods developed here are reusable for more complex cryptographic challenges.