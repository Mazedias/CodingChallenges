// Leetcode Problem 1455

use regex::Regex;

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
        if sentence.len() < search_word.len() {
            return -1;
        }

        let words = sentence.split(" ");
        let escaped_word = regex::escape(&search_word);
        let pattern = format!("^{}", escaped_word);
        let regex = Regex::new(&pattern).unwrap();

        for (i, word) in words.enumerate() {
            if regex.is_match(word) {
                return (i + 1) as i32;
            }
        }
        
        -1
    }
}


#[test]
fn test_is_prefix_of_word() {
    let sentence1 = String::from("i love eating burgers");
    let sentence2 = String::from("this problem is an easy problem");
    let sentence3 = String::from("i am tired");

    assert_eq!(Solution::is_prefix_of_word(sentence1, String::from("burg")), 4);
    assert_eq!(Solution::is_prefix_of_word(sentence2, String::from("pro")), 2);
    assert_eq!(Solution::is_prefix_of_word(sentence3, String::from("you")), -1);

    assert_eq!(Solution::is_prefix_of_word(String::from(" "), String::from("you")), -1);
    assert_eq!(Solution::is_prefix_of_word(String::from(" "), String::from("a")), -1);
}