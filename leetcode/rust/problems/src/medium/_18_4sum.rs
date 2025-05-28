// Leetcode Problem 18

use std::collections::HashSet;

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut quatruples: HashSet<Vec<i32>> = HashSet::new();
        let n = nums.len();
        
        nums.sort();

        // Break the 4 sum problem into n 3 sum problems. 
        for a in 0..n {
            for b in a+1..n {
                let mut left = b+1;
                let mut right = n-1;

                while left < right {
                    let sum = nums[a].saturating_add(nums[b]).saturating_add(nums[left]).saturating_add(nums[right]);

                    if sum > target {
                        right -= 1;
                    } else if sum < target {
                        left += 1;
                    } else {
                        quatruples.insert(vec![nums[a], nums[b], nums[left], nums[right]]);
                        left += 1;

                        while nums[left] == nums[left - 1] && left < right {
                            left += 1;
                        }
                    }
                }
            }
        }

        quatruples.into_iter().collect()
    }
}


#[test]
fn test_four_sum() {
    let nums1 = vec![1, 0, -1, 0, -2, 2];
    let expected1 = vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]];

    let nums2 = vec![2, 2, 2, 2, 2];
    let expected2 = vec![vec![2, 2, 2, 2]];

    assert_eq!(Solution::four_sum(nums1, 0), expected1);
    assert_eq!(Solution::four_sum(nums2, 8), expected2);
}