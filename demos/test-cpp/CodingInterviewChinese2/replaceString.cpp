#include <iostream>
#include <string>

using namespace std;

string replaceString(string s) {
    string newS;
    for (int i = 0; i < s.size(); i++) {
        if (s[i] == ' ') {
            newS.append("%20");
        } else {
            newS.push_back(s[i]);
        }
    }
    return newS;
}

// 提交后这种写法速度和内存和上面那个差不多。
// 可能使用 char* s; 内存占用要小一些
string replaceString2(string s) {
    int count = 0, len = s.size();

    for (auto c : s) {
        if (c == ' ') {
            count++;
        }
    }
    // 改变 string 长度
    s.resize(len + 2 * count);
    int i = len - 1, j = s.size() - 1;
    // s[i] = ' '   之前少写了一个等号导致错误
    while (i >= 0 && j > i) {
        if (s[i] == ' ') {
            s[j--] = '0';
            s[j--] = '2';
            s[j--] = '%';
        } else {
            s[j--] = s[i];
        }
        i--;
    }

    // for (; i >= 0; i--) {
    //     if (s[i] != ' ') {
    //         s[j--] = s[i];
    //     } else {
    //         s[j--] = '0';
    //         s[j--] = '2';
    //         s[j--] = '%';
    //     }
    // }
    return s;
}

int main() {
    string s("We are happy.");
    // string s2 = "We are happy.";
    string newStr = replaceString2(s);
    cout << newStr << endl;
    return 0;
}
