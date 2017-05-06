const CONFIG: &'static [(u32, &'static str)] = &[(3, "Pling"), (5, "Plang"), (7, "Plong")];

pub fn raindrops(n: u32) -> String {
    let is_div_by = |m| -> bool { n % m == 0 };

    let output = CONFIG.iter()
        .fold(String::new(), |mut acc, &(m, word)| {
            if is_div_by(m) {
                acc.push_str(word);
            }

            acc
        });

    if output.is_empty() {
        format!("{}", n)
    } else {
        output
    }
}
