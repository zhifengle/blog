pub struct Solution {}

// problem: https://leetcode.com/problems/letter-combinations-of-a-phone-number/
// discuss: https://leetcode.com/problems/letter-combinations-of-a-phone-number/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::collections::HashMap;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        // '0' and '1' as placeholder to avoid index shifting
        let table: HashMap<_, _> = vec![
            ('0', vec![]),
            ('1', vec![]),
            ('2', vec!['a', 'b', 'c']),
            ('3', vec!['d', 'e', 'f']),
            ('4', vec!['g', 'h', 'i']),
            ('5', vec!['j', 'k', 'l']),
            ('6', vec!['m', 'n', 'o']),
            ('7', vec!['p', 'q', 'r', 's']),
            ('8', vec!['t', 'u', 'v']),
            ('9', vec!['w', 'x', 'y', 'z']),
        ]
        .into_iter()
        .collect();
        if digits.len() < 1 {
            return vec![];
        }
        let mut result: Vec<String> = vec![];
        for digit in digits.chars() {
            if result.len() == 0 {
                for c in table[&digit].iter() {
                    result.push(c.clone().to_string());
                }
                continue;
            }
            let mut tmp: Vec<String> = vec![];
            for s in result.iter() {
                for c in table[&digit].iter() {
                    let s = s.clone() + &c.to_string();
                    tmp.push(s);
                }
            }
            result = tmp;
        }

        result
    }
}

#[test]
fn test_17() {
    assert_eq!(
        Solution::letter_combinations("23".to_string()),
        ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
    );
}
