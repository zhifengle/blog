use std::cmp::Ord;

fn find_max_nums_idx(nums: &[i32]) -> usize {
    let mut idx = 0;
    for i in 1..nums.len() {
        if nums[idx] < nums[i] {
            idx = i;
        }
    }
    idx
}

fn find_max_idx<T: Ord>(nums: &[T]) -> usize {
    let mut idx = 0;

    for i in 1..nums.len() {
        if nums[idx] < nums[i] {
            idx = i;
        }
    }
    idx
}
fn main() {
    let v = vec![1, 2, 3, 8, 9, 11];
    assert_eq!(find_max_nums_idx(&v), 5);
    assert_eq!(find_max_idx(&v), 5);
}
