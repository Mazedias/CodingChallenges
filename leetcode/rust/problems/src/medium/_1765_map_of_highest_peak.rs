// Leetcode Problem 1765

use std::collections::VecDeque;

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn highest_peak(is_water: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = is_water.len();
        let n = is_water[0].len();
        
        let mut hights: Vec<Vec<i32>> = vec![vec![-1; n]; m];

        let mut queue = VecDeque::new();
        for i in 0..m {
            for j in 0..n {
                if is_water[i][j] == 1 {
                    hights[i][j] = 0;
                    queue.push_back((i, j));
                }
            }
        }

        while let Some((m_pos ,n_pos)) = queue.pop_front() {
            for &(x, y) in &[
                (m_pos.wrapping_sub(1), n_pos),
                (m_pos + 1, n_pos),
                (m_pos, n_pos.wrapping_sub(1)),
                (m_pos, n_pos + 1)
            ] {
                if x < m && y < n && hights[x][y] == -1 {
                    hights[x][y] = hights[m_pos][n_pos] + 1;
                    queue.push_back((x, y));
                }
            }
        }

        hights
    }
}


#[test]
fn test_highest_peak_case1() {
    let is_water = vec![
        vec![0, 1],
        vec![0, 0]
    ];

    let expected_hights = vec![
        vec![1, 0],
        vec![2, 1]
    ];

    assert_eq!(Solution::highest_peak(is_water), expected_hights);
}

#[test]
fn test_highest_peak_case2() {
    let is_water = vec![
        vec![0, 0, 1],
        vec![1, 0, 0],
        vec![0, 0, 0]
    ];

    let expected_hights = vec![
        vec![1, 1, 0],
        vec![0, 1, 1],
        vec![1, 2, 2]
    ];

    assert_eq!(Solution::highest_peak(is_water), expected_hights);
}

#[test]
fn test_highest_peak_case3() {
    let is_water = vec![
        vec![0, 0, 0, 0, 0],
        vec![1, 0, 0, 0, 1],
        vec![0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0],
        vec![0, 1, 1, 0, 0],
        vec![0, 0, 0, 0, 1]
    ];

    let expected_hights = vec![
        vec![1, 2, 3, 2, 1],
        vec![0, 1, 2, 1, 0],
        vec![1, 2, 2, 2, 1],
        vec![2, 1, 1, 2, 2],
        vec![1, 0, 0, 1, 1],
        vec![2, 1, 1, 1, 0]
    ];

    assert_eq!(Solution::highest_peak(is_water), expected_hights);
}