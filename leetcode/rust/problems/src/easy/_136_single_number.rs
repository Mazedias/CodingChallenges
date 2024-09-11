use std::collections::HashSet;

// Problem 136
struct Solution;

#[allow(dead_code)]
impl Solution { 
    pub fn single_number (nums: Vec<i32>) -> i32 {
        let mut seen: HashSet<i32> = HashSet::with_capacity(nums.len() / 2);

        for num in &nums {
            if seen.contains(num) {
                seen.remove(num);
                continue;
            }
            seen.insert(*num);
        }

        if seen.len() != 1 {
            return 0;
        }

        for i in seen.iter() {
            return *i;
        }

        0
    }
}

#[test]
pub fn test_single_number () {
    let a = vec![2, 2, 1];
    let b = vec![4, 1, 2, 1, 2];
    let c = vec![1];
    let d = vec![-1, 4, 10, 5, 5, 4, -1, 10,     -2];

    assert_eq!(Solution::single_number(a), 1);
    assert_eq!(Solution::single_number(b), 4);
    assert_eq!(Solution::single_number(c), 1);
    assert_eq!(Solution::single_number(d), -2);
}