// Leetcode Problem 2558

use std::{cmp, collections::BinaryHeap};

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn pick_gifts(gifts: Vec<i32>, mut k: i32) -> i64 {
        let mut pq: BinaryHeap<i32> = BinaryHeap::from(gifts.clone());

        while k > 0 {
            let mut gift = pq.pop().unwrap();
            
            // Find root with binary search
            let mut left = 1;
            let mut right = cmp::min(gift, 46340); // Avoid overflow 
            let mut ans = 0;
            
            while left <= right {
                let mid = (right + left)/2;

                if (mid * mid) == gift {
                    ans = mid;
                    break;
                } 
                
                if (mid * mid) <= gift {
                    left = mid + 1;
                    ans = mid;
                } else {
                    right = mid - 1;
                }
            }

            gift = ans;
            pq.push(gift);
            k -= 1;
        }

        pq.iter().map(|&x| x as i64).sum()
    }
}


#[test]
fn test_pick_gifts() {
    let gifts1 = vec![25, 64, 9, 4, 100];
    let gifts2 = vec![1, 1, 1, 1];

    assert_eq!(Solution::pick_gifts(gifts1, 4), 29);
    assert_eq!(Solution::pick_gifts(gifts2, 4), 4);
}