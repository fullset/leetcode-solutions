// https://leetcode.com/problems/longest-common-prefix/

#include <algorithm>
#include <string>
#include <vector>

using namespace std;

class Solution {
    public:
        string longestCommonPrefix(vector<string>& strs) {
    
            int left = 0;
            int right = strs[0].length();
            int current = right;
            bool not_found = false;
    
            while (left < right) {
                auto substring = strs[0].substr(0, current);
                for (auto s: strs){
                    if (s.find(substring) != 0){
                        not_found = true;
                        break;   
                    }
                }
    
                if (not_found)
                    right = current - 1;
                else
                    left = current;
                current = (left + right + 1) / 2;
                not_found = false;
            }
    
            return strs[0].substr(0, current);     
        }
    };

int main() {
    return 0;
}