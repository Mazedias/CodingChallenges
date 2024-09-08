// Problem 219
struct Solution;

#[allow(dead_code)]
impl Solution {
    /**
     * Note: Memory efficient (beats 94%) but very bad runtime (beats only 5%)
     */
    pub fn contains_nearby_duplicate (nums: Vec<i32>, mut k: i32) -> bool {
        if k == 0 || nums.len() <= 1 {
            return false;
        }

        if k >= nums.len() as i32 {
            k = nums.len() as i32 - 1;
        }

        for i in 1..nums.len() as i32 {
            for x in 1..=k {
                if i - x < 0 {
                    break;
                }

                if nums[i as usize] == nums[(i - x) as usize] {
                    return true;
                }
            }
        }

        false
    }
}

#[test]
pub fn test_contains_nearby_duplicates () {
    let a = vec![1,2,3,1];
    let b = vec![1,0,1,1];
    let c = vec![1,2,3,1,2,3];
    let d = vec![1,2,3];
    let e = vec![1];
    let f = vec![1,2,1];
    let g = vec![1,2,2,3];

    assert_eq!(Solution::contains_nearby_duplicate(a, 3), true);
    assert_eq!(Solution::contains_nearby_duplicate(b, 1), true);
    assert_eq!(Solution::contains_nearby_duplicate(c, 2), false);
    assert_eq!(Solution::contains_nearby_duplicate(d, 0), false);
    assert_eq!(Solution::contains_nearby_duplicate(e, 2), false);
    assert_eq!(Solution::contains_nearby_duplicate(f, 4), true);
    assert_eq!(Solution::contains_nearby_duplicate(g, 3), true);
}