use std::collections::HashSet;

// Problem 2257
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_unguarded(m: i32, n: i32, guards: Vec<Vec<i32>>, walls: Vec<Vec<i32>>) -> i32 {
        let mut grid: Vec<Vec<i32>> = vec![vec![0; n as usize]; m as usize];
        let mut unguarded = m * n;
        let mut guarded: HashSet<(usize, usize)> = HashSet::new();

        for wall in &walls {
            unguarded -= 1;

            grid[wall[0] as usize][wall[1] as usize] = 1;
        }

        for guard in &guards {
            unguarded -= 1;

            grid[guard[0] as usize][guard[1] as usize] = 2;
        }

        for guard in guards {
            let pos_m = guard[0];
            let pos_n = guard[1];

            let mut north = 1;
            let mut east = 1;
            let mut south = 1;
            let mut west = 1;

            while pos_n - north >= 0 && grid[pos_m as usize][(pos_n - north) as usize] == 0 {
                guarded.insert((pos_m as usize, (pos_n - north) as usize));
                north += 1;
            }

            while pos_m + east < m && grid[(pos_m + east) as usize][pos_n as usize] == 0 {
                guarded.insert(((pos_m + east) as usize, pos_n as usize));
                east += 1;
            }

            while pos_n + south < n && grid[pos_m as usize][(pos_n + south) as usize] == 0 {
                guarded.insert((pos_m as usize, (pos_n + south) as usize));
                south += 1;
            }

            while pos_m - west >= 0 && grid[(pos_m - west) as usize][pos_n as usize] == 0 {
                guarded.insert(((pos_m - west) as usize, pos_n as usize));
                west += 1;
            }
        }

        unguarded - guarded.len() as i32
    }
}

#[test]
pub fn test_count_unguarded() {
    let guards1 = vec![vec![0,0], vec![1, 1], vec![2, 3]];
    let walls1 = vec![vec![0,1], vec![2, 2], vec![1, 4]];

    let guards2 = vec![vec![1, 1]];
    let walls2 = vec![vec![0, 1], vec![1, 0], vec![2, 1], vec![1, 2]];

    assert_eq!(Solution::count_unguarded(4, 6, guards1, walls1), 7);
    assert_eq!(Solution::count_unguarded(3, 3, guards2, walls2), 4);
}