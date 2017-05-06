pub fn primes_up_to(n: u64) -> Vec<u64> {
    if n < 2 {
        return vec![];
    }

    let mut numbers: Vec<u64> = (2..n + 1).collect();

    let fin = (n as f32).sqrt() as u64;
    let mut i = 0;

    loop {
        let m = numbers[i];

        if m > fin {
            break;
        }

        numbers.retain(|&n| n == m || n % m != 0);

        i += 1;
    }

    numbers
}
