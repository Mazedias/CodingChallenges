// Leetcode Problem 407

use std::{cmp::Reverse, collections::BinaryHeap};

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
        let mut total = 0;

        let n = height_map.len();
        let m = height_map[0].len();

        let mut q = BinaryHeap::new();
        let mut level = vec![vec![i32::MAX; m]; n];

        // Add all edge tiles to the queue
        for i in 0..n {
            for &j in &[0, m-1] {
                level[i][j] = height_map[i][j];
                q.push((Reverse(level[i][j]), i, j));
            }
        }
        for j in 1..(m-1) {
            for &i in &[0, n-1] {
                level[i][j] = height_map[i][j];
                q.push((Reverse(level[i][j]), i, j));
            }
        }

        while let Some((Reverse(x), i, j)) = q.pop() {
            let ip = i + 1;
            let im = i.wrapping_sub(1);
            let jp = j + 1;
            let jm = j.wrapping_sub(1);

            for &(ii, jj) in &[(ip, j), (im, j), (i, jp), (i, jm)] {
                if ii < n && jj < m && level[ii][jj] == i32::MAX {
                    level[ii][jj] = x;
                    q.push((Reverse(x.max(height_map[ii][jj])), ii, jj));
                }
            }
        }

        for i in 0..n {
            for j in 0..m {
                total += (level[i][j] - height_map[i][j]).max(0);
            }
        }

        total
    }
}


#[test]
pub fn test_trap_rain_water() {
    let height_map1 = vec![
        vec![1, 4, 3, 1, 3, 2],
        vec![3, 2, 1, 3, 2, 4],
        vec![2, 3, 3, 2, 3, 1]
    ];

    let height_map2 = vec![
        vec![3, 3, 3, 3, 3],
        vec![3, 2, 2, 2, 3],
        vec![3, 2, 1, 2, 3],
        vec![3, 2, 2, 2, 3],
        vec![3, 3, 3, 3, 3]
    ];

    let height_map3 = vec![
        vec![3, 7, 4, 5],
        vec![3, 2, 0, 3],
        vec![0, 4, 2, 1]
    ];

    assert_eq!(Solution::trap_rain_water(height_map1), 4);
    assert_eq!(Solution::trap_rain_water(height_map2), 10);
    assert_eq!(Solution::trap_rain_water(height_map3), 2);
}