use std::collections::HashMap;

pub struct Solution {}

// problem: https://leetcode.com/problems/substring-with-concatenation-of-all-words/
// discuss: https://leetcode.com/problems/substring-with-concatenation-of-all-words/discuss/?currentPage=1&orderBy=most_votes&query=rust
impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        if words.len() < 1 {
            return result;
        }
        let len = words[0].len();
        let n = words.len();
        if len < 1 || n * len > s.len() {
            return result;
        }
        let mut map: HashMap<&str, usize> = HashMap::new();
        words.iter().for_each(|x| {
            *map.entry(x).or_insert(0) += 1;
        });
        let end = s.len() - n * len;
        for i in 0..=end {
            let mut sliced: Vec<&str> = vec![];
            let mut start = i;
            for _ in 0..n {
                sliced.push(&s[start..start + len]);
                start += len;
            }
            let mut counter: HashMap<&str, usize> = HashMap::new();
            for key in sliced.into_iter() {
                *counter.entry(key).or_insert(0) += 1;
            }
            if map.len() == counter.len()
                && map
                    .keys()
                    .all(|k| return counter.contains_key(k) && counter[*k] == map[*k])
            {
                result.push(i as i32);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_30() {
        assert_eq!(
            Solution::find_substring(
                "barfoothefoobarman".to_string(),
                vec!["foo".to_string(), "bar".to_string()]
            ),
            vec![0, 9]
        );
        assert_eq!(
            Solution::find_substring(
                "wordgoodgoodgoodbestword".to_string(),
                vec![
                    "word".to_string(),
                    "good".to_string(),
                    "best".to_string(),
                    "word".to_string()
                ]
            ),
            vec![]
        );
        assert_eq!(
            Solution::find_substring(
                "wordgoodgoodgoodbestword".to_string(),
                vec![
                    "word".to_string(),
                    "good".to_string(),
                    "best".to_string(),
                    "good".to_string()
                ]
            ),
            vec![8]
        );
        assert_eq!(
            Solution::find_substring(
                "xxwordgoodgoodgoodbestword".to_string(),
                vec![
                    "word".to_string(),
                    "good".to_string(),
                    "best".to_string(),
                    "good".to_string()
                ]
            ),
            vec![10]
        );
        assert_eq!(
            Solution::find_substring("a".to_string(), vec!["a".to_string(), "a".to_string(),]),
            vec![]
        );
    }
}
