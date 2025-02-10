// Leetcode Problem 2364

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_bad_pairs(nums:Vec<i32>) -> i64 {
        let n = nums.len() as i64;

        let mut pos_diff: Vec<_> = nums.into_iter()
            .enumerate()
            .map(|(idx, num)| num - idx as i32)
            .collect();
        
        pos_diff.sort_unstable();
        
        let good_pairs: i64 = pos_diff
            .chunk_by(|a, b| a == b)
            .map(|chunk| (chunk.len() as i64 * (chunk.len() as i64 - 1)) / 2)
            .sum();

        let total_pairs = n * (n-1) / 2;
        total_pairs - good_pairs
    }
}


#[test]
fn test_count_bad_pairs() {
    let nums1 = vec![4, 1, 3, 3];
    let nums2 = vec![1, 2, 3, 4, 5];

    assert_eq!(Solution::count_bad_pairs(nums1), 5);
    assert_eq!(Solution::count_bad_pairs(nums2), 0);
}