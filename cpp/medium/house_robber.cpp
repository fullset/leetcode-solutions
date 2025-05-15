// https://leetcode.com/problems/house-robber/

#include <cassert>
#include <vector>

using namespace std;

class Solution {
    public:
        int rob(vector<int>& nums) {
            // It is guaranteed that nums is not empty
            
            if (nums.size() == 1)
                return nums[0];
            
            if (nums.size() == 2)
                return std::max(nums[0], nums[1]);
            
            if (nums.size() == 3)
                return std::max(nums[0] + nums[2], nums[1]);

            std::vector<int> mapped_robs(nums.size(), -1);
            auto rob_cached = [&mapped_robs](auto self, std::vector<int>::const_iterator it, int size, int i) {
                if(mapped_robs[i] != -1)
                    return mapped_robs[i];

                if (size == 1){
                    mapped_robs[i] = *it;
                    return *it;
                }
                
                if (size == 2) {
                    mapped_robs[i] = std::max(*it, *(it + 1));
                    return std::max(*it, *(it + 1));
                }
                
                if (size == 3){
                    mapped_robs[i] = std::max(*it + *(it + 2), *(it + 1));
                    return std::max(*it + *(it + 2), *(it + 1));
                }

                int sum1 = *it + self(self, it + 2, size - 2,  i + 2);
                int sum2 = *(it + 1) + self(self, it + 3, size - 3, i + 3);

                mapped_robs[i] = std::max(sum1, sum2);

                return std::max(sum1, sum2);
                
            };
            
            int sum1 = nums[0] + rob_cached(rob_cached, nums.begin() + 2, nums.size() - 2, 2);
            int sum2 = nums[1] + rob_cached(rob_cached, nums.begin() + 3, nums.size() - 3, 3);

            return std::max(sum1, sum2);
        }
    };

int main() {
    Solution s;

    std::vector<int> vec1{2,7,9,3,1};
    std::vector<int> vec2{1,2,3,1};
    std::vector<int> vec3{2,7,2,3,9};
    std::vector<int> vec4{2,1,1,2};

    assert(s.rob(vec1) == 12);
    assert(s.rob(vec2) == 4);
    assert(s.rob(vec3) == 16);
    assert(s.rob(vec4) == 4);

    return 0;
}