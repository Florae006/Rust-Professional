pub fn odd_fibnacci_sum(threshold: u32) -> u32 {
    // TODO: 这里写逻辑
    if threshold < 1 {
        return 0;
    }

    let mut fib = vec![1, 1];
    let mut sum: u32 = 2;
    while *fib.last().unwrap() < threshold {
        let len = fib.len();
        let x = fib[(len - 1) as usize] + fib[(len - 2) as usize];
        fib.push(x);
        if x % 2 == 1 && x < threshold {
            sum += x;
        }
    }
    sum
}
