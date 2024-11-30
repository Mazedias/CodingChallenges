// Leetcode Problem 23

use std::collections::BinaryHeap;

use utils::*;

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut solution = None;

        let mut queue: BinaryHeap<i32> = BinaryHeap::new();

        for mut list in lists {
            while let Some(item) = list {
                queue.push(item.val);
                list = item.next;
            }
        }

        while let Some(value) = queue.pop() {
            solution = Some(Box::new(ListNode{val: value, next: solution}));
        }

        solution
    }
}


#[test]
fn test_merge_k_lists() {
    let list1 = list!(1, 4, 5);
    let list2 = list!(1, 3, 4);
    let list3 = list!(2, 6);

    let sol1 = list!(1, 1, 2, 3, 4, 4, 5, 6);

    assert_eq!(Solution::merge_k_lists(vec![]), list!());
    assert_eq!(Solution::merge_k_lists(vec![list!()]), list!());
    assert_eq!(Solution::merge_k_lists(vec![list1, list2, list3]), sol1);
}