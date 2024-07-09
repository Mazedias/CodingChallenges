// Problem 1823
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_the_winner (n: i32, k: i32) -> i32 {
        let mut p: i32 = 0;
        let mut nums: Vec<i32>       = (1..=n).collect();

        while nums.len() > 1 {
            p = (p + k - 1) % (nums.len() as i32);

            nums.remove(p as usize);
            
            if p > (nums.len() as i32) - 1 {
                p = p % (nums.len() as i32);
            }
        }

        nums[0]
    }
}

#[test]
pub fn test_find_the_winner () {
    assert_eq!(Solution::find_the_winner(5, 2), 3);
    assert_eq!(Solution::find_the_winner(6, 5), 1);
}
