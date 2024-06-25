use std::collections::HashMap;

// Problem 502
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_maximaized_capital (mut k: i32, mut w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        

        w
    }
}

#[test]
pub fn test_find_maximaized_capital () {
    let profits1 = vec![1, 2, 3]; 
    let capital1 = vec![0, 1, 1];

    let profits2 = vec![1, 2, 3];
    let capital2 = vec![0, 1, 2];

    assert_eq!(Solution::find_maximaized_capital(2, 0, profits1, capital1), 4);
    assert_eq!(Solution::find_maximaized_capital(3, 0, profits2, capital2), 6);
}