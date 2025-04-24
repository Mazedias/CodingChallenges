// Leetcode Problem 2799

use std::collections::{HashMap, HashSet};

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_complete_subarrays(nums: Vec<i32>) -> i32 {
        // Determine the amount of distinct elements in nums
        let k = nums.iter().copied().collect::<HashSet<i32>>().len();
        let n = nums.len();
        let mut counter = 0;

        // Keet track of numbers in the current sliding window
        let mut frequency: HashMap<i32, usize> = HashMap::new();

        let mut right = 0;
        for left in 0..n {
            // While the sliding window did not reached the end of nums and the window does not contain
            // enough distinct numbers move the right pointer (increase the sliding window)
            while right < n && frequency.len() < k {
                frequency.entry(nums[right]).and_modify(|e| *e += 1).or_insert(1);
                right += 1;
            }

            // If we found a sliding window that contains the same amount of distinct numbers as the 
            // nums vector we increase the counter by (n - right + 1) since all subarrays starting from left and going to right + x
            // where (right + x) < n are valid subarrays and have to be counted
            if frequency.len() == k {
                counter += n - right + 1;

                // Shrink the sliding window by moving the left pointer and decrease/delete the number from our frequency map
                frequency.entry(nums[left]).and_modify(|e| *e -= 1);
                if *frequency.get(&nums[left]).unwrap() == 0 {
                    frequency.remove(&nums[left]);
                }
            }
        }

        counter as i32
    }
}


#[test]
fn test_count_complete_subarrays() {
    let nums1 = vec![1, 3, 1, 2, 2];
    let nums2 = vec![5, 5, 5, 5];
    let nums3 = vec![1, 2, 3, 4, 5, 5, 5, 5, 5];
    let nums4 = vec![1, 2, 1, 2, 1, 2];

    assert_eq!(Solution::count_complete_subarrays(nums1), 4);
    assert_eq!(Solution::count_complete_subarrays(nums2), 10);
    assert_eq!(Solution::count_complete_subarrays(nums3), 5);
    assert_eq!(Solution::count_complete_subarrays(nums4), 15);
}