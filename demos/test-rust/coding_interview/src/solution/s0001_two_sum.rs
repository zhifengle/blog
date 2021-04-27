#![allow(dead_code)]
pub struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut mp: HashMap<i32, i32> = HashMap::with_capacity(nums.len());

        for (idx, num) in nums.iter().enumerate() {
            if let Some(&n) = mp.get(&(target - num)) {
                return vec![n, idx as i32];
            }
            mp.insert(*num, idx as i32);
        }

        vec![]
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![0, 1], Solution::two_sum(vec![2, 7, 11, 15], 9));
        assert_eq!(vec![1, 2], Solution::two_sum(vec![3, 2, 4], 6));
    }
}
