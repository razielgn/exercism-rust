pub fn reply(s: &str) -> &'static str {
    let is_nothing = || s.chars().all(|c| c.is_whitespace());
    let says_something = || s.chars().any(|c| c.is_alphanumeric());
    let all_caps =
        || s.chars().filter(|c| c.is_alphabetic()).all(|c| c.is_uppercase());
    let is_shouting = || says_something() && all_caps();
    let is_question = || s.ends_with("?");

    if is_nothing() {
        "Fine. Be that way!"
    } else if is_shouting() {
        "Whoa, chill out!"
    } else if is_question() {
        "Sure."
    } else {
        "Whatever."
    }
}
