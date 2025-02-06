// Leetcode Problem 1726

use std::collections::HashMap;

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn tuple_same_product(nums: Vec<i32>) -> i32 {
        let mut products: HashMap<i32, i32> = HashMap::new();

        for i in 0..nums.len() {
            for j in i+1..nums.len() {
                if j == i {
                    continue;
                }

                let product = nums[i] * nums[j];

                *products.entry(product).or_insert(0) += 1;
            }
        }

        let mut sum = 0;
        for &v in products.values() {
            if v > 1 {
                sum += ((v - 1) * 4) * v;
            }
        }

        sum
    }
}


#[test]
fn test_tuple_same_product() {
    let nums1 = vec![2, 3, 4, 6];
    let nums2 = vec![1, 2, 4, 5, 10];
    let nums3 = vec![1, 2, 4, 5, 10, 20];

    assert_eq!(Solution::tuple_same_product(nums1), 8);
    assert_eq!(Solution::tuple_same_product(nums2), 16);
    assert_eq!(Solution::tuple_same_product(nums3), 40);
}