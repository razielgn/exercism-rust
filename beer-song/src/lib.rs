use std::borrow::Cow;

pub fn verse(n: usize) -> String {
    let first = format!("{} of beer on the wall, {} of beer.",
                        bottles(n),
                        bottles(n));

    let second: Cow<'static, str> = if n == 0 {
        "Go to the store and buy some more".into()
    } else {
        let what = if n == 1 { "it" } else { "one" };
        format!("Take {} down and pass it around", what).into()
    };

    let rem = if n == 0 { 99 } else { n - 1 };
    let third = format!("{} of beer on the wall", bottles(rem));

    format!("{}\n{}, {}.\n", first, second, third)
}

pub fn sing(from: usize, to: usize) -> String {
    let mut song =
        (to..from + 1).rev().map(verse).fold(String::new(), |mut acc, verse| {
            acc.push_str(&verse);
            acc.push_str("\n");
            acc
        });

    let _ = song.pop();
    song
}

fn bottles(n: usize) -> Cow<'static, str> {
    match n {
        0 => "no more bottles".into(),
        1 => format!("{} bottle", n).into(),
        _ => format!("{} bottles", n).into(),
    }
}
