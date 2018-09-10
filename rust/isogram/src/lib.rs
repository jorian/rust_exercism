pub fn check(candidate: &str) -> bool {

    if candidate.chars().all(|c| c.is_whitespace()) {
        return true;
    }

    let mut result = Vec::new();

    for character in candidate.to_lowercase().chars() {
        if result.contains(&character) && !(character.is_whitespace() || character == '-') {
            return false
        } else {
            result.push(character);
        }
    }
    true
}