// Problem 945
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_increment_for_unique (nums: Vec<i32>) -> i32 {
        let mut increment_counter = 0;

        let mut nums = nums;
        nums.sort();

        for i in 0..nums.len()-1 {
            if nums[i+1] <= nums[i] {
                increment_counter += nums[i]+1 - nums[i+1];
                nums[i+1] = nums[i] + 1;
            }
        }

        if nums.len() > 2 && nums[nums.len()-1] == nums[nums.len()-2] {
            increment_counter += 1;
        }

        increment_counter
    }
}

#[test]
pub fn test_min_increment_for_unique () {
    let nums1 = vec![1, 2, 2];
    let nums2 = vec![3, 2, 1, 2, 1, 7];
    let nums3 = vec![1];
    let nums4 = vec![1, 2, 2, 2, 2, 2, 5, 7];
    let nums5 = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
    let nums6 = vec![1, 2, 3, 1, 3, 2, 2, 1, 3, 2, 3, 1, 3, 2, 1, 3, 1, 2];
    let nums7 = vec![12345, 12345];

    assert_eq!(Solution::min_increment_for_unique(nums1), 1);
    assert_eq!(Solution::min_increment_for_unique(nums2), 6);
    assert_eq!(Solution::min_increment_for_unique(nums3), 0);
    assert_eq!(Solution::min_increment_for_unique(nums4), 13);
    assert_eq!(Solution::min_increment_for_unique(nums5), 44);
    assert_eq!(Solution::min_increment_for_unique(nums6), 135);
    assert_eq!(Solution::min_increment_for_unique(nums7), 1);
}