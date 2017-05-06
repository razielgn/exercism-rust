use std::collections::HashMap;

pub struct Brackets<'a> {
    s: &'a str,
    openings: HashMap<char, char>,
    closings: HashMap<char, char>,
}

impl<'a> From<&'a str> for Brackets<'a> {
    fn from(s: &'a str) -> Self { Brackets::new(s) }
}

const BRACKETS: &'static [(char, char)] = &[('(', ')'), ('[', ']'), ('{', '}')];

impl<'a> Brackets<'a> {
    pub fn new(s: &'a str) -> Self {
        Brackets {
            s: s,
            openings: BRACKETS.iter().cloned().collect(),
            closings: BRACKETS.iter().map(|&(o, c)| (c, o)).collect(),
        }
    }

    pub fn are_balanced(&self) -> bool {
        let mut stack = Vec::new();

        for c in self.s.chars() {
            if self.openings.contains_key(&c) {
                stack.push(c);
            } else if let Some(&closing) = self.closings.get(&c) {
                if Some(closing) != stack.pop() {
                    return false;
                }
            }
        }

        stack.is_empty()
    }
}
