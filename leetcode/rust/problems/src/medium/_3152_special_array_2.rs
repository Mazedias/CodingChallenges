// Leetcode Problem 3152

use std::collections::HashMap;

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_array_special(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut positions: HashMap<usize, i32> = HashMap::new();

        let mut section = 0;
        let mut was_last_even: bool = nums[0] % 2 == 0;
        positions.insert(0, 0);
        for i in 1..nums.len() {
            if (was_last_even && nums[i] % 2 != 0) || (!was_last_even && nums[i] % 2 == 0) {
                positions.insert(i, section);
            } else {
                section += 1;
                positions.insert(i, section);
            }
            was_last_even = nums[i] % 2 == 0;
        }

        let mut solution: Vec<bool> = Vec::new();
        for query in queries {
            if positions.get(&(query[0] as usize)) != positions.get(&(query[query.len() - 1] as usize)) {
                solution.push(false);
            } else {
                solution.push(true);
            }
        }

        solution
    }
}


#[test]
fn test_is_array_special() {
    let nums1 = vec![3, 4, 1, 2, 6];
    let queries1 = vec![vec![0, 4]];
    let sol1 = vec![false];

    let nums2 = vec![4, 3, 1, 6];
    let queries2 = vec![vec![0, 2], vec![2, 3]];
    let sol2 = vec![false, true];

    assert_eq!(Solution::is_array_special(nums1, queries1), sol1);
    assert_eq!(Solution::is_array_special(nums2, queries2), sol2);
}