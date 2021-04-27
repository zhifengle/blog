#include <iostream>
#include <unordered_map>
#include <vector>

using namespace std;

vector<int> twoSum(vector<int>& nums, int target) {
    unordered_map<int, int> m;
    vector<int> result;
    for (int i = 0; i < nums.size(); i++) {
        if (m.find(target - nums[i]) == m.end()) {
            m[nums[i]] = i;
        } else {
            result.push_back(m[target - nums[i]]);
            result.push_back(i);
        }
    }
    return result;
}
// ans.count(t) unordered_map 不允许有重复，因此 count 为 1 说明存在
vector<int> twoSum2(vector<int>& nums, int target) {
    unordered_map<int, int> ans;
    for (int i = 0; i < nums.size(); i++) {
        int t = target - nums[i];
        if (ans.count(t))
            return {ans[t], i};
        ans[nums[i]] = i;
    }
    return {};
}

// C++ 可以 "pass by reference"
void swap(int& a, int& b) {
    int tmp;
    tmp = a;
    a = b;
    b = tmp;
}

int main() {
    vector<int> aa = {2, 7, 11, 15};
    vector<int> res = twoSum(aa, 9);
    for (auto i : res) {
        cout << "i: " << i << endl;
    }
    return 0;
}