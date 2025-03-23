// Leetcode Problem 16

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut closest = i32::MAX;
        let mut closest_sum = i32::MAX;

        let n = nums.len();

        for i in 0..n {
            for j in i+1..n {
                for x in j+1..n {
                    let sum = nums[i] + nums[j] + nums[x];

                    if (target - sum).abs() < closest {
                        closest = (target - sum).abs();
                        closest_sum = sum;
                    }
                }
            }
        }

        closest_sum
    }
}

#[test]
pub fn test_three_sum_closest() {
    let nums1 = vec![-1, 2, 1, -4];
    let nums2 = vec![0, 0, 0];

    assert_eq!(Solution::three_sum_closest(nums1, 1), 2);
    assert_eq!(Solution::three_sum_closest(nums2, 1), 0);
}