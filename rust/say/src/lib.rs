pub fn encode(n: u64) -> String {
    if n == 0 {
        return String::from("zero")
    }

    let mut result = String::new();
    let mut n = n;

    if n > 999_999_999_999_999_999 {
        result.push_str(&format!("{} quintillion", &triplet_text(n / 1_000_000_000_000_000_000 as u64)));
        n = n % 1_000_000_000_000_000_000;
    }

    if n > 999_999_999_999_999 {
        result.push_str(&format!("{} quadrillion", &triplet_text(n / 1_000_000_000_000_000 as u64)));
        n = n % 1_000_000_000_000_000;
    }

    if n > 999_999_999_999 {
        result.push_str(&format!("{} trillion", &triplet_text(n / 1_000_000_000_000 as u64)));
        n = n % 1_000_000_000_000;
    }

    if n > 999_999_999 {
        result.push_str(&format!("{} billion", &triplet_text(n / 1_000_000_000 as u64)));
        n = n % 1_000_000_000;
    }

    if n > 999_999 {
        result.push_str(&format!("{} million", &triplet_text(n / 1_000_000 as u64)));
        n = n % 1_000_000;
    }

    if n > 999 {
        result.push_str(&format!("{} thousand", &triplet_text(n / 1000 as u64)));
        n = n % 1_000;
    }

    if n > 0 {
        result.push_str(
            &format!("{}", &triplet_text(n % 1000)));
    }

    String::from(result.trim())
}

fn triplet_text(n: u64) -> String {
    let mut temp_result = String::new();
    let mut n = n;

    if n > 99 {
        temp_result.push_str(&format!(" {} hundred", get_single((n - (n % 100)) / 100)));
        n = n % 100;
    }

    if n > 9 {
        if n >= 20 {
            // need for a hyphen
            temp_result.push_str(&format!(" {}", &get_tenth((n - (n % 10)) / 10)));

            if n % 10 != 0 {
                temp_result.push_str(&format!("-{}", get_single(n % 10)));
            }
        } else {
            if n >= 10 && n < 20 {
                temp_result.push_str(&format!(" {}", &get_teens(n)));
            }
        }
    } else {
        temp_result.push_str(&format!(" {}", get_single(n % 10)));
    }
    temp_result
}

fn get_single<'a>(n: u64) -> &'a str {
    match n {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        _ => ""
    }
}

fn get_tenth<'a>(n: u64) -> &'a str {
    match n {
        2 => "twenty",
        3 => "thirty",
        4 => "forty",
        5 => "fifty",
        6 => "sixty",
        7 => "seventy",
        8 => "eighty",
        9 => "ninety",
        _ => ""
    }
}

fn get_teens<'a>(n: u64) -> &'a str {
    match n {
        10 => "ten",
        11 => "eleven",
        12 => "twelve",
        13 => "thirteen",
        14 => "fourteen",
        15 => "fifteen",
        16 => "sixteen",
        17 => "seventeen",
        18 => "eighteen",
        19 => "nineteen",
        _ => ""
    }
}