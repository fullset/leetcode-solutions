# https://leetcode.com/problems/roman-to-integer/description/

class Solution:
    def romanToInt(self, s: str) -> int:
        m = {
            'I': 1,
            'V': 5,
            'X': 10,
            'L': 50,
            'C': 100,
            'D': 500,
            'M': 1000
        }

        result = 0;

        for i, char in enumerate(s):
            if i > 0 and m[s[i-1]] < m[char]:
                result = result - m[s[i-1]] + (m[char] - m[s[i-1]])
            else:
                result += m[char]


        return result;

if __name__ == '__main__':
    s = Solution()
    assert s.romanToInt("I") == 1
    assert s.romanToInt("V") == 5
    assert s.romanToInt("IV") == 4
    assert s.romanToInt("VI") == 6
    assert s.romanToInt("XIII") == 13
    assert s.romanToInt("XVI") == 16
    assert s.romanToInt("CXIII") == 113
    assert s.romanToInt("MXIII") == 1013
    assert s.romanToInt("CMXIII") == 913
    assert s.romanToInt("MCMXIII") == 1913
    assert s.romanToInt("MCMXCIV") == 1994
