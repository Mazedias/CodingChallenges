// Problem 3
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn length_of_longest_substring (s: String) -> i32 {
        if s.len() == 0 {
            return 0;
        }

        let mut length = 0;

        let mut i = 0;
        let mut p = 0;
        let mut already = String::new();

        for x in 0..s.len() {
            if already.contains(s.chars().nth(x).unwrap()) {
                length = if already.len() > length { already.len() } else { length };

                while already.contains(s.chars().nth(x).unwrap()) {
                    if already.len() == 1 {
                        already = "".to_string();
                    } else {
                        already = already[1..already.len()-1].to_string();
                    }
                }
            } else {
                already.push(s.chars().nth(x).unwrap());
            }
        }

        /*
        while i < s.len()-1 || i-p > length {
            if already.contains(s.chars().nth(i).unwrap()) {
                length = if already.len() > length { already.len() } else { length };
                p += 1;
                if already.len() == 1 {
                    already = "".to_string()
                } else {
                    already = already[1..already.len()-1].to_string();
                }
            } else {
                already.push(s.chars().nth(i).unwrap());
                i += 1;
            }
        }
        */

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

    assert_eq!(Solution::length_of_longest_substring(s1), 3);
    assert_eq!(Solution::length_of_longest_substring(s2), 1);
    assert_eq!(Solution::length_of_longest_substring(s3), 3);
    assert_eq!(Solution::length_of_longest_substring(s4), 0);
    assert_eq!(Solution::length_of_longest_substring(s5), 1);
}