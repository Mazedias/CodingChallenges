// Problem 4
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_median_sorted_arrays (nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut nums = Vec::new();

        nums.extend_from_slice(&nums1);
        nums.extend_from_slice(&nums2);

        nums.sort_unstable();

        if nums.len() % 2 == 0 {
            return (nums[nums.len() / 2 - 1] + nums[nums.len() / 2]) as f64 / 2.0;
        } else {
            return nums[nums.len() / 2] as f64;
        }
    }
}

#[test]
pub fn test_find_median_sorted_arrays () {
    let nums1 = vec![1, 2, 3, 4, 5];
    let nums2 = vec![6, 7, 8, 9];

    let nums3 = vec![1, 2];
    let nums4 = vec![3, 4];

    let nums5 = vec![1, 2, 3, 40, 41];
    let nums6 = vec![6, 7, 8, 9, 10];

    let nums7: Vec<i32> = vec![];
    let nums8 = vec![1];

    assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), 5.0);
    assert_eq!(Solution::find_median_sorted_arrays(nums3, nums4), 2.5);
    assert_eq!(Solution::find_median_sorted_arrays(nums5, nums6), 7.5);
    assert_eq!(Solution::find_median_sorted_arrays(nums7, nums8), 1.0);
    
}
