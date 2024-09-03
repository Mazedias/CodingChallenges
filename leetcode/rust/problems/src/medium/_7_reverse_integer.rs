// Problem 7
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn reverse (mut x: i32) -> i32 {
        let mut rev: i32 = 0;
        let mut remainder;
        let mut neg = 1;

        if x < 0 {
            neg = -1;
            x *= -1;
        }

        while x > 0 {
            remainder = x % 10;

            let result = rev.checked_mul(10).and_then(|n| n.checked_add(remainder));

            match result {
                Some(new_value) => rev = new_value,
                None => return 0,
            }
            x = x / 10;
        }

        rev * neg
    }
}

#[test]
pub fn test_reverse () {
    assert_eq!(Solution::reverse(123), 321);
    assert_eq!(Solution::reverse(-123), -321);
    assert_eq!(Solution::reverse(120), 21);
    assert_eq!(Solution::reverse(i32::MAX), 0);
}