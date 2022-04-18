pub struct Solution {}

// problem: https://leetcode.com/problems/reverse-integer/

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut input: i64 = x as i64;
        let mut result: i64 = 0;
        let base: i64 = 2;
        let upper_bound: i64 = base.pow(31) - 1;
        let lower_bound: i64 = -base.pow(31);
        while input != 0 {
            let digit = input % 10;
            result = result * 10 + digit;
            input = input / 10;
        }
        if result > upper_bound || result < lower_bound {
            return 0;
        }
        result as i32
    }
}

#[test]
fn test_7() {
    assert_eq!(Solution::reverse(123), 321);
    assert_eq!(Solution::reverse(-123), -321);
    assert_eq!(Solution::reverse(0), 0);
    assert_eq!(Solution::reverse(-123000), -321);
    let base: i64 = 2;
    assert_eq!(Solution::reverse((base.pow(31) - 1) as i32), 0);
    assert_eq!(Solution::reverse((-base.pow(31)) as i32), 0);
}
