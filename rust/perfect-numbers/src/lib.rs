use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    match num {
        0 => None,
        _ => {
            // everything above n/2 can't be a 0 mod.
            let sum: u64 = (1..=(num/2)).filter(|i| num % i == 0).sum();

            match num.cmp(&sum) {
                Ordering::Less => Some(Classification::Abundant),
                Ordering::Greater => Some(Classification::Deficient),
                Ordering::Equal => Some(Classification::Perfect),
            }
        }
    }
}