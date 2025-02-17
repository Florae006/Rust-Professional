pub fn goldbach_conjecture() -> String {
    // let result = vec![5777, 5993];
    // return format!("{},{}", result[0], result[1]);
    let n = 10000;
    let mut is_prime = vec![true; n as usize + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    let mut primes = Vec::new();
    for i in 2..=n {
        if is_prime[i as usize] {
            primes.push(i);
            let mut j = i * i;
            while j <= n {
                is_prime[j as usize] = false;
                j += i;
            }
        }
    }

    let mut result = Vec::new();
    let mut x = 9;

    while result.len() < 2 {
        if x % 2 == 1 && !is_prime[x as usize] {
            let mut flag = false;
            for &i in &primes {
                if i >= x {
                    break;
                }
                let mut y = x - i;
                if y % 2 == 0 {
                    let mut k2 = y / 2;
                    let mut k = (k2 as f64).sqrt() as i32;
                    if k * k == k2 {
                        flag = true;
                        break;
                    }
                }
            }
            if !flag {
                result.push(x);
            }
        }
        x += 2;
    }
    return format!("{},{}", result[0], result[1]);
}
