fn prime(limit: u64) -> Vec<u64> {
    let mut is_prime = vec![true; (limit + 1) as usize];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..=((limit as f64).sqrt() as u64) {
        if is_prime[i as usize] {
            for j in (i * i..=limit).step_by(i as usize) {
                is_prime[j as usize] = false;
            }
        }
    }
    is_prime
        .iter()
        .enumerate()
        .filter_map(|(i, &is_prime)| if is_prime { Some(i as u64) } else { None })
        .collect()
}

fn bmul(a: u128, b: u128) -> u128 {
    let mut a = a;
    let mut b = b;
    let mut result = 0;
    while b > 0 {
        if b & 1 == 1 {
            result += a;
        }
        a <<= 1;
        b >>= 1;
    }
    result
}

pub fn find_max_prime_factor(number: u128) -> u128 {
    let mut number = number;
    let mut max_factor = 2;
    let mut small_primes = prime(1000);

    for &prime in &small_primes {
        while number % prime as u128 == 0 {
            max_factor = prime as u128;
            number /= prime as u128;
        }
    }

    let limit = ((number as f64).sqrt() as u64).min(1_000_000);
    small_primes = prime(limit);

    for &prime in &small_primes {
        while number % prime as u128 == 0 {
            max_factor = prime as u128;
            number /= prime as u128;
        }
    }

    let last = *small_primes.last().unwrap() as u128;
    let mut factor = if (number as f64).sqrt() as u128 > last {
        last * last
    } else {
        last
    };

    while bmul(factor, factor) <= number {
        while number % factor == 0 {
            max_factor = factor;
            number /= factor;
        }
        while number % (factor + 2) == 0 {
            max_factor = factor + 2;
            number /= factor + 2;
        }
        factor += 6;
    }

    if number > 2 {
        number
    } else {
        max_factor
    }
}
