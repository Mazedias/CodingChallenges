// Leetcode Problem 2444

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
        let mut counter: i64 = 0;
        let mut last_min = -1;
        let mut last_right = -1;
        let mut last_invalid = -1;

        for (i, &v) in nums.iter().enumerate() {
            if v < min_k || v > max_k {
                last_invalid = i as i32;
            }

            if v == min_k {
                last_min = i as i32;
            }
            
            if v == max_k {
                last_right = i as i32;
            }

            counter += std::cmp::max(0, std::cmp::min(last_min, last_right) - last_invalid) as i64;
        }

        counter
    }
}


#[test]
fn test_count_subarrays() {
    let nums1 = vec![1, 3, 5, 2, 7, 5];
    let nums2 = vec![1, 1, 1, 1];

    assert_eq!(Solution::count_subarrays(nums1, 1, 5), 2);
    assert_eq!(Solution::count_subarrays(nums2, 1, 1), 10);
}