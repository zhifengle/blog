#include <iostream>
#include <vector>

using namespace std;

bool findNumberIn2DArray(vector<vector<int>>& matrix, int target) {
    int m = matrix.size();
    if (m == 0) {
        return false;
    }
    int n = matrix[0].size();
    if (n == 0) {
        return false;
    }
    int row = 0;
    int col = n - 1;
    while (row < m) {
        if (matrix[row][col] == target) {
            return true;
        } else if (matrix[row][col] > target) {
            if (col == 0) {
                return false;
            }
            col--;
        } else {
            row++;
        }
    }
    return false;
}
int main() {
    // 定义一个二维数组
    // std::vector<std::vector<double>> x (m, std::vector<double>(n));

    // 初始值为 0
    // std::vector<std::vector<int>> fog(
    //     ROW_COUNT,
    //     std::vector<int>(COLUMN_COUNT));  // Defaults to zero initial value

    // 初始化值
    vector<vector<int>> matrix{{1, 4, 7, 11, 15},
                               {2, 5, 8, 12, 19},
                               {3, 6, 9, 16, 22},
                               {10, 13, 14, 17, 24},
                               {18, 21, 23, 26, 30}};
    return 0;
}