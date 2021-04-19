#include <iostream>
#include <unordered_map>
#include <vector>

using namespace std;

int findRepeatNumber(vector<int>& nums) {
    for (int i = 0; i < nums.size(); i++) {
        if (nums[nums[i]] == nums[i]) {
            return nums[i];
        }
        int tmp = nums[i];
        nums[i] = nums[tmp];
        nums[tmp] = tmp;
    }
    return -1;
}

int findRepeatNumber1(vector<int>& nums) {
    unordered_map<int, int> mp;
    for (int i = 0; i < nums.size(); i++) {
        if (mp.find(nums[i]) != mp.end()) {
            return nums[i];
        } else {
            mp[nums[i]]++;
        }
    }
    return -1;
}

int main() {
    cout << "jjj: " << endl;
    return 0;
}
