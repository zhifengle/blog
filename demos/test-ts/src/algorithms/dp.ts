export function minDistance(word1: string, word2: string): number {
  const dp: number[][] = Array.from({length: word1.length + 1}, () =>
    Array.from({length: word2.length + 1})
  );
  for (let i = 1; i < word2.length + 1; i++) {
    dp[0][i] = (dp[0][i - 1] || 0) + 1;
  }
  for (let j = 1; j < word1.length + 1; j++) {
    dp[j][0] = (dp[j - 1][0] || 0) + 1;
  }
  for (let i = 1; i < word1.length + 1; i++) {
    for (let j = 1; j < word2.length + 1; j++) {
      if (word1[i - 1] === word2[j - 1]) {
        dp[i][j] = dp[i - 1][j - 1];
      } else {
        dp[i][j] = (Math.min(dp[i][j - 1], dp[i - 1][j], dp[i - 1][j - 1]) || 0) + 1;
      }
    }
  }
  return dp[word1.length][word2.length];
}
