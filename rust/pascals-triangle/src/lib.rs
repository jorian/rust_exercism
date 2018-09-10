pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle {
            rows: (0..row_count)
                .map(|x| {
                    (0..=x)
                        .map(|y| nck(x, y))
                        .collect::<Vec<u32>>()
                })
                .collect()
        }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.clone()
    }
}

fn nck(n: u32, k: u32) -> u32 {
    // nCk: Binomial coefficients: n! / (k! * (n-k)!)
    let numerator = fact(n);
    let denumerator = fact(k) * fact(n - k);

    numerator / denumerator
}

fn fact(f: u32) -> u32 {
    (1..=f).fold(1, |acc, n| acc * n)
}




//http://www.javawithus.com/programs/pascal-triangle