pub fn factors(n: u64) -> Vec<u64> {
    let mut result: Vec<u64> = Vec::new();
    let mut m = n;

    while m % 2 == 0 {
        result.push(2);
        m /= 2;
    }

    let mm = (m as f64).sqrt() as u64 + 1;
    for i in (3..=mm).filter(|x| (x % 2 != 0)) {

        while m % i == 0 {
            result.push(i);
            m /= i;
        }
    }

    if m > 2 {
        result.push(m);
    }

    result
}
