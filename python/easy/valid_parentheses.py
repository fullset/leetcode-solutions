# https://leetcode.com/problems/valid-parentheses/

class Solution:
    def isValid(self, s: str) -> bool:
        opened = ['(', '{', '[']
        closed = {')': '(', '}' : '{', ']': '['}
        l = []
        for ch in s:
            if ch in opened:
                l.append(ch)
            else:
                if len(l) == 0:
                    return False
                top = l.pop()
                if not top in opened:
                    return False
                else:
                    if closed[ch] != top:
                        return False

        return len(l) == 0


if __name__ == '__main__':
    s = Solution()
    assert(s.isValid('()'))
    assert(s.isValid('(){}[]'))
    assert(not s.isValid('('))
    assert(not s.isValid(')'))
    assert(not s.isValid('()}'))

    assert(s.isValid('({[]})'))
    assert(not s.isValid('({[])}'))

        