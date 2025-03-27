// Leetcode Problem 2780

use std::collections::HashMap;

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn minimum_index(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut frequency: HashMap<i32, i32> = HashMap::new();
        for &num in &nums {
            *frequency.entry(num).or_insert(0) += 1;
        }

        let mut left_frequency: HashMap<i32, i32> = HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            *left_frequency.entry(num).or_insert(0) += 1;

            let total_count = frequency[&num];
            let left_count = left_frequency[&num];
            let right_count = total_count - left_count;

            if left_count > ((i + 1) / 2) as i32 && right_count > ((n - i - 1) / 2) as i32 {
                return i as i32;
            }
        }

        -1
    }
}

#[test]
pub fn test_minimum_index() {
    let nums1 = vec![1, 2, 2, 2];
    let nums2 = vec![2, 1, 3, 1, 1, 1, 7, 1, 2, 1];
    let nums3 = vec![3, 3, 3, 3, 7, 2, 2];

    assert_eq!(Solution::minimum_index(nums1), 2);
    assert_eq!(Solution::minimum_index(nums2), 4);
    assert_eq!(Solution::minimum_index(nums3), -1);
}