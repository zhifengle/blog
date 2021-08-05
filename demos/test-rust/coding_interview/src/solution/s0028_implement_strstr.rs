#![allow(unused_imports)]
#![allow(dead_code)]

pub struct Solution {}

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let mut i = 0;
        let mut j: i32 = 0;
        if needle.len() == 0 {
            return 0;
        }
        let next = Self::get_next(&needle);
        while i < haystack.len() && j < needle.len() as i32 {
            if j == -1 || haystack.chars().nth(i) == needle.chars().nth(j as usize) {
                i += 1;
                j += 1;
            } else {
                j = next[j as usize];
            }
        }
        if j == needle.len() as i32 {
            return i as i32 - j;
        }
        -1
    }
    fn get_next(pattern: &str) -> Vec<i32> {
        let mut i = 0;
        let mut j = -1;
        // let pattern: Vec<char> = pattern.chars().collect();
        // 使用 array ??
        // [0; 5]
        let len = pattern.len();
        // 边界
        if len == 0 {
            return vec![];
        }
        // 初始化为 0
        let mut next: Vec<i32> = vec![0; len];
        next[0] = -1;
        while i < len - 1 {
            if j == -1 || pattern.chars().nth(i) == pattern.chars().nth(j as usize) {
                i += 1;
                j += 1;
                next[i] = j;
            } else {
                j = next[j as usize];
            }
        }
        next
    }
}

// 参考别人的
fn str2(haystack: String, needle: String) -> i32 {
    let window_size = needle.len();
    let needle = needle.as_bytes();
    if window_size == 0 {
        return 0;
    }
    match haystack
        .as_bytes()
        .windows(window_size)
        .position(|s| s == needle)
    {
        Some(x) => x as i32,
        _ => -1,
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str() {
        assert_eq!(str2("hello".to_string(), "ll".to_string()), 2);
        assert_eq!(Solution::str_str("hello".to_string(), "ll".to_string()), 2);
        assert_eq!(
            Solution::str_str("aaaaa".to_string(), "bba".to_string()),
            -1
        );
        assert_eq!(Solution::str_str("".to_string(), "".to_string()), 0);
    }
    #[test]
    fn test_str2() {
        assert_eq!(str2("hello".to_string(), "ll".to_string()), 2);
    }
}
