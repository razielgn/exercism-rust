
use std::ascii::AsciiExt;
use std::collections::BTreeMap;

pub fn transform(from: &BTreeMap<i32, Vec<String>>) -> BTreeMap<String, i32> {
    from.iter()
        .fold(BTreeMap::new(), |mut to, (score, words)| {
            for word in words {
                to.insert(word.to_ascii_lowercase(), *score);
            }

            to
        })
}
