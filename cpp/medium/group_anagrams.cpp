#include <algorithm>
#include <cassert>
#include <iostream>
#include <unordered_map>
#include <string>
#include <vector>

using namespace std;

class Solution {
    public:
        vector<vector<string>> groupAnagrams(vector<string>& strs) {
            std::unordered_map<std::string, std::vector<std::string>> anagrams;
            for (auto str: strs) {
                auto copy = std::string(str);
                std::sort(copy.begin(),copy.end());
                if (anagrams.find(copy) != anagrams.end())
                    anagrams[copy].push_back(str);
                else
                    anagrams[copy] = std::vector<std::string>({str});
            }
            
            std::vector<std::vector<std::string>> result;
            result.reserve(anagrams.size());
            for (auto it = anagrams.begin(); it != anagrams.end(); it++) {
                result.push_back(it->second);
            }

            return result;
        }
    };

int main() {

    Solution s;

    std::vector<std::string> vec = {"eat","tea","tan","ate","nat","bat"};

    // std::vector<std::vector<std::string>> vec2 = {{"bat"},{"nat","tan"},{"ate","eat","tea"}};
    std::vector<std::vector<std::string>> vec2 = s.groupAnagrams(vec);

    for (auto v: vec2){
        for (auto s: v){
            std::cout << s << " ";
        }
        std::cout << std::endl;
    }

    return 0;
}