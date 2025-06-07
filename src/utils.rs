use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

pub trait XOR {
    fn xor(&self, _: &Self) -> Vec<u8>;
    fn xor_inplace(&mut self, _: &Self);
}

impl XOR for [u8] {
    fn xor(&self, t: &[u8]) -> Vec<u8> {
        let mut result = self.to_vec();
        result[..].xor_inplace(t);
        result
    }

    fn xor_inplace(&mut self, t: &[u8]) {
        for chunk in self.chunks_mut(t.len()) {
            let len = chunk.len();
            for (c, &d) in chunk.iter_mut().zip(t[..len].iter()) {
                *c ^= d;
            }
        }
    }
}

fn u8_from_hex(c: char) -> Result<u8> {
    match c.to_digit(16) {
        Some(i) => Ok(i as u8),
        _ => Err(format!("invalid character {}", c).into()),
    }
}

pub fn from_hex(s: &str) -> Result<Vec<u8>> {
    if s.len() % 2 != 0 {
        return Err("input length needs to be multiple of 2".into());
    }

    let mut digits = Vec::with_capacity(s.len());
    for c in s.chars() {
        digits.push(u8_from_hex(c)/*.context(format!("not a valid hex string: {}", s))*/?);
    }
    Ok(digits
        .chunks(2)
        .map(|c| (c[0] << 4) + c[1])
        .collect::<Vec<u8>>())
}

fn from_lines(path: &Path, converter: fn(&str) -> Result<Vec<u8>>) -> Result<Vec<Vec<u8>>> {
    let mut content = Vec::new();
    let file = File::open(&path)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        content.push(converter(line.unwrap().trim())?);
    }
    Ok(content)
}

pub fn from_hex_lines(path: &Path) -> Result<Vec<Vec<u8>>> {
    from_lines(path, from_hex)
}
