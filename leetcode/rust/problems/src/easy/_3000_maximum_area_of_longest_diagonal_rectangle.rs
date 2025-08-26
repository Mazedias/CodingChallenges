// Leetcode Problem 3000

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn area_of_max_diagonal(dimensions: Vec<Vec<i32>>) -> i32 {
        let mut longest_diagonal = 0.0;
        let mut area = 0;

        for rectangle in dimensions {
            let diagonal = f64::sqrt(rectangle[0] as f64 * rectangle[0] as f64 + rectangle[1] as f64 * rectangle[1] as f64);
            if diagonal > longest_diagonal {
                longest_diagonal = diagonal;
                area = rectangle[0] * rectangle[1];
            } else if diagonal == longest_diagonal {
                area = area.max(rectangle[0] * rectangle[1]);
            }
        }

        area
    }
}


#[test]
fn test_area_of_max_diagonal() {
    let dimensions1 = vec![vec![9, 3], vec![8, 6]];
    let dimensions2 = vec![vec![3, 4], vec![3, 4]];

    assert_eq!(Solution::area_of_max_diagonal(dimensions1), 48);
    assert_eq!(Solution::area_of_max_diagonal(dimensions2), 12);
}