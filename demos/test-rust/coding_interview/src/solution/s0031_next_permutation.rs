pub struct Solution {}

// problem: https://leetcode.com/problems/next-permutation/
// discuss: https://leetcode.com/problems/next-permutation/discuss/?currentPage=1&orderBy=most_votes&query=rust

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let len = nums.len();
        let mut i = len - 1;
        let mut j = len - 1;
        while i > 0 && nums[i - 1] >= nums[i] {
            i -= 1;
        }
        if i > 0 {
            while j >= i && nums[j] <= nums[i - 1] {
                j -= 1;
            }
            nums.swap(i - 1, j);
        }
        nums[i..len].reverse();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_31() {
        let mut vec1 = vec![1, 2, 3, 4, 5];
        Solution::next_permutation(&mut vec1);
        assert_eq!(vec1, vec![1, 2, 3, 5, 4]);

        let mut vec2 = vec![5, 4, 3, 2, 1];
        Solution::next_permutation(&mut vec2);
        assert_eq!(vec2, vec![1, 2, 3, 4, 5]);
    }
}
