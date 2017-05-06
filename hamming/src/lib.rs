pub fn hamming_distance(x: &str, y: &str) -> Result<usize, ()> {
    if x.len() != y.len() {
        return Err(());
    }

    Ok(x.chars()
        .zip(y.chars())
        .map(|(a, b)| if a == b { 0 } else { 1 })
        .sum())
}
