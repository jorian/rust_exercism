pub fn is_valid_isbn(isbn: &str) -> bool {

    let mut counter = 0;
    let isbn = isbn.chars()
        .filter(|c| c.is_alphanumeric())
        .enumerate();

    for (index, ch) in isbn {
        match index {
            0 ... 8 => {
                match ch.to_digit(10) {
                    Some(n) => counter += n * (10 - index as u32),
                    None => { }
                }
            },
            9 => {
                if ch.is_digit(10) {
                    counter += ch.to_digit(10).unwrap() * (10 - index as u32);
                } else if ch == 'X' {
                    counter += 10;
                }
            },
            _ => return false
        }
    }

    counter % 11 == 0
}