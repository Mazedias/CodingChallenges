// Leetcode Problem 1277
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_squares(mut matrix: Vec<Vec<i32>>) -> i32 {
        let m = matrix.len();
        if m == 0 { return 0; }
        let n = matrix[0].len();
        let mut total = 0;

        for i in 0..m {
            for j in 0..n {
                if matrix[i][j] == 1 && j > 0 && i > 0 {
                    let top_left = matrix[i-1][j-1];
                    let top = matrix[i-1][j];
                    let left = matrix[i][j-1];
                    matrix[i][j] = 1 + top_left.min(top).min(left);
                }
                total += matrix[i][j];
            }
        }
        total
    }
}

#[test]
fn test_count_squares() {
    let matrix1: Vec<Vec<i32>> = vec![
        vec![0, 1, 1, 1],
        vec![1, 1, 1, 1],
        vec![0, 1, 1, 1]
    ];

    let matrix2: Vec<Vec<i32>> = vec![
        vec![1, 0, 1],
        vec![1, 1, 0],
        vec![1, 1, 0]
    ];

    assert_eq!(Solution::count_squares(matrix1), 15);
    assert_eq!(Solution::count_squares(matrix2), 7);
}
