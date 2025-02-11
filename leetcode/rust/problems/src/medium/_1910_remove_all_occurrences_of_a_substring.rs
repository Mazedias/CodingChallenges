// Leetcode Problem 1910

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn remove_occurrences(s: String, part: String) -> String {
        let mut stack: Vec<char> = Vec::new();
        let needle: Vec<char> = part.chars().collect();

        for char in s.chars() {
            stack.push(char);

            if stack.len() >= needle.len() && stack[stack.len() - needle.len()..] == *needle {
                for _ in 0..needle.len() {
                    stack.pop();
                }
            }
        }

        stack.iter().collect()
    }
}


#[test]
fn test_remove_occurrences() {
    let s1 = String::from("daabcbaabcbc");
    let part1 = String::from("abc");
    let expected1 = String::from("dab");

    let s2 = String::from("axxxxyyyyb");
    let part2 = String::from("xy");
    let expected2 = String::from("ab");

    assert_eq!(Solution::remove_occurrences(s1, part1), expected1);
    assert_eq!(Solution::remove_occurrences(s2, part2), expected2);
}