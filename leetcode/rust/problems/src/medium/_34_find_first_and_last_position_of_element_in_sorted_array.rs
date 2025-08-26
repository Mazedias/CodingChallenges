// Leetcode Problem 34

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if let Ok(_index) = nums.binary_search(&target) {
            vec![nums.partition_point(|&i| i < target) as i32, nums.partition_point(|&i| i <= target) as i32 - 1]
        } else {
            vec![-1, -1]
        }
    }
}


#[test]
fn test_search_range() {
    let nums1 = vec![5, 7, 7, 8, 8, 10];
    let nums2 = vec![5, 7, 7, 8, 8, 10];
    let nums3 = vec![];
    let nums4 = vec![1];

    assert_eq!(Solution::search_range(nums1, 8), vec![3, 4]);
    assert_eq!(Solution::search_range(nums2, 6), vec![-1, -1]);
    assert_eq!(Solution::search_range(nums3, 0), vec![-1, -1]);
    assert_eq!(Solution::search_range(nums4, 1), vec![0, 0]);
}