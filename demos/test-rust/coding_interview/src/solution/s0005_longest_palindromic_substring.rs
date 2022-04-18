#![allow(unused_imports)]
#![allow(dead_code)]

use std::collections::HashMap;

struct Solution {}
impl Solution {
    pub fn longest_palindrome(_s: String) -> String {
        todo!()
    }
}

fn window_solution(s: String) -> String {
    let mut window_size = s.len();
    while window_size > 0 {
        match s.as_bytes().windows(window_size).find(|s| {
            let iter = s.iter();
            iter.clone().eq(iter.clone().rev())
        }) {
            Some(s) => return String::from_utf8(s.to_vec()).unwrap_or("".to_string()),
            None => window_size -= 1,
        }
    }
    "".to_string()
}

fn center_solution(s: String) -> String {
    let mut longest: Vec<char> = vec![];
    let s: Vec<char> = s.chars().collect::<Vec<char>>();
    for i in 0..s.len() {
        find(&s, i, i, &mut longest);
        find(&s, i, i + 1, &mut longest);
    }
    longest.into_iter().collect()
}

fn find(s: &Vec<char>, left: usize, right: usize, longest: &mut Vec<char>) -> Vec<char> {
    let len = s.len();
    let mut left = left;
    let mut right = right;
    let mut sub: &[char] = &[];
    while left != usize::MAX && right < len && s[left] == s[right] {
        sub = &s[left..right + 1];
        left -= 1;
        right += 1;
    }
    if sub.len() > longest.len() {
        return sub.to_vec();
    }
    longest.to_vec()
}

pub fn longest_palindrome(s: String) -> String {
    let (s, mut max) = (s.chars().collect::<Vec<char>>(), vec![]);
    fn find_max(s: &Vec<char>, max: Vec<char>, i: usize, j: usize) -> Vec<char> {
        let (mut i, mut j) = (i, j);
        let mut sub: &[char] = &[];
        while i != std::usize::MAX && j < s.len() && s[i] == s[j] {
            sub = &s[i..j + 1];
            i -= 1;
            j += 1;
        }
        if sub.len() > max.len() {
            return sub.to_vec();
        }
        max.to_vec()
    }
    for i in 0..s.len() {
        max = find_max(&s, max, i, i);
        max = find_max(&s, max, i, i + 1);
    }
    max.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(longest_palindrome("babad".to_string()), "bab".to_string());
    }
}
