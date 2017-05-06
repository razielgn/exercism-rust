use std::ascii::AsciiExt;

pub fn is_pangram(s: &str) -> bool {
    s.chars()
        .filter(|c| c.is_ascii() && c.is_alphabetic())
        .map(|c| c.to_ascii_lowercase())
        .map(|c| c as usize - 'a' as usize)
        .fold(0, |acc, c| acc | 1 << c) == 0x3ffffff // 26 bits
}
