pub fn is_leap_year(y: u16) -> bool {
    let is_div_by = |m| -> bool { y % m == 0 };

    if is_div_by(400) {
        true
    } else if is_div_by(100) {
        false
    } else if is_div_by(4) {
        true
    } else {
        false
    }
}
