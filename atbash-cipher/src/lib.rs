extern crate itertools;

use itertools::Itertools;
use std::ascii::AsciiExt;

static KEY: u8 = 'a' as u8 + 'z' as u8;

pub fn encode(s: &str) -> String {
    s.chars()
        .filter(|c| c.is_ascii() && c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .map(trans)
        .chunks(5)
        .into_iter()
        .map(|s| s.collect::<String>())
        .join(" ")
}

pub fn decode(s: &str) -> String {
    s.chars()
        .filter(|c| c.is_ascii() && c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .map(trans)
        .collect()
}

fn trans(c: char) -> char {
    if c.is_digit(10) {
        return c;
    }

    (KEY - c as u8) as char
}
