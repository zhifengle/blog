#![allow(unused_imports)]
#![allow(dead_code)]

pub struct Solution {}

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        kmp_search(haystack, needle)
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
fn kmp_search(haystack: String, needle: String) -> i32 {
    let m = haystack.len();
    let n = needle.len();
    if m < n {
        return -1;
    }
    if n == 0 {
        return 0;
    }
    let haystack = haystack.as_bytes();
    let needle = needle.as_bytes();
    let mut next = vec![0; n];
    let mut j = 0;
    for i in 1..n {
        // 换成 if 也通过了
        while j > 0 && needle[i] != needle[j] {
            j = next[j - 1];
        }
        if needle[i] == needle[j] {
            j += 1;
        }
        next[i] = j;
    }
    j = 0;
    for i in 0..m {
        // 换成 if 会超时
        while j > 0 && haystack[i] != needle[j] {
            j = next[j - 1];
        }
        if haystack[i] == needle[j] {
            j += 1;
        }
        if j == n {
            return (i - n + 1) as i32;
        }
    }
    -1
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
        assert_eq!(kmp_search("hello".to_string(), "ll".to_string()), 2);
    }
}
