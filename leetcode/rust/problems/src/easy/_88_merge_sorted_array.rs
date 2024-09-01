// Problem 88
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn merge (nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        if n == 0 {
            return;
        }

        let mut m: usize = m as usize;
        let mut n: usize = n as usize;

        while m > 0 || n > 0 {
            if m ==  0 {
                nums1[n - 1] = nums2[n - 1];
                n -= 1;
            } else if n == 0 {
                break;
            } else if nums1[m - 1] >= nums2[n - 1] {
                nums1[n + m - 1] = nums1[m - 1];
                m -= 1;
            } else {
                nums1[n + m - 1] = nums2[n - 1];
                n -= 1;
            }
        }

    }
}

#[test]
pub fn test_merge () {
    let mut nums1 = vec![1, 2, 3, 0, 0, 0];
    let mut nums2 = vec![2, 5, 6];

    let mut nums3 = vec![1];
    let mut nums4 = vec![];

    let mut nums5 = vec![0];
    let mut nums6 = vec![1];

    let mut nums7 = vec![4, 5, 6, 0, 0, 0];
    let mut nums8 = vec![1, 2, 3];

    Solution::merge(&mut nums1, 3, &mut nums2, 3);
    Solution::merge(&mut nums3, 1, &mut nums4, 0);
    Solution::merge(&mut nums5, 0, &mut nums6, 1);
    Solution::merge(&mut nums7, 3, &mut nums8, 3);

    assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
    assert_eq!(nums3, vec![1]);
    assert_eq!(nums5, vec![1]);
}