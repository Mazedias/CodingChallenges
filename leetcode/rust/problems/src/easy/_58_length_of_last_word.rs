// Problem 58
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn length_of_last_word (s: String) -> i32 {

        let text = s.as_str().trim_end();
        let mut counter = 0;
        
        for c in text.chars().into_iter().rev() {
            if c.is_whitespace() {
                return counter;
            }
            counter += 1;
        }
        counter
    }
}

#[test]
pub fn test_length_of_last_word () {
    let s1 = "Hello World".to_string();
    let s2 = "   fly me   to   the moon  ".to_string();
    let s3 = "luffy is still joyboy".to_string();

    assert_eq!(Solution::length_of_last_word(s1), 5);
    assert_eq!(Solution::length_of_last_word(s2), 4);
    assert_eq!(Solution::length_of_last_word(s3), 6);
}