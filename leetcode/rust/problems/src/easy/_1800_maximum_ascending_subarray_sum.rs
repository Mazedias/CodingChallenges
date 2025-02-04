// Leetcode Problem 1800

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {
        let mut max_sum = nums[0];
        let mut temp_sum = nums[0];

        for i in 1..nums.len() {
            if nums[i-1] >= nums[i] {
                max_sum = max_sum.max(temp_sum);
                temp_sum = nums[i];
            } else {
                temp_sum += nums[i];
            }
        }

        max_sum.max(temp_sum)
    }
}


#[test]
fn test_max_ascending_sum() {
    let nums1 = vec![10, 20, 30, 5, 10, 50];
    let nums2 = vec![10, 20, 30, 40, 50];
    let nums3 = vec![12, 17, 15, 13, 10, 11, 12];
    let nums4 = vec![23, 20, 15, 40, 30, 10, 70, 10];
    let nums5 = vec![3, 6, 10, 1, 8, 9, 9, 8, 9];

    assert_eq!(Solution::max_ascending_sum(nums1), 65);
    assert_eq!(Solution::max_ascending_sum(nums2), 150);
    assert_eq!(Solution::max_ascending_sum(nums3), 33);
    assert_eq!(Solution::max_ascending_sum(nums4), 80);
    assert_eq!(Solution::max_ascending_sum(nums5), 19);
}