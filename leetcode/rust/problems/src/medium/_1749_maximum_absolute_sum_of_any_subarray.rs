// Leetcode Problem 1749

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_absolute_sum(nums: Vec<i32>) -> i32 {
        let mut max_sum = 0;
        let mut min_sum = 0;
        let mut current_max = 0;
        let mut current_min = 0;

        for num in nums {
            current_max = (current_max + num).max(num);
            max_sum = max_sum.max(current_max);

            current_min = (current_min + num).min(num);
            min_sum = min_sum.min(current_min);
        }

        max_sum.max(min_sum.abs())
    }
}


#[test]
fn test_max_absolut_sum() {
    let nums1 = vec![1, -3, 2, 3, -4];
    let nums2 = vec![2, -5, 1, -4, 3, -2];

    assert_eq!(Solution::max_absolute_sum(nums1), 5);
    assert_eq!(Solution::max_absolute_sum(nums2), 8);

}