use std::cmp;

// Problem 69
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn my_sqrt (x: i32) -> i32 {
        if x == 0 || x == 1 {
            return x;
        }

        // Find root with binary search
        let mut left = 1;
        let mut right = cmp::min(x, 46340); // Avoid overflow 
        let mut ans = 0;
        
        while left <= right {
            let mid = (right + left)/2;

            if (mid * mid) == x {
                return mid;
            } 
            
            if (mid * mid) <= x {
                left = mid + 1;
                ans = mid;
            } else {
                right = mid - 1;
            }
        }

        ans
    }
}

#[test]
pub fn test_my_sqrt () {
    assert_eq!(Solution::my_sqrt(4), 2);
    assert_eq!(Solution::my_sqrt(8), 2);
}