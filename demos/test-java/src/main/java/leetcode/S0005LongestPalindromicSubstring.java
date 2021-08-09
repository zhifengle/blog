package leetcode;

public class S0005LongestPalindromicSubstring {
    public static void main(String[] args) {
        S0005LongestPalindromicSubstring p = new S0005LongestPalindromicSubstring();
        String s = p.longestPalindrome("babad");
    }

    public String longestPalindrome(String s) {
        int len = s.length();
        if (len < 2) {
            return s;
        }
        String str = "";
        String longest = "";
        for (int i = 0; i < len - 1; i++) {
            str = find(s, i, i);
            if (str.length() > longest.length()) {
                longest = str;
            }
            str = find(s, i, i + 1);
            if (str.length() > longest.length()) {
                longest = str;
            }
        }
        return longest;
    }

    public String find(String s, int lo, int hi) {
        int len = s.length();
        while (lo >= 0 && hi < len && s.charAt(lo) == s.charAt(hi)) {
            lo--;
            hi++;
        }
        return s.substring(lo + 1, hi);
    }
}
