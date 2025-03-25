// Leetcode Problem 3394

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn check_valid_cuts(_n: i32, mut rectangles: Vec<Vec<i32>>) -> bool {
        if rectangles.len() < 3 {
            // Without 3 rectangles we cannot cut the grid into 3 sections where each section contains at least one rectangle
            return false;
        }

        let mut cuts = 0;
        let mut filled = false;

        // Sort the rectangles regarding their start_x coordinate to search for vertical cuts
        rectangles.sort_by(|a, b| a[0].cmp(&b[0]));
        let mut x_cut_pos = 0;

        for rectangle in &rectangles {
            if x_cut_pos <= rectangle[0] && filled {
                // Current position for a cut before a rectangle and we found a rectangle for the section already
                cuts += 1;
                filled = false;
            }

            if x_cut_pos < rectangle[2] {
                // We found a rectangle for the section the next cut would create
                filled = true;
            }

            // Set current cut position to the point behind the current rectangle if it isn't already
            x_cut_pos = x_cut_pos.max(rectangle[2]);

            if cuts == 2 && filled {
                return true;
            }
        }

        cuts = 0;
        filled = false;
        
        // Sort the rectangles redarding their start_z coordinate to search for vertical cuts
        rectangles.sort_by(|a, b| a[1].cmp(&b[1]));
        let mut y_cut_pos = 0;

        for rectangle in &rectangles {
            if y_cut_pos <= rectangle[1] && filled {
                cuts += 1;
                filled = false;
            }

            if y_cut_pos < rectangle[3] {
                filled = true;
            }

            y_cut_pos = y_cut_pos.max(rectangle[3]);

            if cuts == 2 && filled {
                return true;
            }
        }

        false
    }
}

#[test]
pub fn test_check_valid_cuts() {
    let rectangles1 = vec![vec![1, 0, 5, 2], vec![0, 2, 2, 4], vec![3, 2, 5, 3], vec![0, 4, 4, 5]];
    let rectangles2 = vec![vec![0, 0, 1, 1], vec![2, 0, 3, 4], vec![0, 2, 2, 3], vec![3, 0, 4, 3]];
    let rectangles3 = vec![vec![0, 2, 2, 4], vec![1, 0, 3, 2], vec![2, 2, 3, 4], vec![3, 0, 4, 2], vec![3, 2, 4, 4]];
    let rectangles4 = vec![vec![0, 0, 1, 1], vec![1, 0, 2, 1], vec![2, 0, 3, 1]];
    let rectangles5 = vec![vec![0, 0, 1, 1], vec![0, 1, 1, 2], vec![0, 2, 1, 3]];
    let rectangles6 = vec![vec![0, 0, 6, 1], vec![0, 1, 3, 5], vec![3, 1, 4, 5], vec![4, 1, 5, 5], vec![5, 1, 6, 5], vec![0, 5, 6, 6]];
    let rectangles7 = vec![vec![0, 0, 3, 5], vec![3, 0, 4, 5], vec![4, 0, 5, 5], vec![5, 0, 6, 5], vec![6, 0, 7, 5], vec![0, 5, 7, 6], vec![0, 6, 7, 7]];

    assert!(Solution::check_valid_cuts(5, rectangles1));
    assert!(Solution::check_valid_cuts(4, rectangles2));
    assert!(!Solution::check_valid_cuts(4, rectangles3));
    assert!(Solution::check_valid_cuts(3, rectangles4));
    assert!(Solution::check_valid_cuts(3, rectangles5));
    assert!(Solution::check_valid_cuts(6, rectangles6));
    assert!(Solution::check_valid_cuts(7, rectangles7));
}