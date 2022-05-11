#![allow(unused_imports)]
#![allow(dead_code)]

pub struct Solution {}

// problem: https://leetcode.com/problems/4sum/

impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec![];
        let mut nums = nums;
        nums.sort();
        let len = nums.len();
        for i in 0..len {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            let a = nums[i];
            for j in i + 1..len {
                if j > i + 1 && nums[j] == nums[j - 1] {
                    continue;
                }
                let mut lo = j + 1;
                let mut hi = len - 1;
                while lo < hi {
                    let sum = a + nums[j] + nums[lo] + nums[hi];
                    if sum < target {
                        lo += 1;
                    } else if sum > target {
                        hi -= 1;
                    } else {
                        res.push(vec![a, nums[j], nums[lo], nums[hi]]);
                        lo += 1;
                        while lo < hi && nums[lo] == nums[lo - 1] {
                            lo += 1;
                        }
                    }
                }
            }
        }
        res
    }
}

#[test]
fn test_18() {
    assert_eq!(
        Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 0),
        vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]]
    );
}
