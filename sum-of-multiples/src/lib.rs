use std::collections::HashSet;

struct Multiples {
    of: u64,
    curr: u64,
}

impl Multiples {
    pub fn of(of: u64) -> Self {
        Multiples { of, curr: 0 }
    }
}

impl Iterator for Multiples {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.curr += self.of;
        Some(self.curr)
    }
}

pub fn sum_of_multiples(limit: u64, numbers: &[u64]) -> u64 {
    let mut multiples = HashSet::new();

    for n in numbers {
        multiples.extend(Multiples::of(*n).take_while(|m| *m < limit));
    }

    multiples.iter().sum()
}
