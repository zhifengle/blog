// https://leetcode-cn.com/problems/edit-distance/solution/edit-distance-by-ikaruga/
pub fn min_distance(word1: String, word2: String) -> i32 {
    let len1 = word1.len();
    let len2 = word2.len();
    let mut dp = vec![vec![0; len2 + 1]; len1 + 1];
    for i in 0..len1 {
        dp[i][0] = i;
    }
    for j in 0..len2 {
        dp[0][j] = j;
    }
    let word1 = word1.as_bytes();
    let word2 = word2.as_bytes();
    for i in 1..=len1 {
        for j in 1..=len2 {
            if word1[i - 1] == word2[j - 1] {
                dp[i][j] = dp[i - 1][j - 1]
            } else {
                dp[i][j] = vec![dp[i][j - 1], dp[i - 1][j], dp[i - 1][j - 1]]
                    .into_iter()
                    .min()
                    .unwrap()
                    + 1;
            }
        }
    }
    dp[len1][len2] as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(min_distance("horse".to_string(), "ros".to_string()), 3);
        assert_eq!(
            min_distance("intention".to_string(), "execution".to_string()),
            5
        );
    }
}
