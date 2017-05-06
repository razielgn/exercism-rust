pub struct PascalsTriangle(u32);

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self { PascalsTriangle(row_count) }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        (1..self.0 + 1)
            .map(|n| {
                let np = n - 1;

                (0..n)
                    .map(|k| fact(np) / (fact(k) * fact(np - k)))
                    .collect()
            })
            .collect()
    }
}

fn fact(n: u32) -> u32 {
    (1..n + 1).product()
}
