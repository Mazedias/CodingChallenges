// Leetcode Problem 3264

use std::{cmp::Reverse, collections::BinaryHeap};

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn get_final_state(mut nums: Vec<i32>, k: i32, multiplier: i32) -> Vec<i32> {
        let mut pq: BinaryHeap<_> = nums.iter().copied().zip(0_usize..).map(Reverse).collect();

        for _ in 0..k {
            pq.peek_mut().unwrap().0 .0 *= multiplier;
        }

        for Reverse((num, i)) in pq.into_iter() {
            nums[i] = num;
        }

        nums
    }
}


#[test]
fn test_get_final_state() {
    let nums1 = vec![2, 1, 3, 5, 6];
    let sol1 = vec![8, 4, 6, 5, 6];

    let nums2 = vec![1, 2];
    let sol2 = vec![16, 8];

    assert_eq!(Solution::get_final_state(nums1, 5, 2), sol1);
    assert_eq!(Solution::get_final_state(nums2, 3, 4), sol2);
}