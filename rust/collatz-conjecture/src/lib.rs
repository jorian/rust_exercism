pub fn collatz(n: u64) -> Option<u64> {
//    unimplemented!(
//        "return Some(x) where x is the number of steps required to reach 1 starting with {}",
//        n,
//    )

    let mut n = n;
    let mut sum: u64 = 0;

    if n < 1 {
        return None
    }

    while n > 1 {
        sum += 1;

        if n % 2 == 0 {
            n /= 2;
        } else {
            n = n * 3 + 1;
        }
    }
    Some(sum)
}
