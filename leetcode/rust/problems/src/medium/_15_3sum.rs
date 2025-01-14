// Leetcode Problem 15

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut triplets: Vec<Vec<i32>> = Vec::new();

        nums.sort();

        for i in 0..nums.len() {         
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let mut left: usize = i + 1;
            let mut right: usize = nums.len() - 1;

            while left < right {
                let sum = nums[i] + nums[left] + nums[right];

                if sum > 0 {
                    right -= 1;
                } else if sum < 0 {
                    left += 1;
                } else {
                    triplets.push(vec![nums[i], nums[left], nums[right]]);
                    left += 1;

                    while nums[left] == nums[left - 1] && left < right {
                        left += 1;
                    }
                }
            }
        }

        triplets
    }
}


#[test]
fn test_three_sum() {
    let nums1: Vec<i32> = vec![-1, 0, 1, 2, -1, -4];
    let expected1: Vec<Vec<i32>> = vec![vec![-1, -1, 2], vec![-1, 0, 1]];

    let nums2: Vec<i32> = vec![0, 1, 1];
    let expected2: Vec<Vec<i32>> = vec![];

    assert_eq!(Solution::three_sum(nums1), expected1);
    assert_eq!(Solution::three_sum(nums2), expected2);
}