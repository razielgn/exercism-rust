use std::ascii::AsciiExt;

pub fn score(s: &str) -> usize {
    s.chars().map(score_char).sum()
}

fn score_char(c: char) -> usize {
    match c.to_ascii_lowercase() {
        'a' | 'e' | 'i' | 'o' | 'u' | 'l' | 'n' | 'r' | 's' | 't' => 1,
        'd' | 'g'                                                 => 2,
        'b' | 'c' | 'm' | 'p'                                     => 3,
        'f' | 'h' | 'v' | 'w' | 'y'                               => 4,
        'k'                                                       => 5,
        'j' | 'x'                                                 => 8,
        'q' | 'z'                                                 => 10,
        _                                                         => 0,
    }
}
