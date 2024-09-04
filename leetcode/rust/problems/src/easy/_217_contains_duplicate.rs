use std::collections::HashMap;

// Problem 217
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn contains_duplicate (nums: Vec<i32>) -> bool {
        let mut seen: HashMap<i32, i32> = HashMap::with_capacity(10);

        for num in nums {
            match seen.get(&num) {
                Some(_) => return true,
                None => { seen.insert(num, 1); },
            }
        }

        false
    }
}

#[test]
pub fn test_contains_duplicate () {
    let a = vec![1, 2, 3, 1];
    let b = vec![1, 2, 3, 4];
    let c = vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2];

    assert_eq!(Solution::contains_duplicate(a), true);
    assert_eq!(Solution::contains_duplicate(b), false);
    assert_eq!(Solution::contains_duplicate(c), true);
}