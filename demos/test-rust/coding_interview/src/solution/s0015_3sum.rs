#![allow(unused_imports)]
#![allow(dead_code)]

pub struct Solution {}

// problem: https://leetcode.com/problems/3sum/
// discuss: https://leetcode.com/problems/3sum/discuss/?currentPage=1&orderBy=most_votes&query=&tag=rust

// submission codes start here

impl Solution {
    pub fn three_sum2(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let len = nums.len();
        if len < 3 {
            return vec![];
        }
        let mut nums = nums;
        nums.sort();
        let mut result: Vec<Vec<i32>> = vec![];
        for i in 0..len - 2 {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            let a = nums[i];
            let mut lo = i + 1;
            let mut hi = len - 1;
            while lo < hi {
                let b = nums[lo];
                let c = nums[hi];
                if a + b + c == 0 {
                    result.push(vec![a, b, c]);
                    while lo < len - 1 && nums[lo] == nums[lo + 1] {
                        lo += 1;
                    }
                    while hi > 0 && nums[hi] == nums[hi - 1] {
                        hi -= 1;
                    }
                    lo += 1;
                    if hi == 0 {
                        break;
                    }
                    hi -= 1
                } else if a + b + c > 0 {
                    while hi > 0 && nums[hi] == nums[hi - 1] {
                        hi -= 1;
                    }
                    if hi == 0 {
                        break;
                    }
                    hi -= 1
                } else if a + b + c < 0 {
                    while lo < len - 1 && nums[lo] == nums[lo + 1] {
                        lo += 1;
                    }
                    lo += 1;
                }
            }
        }

        result
    }
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        nums.sort();

        for i in 0..nums.len() {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let mut left = i + 1;
            let mut right = nums.len() - 1;

            while left < right {
                let sum = nums[i] + nums[left] + nums[right];
                if sum > 0 {
                    right -= 1;
                } else if sum < 0 {
                    left += 1
                } else {
                    res.push(vec![nums[i], nums[left], nums[right]]);
                    left += 1;

                    while nums[left] == nums[left - 1] && left < right {
                        left += 1;
                    }
                }
            }
        }

        return res;
    }
}

#[test]
fn test_15() {
    assert_eq!(Solution::three_sum(vec![0, 0, 0]), vec![vec![0, 0, 0]]);
    assert_eq!(
        Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
        vec![vec![-1, -1, 2], vec![-1, 0, 1]]
    );
    assert_eq!(
        Solution::three_sum(vec![
            -7, -4, -6, 6, 4, -6, -9, -10, -7, 5, 3, -1, -5, 8, -1, -2, -8, -1, 5, -3, -5, 4, 2,
            -5, -4, 4, 7
        ]),
        vec![
            vec![-10, 2, 8],
            vec![-10, 3, 7],
            vec![-10, 4, 6],
            vec![-10, 5, 5],
            vec![-9, 2, 7],
            vec![-9, 3, 6],
            vec![-9, 4, 5],
            vec![-8, 2, 6],
            vec![-8, 3, 5],
            vec![-8, 4, 4],
            vec![-7, -1, 8],
            vec![-7, 2, 5],
            vec![-7, 3, 4],
            vec![-6, -2, 8],
            vec![-6, -1, 7],
            vec![-6, 2, 4],
            vec![-5, -3, 8],
            vec![-5, -2, 7],
            vec![-5, -1, 6],
            vec![-5, 2, 3],
            vec![-4, -4, 8],
            vec![-4, -3, 7],
            vec![-4, -2, 6],
            vec![-4, -1, 5],
            vec![-3, -2, 5],
            vec![-3, -1, 4],
            vec![-2, -1, 3],
            vec![-1, -1, 2]
        ]
    );
    assert_eq!(
        Solution::three_sum(vec![2, 0, -2, -5, -5, -3, 2, -4]),
        vec![vec![-4, 2, 2], vec![-2, 0, 2]]
    );
    let empty_vec: Vec<Vec<i32>> = vec![];
    assert_eq!(Solution::three_sum(vec![]), empty_vec);
}
