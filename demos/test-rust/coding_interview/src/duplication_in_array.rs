/// 03
use std::collections::HashMap;

// arr: &mut [i32]
#[allow(dead_code)]
fn find_repeat_number(nums: &mut Vec<isize>) -> isize {
    for i in 0..nums.len() {
        let num = nums[i] as usize;
        if nums[num] == nums[i] {
            return nums[i];
        }
        nums.swap(i, num);
    }
    -1
}

#[allow(dead_code)]
fn find_repeat_number_01(nums: &Vec<isize>) -> isize {
    let mut mp: HashMap<isize, isize> = HashMap::new();
    for i in 0..nums.len() {
        if let Some(v) = mp.get(&nums[i]) {
            return *v;
        } else {
            mp.insert(nums[i], 1);
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    // cargo test find_repeat_number_test
    #[test]
    fn find_repeat_number_test() {
        let mut arr = vec![2,1,3,1,4];
        assert_eq!(1, find_repeat_number(&mut arr));
        let arr2 = vec![2,1,3,1,4];
        assert_eq!(1, find_repeat_number_01(&arr2));
    }
}
