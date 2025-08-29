// Leetcode Problem 3021

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn flower_game(n: i32, m: i32) -> i64 {
        let uneven_n = if n % 2 == 1 { ((n+1)/2) as i64 } else { (n/2) as i64 };
        let even_n = (n/2) as i64;
        let unevem_m = if m % 2 == 1 { ((m+1)/2) as i64 } else { (m/2) as i64 };
        let even_m = (m/2) as i64;

        (uneven_n * even_m) + (even_n * unevem_m)
    }
}

#[test]
fn test_flower_game() {
    assert_eq!(Solution::flower_game(3, 2), 3);
    assert_eq!(Solution::flower_game(1, 1), 0);
    assert_eq!(Solution::flower_game(5, 5), 12);
    assert_eq!(Solution::flower_game(100000, 100000), 5000000000);
}
