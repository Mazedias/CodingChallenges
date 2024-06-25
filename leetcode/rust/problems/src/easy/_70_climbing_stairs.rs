// Problem 70
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn climb_stairs (n: i32) -> i32 {
        let (mut a, mut b) = (1, 1);

        for _ in 2..=n {
            (a, b) = (b, a+b);
        }

        b
    }

    // This was my first idea but is much slower
    pub fn recursion_climb_stairs (n: i32) -> i32 {
        if n == 1 || n == 2 {
            return n;           
        }

        let one_step = Self::climb_stairs(n-1);
        let two_step = Self::climb_stairs(n-2);

        one_step + two_step
    }
}

#[test]
pub fn test_climb_stairs () {
    assert_eq!(Solution::climb_stairs(2), 2);
    assert_eq!(Solution::climb_stairs(3), 3);
    assert_eq!(Solution::climb_stairs(4), 5);
    assert_eq!(Solution::climb_stairs(5), 8);
    assert_eq!(Solution::climb_stairs(45), 1836311903);
}