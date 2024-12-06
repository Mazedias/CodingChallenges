// Leetcode Problem 2554

use std::collections::HashSet;

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_count(banned: Vec<i32>, n: i32, max_sum: i32) -> i32 {
        let banned_numbers: HashSet<i32> = banned.into_iter().collect();

        let mut sum = 0;
        let mut amount = 0;

        for i in 1..=n {
            if banned_numbers.contains(&i) {
                continue;
            }
            
            sum += i;
            if sum > max_sum {
                break;
            }
            amount += 1;
        }

        amount
    }
}


#[test]
fn test_max_count() {
    let banned1 = vec![1, 6, 5];
    let banned2 = vec![1, 2, 3, 4, 5, 6, 7];
    let banned3 = vec![11];

    assert_eq!(Solution::max_count(banned1, 5, 6), 2);
    assert_eq!(Solution::max_count(banned2, 8, 1), 0);
    assert_eq!(Solution::max_count(banned3, 7, 50), 7);
}