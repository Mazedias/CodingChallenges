// Problem 35
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn search_insert_position (nums: Vec<i32>, target: i32) -> i32 {
        let mut i = 0;

        if nums[0] >= target {
            return 0;
        }

        while i < nums.len()-1 {
            if nums[i] == target {
                return i as i32;
            } else if nums[i+1] >= target && nums[i] < target {
                return (i+1) as i32;
            }
            i += 1;
        }

        nums.len() as i32
    }

    /**
     * Better Solution
     */
    pub fn search_insert_position_b (nums: Vec<i32>, target: i32) -> i32 {
        let mut s = 0;
        for i in 0..nums.len() {
            if nums[i] < target {
                s += 1;
            }
        }
        s
    }
}

#[test]
pub fn test_search_inser_position () {
    let nums = vec![1,3,5,6];

    assert_eq!(Solution::search_insert_position(nums.clone(), 5), 2);
    assert_eq!(Solution::search_insert_position(nums.clone(), 2), 1);
    assert_eq!(Solution::search_insert_position(nums.clone(), 7), 4);

    assert_eq!(Solution::search_insert_position(nums.clone(), 1), 0);
    assert_eq!(Solution::search_insert_position(nums.clone(), 0), 0);
}