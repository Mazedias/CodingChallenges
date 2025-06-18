// Leetcode Problem 2966

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn divide_array(mut nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        nums.sort();

        let n = nums.len();
        let mut solution: Vec<Vec<i32>> = Vec::new();

        for i in 0..n/3 {
            if nums[i * 3 + 2] - nums[i * 3] > k {
                return vec![];
            }
            solution.push(vec![nums[i*3], nums[i*3+1], nums[i*3+2]]);
        }

        solution
    }
}


#[test]
fn test_divide_array() {
    let nums1 = vec![1, 3, 4, 8, 7, 9, 3, 5, 1];
    // let expected1 = vec![vec![1, 1, 3], vec![3, 4, 5], vec![7, 8, 9]];

    let nums2 = vec![2, 4, 2, 2, 5, 2];
    // let expected2: Vec<Vec<i32>> = vec![];
    
    let num3 = vec![4, 2, 9, 8, 2, 12, 7, 12, 10, 5, 8, 5, 5, 7, 9, 2, 5, 11];
    // let expected3 = vec![vec![2, 2, 12], vec![4, 8, 5], vec![5, 9, 7], vec![7, 8, 5], vec![5, 9, 10], vec![11, 12, 2]];

    assert!(check_array(&mut Solution::divide_array(nums1, 2), 2));
    assert!(check_array(&mut Solution::divide_array(nums2, 2), 2));
    assert!(check_array(&mut Solution::divide_array(num3, 14), 14));
}

#[allow(dead_code)]
fn check_array(arr: &mut Vec<Vec<i32>>, k: i32) -> bool {
    for v in arr {
        v.sort();
        if v[2] - v[0] > k {
            return false;
        }
    }
    true
}