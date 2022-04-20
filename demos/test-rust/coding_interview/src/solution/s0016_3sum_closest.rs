pub struct Solution {}

// problem: https://leetcode.com/problems/3sum-closest/
// discuss: https://leetcode.com/problems/3sum-closest/discuss/?currentPage=1&orderBy=most_votes&query=rust

// submission codes start here

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        if nums.len() <= 3 {
            return nums.into_iter().sum();
        }

        let mut nums = nums;
        nums.sort();
        let mut closest = nums[0] + nums[1] + nums[2];
        let mut min_distance = (target - closest).abs();
        for i in 0..nums.len() {
            let mut lo = i + 1;
            let mut hi = nums.len() - 1;
            while lo < hi {
                let sum = nums[i] + nums[lo] + nums[hi];
                let distance = (sum - target).abs();
                if distance < min_distance {
                    closest = sum;
                    min_distance = distance
                }
                if sum < target {
                    lo += 1;
                } else if sum > target {
                    hi -= 1;
                } else {
                    return target;
                }
            }
        }

        closest
    }
}

#[test]
fn test_16() {
    assert_eq!(Solution::three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
    assert_eq!(Solution::three_sum_closest(vec![1, 2, 3], 1), 6);
    assert_eq!(
        Solution::three_sum_closest(vec![1, 2, 4, 8, 16, 32, 64, 128], 82),
        82
    );
}
