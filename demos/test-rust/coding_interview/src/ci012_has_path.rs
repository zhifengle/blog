#![allow(unused_imports)]
#![allow(dead_code)]

use std::usize;

struct Solution {}

impl Solution {
    pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
        if board.is_empty() || board[0].is_empty() {
            return false;
        }
        if word.is_empty() {
            return true;
        }
        let h = board.len();
        let w = board[0].len();
        let word: Vec<char> = word.chars().collect();

        for i in 0..h {
            for j in 0..w {
                if Self::helper(&mut board, &word, i as isize, j as isize, 0) {
                    return true;
                }
            }
        }
        false
    }
    pub fn helper(board: &mut Vec<Vec<char>>, word: &[char], i: isize, j: isize, k: usize) -> bool {
        if i < 0
            || j < 0
            || i >= board.len() as isize
            || j >= board[0].len() as isize
            || board[i as usize][j as usize] != word[k]
        {
            return false;
        }
        if k == word.len() - 1 {
            return true;
        }
        board[i as usize][j as usize] = '\0';

        // rust 使用 usize 时， i - 1, i 为 0 时会导致错误
        let res = Self::helper(board, &word, i + 1, j, k + 1)
            || Self::helper(board, &word, i - 1, j, k + 1)
            || Self::helper(board, &word, i, j + 1, k + 1)
            || Self::helper(board, &word, i, j - 1, k + 1);
        board[i as usize][j as usize] = word[k];
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exist_test() {
        assert_eq!(Solution::exist(vec![vec!['a']], "a".to_owned()), true);

        assert_eq!(
            Solution::exist(
                vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E'],
                ],
                "ABCCED".to_owned()
            ),
            true
        );
        // assert_eq!(Solution::exist(board.clone(), "aa".to_string()), false);
    }
}
