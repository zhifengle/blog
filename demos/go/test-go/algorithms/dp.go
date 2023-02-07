package algorithms

import "fmt"

func init() {
	fmt.Println(MinDistance("horse", "ros"))
	fmt.Println(CoinCount(100))
	fmt.Println("end of dp\n")
}

func MinDistance(word1, word2 string) int {
	len1 := len(word1)
	len2 := len(word2)

	dp := make([][]int, len1+1)
	for i := 0; i < len1+1; i++ {
		dp[i] = make([]int, len2+1)
		dp[i][0] = i
	}
	for j := 0; j < len2+1; j++ {
		dp[0][j] = j
	}

	for i := 1; i < len1+1; i++ {
		for j := 1; j < len2+1; j++ {
			if word1[i-1] == word2[j-1] {
				dp[i][j] = dp[i-1][j-1]
			} else {
				min := dp[i-1][j-1]   //替换
				if dp[i-1][j] < min { //删除操作,要求的是i到j的位置最短操作，这里是i-1到j，自然是多了一位，之后会删除一位，所以对应的是删除操作
					min = dp[i-1][j]
				}
				if dp[i][j-1] < min { //插入操作
					min = dp[i][j-1]
				}
				dp[i][j] = min + 1
			}
		}
	}
	return dp[len1][len2]
}

func CoinCount(amount int) int {
	coins := [...]int{1, 5, 10, 25, 50}
	dp := make([]int, amount+1)
	dp[0] = 1

	for i := 0; i < len(coins); i++ {
		for j := coins[i]; j < amount+1; j++ {
			dp[j] = dp[j] + dp[j-coins[i]]
		}
	}
	return dp[amount]
}
