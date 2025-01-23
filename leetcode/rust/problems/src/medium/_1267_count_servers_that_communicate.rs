// Leetcode Problem 1267

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        let mut rows = vec![0; m];
        let mut columns = vec![0; n];

        let mut sum = 0;

        for j in 0..m {
            for i in 0..n {
                if grid[j][i] == 1 {
                    rows[j] += 1;
                    columns[i] += 1;
                    sum += 1;
                }
            }
        }

        for j in 0..m {
            for i in 0..n {
                if grid[j][i] == 1 && rows[j] == 1 && columns[i] == 1 {
                    sum -= 1;
                }
            }
        }

        sum
    }
}


#[test]
fn test_count_servers() {
    let grid1 = vec![
        vec![1, 0],
        vec![0, 1]
    ];

    let grid2 = vec![
        vec![1, 0],
        vec![1, 1]
    ];

    let grid3 = vec![
        vec![1, 1, 0, 0],
        vec![0, 0, 1, 0],
        vec![0, 0, 1, 0],
        vec![0, 0, 0, 1]
    ];

    assert_eq!(Solution::count_servers(grid1), 0);
    assert_eq!(Solution::count_servers(grid2), 3);
    assert_eq!(Solution::count_servers(grid3), 4);
}