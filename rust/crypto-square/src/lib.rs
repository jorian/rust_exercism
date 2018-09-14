extern crate itertools;
use itertools::Itertools;

pub fn encrypt(input: &str) -> String {
    let input = input
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .flat_map(char::to_lowercase)
        .collect::<Vec<_>>();

    let c = (input.len() as f64).sqrt().ceil() as usize;
    let r = ((input.len() as f64) / (c as f64)).ceil() as usize;
    let mut result = vec![' '; c * r];

    for x in 0..c {
        for y in 0..r {
            if let Some(c) = input.get(y * c + x) {
                result[x * r + y] = *c;
            }
        }
    }

    result
        .chunks(r)
        .map(|w| w.iter().collect::<String>())
        .join(" ")
}