// Leetcode Problem 1524

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>) -> i32 {
        let mut odd_subarray_coutner = 0;
        let mut even_subarray_counter = 1;
        let mut prefix_sum = 0;
        let mut result = 0;
        let modulo = 1_000_000_007;

        for &num in &arr {
            prefix_sum += num;
            if prefix_sum % 2 == 0 {
                result = (result + odd_subarray_coutner) % modulo;
                even_subarray_counter += 1;
            } else {
                result = (result + even_subarray_counter) % modulo;
                odd_subarray_coutner += 1;
            }
        }

        result as i32
    }
}


#[test]
fn test_num_of_subarrays() {
    let arr1 = vec![1, 3, 5];
    let arr2 = vec![2, 4, 6];
    let arr3 = vec![1, 2, 3, 4, 5, 6, 7];

    assert_eq!(Solution::num_of_subarrays(arr1), 4);
    assert_eq!(Solution::num_of_subarrays(arr2), 0);
    assert_eq!(Solution::num_of_subarrays(arr3), 16);
}