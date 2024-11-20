use std::collections::HashSet;

// Problem 2461
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let n = nums.len();
        let k = k as usize;
        let mut elements = HashSet::new();
        let mut current_sum: i64 = 0;
        let mut max_sum: i64 = 0;
        let mut begin = 0;

        for end in 0..n {
            if !elements.contains(&nums[end]) {
                current_sum += nums[end] as i64;
                elements.insert(nums[end]);

                if end - begin + 1 == k {
                    max_sum = max_sum.max(current_sum);
                    current_sum -= nums[begin] as i64;
                    elements.remove(&nums[begin]);
                    begin += 1;
                }
            } else {
                while nums[begin] != nums[end] {
                    current_sum -= nums[begin] as i64;
                    elements.remove(&nums[begin]);
                    begin += 1;
                }
                begin += 1;
            }
        }

        max_sum
    }
}

#[test]
pub fn test_maximum_subarray_sum() {
    let nums1 = vec![1, 5, 4, 2, 9, 9, 9];
    let nums2 = vec![4, 4, 4];

    assert_eq!(Solution::maximum_subarray_sum(nums1, 3), 15);
    assert_eq!(Solution::maximum_subarray_sum(nums2, 3), 0);
}