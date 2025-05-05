#include <vector>
#include <unordered_map>
#include <cassert>

using namespace std;

class Solution {
    public:
        vector<int> intersect(vector<int>& nums1, vector<int>& nums2) {
            std::unordered_map<int, int> m1;
            std::vector<int> result;
            result.reserve(std::min(nums1.size(), nums2.size()));
    
            for (auto it = nums1.begin(); it != nums1.end(); it++) {
                ++m1[*it];
            }
    
            for (auto it = nums2.begin(); it != nums2.end(); it++) {
                auto val = m1[*it]--;
                if (val > 0) {
                    result.push_back(*it);
                }
            }
    
            return result;
        }
    };

int main() {
    auto s = Solution();

    std::vector<int> vec1 = {1, 4, 5, 6, 7, 15, 3, 2};
    std::vector<int> vec2 = {1, 2, 55, 12, 20};
    
    std::vector<int> vec3 = {1, 2};

    assert(s.intersect(vec1, vec2) == vec3);

    return 0;
}