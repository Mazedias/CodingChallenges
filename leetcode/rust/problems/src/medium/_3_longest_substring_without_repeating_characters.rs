use std::collections::HashMap;

// Problem 3
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn length_of_longest_substring (s: String) -> i32 {
        let mut char_indices: HashMap<char, usize> = HashMap::with_capacity(128);
        let mut length = 0;
        let mut start = 0;

        for (index, character) in s.char_indices() {
            if let Some(&prev_index) = char_indices.get(&character) {
                start = start.max(prev_index + 1);
            }

            length = length.max(index - start + 1);
            char_indices.insert(character, index);
        }

        length as i32
    }

    /**
     * This was my first idea but i realised it is slow as hell
     */
    pub fn slow_length_of_longest_substring (s: String) -> i32 {
        if s.len() == 0 {
            return 0;
        }

        let mut length = 0;
        let mut already = String::new();

        let mut i = 0;
        while i < s.len() {
            length = if already.len() > length { already.len() } else { length };

            while already.contains(s.chars().nth(i).unwrap()) {
                if already.len() == 1 {
                    already = "".to_string();
                } else {
                    already = already[1..=already.len()-1].to_string();
                }
            }

            already.push(s.chars().nth(i).unwrap());

            i += 1;
        }

        if already.len() > length {
            length = already.len();
        }

        length as i32
    }
}

#[test]
pub fn test_length_of_longest_substring () {
    let s1 = "abcabcbb".to_string();
    let s2 = "bbbbb".to_string();
    let s3 = "pwwkew".to_string();
    let s4 = "".to_string();
    let s5 = " ".to_string();
    let s6 = "aaabcdabca".to_string();

    assert_eq!(Solution::length_of_longest_substring(s1), 3);
    assert_eq!(Solution::length_of_longest_substring(s2), 1);
    assert_eq!(Solution::length_of_longest_substring(s3), 3);
    assert_eq!(Solution::length_of_longest_substring(s4), 0);
    assert_eq!(Solution::length_of_longest_substring(s5), 1);
    assert_eq!(Solution::length_of_longest_substring(s6), 4);
}