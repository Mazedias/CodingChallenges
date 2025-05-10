// Leetcode Problem 2918

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
        let mut contains_zero1 = false;
        let mut contains_zero2 = false;
        let sum_nums1: i64 = nums1.iter().map(|e| if *e == 0 { contains_zero1 = true; 1 as i64 } else { *e as i64 }).sum::<i64>();
        let sum_nums2: i64 = nums2.iter().map(|e| if *e == 0 { contains_zero2 = true; 1 as i64 } else { *e as i64 }).sum::<i64>();
        
        if (sum_nums1 < sum_nums2 && !contains_zero1) || (sum_nums2 < sum_nums1 && !contains_zero2) {
            return -1;
        } else {
            return sum_nums1.max(sum_nums2);
        }
    }
}

#[test]
pub fn test_min_sum() {
    let nums11 = vec![3, 2, 0, 1, 0];
    let nums21 = vec![6, 5, 0];

    let nums12 = vec![2, 0, 2, 0];
    let nums22 = vec![1, 4];

    let nums13 = vec![0, 0, 0, 100];
    let nums23 = vec![100, 0];

    assert_eq!(Solution::min_sum(nums11, nums21), 12);
    assert_eq!(Solution::min_sum(nums12, nums22), -1);
    assert_eq!(Solution::min_sum(nums13, nums23), 103);
}