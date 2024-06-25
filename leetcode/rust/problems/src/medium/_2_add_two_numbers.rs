// Problem 2
use utils::*;

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn add_two_numbers (l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode::new(0)));
        let mut list_ptr = dummy.as_mut();

        let mut p1 = l1;
        let mut p2 = l2;

        let mut carry = 0;
        while p1.is_some() || p2.is_some() {
            let sum = p1.as_ref().map_or(0, |node| node.val) + p2.as_ref().map_or(0, |node| node.val) + carry;
            
            carry = sum / 10;

            if let Some(ref mut node) = list_ptr {
                node.next = Some(Box::new(ListNode::new(sum % 10)));
                
            }

            list_ptr = list_ptr.unwrap().next.as_mut();

            p1 = p1.and_then(|node| node.next);
            p2 = p2.and_then(|node| node.next);
        }

        if carry != 0 {
            if let Some(ref mut node) = list_ptr {
                node.next = Some(Box::new(ListNode::new(carry)));
            }
        }

        dummy.unwrap().next
    }
}

#[test]
pub fn test_add_two_numbers () {
    let l1_a = list!(2, 4, 3);
    let l2_a = list!(5, 6, 4);
    let lsol_a = list!(7, 0, 8);

    let l1_b = list!(0);
    let l2_b = list!(0);
    let lsol_b = list!(0);

    let l1_c = list!(9, 9, 9, 9, 9, 9, 9);
    let l2_c = list!(9, 9, 9, 9);
    let lsol_c = list!(8, 9, 9, 9, 0, 0, 0, 1);

    assert_eq!(Solution::add_two_numbers(l1_a, l2_a), lsol_a);
    assert_eq!(Solution::add_two_numbers(l1_b, l2_b), lsol_b);
    assert_eq!(Solution::add_two_numbers(l1_c, l2_c), lsol_c);
}