// Leetcode Problem 2375

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn smallest_number(pattern: String) -> String {
        let n = pattern.len();
        let mut result = String::new();
        let mut stack = Vec::new();

        for i in 0..=n {
            stack.push((i+1).to_string());

            if i == n || pattern.chars().nth(i).unwrap() == 'I' {
                while let Some(top) = stack.pop() {
                    result.push_str(&top);
                }
            }
        }

        result
    }
}


#[test]
fn test_smallest_number() {
    let pattern1 = String::from("IIIDIDDD");
    let expected1 = String::from("123549876");
    
    let pattern2 = String::from("DDD");
    let expected2 = String::from("4321");

    let pattern3 = String::from("DDDIII");
    let expected3 = String::from("4321567");

    assert_eq!(Solution::smallest_number(pattern1), expected1);
    assert_eq!(Solution::smallest_number(pattern2), expected2);
    assert_eq!(Solution::smallest_number(pattern3), expected3);
}