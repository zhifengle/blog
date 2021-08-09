import java.util.Arrays;
import java.util.HashMap;
import java.util.Map;

class CoinChange {
    public static void main(String[] args) {
        Solution solution = new Solution();
        int[] coins = {1, 2, 5};
//        System.out.println(solution.coinChange(coins,11));
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

