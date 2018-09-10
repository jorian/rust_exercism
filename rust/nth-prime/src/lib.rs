pub fn nth(n: u32) -> Option<u32> {
//    unimplemented!("What is the {}th prime number?", n)
    match n {
        1 | 2 => Some(n + 1), // 1st prime = 2, 2nd prime = 3
        3 ... 10003 => Some(trial(n)),
        _ => None // we don't do more.
    }
}

fn trial(x: u32) -> u32 {
    let mut primes: Vec<u32> = Vec::with_capacity(x as usize);

    primes.push(2);
    primes.push(3);

    let mut next_checked = primes.last().unwrap() + 2; // to get rid of even numbers

    while primes.len() < x as usize { // x == nth prime

        // invoking all on iter() will check all elements in the iterator. if all are true, it will execute
        // the code block. if one check of the elements returns false, it will stop and skip the code block
        if primes.iter().all(|&i| next_checked % i != 0) {
            primes.push(next_checked)
        }

        // if x is not divisible by any of existing primes, then it is prime.

        next_checked += 2;
    }

    *primes.last().unwrap() // end of while loop, we reached nth prime so return it.
}
