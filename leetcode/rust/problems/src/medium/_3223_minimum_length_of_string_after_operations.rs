// Leetcode Problem 3223

use std::collections::HashMap;

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn minimum_length(s: String) -> i32 {
        let mut map: HashMap<char, i32> = HashMap::new();

        for char in s.chars() {
            map.entry(char).and_modify(|v| *v += 1).or_insert(1);
        }

        map.iter().map(|(&_key, &value)| {
            if value % 2 != 0 {
                return 1;
            } else {
                return 2;
            }
        }).sum()
    }
}


#[test]
fn test_minimum_length() {
    let s1: String = String::from("abaacbcbb");
    let s2: String = String::from("aa");
    
    assert_eq!(Solution::minimum_length(s1), 5);
    assert_eq!(Solution::minimum_length(s2), 2);
}