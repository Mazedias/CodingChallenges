// Problem 2425

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn xor_all_nums(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let length1 = nums1.len();
        let length2 = nums2.len();

        let mut ans = 0;

        if length1 % 2 == 1 {
            ans ^= nums2.iter().fold(0, |acc, &x| acc ^ x);
        }

        if length2 % 2 == 1 {
            ans ^= nums1.iter().fold(0, |acc, &x| acc ^ x);
        }

        ans
    }
}

#[test]
pub fn test_xor_all_nums() {
    assert_eq!(Solution::xor_all_nums(vec![2, 1, 3], vec![10, 2, 5, 0]), 13);
    assert_eq!(Solution::xor_all_nums(vec![1, 2], vec![3, 4]), 0);
}