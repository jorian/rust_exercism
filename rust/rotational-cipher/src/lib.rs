pub fn rotate(input: &str, key: i8) -> String {
    input
        .chars()
        .map(|c| {
            match c {
                'A'...'Z' => (65 + ((((c as i8) - 65) + key) % 26) as u8) as char,
                'a'...'z' => (97 + ((((c as i8) - 97) + key) % 26) as u8) as char,
                _ => c
            }
        })
        .collect::<String>()
}