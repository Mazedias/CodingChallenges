// Leetcode Problem 1980

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        let mut result = String::new();

        for (idx, num) in nums.iter().enumerate() {
            result.push(if num.chars().nth(idx).unwrap() == '0' {'1'} else {'0'});
        }

        result
    }
}


#[test]
fn test_find_different_binary_string() {
    let nums1 = vec![String::from("01"), String::from("10")];
    let expected1 = vec![String::from("00"), String::from("11")];

    let nums2 = vec![String::from("00"), String::from("01")];
    let expected2 = vec![String::from("10"), String::from("11")];

    let nums3 = vec![String::from("111"), String::from("011"), String::from("001")];
    let expected3 = vec![
        String::from("000"),
        String::from("010"),
        String::from("100"),
        String::from("101"),
        String::from("110")
    ];

    assert!(expected1.contains(&Solution::find_different_binary_string(nums1)));
    assert!(expected2.contains(&Solution::find_different_binary_string(nums2)));
    assert!(expected3.contains(&Solution::find_different_binary_string(nums3)));
}