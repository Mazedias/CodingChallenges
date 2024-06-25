// Problem 27
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut i = 0;
        while i < nums.len() {
            if nums[i] == val {
                nums.remove(i);
                continue;
            }
            i += 1;
        }

        nums.len() as i32
    }
}

#[test]
pub fn test_remove_element () {
    let mut nums: Vec<i32> = vec![3,2,2,3];
    let expected_nums: Vec<i32> = vec![2,2];

    let k = Solution::remove_element(&mut nums, 3);

    assert_eq!(&nums[..k as usize], expected_nums);
}