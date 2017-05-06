use std::collections::BTreeMap;

#[derive(Default)]
pub struct School {
    people: BTreeMap<u8, Vec<String>>,
}

impl School {
    pub fn new() -> Self { Default::default() }

    pub fn add(&mut self, g: u8, name: &str) {
        let entry = self.people.entry(g).or_insert_with(|| vec![]);
        let name = name.into();

        if !entry.contains(&name) {
            entry.push(name);
            entry.sort();
        }
    }

    pub fn grades(&self) -> Vec<u8> { self.people.keys().cloned().collect() }

    pub fn grade(&self, g: u8) -> Option<&Vec<String>> { self.people.get(&g) }
}
