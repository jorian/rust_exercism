pub fn is_pangram(sentence: &str) -> bool {
    let chars = sentence.chars().map(|x| x.to_ascii_lowercase());
    let mut v = Vec::new();

    for c in chars {
        if !c.is_whitespace() && c.is_ascii_alphabetic() {
            if !v.contains(&c) {
                v.push(c);
            }
        }
    }

    v.len() >= 26
}
