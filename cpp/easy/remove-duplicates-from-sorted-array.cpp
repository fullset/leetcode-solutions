// https://leetcode.com/problems/remove-duplicates-from-sorted-array/description/

#include <algorithm>
#include <cassert>
#include <iostream>
#include <vector>

using namespace std;

class Solution {
    public:
        int removeDuplicates(vector<int>& nums) {
            auto it1 = nums.begin();
            auto it2 = nums.begin();

            uint k = 1;

            while (it2 < nums.end()){
                while (it2 < nums.end() && *it1 == *it2 )
                    it2++;
                if (it2 == nums.end())
                    break;
                it1++;
                if (it1 != it2)
                    std::iter_swap(it1, it2);
                it2++;
                k++;
            }

            return k;
        }
    };

int main() {
    auto s = Solution();
    std::vector<int> vec1 = {1, 1, 2};
    std::vector<int> vec2 = {1, 2, 1};

    assert(s.removeDuplicates(vec1) == 2);
    assert(std::equal(vec1.begin(), vec1.begin() + 2, vec2.begin()));

    std::vector<int> vec3 = {0,0,1,1,1,2,2,3,3,4};
    std::vector<int> vec4 = {0,1,2,3,4,0,2,1,3,1};

    assert(s.removeDuplicates(vec3) == 5);
    assert(std::equal(vec3.begin(), vec3.begin() + 5, vec3.begin()));

    std::vector<int> vec5 = {1};
    std::vector<int> vec6 = {1};

    assert(s.removeDuplicates(vec5) == 1);
    assert(std::equal(vec5.begin(), vec5.begin() + 1, vec6.begin()));

    return 0;
}