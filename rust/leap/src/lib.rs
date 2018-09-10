pub fn is_leap_year(year: i32) -> bool {
//    unimplemented!("true if {} is a leap year", year)

//    match year {
//        x if x % 400 == 0 => true,
//        x if x % 100 == 0 => false,
//        x if x % 4 == 0 => true,
//        _ => false
//    }

    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}
