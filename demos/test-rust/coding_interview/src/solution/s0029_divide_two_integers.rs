pub struct Solution {}

// problem: https://leetcode.com/problems/divide-two-integers/
// discuss: https://leetcode.com/problems/divide-two-integers/discuss/?currentPage=1&orderBy=most_votes&query=rust

// submission codes start here

impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        if dividend == divisor {
            return 1;
        }
        if divisor == i32::MIN {
            return 0;
        }
        if divisor == 1 {
            return dividend;
        }
        if dividend == 0 || divisor == 0 {
            return 0;
        }
        let mut dividend = dividend;
        let mut res = 0;
        if dividend == i32::MIN {
            if divisor == -1 {
                return i32::MAX;
            }
            // 判断了最小值，下面的 abs 不会溢出
            if divisor == i32::MIN {
                return 1;
            }
            // dividend.abs(); 为最小值时会溢出
            dividend += divisor.abs();
            res += 1;
        }
        let is_neg = (dividend > 0) ^ (divisor > 0);
        let mut dividend = dividend.abs();
        let divisor = divisor.abs();

        while dividend >= divisor {
            let mut sub = divisor;
            let mut count = 1;
            while dividend - sub >= sub {
                sub += sub;
                count += count;
            }
            dividend -= sub;
            res += count;
        }

        if is_neg {
            -res
        } else {
            res
        }
    }
    pub fn divide2(dividend: i32, divisor: i32) -> i32 {
        if divisor == 1 {
            return dividend;
        }
        let is_neg = (dividend > 0) ^ (divisor > 0);
        // 改成相反符号
        let mut dividend = if dividend > 0 { -dividend } else { dividend };
        let divisor = if divisor > 0 { -divisor } else { divisor };

        let mut res = 0;

        for shift in (0..divisor.leading_ones()).rev() {
            if dividend <= (divisor << shift) {
                dividend -= divisor << shift;
                res += -1 << shift;
            }
        }

        if is_neg {
            res
        } else if res == i32::MIN {
            i32::MAX
        } else {
            -res
        }
    }
}

#[test]
fn test_29() {
    assert_eq!(Solution::divide(9, 3), 3);
    assert_eq!(Solution::divide(7, -2), -3);
    assert_eq!(Solution::divide(1, 2), 0);
    assert_eq!(Solution::divide(-7, 2), -3);
    assert_eq!(Solution::divide(i32::MIN, -1), i32::MAX);
    assert_eq!(Solution::divide(i32::MIN, i32::MIN), 1);
    assert_eq!(Solution::divide(i32::MIN, 2), -1073741824);
    assert_eq!(Solution::divide(i32::MIN, 1), i32::MIN);
    assert_eq!(Solution::divide(i32::MAX, 2), 1073741823);
}
