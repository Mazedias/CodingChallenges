// Leetcode Problem 3341

use std::{cmp::Reverse, collections::BinaryHeap};

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_time_to_reach(move_time: Vec<Vec<i32>>) -> i32 {
        let n = move_time.len();
        let m = move_time[1].len();

        // Use a distance vector to keep track of distances to each position
        let mut distance = vec![vec![None; m]; n];
        // Use a priority queue to always get the position with the shortest time to reach
        let mut q = BinaryHeap::new();

        distance[0][0] = Some(0);
        q.push(Reverse((0, 0, 0)));

        // Loop as long as the priortiy queue is not empty
        while let Some(Reverse((time, row, col))) = q.pop() {
            // Skip if we already found a faster way to this position
            if distance[row][col].unwrap() < time {
                continue;
            }

            // Calculate the neighbors positions
            let neighbors = [
                (row.saturating_sub(1), col),
                (row, col.saturating_sub(1)),
                ((row + 1).min(n - 1), col),
                (row, (col + 1).min(m - 1)),
            ];

            for (r, c) in neighbors {
                // Earliest time we can enter this neighbor is the maximum of given minimum in move_time and the current time
                // SInce moving takes 1 second we add it at the end
                let next_time = move_time[r][c].max(time) + 1;

                // Update the distance if it is the first time that we reach this cell or if the path we found is faster
                if distance[r][c].map_or(true, |prev| next_time < prev) {
                    distance[r][c] = Some(next_time);
                    q.push(Reverse((next_time, r, c)));
                }
            }
        }

        distance[n-1][m-1].unwrap()
    }
}

#[test]
fn test_min_time_to_reach() {
    let move_time1 = vec![vec![0, 4], vec![4, 4]];
    let move_time2 = vec![vec![0, 0, 0], vec![0, 0, 0]];

    assert_eq!(Solution::min_time_to_reach(move_time1), 6);
    assert_eq!(Solution::min_time_to_reach(move_time2), 3);
}