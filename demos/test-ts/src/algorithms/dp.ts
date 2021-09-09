export function minDistance(word1: string, word2: string): number {
  const len1 = word1.length;
  const len2 = word2.length;
  const dp: number[][] = Array.from(Array(len1 + 1), (v, k) => {
    if (k === 0) {
      return Array.from(Array(len2 + 1), (v, idx) => idx);
    }
    return [k, ...Array(len2).fill(0)];
  });

  for (let i = 1; i <= len1; i++) {
    for (let j = 1; j <= len2; j++) {
      if (word1[i - 1] === word2[j - 1]) {
        dp[i][j] = dp[i - 1][j - 1];
      } else {
        dp[i][j] =
          (Math.min(dp[i][j - 1], dp[i - 1][j], dp[i - 1][j - 1]) || 0) + 1;
      }
    }
  }
  return dp[len1][len2];
}

// https://en.wikibooks.org/wiki/Algorithm_Implementation/Strings/Levenshtein_distance#C
export function levenshtein(word1: string, word2: string): number {
  const len1 = word1.length;
  const len2 = word2.length;
  const dp: number[] = Array.from(Array(len1 + 1), (v, k) => k);
  let old = 0;
  for (let j = 1; j <= len2; j++) {
    dp[0] = j;
    for (let i = 1, last = j - 1; i <= len1; i++) {
      old = dp[i];
      dp[i] = Math.min(
        dp[i] + 1,
        dp[i - 1] + 1,
        last + (word1[i - 1] === word2[j - 1] ? 0 : 1)
      );
      last = old;
    }
  }
  return dp[len1];
}

export function rob(nums: number[]): number {
  const dp = [0, 0];
  for (let i = 2; i < nums.length + 2; i++) {
    dp[i] = Math.max(dp[i - 2] + nums[i - 2], dp[i - 1]);
  }
  return dp[nums.length + 1];
}

export function rob2(nums: number[]): number {
  let a = 0;
  let b = 0;
  for (let i = 0; i < nums.length; i++) {
    const temp = b;
    b = Math.max(a + nums[i], b);
    a = temp;
  }
  return b;
}

export function coinChange(coins: number[], amount: number): number {
  if (amount === 0) return 0;
  const dp: number[] = Array(amount + 1).fill(amount + 1);
  dp[0] = 0;
  for (let i = 0; i < dp.length; i++) {
    for (let coin of coins) {
      if (i - coin < 0) continue;
      dp[i] = Math.min(dp[i], 1 + dp[i - coin]);
    }
  }
  return dp[amount] == amount + 1 ? -1 : dp[amount];
}
