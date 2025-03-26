// Leetcode Problem 2033

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_operations(grid: Vec<Vec<i32>>, x: i32) -> i32 {
        // Flatten the grid into a 1D vector
        let mut values: Vec<i32> = grid.iter().flat_map(|row| row.iter()).cloned().collect();

        values.sort();

        // Check whether the difference between the first value and all other values is not divisble by 0
        // If so we cannot equal all numbers and return -1
        if values.iter().any(|&val| (val - values[0]).abs() % x != 0) {
            return -1;
        }

        // Calculate the needed operation to bring every number in the grid up/down to the median
        let median = values[values.len() / 2];
        values.iter().map(|&val| (val - median).abs() / x).sum()
    }
}

#[test]
pub fn test_min_operations() {
    let grid1 = vec![
        vec![2, 4],
        vec![6, 8]
    ];
    let grid2 = vec![
        vec![1, 5],
        vec![2, 3]
    ];
    let grid3 = vec![
        vec![1, 2],
        vec![3, 4]
    ];
    let grid4 = vec![
        vec![980, 476 ,644, 56],
        vec![644, 140, 812, 308],
        vec![812, 812, 896, 560],
        vec![728, 476, 56, 812]
    ];
    let grid5 = vec![
        vec![596, 904, 960, 232, 120, 932, 176],
        vec![372, 792, 288, 848, 960, 960, 764],
        vec![652,  92, 904, 120, 680, 904, 120],
        vec![372, 960,  92, 680, 876, 624, 904],
        vec![176, 652,  64, 344, 316, 764, 316],
        vec![820, 624, 848, 596, 960, 960, 372],
        vec![708, 120, 456,  92, 484, 932, 540]
    ];

    assert_eq!(Solution::min_operations(grid1, 2), 4);
    assert_eq!(Solution::min_operations(grid2, 1), 5);
    assert_eq!(Solution::min_operations(grid3, 2), -1);
    assert_eq!(Solution::min_operations(grid4, 84), 45);
    assert_eq!(Solution::min_operations(grid5, 28), 473);
}