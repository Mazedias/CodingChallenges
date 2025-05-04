// Leetcode Problem 1007

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_domino_rotations(tops: Vec<i32>, bottoms: Vec<i32>) -> i32 {
        let n = tops.len();
        
        // We can either build a equal row with the first number on top or with the first number on the bottom
        // Check if those numbers appear at every index and count how often they are on top and how often on the bottom
        let mut top_count_first_top_number: i32 = 0;
        let mut bottom_count_first_top_number: i32 = 0;
        let mut equal_count_first_top_number: i32 = 0;
        let mut bottom_count_first_bottom_number: i32 = 0;
        let mut top_count_first_bottom_number: i32 = 0;
        let mut equal_count_first_bottom_number: i32 = 0;

        for i in 0..n {
            if tops[i] == bottoms[i]  && tops[i] == tops[0] {
                equal_count_first_top_number += 1;
            } else {
                if tops[i] == tops[0] {
                top_count_first_top_number += 1;
                }
                if bottoms[i] == tops[0] {
                    bottom_count_first_top_number += 1;
                }
            }
            
            if bottoms[i] == tops[i] && bottoms[i] == bottoms[0] {
                equal_count_first_bottom_number += 1;
            } else {
                if bottoms[i] == bottoms[0] {
                    bottom_count_first_bottom_number += 1;
                }
                if tops[i] == bottoms[0] {
                    top_count_first_bottom_number += 1;
                }
            }
        }

        if top_count_first_top_number + bottom_count_first_top_number + equal_count_first_top_number != n as i32 && top_count_first_bottom_number + bottom_count_first_bottom_number+ equal_count_first_bottom_number != n as i32 {
            return -1;
        } else if top_count_first_top_number + bottom_count_first_top_number + equal_count_first_top_number != n as i32 {
            return bottom_count_first_bottom_number.min(top_count_first_bottom_number);
        } else if top_count_first_bottom_number + bottom_count_first_bottom_number+ equal_count_first_bottom_number != n as i32{
            return top_count_first_top_number.min(bottom_count_first_top_number);
        } else {
            return bottom_count_first_bottom_number.min(top_count_first_bottom_number).min(top_count_first_top_number).min(bottom_count_first_top_number);
        }
    }
}


#[test]
fn test_min_domino_rotation() {
    let tops1 = vec![2,1,2,4,2,2];
    let bottoms1 = vec![5,2,6,2,3,2];

    let tops2 = vec![3,5,1,2,3];
    let bottoms2 = vec![3,6,3,3,4];

    let tops3 = vec![1, 1, 1, 1, 1, 1];
    let bottoms3 = vec![2, 2, 2, 2, 2, 2];

    let tops4 = vec![1, 1, 1, 1, 1, 1];
    let bottoms4 = vec![1, 1, 1, 1, 1, 1];

    let tops5 = vec![1, 2, 1, 2, 1, 2];
    let bottoms5 = vec![2, 1, 2, 1, 2, 1];

    assert_eq!(Solution::min_domino_rotations(tops1, bottoms1), 2);
    assert_eq!(Solution::min_domino_rotations(tops2, bottoms2), -1);
    assert_eq!(Solution::min_domino_rotations(tops3, bottoms3), 0);
    assert_eq!(Solution::min_domino_rotations(tops4, bottoms4), 0);
    assert_eq!(Solution::min_domino_rotations(tops5, bottoms5), 3);
}