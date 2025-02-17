use std::cmp::min;
pub fn dp_rec_mc(amount: u32) -> u32 {
    // TODO: 这里写逻辑
    if amount == 0 {
        return 0;
    }
    let coins = vec![1, 2, 5, 10, 20, 30, 50, 100];
    let inf = amount + 1;
    let mut dp = vec![inf; (amount + 1) as usize];
    dp[0] = 0;
    for i in 0..=amount {
        for &j in &coins {
            let tot = i + j;
            if tot > amount {
                break;
            }
            dp[tot as usize] = min(dp[tot as usize], dp[i as usize] + 1);
        }
    }

    dp[amount as usize]
}
