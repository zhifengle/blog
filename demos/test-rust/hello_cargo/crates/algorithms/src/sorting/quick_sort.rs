#![allow(unused_imports)]
#![allow(dead_code)]

// https://github.com/TheAlgorithms/Rust/blob/master/src/sorting/quick_sort.rs

pub fn quick_sort<T: Ord>(arr: &mut [T]) {
    // 直接写在参数里面会报错
    let len = arr.len() as isize;
    quick_sort_helper(arr, 0, len - 1);
}

fn quick_sort_helper<T: Ord>(arr: &mut [T], lo: isize, hi: isize) {
    if lo < hi {
        let p = partition(arr, lo, hi);
        quick_sort_helper(arr, lo, p - 1);
        quick_sort_helper(arr, p + 1, hi);
    }
}

fn partition<T: Ord>(arr: &mut [T], lo: isize, hi: isize) -> isize {
    let pivot = hi as usize;
    let mut i = lo - 1;
    let mut j = hi;
    loop {
        i += 1;
        while arr[i as usize] < arr[pivot] {
            i += 1;
        }
        j -= 1;
        while j >= 0 && arr[j as usize] > arr[pivot] {
            j -= 1;
        }
        if i >= j {
            break;
        } else {
            arr.swap(i as usize, j as usize);
        }
    }
    arr.swap(i as usize, pivot);
    i
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test() {
        let mut arr = vec![3, 7, 8, 5, 2, 1, 9, 5, 4];
        quick_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5, 5, 7, 8, 9]);
    }
}
