/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    let mut result = String::new();

    plain
        .chars()
        .for_each(| c | {
            result = push_char(&c, &result);

            if (result.len() + 1) % 6 == 0 {
                result.push(' ')
            }
        });

    result.as_str().trim().to_string()
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    let mut result = String::new();

    cipher
        .chars()
        .for_each(| c | result = push_char(&c, &result));

    result.as_str().trim().to_string()
}

/// Help method for the sake of preventing duplicate code
fn push_char(c: &char, str: &str) -> String {
    let mut result = String::from(str);

    match c.to_ascii_lowercase() as u8 {
        48..=57  => result.push(*c),
        97..=122 => result.push((((25u8 - ((c.to_ascii_lowercase() as u8) - 97u8)) + 97u8) as u8) as char),
        _ => ()
    };

    result
}