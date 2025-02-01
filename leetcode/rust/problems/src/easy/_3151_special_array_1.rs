// Leetcode Problem 3151

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_array_special(nums: Vec<i32>) -> bool {
        let mut last_parity = nums[0] % 2 == 0;

        for i in 1..nums.len() {
            if (nums[i] % 2 == 0) == last_parity {
                return false;
            }
            last_parity = !last_parity;
        }

        true
    }
}


#[test]
fn test_is_array_special() {
    let nums1 = vec![1];
    let nums2 = vec![2, 1, 4];
    let nums3 = vec![4, 3, 1, 6];

    assert_eq!(Solution::is_array_special(nums1), true);
    assert_eq!(Solution::is_array_special(nums2), true);
    assert_eq!(Solution::is_array_special(nums3), false);
}