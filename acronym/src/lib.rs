use std::ascii::AsciiExt;

pub fn abbreviate(s: &str) -> String {
    s.split(|c: char| c.is_whitespace() || c == '-')
        .map(capitalize)
        .map(|mut word| {
            let uppercase = word
                .chars()
                .filter(|c| c.is_alphabetic())
                .all(|c| c.is_uppercase());

            if uppercase {
                word.remove(0).to_string()
            } else {
                word.chars().filter(|c| c.is_uppercase()).collect()
            }
        })
        .fold(String::new(), |mut abbr, s| {
            abbr.push_str(&s);
            abbr
        })
}

fn capitalize(s: &str) -> String {
    s.chars()
        .enumerate()
        .map(|(i, c)| if i == 0 { c.to_ascii_uppercase() } else { c })
        .collect()
}
