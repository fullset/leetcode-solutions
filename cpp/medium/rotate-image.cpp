#include <algorithm>
#include <cassert>
#include <vector>


using namespace std;

class Solution {
public:
    void rotate(vector<vector<int>>& matrix) {
        int n = matrix.size();
        int last = n - 1;
        for (int i = 0; i < n / 2; i++){
            for (int j = i; j < n - i - 1; j++) {
                // i = 0; j = 1; n = 3; last = 2
                std::swap(matrix[i][j], matrix[j][last-i]);
                std::swap(matrix[last-j][i], matrix[i][j]);
                std::swap(matrix[last-i][last-j], matrix[last-j][i]);
            }
        }
    }
};

int main() {

    auto row1 = {1,2,3};
    auto row2 = {4,5,6};
    auto row3 = {7,8,9};

    auto res_row1 = {7,4,1};
    auto res_row2 = {8,5,2};
    auto res_row3 = {9,6,3};

    Solution s;

    vector<vector<int>> vec = {row1, row2, row3};
    vector<vector<int>> result = {res_row1, res_row2, res_row3};

    s.rotate(vec);

    assert(std::equal(vec.begin(), vec.end(), result.begin()));

    return 0;
}