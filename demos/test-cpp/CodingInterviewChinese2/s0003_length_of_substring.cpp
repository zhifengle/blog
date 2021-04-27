#include <iostream>
#include <map>

using namespace std;
int lengthOfLongestSubstring(string s) {
    int maxLen = 0;
    int lastRepeatPos = -1;
    map<char, int> m;
    for (int i = 0; i < s.size(); i++) {
        char c = s[i];
        if (m.find(c) != m.end() && lastRepeatPos < m[c]) {
            lastRepeatPos = m[c];
        }
        if (i - lastRepeatPos > maxLen) {
            maxLen = i - lastRepeatPos;
        }
        m[c] = i;
    }
    return maxLen;
}
int main() {
    return 0;
}