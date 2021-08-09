package leetcode;

import java.util.HashMap;

// 3.Longest Substring Without Repeating Characters
public class S0003 {
    public int lengthOfLongestSubstring(String s) {
        int left = 0, right = 0, count = 0;
        HashMap<Character, Integer> m = new HashMap<>();
        while (right < s.length()) {
            char c = s.charAt(right);
            if (m.containsKey(c)) {
                left = Math.max(left, m.get(c) + 1);
            }
            count = Math.max(count, right - left + 1);
            m.put(c, right++);
        }
        return count;
    }
    public int lengthOfLongestSubstring2(String s) {
        HashMap<Character, Integer> m = new HashMap<>();
        int maxLen = 0;
        int lastRepeatPos = -1;
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
}
