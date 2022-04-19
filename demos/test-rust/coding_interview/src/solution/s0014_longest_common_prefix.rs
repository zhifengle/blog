pub struct Solution {}

// problem: https://leetcode.com/problems/longest-common-prefix/

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return "".to_string();
        }
        strs.iter().skip(1).fold(strs[0].clone(), |acc, next| {
            acc.chars()
                .zip(next.chars())
                .take_while(|(x, y)| x == y)
                .map(|v| v.0)
                .collect()
        })
    }
    pub fn longest_common_prefix2(strs: Vec<String>) -> String {
        strs.iter().skip(1).fold(strs[0].clone(), |acc, next| {
            let mut i = 0;
            while i < acc.len() && acc.chars().nth(i) == next.chars().nth(i) {
                i += 1;
            }
            acc.chars().take(i).collect()
        })
    }
}

#[test]
fn test_14() {
    assert_eq!(
        Solution::longest_common_prefix2(vec![
            "".to_string(),
            "racecar".to_string(),
            "car".to_string()
        ]),
        ""
    );
    assert_eq!(
        Solution::longest_common_prefix2(vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string()
        ]),
        "fl"
    );
    assert_eq!(Solution::longest_common_prefix(vec![]), "");
    assert_eq!(
        Solution::longest_common_prefix2(vec!["".to_string(), "".to_string()]),
        ""
    );
    assert_eq!(
        Solution::longest_common_prefix2(vec![
            "flower".to_string(),
            "flower".to_string(),
            "flower".to_string(),
            "flower".to_string()
        ]),
        "flower"
    );
}
