pub fn factors(mut n: u64) -> Vec<u64> {
    let mut res: Vec<u64> = Vec::new();
    let mut number: u64 = 0;

    loop {
        if n <= 1 {
            break;
        }
        if is_prime(number) && n % number == 0 {
            n /= number;
            res.push(number);
            continue;
        }
        number += 1;
    }
    res
}

fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }

    for i in 2..(n as f64).sqrt() as u64 + 1 {
        if n % i == 0 {
            return false;
        }
    }
    true
}
