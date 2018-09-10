pub fn is_armstrong_number(num: u32) -> bool {

    let string = &num.to_string();

    let multiplied_num = string.chars()
        .map(|x| x.to_digit(10).unwrap())
        .fold(0, |acc, x| acc + (x.pow(string.len() as u32)));

    num == multiplied_num
}
