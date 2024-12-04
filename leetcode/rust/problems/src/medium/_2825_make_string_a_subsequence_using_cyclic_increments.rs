// Leetcode Problem 2825

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn can_make_subsequence(str1: String, str2: String) -> bool {
        let source_chars: Vec<char> = str1.chars().collect();
        let target_chars: Vec<char> = str2.chars().collect();
        let t_len = target_chars.len();

        let mut target_char = target_chars[0];
        let mut target_index = 0;

        for source_char in source_chars {
            if source_char == target_char || ((source_char as u8 + 1) as char == target_char) || (source_char == 'z' && target_char == 'a') {
                target_index += 1;
                if target_index == t_len {
                    break;
                }
                target_char = target_chars[target_index];
            }
        }

        target_index == t_len
    }
}


#[test]
fn test_can_make_subsequence() {
    let str1a = String::from("abc");
    let str2a = String::from("ad");

    let str1b = String::from("zc");
    let str2b = String::from("ad");

    let str1c = String::from("ab");
    let str2c = String::from("d");

    assert!(Solution::can_make_subsequence(str1a, str2a));
    assert!(Solution::can_make_subsequence(str1b, str2b));
    assert!(!Solution::can_make_subsequence(str1c, str2c));
}