#![allow(unused_imports)]
#![allow(dead_code)]

pub struct Solution {}

impl Solution {
    pub fn min_array(numbers: Vec<i32>) -> i32 {
        let mut low = 0;
        let mut high = numbers.len() - 1;
        while low < high {
            let pivot = low + (high - low) / 2;
            if numbers[pivot] < numbers[high] {
                high = pivot;
            } else if numbers[pivot] > numbers[high] {
                low = pivot + 1;
            } else {
                high -= 1;
            }
        }

        numbers[low]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_array_test() {
        let v = vec![1, 3, 5];

        assert_eq!(Solution::min_array(v), 1)
    }
}
