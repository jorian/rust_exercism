pub fn find() -> Option<u32> {
    let sum: u32 = 1000;

    for a in 0..sum/3 {
        for b in 0..sum/2 {
            let c: u32 = sum - a - b;
            if a*a + b*b == c*c {
                return Some(a*b*c);
            }
        }
    }
    None
}
