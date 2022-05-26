pub fn nth(mut n: u32) -> u32 {
    n += 1;
    let mut number: u32 = 0;
    loop {
        if is_prime(number) {
            n -= 1;
        }
        if n == 0 {
            break;
        }
        number += 1;
    }
    number
}

fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }

    for i in 2..(n as f64).sqrt() as u32 + 1 {
        if n % i == 0 {
            return false;
        }
    }
    true
}
