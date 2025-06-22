// Leetcode Problem 2138

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn divide_string(s: String, k: i32, fill: char) -> Vec<String> {
        let mut chunks: Vec<String> = Vec::new();
        let mut chars = s.chars().peekable();

        while chars.peek().is_some() {
            let mut chunk: String = chars.by_ref().take(k as usize).collect();

            if chunk.len() < k as usize {
                chunk.extend(std::iter::repeat(fill).take(k as usize - chunk.len()));
            }

            chunks.push(chunk);
        }

        chunks
    }
}


#[test]
fn test_divide_string() {
    assert_eq!(Solution::divide_string("abcdefghi".to_string(), 3, 'x'), vec!["abc".to_string(), "def".to_string(), "ghi".to_string()]);
    assert_eq!(Solution::divide_string("abcdefghij".to_string(), 3, 'x'), vec!["abc".to_string(), "def".to_string(), "ghi".to_string(), "jxx".to_string()]);
}