//https://leetcode.com/problems/remove-element/

#include <algorithm>
#include <cassert>
#include <iostream>
#include <vector>

using namespace std;

class Solution {
    public:
        int removeElement(vector<int>& nums, int val) {
            auto it1 = nums.begin();
            auto it2 = nums.begin();
            int removed = 0;

            while (it2 != nums.end()) {
                while (it2 != nums.end() && *it1 != val){
                    it1++;
                    it2++;
                }
                while (it2 != nums.end() && *it2 == val) {
                    it2++;
                    removed++;
                }
                if (it2 == nums.end())
                    break;
                std::iter_swap(it1, it2);
                it2++;
                it1++;
            }

            return nums.size() - removed;
        }
    };

int main() {

    Solution s;
    vector<int> vec1 = {3, 2, 2, 3};
    vector<int> vec2 = {2, 2};

    vector<int> vec3 = {0,1,2,2,3,0,4,2};
    vector<int> vec4 = {0,1,3,0,4};

    assert(s.removeElement(vec1, 3) == 2);
    assert(std::equal(vec1.begin(), vec1.begin() + 2, vec2.begin()));

    assert(s.removeElement(vec3, 2) == 5);
    assert(std::equal(vec3.begin(), vec3.begin() + 5, vec4.begin()));    

    assert(s.removeElement(vec2, 2) == 0);
    
    return 0;
}
