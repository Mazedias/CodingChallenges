// Leetcode Problem 3105

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
        let mut longest = 1;

        let mut increase_start = 0;
        let mut decrease_start = 0;

        for i in 1..nums.len() {
            // Check if the currect sequence is bigger than the longest one
            longest = longest.max((i-increase_start).max(i-decrease_start));
            
            if nums[i-1] < nums[i] {
                // Number before was greater -> "finish" a increasing sequnce
                decrease_start = i;
            } else if nums[i-1] > nums[i] {
                // Number before was smaller -> "finish" a decreasing sequnce
                increase_start = i;
            } else {
                increase_start = i;
                decrease_start = i;
            }
        }

        longest.max((nums.len()-increase_start).max(nums.len()-decrease_start)) as i32
    }
}


#[test]
fn test_longest_monotonic_subarray() {
    let nums1 = vec![1, 4, 3, 3, 2];
    let nums2 = vec![3, 3, 3, 3];
    let nums3 = vec![3, 2, 1];
    let nums4 = vec![1, 4, 3, 2, 1, 2, 3, 4, 5, 6];

    assert_eq!(Solution::longest_monotonic_subarray(nums1), 2);
    assert_eq!(Solution::longest_monotonic_subarray(nums2), 1);
    assert_eq!(Solution::longest_monotonic_subarray(nums3), 3);
    assert_eq!(Solution::longest_monotonic_subarray(nums4), 6);
}