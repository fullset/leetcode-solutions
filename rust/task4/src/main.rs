// https://leetcode.com/problems/longest-substring-without-repeating-characters/

use std::collections::HashSet;

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
            for ch_begin in i_iter.by_ref() {
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

fn main() {
    println!("{}", length_of_longest_substring("bbb".to_string()));
    println!("{}", length_of_longest_substring("abcabcabc".to_string()));

    println!("{}", length_of_longest_substring("ab cabc".to_string()));
    println!("{}", length_of_longest_substring(" ".to_string()));
    println!("{}", length_of_longest_substring("pwwkew".to_string()));
    println!(
        "{}",
        length_of_longest_substring("sgjqlktjdidigifjhldkdnfjs".to_string())
    );

    println!(
        "{}",
        length_of_longest_substring("sgjqlk   tjdi".to_string())
    );
    println!("{}", length_of_longest_substring("aab".to_string()));
}
