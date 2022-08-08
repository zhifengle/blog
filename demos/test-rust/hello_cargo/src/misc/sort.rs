#[allow(dead_code)]
pub fn bubble_sort(arr: &mut [i32]) {
    for i in 0..arr.len() {
        let mut swapped = false;
        for j in 0..arr.len() - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
}

#[allow(dead_code)]
pub fn quick_sort(arr: &mut [i32], lo: i32, hi: i32) {
    if lo < hi {
        let p = partition(arr, lo, hi);
        quick_sort(arr, lo, p - 1);
        quick_sort(arr, p + 1, hi);
    }
}

#[allow(dead_code)]
fn partition(arr: &mut [i32], lo: i32, hi: i32) -> i32 {
    let p = arr[hi as usize];
    let mut i = lo;
    for j in lo..hi + 1 {
        if arr[j as usize] < p {
            arr.swap(j as usize, i as usize);
            i += 1;
        }
    }
    arr.swap(i as usize, hi as usize);
    i
}

#[allow(dead_code)]
pub fn selection_sort(arr: &mut [i32]) {
    for i in 0..arr.len() - 1 {
        let mut min_index = i;
        for j in i + 1..arr.len() {
            if arr[j] < arr[min_index] {
                min_index = j;
            }
        }
        arr.swap(i, min_index);
    }
}
#[allow(dead_code)]
fn insert_sort(arr: &mut [i32]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j] < arr[j - 1] {
            arr.swap(j, j - 1);
            j -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn selection_sort_test() {
        let mut arr = [3, 7, 8, 5, 2, 1, 9, 5, 4];
        let sorted_arr = [1, 2, 3, 4, 5, 5, 7, 8, 9];
        selection_sort(&mut arr);
        // println!("{:?}", arr);
        // 转换为 slice 也能使用 assert_eq!
        assert_eq!(
            arr.len(),
            sorted_arr.len(),
            "Arrays don't have the same length"
        );
        assert!(arr.iter().eq(sorted_arr.iter()), "Arrays are not equal");
    }
    #[test]
    fn insert_sort_test() {
        let mut arr = [3, 7, 8, 5, 2, 1, 9, 5, 4];
        let sorted_arr = [1, 2, 3, 4, 5, 5, 7, 8, 9];
        insert_sort(&mut arr);
        assert_eq!(
            arr.len(),
            sorted_arr.len(),
            "Arrays don't have the same length"
        );
        assert!(arr.iter().eq(sorted_arr.iter()), "Arrays are not equal");
    }
}
