// Leetcode Problem 2661

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn first_complete_index(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32 {
        let m = mat.len();
        let n = mat[0].len();

        let mut frequency_row = vec![0; n];
        let mut frequency_column = vec![0; m];
        let mut positions: Vec<(usize, usize)> = vec![(0, 0); m*n];

        for i in 0..m {
            for j in 0..n {
                positions[mat[i][j] as usize - 1] = (i, j);
            }
        }

        for (pos, num) in arr.iter().enumerate() {
            let position = positions[*num as usize - 1];

            frequency_column[position.0] += 1;
            frequency_row[position.1] += 1;

            if frequency_row[position.1] == m || frequency_column[position.0] == n {
                return pos as i32;
            }
        }

        (arr.len() - 1) as i32
    }
}


#[test]
fn test_first_complete_index() {
    let arr1 = vec![1, 3, 4, 2];
    let mat1 = vec![
        vec![1, 4],
        vec![2, 3]
    ];

    let arr2 = vec![2, 8, 7, 4, 1, 3, 5, 6, 9];
    let mat2 = vec![
        vec![3, 2, 5],
        vec![1, 4, 6],
        vec![8, 7, 9]
    ];

    let arr3 = vec![1, 4, 5, 2, 6, 3];
    let mat3 = vec![
        vec![4, 3, 5],
        vec![1, 2, 6]
    ];

    let arr4 = vec![6, 2, 3, 1, 4, 5];
    let mat4 = vec![
        vec![5, 1],
        vec![2, 4],
        vec![6, 3]
    ];

    assert_eq!(Solution::first_complete_index(arr1, mat1), 2);
    assert_eq!(Solution::first_complete_index(arr2, mat2), 3);
    assert_eq!(Solution::first_complete_index(arr3, mat3), 1);
    assert_eq!(Solution::first_complete_index(arr4, mat4), 2);
}