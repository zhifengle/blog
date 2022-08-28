pub struct Solution {}

// problem: https://leetcode.com/problems/container-with-most-water/

use std::cmp;

impl Solution {
    pub fn max_area2(heights: Vec<i32>) -> i32 {
        let size = heights.len();

        let mut water: i32 = 0;
        let mut lo = 0;
        let mut hi = size - 1;
        while lo < hi {
            let height = cmp::min(heights[lo], heights[hi]);
            water = cmp::max(water, (hi - lo) as i32 * height);

            while lo < hi && heights[lo] <= height {
                lo += 1
            }
            while lo < hi && heights[hi] <= height {
                hi -= 1
            }
        }
        water
    }
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut water = 0;
        let mut lo = 0;
        let mut hi = height.len() - 1;
        let mut i = 0;
        while lo < hi {
            let width = hi - lo;
            if height[lo] > height[hi] {
                i = hi;
                hi -= 1;
            } else {
                i = lo;
                lo += 1;
            }
            let cur = height[i] * width as i32;
            if cur > water {
                water = cur;
            }
        }

        water
    }
}
#[test]
fn test_11() {
    assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    assert_eq!(Solution::max_area(vec![6, 9]), 6);
    assert_eq!(Solution::max_area(vec![1, 1, 2, 1, 1, 1]), 5);
}
