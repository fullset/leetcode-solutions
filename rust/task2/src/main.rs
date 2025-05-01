// https://leetcode.com/problems/ransom-note/description/

use std::collections::HashMap;

// impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut rn_letters = HashMap::new();
        let mut m_letters = HashMap::new();

        for ch in ransom_note.chars() {
            rn_letters.entry(ch).and_modify(|counter| *counter += 1).or_insert(1);
        }

        for ch in magazine.chars() {
            m_letters.entry(ch).and_modify(|counter| *counter += 1).or_insert(1);
        }

        for (key, val) in rn_letters.iter() {
            match m_letters.get(&key) {
                None => return false,
                Some(m_val) => if m_val < val {
                    return false;
                },
            }
        }

        return true;
    }
// }

fn main() {
    println!("{:?}", can_construct("a".to_string(), "b".to_string()));
    println!("{:?}", can_construct("aa".to_string(), "ab".to_string()));
    println!("{:?}", can_construct("aa".to_string(), "aab".to_string()));
    println!("{:?}", can_construct("amy".to_string(), "my name is".to_string()));
    println!("{:?}", can_construct("ammy".to_string(), "my name is".to_string()));
}
