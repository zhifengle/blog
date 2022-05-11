#![allow(unused_imports)]
#![allow(dead_code)]

pub struct Solution {}

// problem: https://leetcode.com/problems/valid-parentheses/
// discuss: https://leetcode.com/problems/valid-parentheses/discuss/?currentPage=1&orderBy=most_votes&query=

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();
        for c in s.chars() {
            match stack.last() {
                None => {}
                Some(&last) => {
                    if Solution::pair(last, c) {
                        stack.pop();
                        continue;
                    }
                }
            }
            stack.push(c);
        }

        stack.is_empty()
    }
    #[inline(always)]
    fn pair(open: char, close: char) -> bool {
        (open == '{' && close == '}')
            || (open == '(' && close == ')')
            || (open == '[' && close == ']')
    }
}

#[test]
fn test_20() {
    assert_eq!(Solution::is_valid("()[]{}".to_string()), true);
    assert_eq!(Solution::is_valid("([)]".to_string()), false);
}
