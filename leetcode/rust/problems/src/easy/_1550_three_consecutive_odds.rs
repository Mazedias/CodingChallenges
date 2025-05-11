// Leetcode Problem 1550

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
        let mut count_odd = 0;
        for n in &arr {
            if n % 2 == 1 {
                count_odd += 1;
                if count_odd == 3 {
                    return true;
                }
            } else {
                count_odd = 0;
            }
        }
        false
    }
}


#[test]
fn test_three_consecutive_odds() {
    let arr1 = vec![2, 6, 4, 1];
    let arr2 = vec![1, 2, 34, 3, 4, 5, 7, 23, 12];

    assert!(!Solution::three_consecutive_odds(arr1));
    assert!(Solution::three_consecutive_odds(arr2));
}