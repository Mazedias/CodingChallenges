// Leetcode Problem 2929

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn distribute_candies(n: i32, limit: i32) -> i64 {
        if n > limit*3 {
            return 0 as i64;
        }
        if n == limit*3 {
            return 1 as i64;
        }

        let (n, limit) = (n as i64, limit as i64);

        let mut answer: i64 = 0;

        for i in 0..=limit.min(n) {
            if n - i <= 2 * limit {
                answer += limit.min(n - i) - 0.max(n-i-limit) + 1;
            }
        }

        answer
    }

    // My solution but is too slow
    pub fn my_distribute_candies(n: i32, limit: i32) -> i64 {
        let mut counter: i64 = 0;

        for i in 0..=limit.min(n) {
            for j in 0..=limit.min(n - i) {
                // Now the first kid has i candies and the second kid as j candies therefor the third kid has n-i-j candies

                if n-i-j > limit || n-i-j < 0 {
                    continue;
                }
                counter += 1;
            }
        }

        counter
    }
}

#[test]
fn test_distribute_candies() {
    assert_eq!(Solution::distribute_candies(5, 2), 3);
    assert_eq!(Solution::distribute_candies(3, 3), 10);
}