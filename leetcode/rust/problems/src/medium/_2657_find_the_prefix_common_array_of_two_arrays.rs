// Leetcode Problem 2657

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let n: usize = a.len();
        let mut frequency: Vec<i32> = vec![0; n+1];
        let mut solution: Vec<i32> = vec![0; n];

        let mut common: i32 = 0;
        for i in 0..n {
            if { frequency[a[i] as usize] += 1; frequency[a[i] as usize] } == 2 {
                common += 1;
            }

            if { frequency[b[i] as usize] += 1; frequency[b[i] as usize] } == 2 {
                common += 1;
            }

            solution[i] = common;
        }

        solution
    }
}


#[test]
fn test_find_the_prefix_common_array() {
    let a1 = vec![1, 3, 2, 4];
    let b1 = vec![3, 1, 2, 4];
    let expected1 = vec![0, 2, 3, 4];

    let a2 = vec![2, 3, 1];
    let b2 = vec![3, 1, 2];
    let expected2 = vec![0, 1, 3];

    assert_eq!(Solution::find_the_prefix_common_array(a1, b1), expected1);
    assert_eq!(Solution::find_the_prefix_common_array(a2, b2), expected2);
}