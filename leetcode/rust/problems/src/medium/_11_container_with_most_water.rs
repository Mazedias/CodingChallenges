use std::cmp::min;

// Problem 11
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_area (height: Vec<i32>) -> i32 {
        let mut left: usize = 0;
        let mut right: usize = height.len() - 1;
        let mut max: usize = 0;

        while left != right {
            let capacity: usize = min(height[left], height[right]) as usize * (right - left);
            if capacity > max {
                max = capacity;
            }

            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }

        max as i32
    }
}

#[test]
pub fn test_max_area () {
    let height1 = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
    let height2 = vec![1, 1];

    assert_eq!(Solution::max_area(height1), 49);
    assert_eq!(Solution::max_area(height2), 1);
}