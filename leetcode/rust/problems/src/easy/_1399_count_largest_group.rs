// Leetcode Problem 1399

use std::collections::HashMap;

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_largest_group(n: i32) -> i32 {
        let mut groups: HashMap<i32, i32> = HashMap::new();
        
        let mut sum;
        for i in 1..=n {
            sum = 0;
            let mut temp = i;
            while temp > 0 {
                sum += temp % 10;
                temp /= 10;
            }
            groups.entry(sum).and_modify(|e| *e += 1).or_insert(1);
        }

        if let Some(&max_occurance) = groups.values().max() {
            groups.values().filter(|&&group| group == max_occurance).count() as i32
        } else {
            0
        }
    }
}


#[test]
fn test_count_largest_group() {
    assert_eq!(Solution::count_largest_group(13), 4);
    assert_eq!(Solution::count_largest_group(2), 2);
    assert_eq!(Solution::count_largest_group(16), 7);
    assert_eq!(Solution::count_largest_group(99), 1);
}