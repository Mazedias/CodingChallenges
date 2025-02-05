// Leetcode Problem 1790

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn are_almost_equal(s1: String, s2: String) -> bool {
        let mut pos1: i32 = -1;
        let mut pos2: i32 = -1;

        for i in 0..s1.len() {
            if s1.chars().nth(i) != s2.chars().nth(i) {
                if pos1 == -1 {
                    pos1 = i as i32;
                } else if pos2 == -1 {
                    pos2 = i as i32;
                } else {
                    return false;
                }
            }
        }

        if pos1 != -1 && pos2 == -1 {
            return false;
        }

        (s1.chars().nth(pos1 as usize) == s2.chars().nth(pos2 as usize)) && (s2.chars().nth(pos1 as usize) == s1.chars().nth(pos2 as usize))
    }
}


#[test]
fn test_are_almost_equal() {
    let s11 = String::from("bank");
    let s12 = String::from("kanb");

    let s21 = String::from("attack");
    let s22 = String::from("defend");

    let s31 = String::from("kelb");
    let s32 = String::from("kelb");

    let s41 = String::from("ahusgbejsh");
    let s42 = String::from("ahusgbjesh");

    let s51 = String::from("a");
    let s52 = String::from("b");

    assert!(Solution::are_almost_equal(s11, s12));
    assert!(!Solution::are_almost_equal(s21, s22));
    assert!(Solution::are_almost_equal(s31, s32));
    assert!(Solution::are_almost_equal(s41, s42));
    assert!(!Solution::are_almost_equal(s51, s52));
}