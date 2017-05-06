use std::ascii::AsciiExt;
use std::collections::HashMap;

pub fn word_count(s: &str) -> HashMap<String, usize> {
    s.chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .collect::<String>()
        .split(" ")
        .filter(|word| !word.is_empty())
        .map(|word| word.to_ascii_lowercase())
        .fold(HashMap::new(), |mut acc, word| {
            *acc.entry(word).or_insert(0) += 1;
            acc
        })
}
