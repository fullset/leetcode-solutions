// https://leetcode.com/problems/top-k-frequent-elements/

#include <algorithm>
#include <iostream>
#include <list>
#include <vector>
#include <unordered_map>

using namespace std;

class Solution {
    public:
        vector<int> topKFrequent(vector<int>& nums, int k) {
            // num | counter
            std::unordered_map<int, int> nums_count;
            std::vector<int> unique;
            unique.reserve(nums.size());

            for (auto item: nums) {
                if (++nums_count[item] <= 1)
                    unique.push_back(item);
            }

            auto comparator = [&nums_count](auto val1, auto val2) {return nums_count[val1] < nums_count[val2];};

            std::make_heap(unique.begin(), unique.end(), comparator);
            // It is guaranteed that there are no less unique items than k
            std::vector<int> result;
            result.reserve(k);

            for (int i = 0; i < k; i++) {
                std::pop_heap(unique.begin(), unique.end(), comparator);
                result.push_back(unique.back());
                unique.pop_back();
            }

            return result;
        }
    };



int main() {

    std::vector<int> vec1 = {1,1,1,2,2,3};

    Solution s;

    auto result = s.topKFrequent(vec1, 2);

    for (int item: result)
        std::cout << item << std::endl;

    return 0;
}