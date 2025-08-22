// Leetcode Problem 3195
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn minimum_area(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        
        let mut top = n;
        let mut left = m;
        let mut right = 0;
        let mut bottom = 0;

        for i in 0..n {
            for j in 0..m {
                // Check whether the enclosing positions have to be updated
                if grid[i][j] == 1 {
                    if i < top {
                        top = i;
                    }
                    if i > bottom {
                        bottom = i;
                    }
                    if j < left {
                        left = j;
                    }
                    if j > right {
                        right = j;
                    }
                }
            }
        }
        (right as i32 - left as i32 + 1) * (bottom as i32 - top as i32 + 1)
    }
}

#[test]
fn test_minimum_area() {
    let grid1 = vec![
        vec![0, 1, 0],
        vec![1, 0, 1]
    ];

    let grid2 = vec![
        vec![1, 0],
        vec![0, 0]
    ];

    let grid3 = vec![
        vec![0, 0, 0, 0],
        vec![0, 1, 0, 0],
        vec![0, 1, 1, 0],
        vec![0, 1, 0, 1]
    ];

    let grid4 = vec![
        vec![0, 0, 0, 1],
        vec![0, 0, 0, 0],
        vec![0, 0, 1, 0],
        vec![0, 0, 1, 1]
    ];

    let grid5 = vec![
        vec![0, 0, 0],
        vec![0, 1, 1],
        vec![1, 0, 0],
        vec![0, 1, 1]
    ];

    assert_eq!(Solution::minimum_area(grid1), 6);
    assert_eq!(Solution::minimum_area(grid2), 1);
    assert_eq!(Solution::minimum_area(grid3), 9);
    assert_eq!(Solution::minimum_area(grid4), 8);
    assert_eq!(Solution::minimum_area(grid5), 9);
}
