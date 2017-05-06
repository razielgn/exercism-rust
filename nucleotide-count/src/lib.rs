use std::collections::HashMap;

pub fn count(c: char, s: &str) -> usize {
    s.chars().filter(|cp| *cp == c).count()
}

pub fn nucleotide_counts(s: &str) -> HashMap<char, usize> {
    let counts = ['A', 'T', 'C', 'G'].iter().map(|c| (*c, 0)).collect();

    s.chars().fold(counts, |mut counts, nucleotide| {
        counts.get_mut(&nucleotide).map(|mut n| *n += 1);
        counts
    })
}
