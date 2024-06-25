// Problem 9
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_palindrom (x: i32) -> bool {
        if x < 0 { return false; }
    
        let mut num: i32 = x;
        let mut rev: i32 = 0;

        while num > 0 {
            let dig = num % 10;
            rev = rev * 10 + dig;
            num /= 10;
        }
        
        return rev == x;
    }
}

#[test]
pub fn test_is_palindrom () {
    assert!(Solution::is_palindrom(121));
    assert!(!Solution::is_palindrom(-121));
    assert!(!Solution::is_palindrom(10));
}