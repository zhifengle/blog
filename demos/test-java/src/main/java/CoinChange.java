import java.util.Arrays;
import java.util.HashMap;
import java.util.Map;

class CoinChange {
    public static void main(String[] args) {
        Solution solution = new Solution();
        int[] coins = {1, 2, 5};
//        System.out.println(solution.coinChange(coins,11));
        System.out.println(solution.lengthOfLongestSubstring("abcabcbb"));
        int[] arr = {4, 1, 8, 2};
        solution.bubbleSort(arr);
        System.out.println(Arrays.toString(arr));
    }
}

class Solution {
    public int coinChange(int[] coins, int amount) {
        if (coins == null || coins.length == 0) {
            return -1;
        }
        if (amount < 0) return 0;
        int[] dp = new int[amount + 1];
        for (int i = 0; i < dp.length; i++) {
            dp[i] = amount + 1;
        }
        dp[0] = 0;
        for (int i = 0; i < dp.length; i++) {
            for (int coin : coins) {
                if (i - coin < 0) continue;
                dp[i] = Math.min(dp[i], dp[i - coin] + 1);
            }
        }
        return dp[amount] == amount + 1 ? -1 : dp[amount];
    }

    public int lengthOfLongestSubstring(String s) {
        int maxLen = 0;
        int lastRepeatPos = -1;
        Map<Character, Integer> m = new HashMap<>();
        for (int i = 0; i < s.length(); i++) {
            char c = s.charAt(i);
            if (m.containsKey(c) && m.get(c) > lastRepeatPos) {
                lastRepeatPos = m.get(c);
            }
            if (i - lastRepeatPos > maxLen) {
                maxLen = i - lastRepeatPos;
            }
            m.put(c, i);
        }
        return maxLen;
    }
    public int lengthOfLongestSubstring2(String s) {
        int left = 0, right = 0, count = 0;
        Map<Character, Integer> m = new HashMap<>();
        while (right < s.length()) {
            char c = s.charAt(right);
            if (m.containsKey(c)) {
                left = Math.max(left, m.get(c) + 1);
            }
            count = Math.max(count, right - left +1);
            m.put(c, right++);
        }
        return count;
    }
    public void bubbleSort(int[] arr) {
        for(int i = 0; i < arr.length - 1; i++) {
            for (int j = 0; j < arr.length - i - 1; j++) {
                if (arr[j] > arr[j+1]) {
                    int temp = arr[j];
                    arr[j] = arr[j+1];
                    arr[j+1] = temp;
                }
            }
        }
    }
}

