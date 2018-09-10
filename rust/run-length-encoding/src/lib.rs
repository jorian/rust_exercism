use std::iter;

pub fn encode(source: &str) -> String {
    let mut result = String::new();
    let chars: Vec<_> = source.chars().collect();
    let len = chars.len();
    let mut count:u32 = 0;

    for i in 0..len {
        count += 1;
        if i == len - 1 || chars[i] != chars[i+1] {
            if count > 1 {
                result.push_str(&format!("{}", count))
            }
            result.push(chars[i]);
            count = 0;
        }
    }
    result
}



pub fn decode(source: &str) -> String {
    let mut result = Vec::new();
    let mut x = 0;

    for c in source.chars() {
        if let Some(n) = c.to_digit(10) {
            x = x * 10 + n;
        } else if c.is_alphabetic() || c.is_whitespace() {
            result.push((c, if x > 0 {x} else {1} ));
            x = 0;
        } else {
            panic!("char not supported");
        }
    }

    result.into_iter()
        .flat_map( |(c, x)| iter::repeat(c).take(x as usize))
        .collect()
}
