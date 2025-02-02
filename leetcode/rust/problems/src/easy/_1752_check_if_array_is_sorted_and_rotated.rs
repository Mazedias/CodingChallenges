// Problem 1752

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn check(nums: Vec<i32>) -> bool {
        let mut rotation_point = false;

        for i in 1..nums.len() {
            if nums[i-1] > nums[i] && rotation_point {
                return false;
            } else if nums[i-1] > nums[i] && !rotation_point {
                rotation_point = true;
            }
        }

        if rotation_point && nums[0] < nums[nums.len() - 1] {
            return false;
        }

        true
    }
}

#[test]
pub fn test_check() {
    let nums1 = vec![3, 4, 5, 1, 2];
    let nums2 = vec![2, 1, 3, 4];
    let nums3 = vec![1, 2, 3];

    assert_eq!(Solution::check(nums1), true);
    assert_eq!(Solution::check(nums2), false);
    assert_eq!(Solution::check(nums3), true);
}