// Leetcode Problem 2342

use std::collections::{HashMap, HashSet};

struct Solution;

#[allow(dead_code)]
impl Solution {

    // Fast Solution
    pub fn maximum_sum(nums: Vec<i32>) -> i32 {
        let mut max_by_ds = [0; 100];
        let mut max_sum = -1;
        
        for x in nums {
            let mut val = x;
            let mut ds = 0;

            while val > 0 {
                ds += (val % 10) as usize;
                val /= 10;
            }

            let y = max_by_ds[ds];

            if y > 0 {
                max_sum = max_sum.max(x + y);
            }

            max_by_ds[ds] = y.max(x);
        }

        max_sum
    }

    // My solution, but it is very slow
    pub fn maximum_sum_slow(nums: Vec<i32>) -> i32 {
        let mut sums: HashMap<i32, HashSet<usize>> = HashMap::new();
        let mut max_val = -1;

        // Store the sum of digits for every number in nums and map it to the index
        for (idx, num) in nums.iter().enumerate() {
            let mut sum = 0;
            let mut n = *num;

            while n > 0 {
                sum += n % 10;
                n /= 10;
            }

            sums.entry(sum)
                .and_modify(|v| { 
                    v.insert(idx); 
                })
                .or_insert_with(HashSet::new)
                .insert(idx);
        }

        // Calculate the sum by adding all possible pairs that where found and stored in sums
        for value in sums.values() {
            let v = (*value.iter().map(|v| *v).collect::<Vec<usize>>()).to_vec();

            for (i, &a) in v.iter().enumerate() {
                for &b in &v[i + 1..] {
                    if nums[a] + nums[b] > max_val {
                        max_val = nums[a] + nums[b];
                    }
                }
            }
        }

        max_val
    }
}


#[test]
fn test_maximum_sum() {
    let nums1 = vec![18, 43, 36, 13, 7];
    let nums2 = vec![10, 12, 19, 14];
    let nums3 = vec![];
    let nums4 = vec![10, 10, 10, 10];

    assert_eq!(Solution::maximum_sum(nums1), 54);
    assert_eq!(Solution::maximum_sum(nums2), -1);
    assert_eq!(Solution::maximum_sum(nums3), -1);
    assert_eq!(Solution::maximum_sum(nums4), 20);
}