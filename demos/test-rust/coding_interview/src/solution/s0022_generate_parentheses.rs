pub struct Solution {}

// problem: https://leetcode.com/problems/generate-parentheses/
// discuss: https://leetcode.com/problems/generate-parentheses/discuss/?currentPage=1&orderBy=most_votes&query=rust

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        if n < 1 {
            return vec![];
        }
        let mut result = Vec::new();
        Solution::dfs(n, 0, 0, &mut result, String::new());
        result
    }
    fn dfs(n: i32, left: i32, right: i32, result: &mut Vec<String>, mut path: String) {
        if left == n && right == n {
            result.push(path);
            return;
        }
        if left < n {
            Self::dfs(n, left + 1, right, result, path.clone() + "(");
        }
        if right < left {
            path.push(')');
            Self::dfs(n, left, right + 1, result, path);
        }
    }
    // Self::generate(n, 0, "".to_string())
    fn generate(left: i32, right: i32, s: String) -> Vec<String> {
        let mut res = vec![];
        if left == 0 && right == 0 {
            return vec![s];
        }
        if left > 0 {
            res.append(&mut Self::generate(left - 1, right + 1, s.clone() + "("));
        }
        if right > 0 {
            res.append(&mut Self::generate(left, right - 1, s.clone() + ")"));
        }
        res
    }
}

#[test]
fn test_22() {
    assert_eq!(Solution::generate_parenthesis(1), vec!["()"]);
    assert_eq!(Solution::generate_parenthesis(2), vec!["(())", "()()"]);
    assert_eq!(
        Solution::generate_parenthesis(3),
        vec!["((()))", "(()())", "(())()", "()(())", "()()()"]
    );
}
