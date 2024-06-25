// Problem 21
struct Solution;
use utils::*;


#[allow(dead_code)]
impl Solution {
    pub fn merge_two_sorted_lists (list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if list1.is_none() {
            return list2;
        }

        if list2.is_none() {
            return list1;
        }

        let mut l1 = list1.unwrap();
        let mut l2 = list2.unwrap();

        if l1.val < l2.val {
            l1.next = Self::merge_two_sorted_lists(l1.next, Some(l2));
            Some(l1)
        } else {
            l2.next = Self::merge_two_sorted_lists(Some(l1), l2.next);
            Some(l2)
        }

    }
}

#[test]
pub fn test_merge_two_sorted_lists () {
    let a = list!(1, 2, 4);
    let b = list!(1, 3, 4);
    let c = list!(1, 1, 2, 3, 4, 4);

    assert_eq!(Solution::merge_two_sorted_lists(a, b), c);
}