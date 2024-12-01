// Leetcode Problem 1346

use std::collections::HashMap;

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn check_if_exist(arr: Vec<i32>) -> bool {
        let mut seen: HashMap<i32, usize> = HashMap::new();

        for i in 0..arr.len() {
            if let Some(_) = seen.get(&(arr[i] * 2)) {
                return true;
            }
            
            if arr[i] % 2 == 0 {
                if let Some(_) = seen.get(&(arr[i] / 2)) {
                    return true;
                }
            }

            seen.insert(arr[i], i);
        }

        false
    }
}


#[test]
fn test_check_if_exist() {
    let arr1 = vec![10, 2, 5, 3];
    let arr2 = vec![3, 1, 7, 11];
    let arr3 = vec![7, 1, 14, 11];
    let arr4 = vec![0, 0];

    assert_eq!(Solution::check_if_exist(arr1), true);
    assert_eq!(Solution::check_if_exist(arr2), false);
    assert_eq!(Solution::check_if_exist(arr3), true);
    assert_eq!(Solution::check_if_exist(arr4), true);
}