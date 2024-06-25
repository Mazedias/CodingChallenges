// Problem 26
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn remove_duplicates (nums: &mut Vec<i32>) -> i32 {
        let mut last_unique = 1;
        let mut i = 1;

        while i < nums.len() {
            if nums[i] != nums[i-1] {
                nums[last_unique] = nums[i];
                last_unique += 1;
            }
            i += 1;
        }

        last_unique as i32
    }

    pub fn remove_duplicates_b(nums: &mut Vec<i32>) -> i32 {
        nums.dedup();
        nums.len().try_into().unwrap()
    }
}

#[test]
pub fn test_remove_duplicates () {
    let mut nums: Vec<i32> = vec![1, 2, 2, 3, 5, 6, 6, 8, 9, 9];
    let expected_nums = vec![1, 2, 3, 5, 6, 8, 9];

    let unique = Solution::remove_duplicates(nums.as_mut());

    assert_eq!(&nums[..unique as usize], expected_nums);
}