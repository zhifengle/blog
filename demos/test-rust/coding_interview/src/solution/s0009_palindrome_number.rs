#![allow(unused_imports)]
#![allow(dead_code)]

pub struct Solution {}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let mut num = x;
        let mut y = 0;
        while num != 0 {
            let m = num % 10;
            y = y * 10 + m;
            num = num / 10;
        }
        x == y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::is_palindrome(121), true);
        assert_eq!(Solution::is_palindrome(-121), false);
    }
}
