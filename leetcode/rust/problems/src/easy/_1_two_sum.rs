// Problem 1
struct Solution;
use std::collections::HashMap;

#[allow(dead_code)]
impl Solution {
    pub fn two_sum (nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();

        for (index, &num) in nums.iter().enumerate() {
            let complement = target - num;
            if let Some(&prev_index) = map.get(&complement) {
                return vec![prev_index as i32, index as i32];
            }
            map.insert(num, index);
        }
    
        panic!("No two sum solution");
    }
}

#[test]
pub fn test_two_sum () {
    let vec1 = vec![2,7,11,15];
    let vec2 = vec![3,2,4];
    let vec3 = vec![3,3];

    assert_eq!(Solution::two_sum(vec1, 9), vec![0,1]);
    assert_eq!(Solution::two_sum(vec2, 6), vec![1,2]);
    assert_eq!(Solution::two_sum(vec3, 6), vec![0,1]);
}