/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    code.chars()
        .rev()
        .filter(|c| !c.is_whitespace())
        .map(|c| c.to_digit(10))
        .enumerate()
        .try_fold((0, 0), |(acc, _ln), (i, d)| {
            d.map(|d| {
                (match (i % 2, d > 4) {
                    (0, _) => acc + d,
                    (_, true) => acc + (d * 2) - 9,
                    (_, false) => acc + d * 2,
                }, i)
            })
        })
        .map_or(false, |(s, ln)| ln > 0 && s % 10 == 0)
}

// try_fold: making use of Some and None.
// If `d` is None upon finding a non-digit from the `c.to_digit(10) conversion, let try_fold return
// None. Then map_or sees this None and returns the default value, in this case `false`.
// If `d` is not None, update acc and ln, where ln will be just the index of enumerate. if > 0, OK.

// inspiration from https://exercism.io/tracks/rust/exercises/luhn/solutions/11460961e85f45f79bc5d68ddcee0ac8