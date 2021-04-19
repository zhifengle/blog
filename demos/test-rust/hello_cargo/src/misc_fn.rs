#[allow(dead_code)]
fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}
#[allow(dead_code)]
pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    if amount == 0 {
        return 0;
    }
    let mut dp = vec![amount + 1; (amount + 1) as usize];
    dp[0] = 0;

    for i in 1..=amount {
        for &coin in coins.iter() {
            if i >= coin {
                dp[i as usize] = i32::min(dp[i as usize], dp[(i - coin) as usize] + 1);
            }
        }
    }
    if dp[amount as usize] > amount {
        -1
    } else {
        dp[amount as usize]
    }
}

#[allow(dead_code)]
pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
    if amount == 0 {
        return 1;
    }
    // 0 是初始值； amount+1 是大小
    let mut dp = vec![0; (amount + 1) as usize];
    dp[0] = 1;
    for i in 0..coins.len() {
        for j in coins[i]..=amount {
            dp[j as usize] = dp[j as usize] + dp[(j - coins[i]) as usize];
        }
    }
    dp[amount as usize]
}
#[cfg(test)]
mod misc_fn_tests {
    use super::*;
    #[test]
    fn coin_change_test() {
        let coins = vec![1, 2, 5];
        assert_eq!(coin_change(coins, 11), 3);
    }
    #[test]
    fn change_test() {
        let coins = vec![1, 2, 5];
        assert_eq!(change(5, coins), 4);
    }
}
