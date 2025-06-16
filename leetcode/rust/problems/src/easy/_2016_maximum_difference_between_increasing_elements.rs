// Leetcode Problem 2016

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn maximum_difference(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut diff = -1;

        for i in 0..n {
            let mut pos = i + 1;

            while pos < n {
                if nums[i] < nums[pos] && nums[pos] - nums[i] > diff {
                    diff = nums[pos] - nums[i];
                }
                pos += 1;
            }
        }

        diff
    }
}


#[test]
fn test_maximum_differences() {
    let nums1 = vec![7, 1, 5, 4];
    let nums2 = vec![9, 4, 3, 2];
    let nums3 = vec![1, 5, 2, 10];

    assert_eq!(Solution::maximum_difference(nums1), 4);
    assert_eq!(Solution::maximum_difference(nums2), -1);
    assert_eq!(Solution::maximum_difference(nums3), 9);
}