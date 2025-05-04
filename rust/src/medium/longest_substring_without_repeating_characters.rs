// https://leetcode.com/problems/longest-substring-without-repeating-characters/

use std::collections::HashSet;

struct Solution;

// Runtime
// 0 ms
// Beats 100.00%

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut max_len = 0;
        let mut current_len: i32 = 0;

        let mut chars = HashSet::new();

        let mut i_iter = s.chars();
        let j_iter = s.chars();
        let mut current: char;

        for ch_end in j_iter {
            current = ch_end;
            current_len += 1;
            if chars.contains(&ch_end) {
                current_len -= 1;
                if current_len > max_len {
                    max_len = current_len;
                }
                while let Some(ch_begin) = i_iter.next() {
                    chars.remove(&ch_begin);
                    if ch_begin == current {
                        break;
                    }
                    current_len -= 1;
                }
            }
            chars.insert(ch_end);
        }
        if current_len > max_len {
            max_len = current_len;
        }
        max_len
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::length_of_longest_substring("bbb".to_string()), 1);
        assert_eq!(
            Solution::length_of_longest_substring("abcabcabc".to_string()),
            3
        );
        assert_eq!(Solution::length_of_longest_substring(" ".to_string()), 1);
        assert_eq!(
            Solution::length_of_longest_substring("pwwkew".to_string()),
            3
        );
        assert_eq!(
            Solution::length_of_longest_substring("sgjqlktjdidigifjhldkdnfjs".to_string()),
            8
        );
        assert_eq!(Solution::length_of_longest_substring("aab".to_string()), 2);
    }
}
