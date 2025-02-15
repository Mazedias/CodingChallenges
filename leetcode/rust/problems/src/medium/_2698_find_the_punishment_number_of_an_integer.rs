// Leetcode Problem 2698

struct Solution;

fn f(i: i32, x: i32) -> bool {
    if x == i {
        return true;
    }
    let mut b = 10;
    while b < x {
        let j = x % b;
        let y = x / b;
        if j <= i && f(i-j, y) {
            return true;
        }
        b *= 10;
    }
    false
}

#[allow(dead_code)]
impl Solution {
    pub fn punishment_number(n: i32) -> i32 {
        (1..=n).filter_map(|i| f(i, i*i).then_some(i*i)).sum()
    }
}

#[test]
pub fn test_punishment_number() {
    assert_eq!(Solution::punishment_number(8), 1);
    assert_eq!(Solution::punishment_number(10), 182);
    assert_eq!(Solution::punishment_number(37), 1478);
    assert_eq!(Solution::punishment_number(40), 1478);
    assert_eq!(Solution::punishment_number(54), 3503);
    assert_eq!(Solution::punishment_number(414), 772866);
    assert_eq!(Solution::punishment_number(656), 772866);
    assert_eq!(Solution::punishment_number(989), 6844475);
    assert_eq!(Solution::punishment_number(990), 7824575);
    assert_eq!(Solution::punishment_number(991), 8806656);
    assert_eq!(Solution::punishment_number(999), 9804657);
    assert_eq!(Solution::punishment_number(1000), 10804657);
}