// Leetcode Problem 2017

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn grid_game(grid: Vec<Vec<i32>>) -> i64 {
        let n = grid[0].len();

        let mut top_sum: i64 = grid[0].iter().map(|num| *num as i64).sum::<i64>() - grid[0][0] as i64;
        let mut bot_sum: i64 = 0;
        let mut combinations = vec![0; n];

        combinations[0] = i64::max(top_sum, bot_sum);

        for i in 1..n {
            top_sum -= grid[0][i] as i64;
            bot_sum += grid[1][i - 1] as i64;

            combinations[i] = i64::max(top_sum, bot_sum);
        }

        combinations.into_iter().min().unwrap()
    }
}


#[test]
fn test_grid_game() {
    let grid1 = vec![
        vec![2, 5, 4],
        vec![1, 5, 1]
    ];

    let grid2 = vec![
        vec![3, 3, 1],
        vec![8, 5, 2]
    ];

    let grid3 = vec![
        vec![1, 3, 1, 15],
        vec![1, 3, 3, 1]
    ];

    let grid4 = vec![
        vec![20,  3, 20, 17,  2, 12, 15, 17,  4, 15],
        vec![20, 10, 13, 14, 15,  5,  2,  3, 14,  3]
    ];

    let grid5 = vec![
        vec![100, 50, 50, 50, 10, 1],
        vec![  1,  1,  1,  1,  1, 1]
    ];

    assert_eq!(Solution::grid_game(grid1), 4);
    assert_eq!(Solution::grid_game(grid2), 4);
    assert_eq!(Solution::grid_game(grid3), 7);
    assert_eq!(Solution::grid_game(grid4), 63);
    assert_eq!(Solution::grid_game(grid5), 4);
}