// Leetcode Problem 19

use utils::*;

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn remove_nth_from_end(head:Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        // Count the amount of list nodes in the passed list
        let cnt = std::iter::successors(head.as_ref(), |last| last.next.as_ref()).count();

        // Create a ListNode that holds the header for later return
        let mut dummy = Some(Box::new(ListNode{val: 0, next: head}));

        // Get the ListNode before the one we want to remove
        let mut prev = (0..cnt - n as usize).fold(dummy.as_mut(), |curr, _| curr.unwrap().next.as_mut() );

        // Remove the pointer from the previous ListNode to the node that we want to delete by changing the pointer to the ListNode the Node we remove points to
        prev.unwrap().next = prev.as_mut().unwrap().next.as_mut().unwrap().next.take();

        dummy.unwrap().next
    }
}


#[test]
fn test_remove_nth_from_end() {
    let list1 = list!(1, 2, 3, 4, 5);
    let list1_sol = list!(1, 2, 3, 5);
    
    let list2 = list!(1);
    let list2_sol = list!();

    let list3 = list!(1, 2);
    let list3_sol = list!(1);

    assert_eq!(Solution::remove_nth_from_end(list1, 2), list1_sol);
    assert_eq!(Solution::remove_nth_from_end(list2, 1), list2_sol);
    assert_eq!(Solution::remove_nth_from_end(list3, 1), list3_sol);
}