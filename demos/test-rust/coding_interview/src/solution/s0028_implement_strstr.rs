#![allow(unused_imports)]
#![allow(dead_code)]

pub struct Solution {}

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.is_empty() {
            return 0;
        }
        if needle.len() > haystack.len() {
            return -1;
        }
        match haystack.find(&needle) {
            Some(x) => x as i32,
            None => -1,
        }
        // str2(haystack, needle)
    }
}

fn boyer_moore(text: Vec<char>, pattern: Vec<char>) -> i32 {
    let p_len = pattern.len();
    let t_len = text.len();

    let mut i = p_len - 1;
    while i < t_len {
        let m = (0..p_len)
            .rev()
            .take_while(|&j| pattern[j] == text[i - (p_len - 1 - j)])
            .count();

        // TODO
        i += 1;
    }

    -1
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
    #[test]
    fn test_bm() {
        boyer_moore("hello".chars().collect(), "ll".chars().collect());
        // boyer_moore("llo".chars().collect(), "ll".chars().collect());
    }
}
