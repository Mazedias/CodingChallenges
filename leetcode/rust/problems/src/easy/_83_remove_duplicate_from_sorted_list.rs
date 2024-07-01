use utils::*;

// Problem 83
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn delete_duplicates (mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut current = &mut head;

        while let Some(ref mut node) = current {
            while node.next.is_some() && node.val == node.next.as_ref().unwrap().val {
                node.next = node.next.as_mut().unwrap().next.take();
            }
            current = &mut node.next;
        }

        head
    }
}

#[test]
pub fn test_delete_duplicated () {
    let a = list!(1, 1, 2);
    let a_sol = list!(1, 2);
    let b = list!(1, 1, 2, 3, 3);
    let b_sol = list!(1, 2, 3);

    assert_eq!(Solution::delete_duplicates(a), a_sol);
    assert_eq!(Solution::delete_duplicates(b), b_sol);
}