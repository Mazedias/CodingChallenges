// Leetcode Problem 2066

use std::{cmp::Reverse, collections::BinaryHeap};

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut pq = BinaryHeap::new();

        for num in nums {
            pq.push(Reverse(num as i64));
        }

        let mut steps = 0;

        while pq.len() >= 2 && *pq.peek().unwrap() > Reverse(k as i64) {
            let x = pq.pop().unwrap().0;
            let y = pq.pop().unwrap().0;

            pq.push(Reverse(x.min(y) * 2 + x.max(y)));

            steps += 1;
        }

        steps
    }
}


#[test]
fn test_min_operations() {
    let nums1 = vec![2, 11, 10, 1, 3];
    let nums2 = vec![1, 1, 2, 4, 9];
    let nums3 = vec![1000000000,999999999,1000000000,999999999,1000000000,999999999];

    assert_eq!(Solution::min_operations(nums1, 10), 2);
    assert_eq!(Solution::min_operations(nums2, 20), 4);
    assert_eq!(Solution::min_operations(nums3, 1000000000), 2);
}