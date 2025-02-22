// Leetcode Problem 1028

use std::{cell::RefCell, rc::Rc};

use utils::TreeNode;

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn recover_from_preorder(traversal: String) -> Option<Rc<RefCell<TreeNode>>> {
        let mut stack: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
        let mut depth;
        let mut i = 0;

        let chars: Vec<char> = traversal.chars().collect();

        while i < chars.len() {
            depth = 0;

            // Cound the depth of the next node
            while i < chars.len() && chars[i] == '-' {
                depth += 1;
                i += 1;
            }

            // Parse the value of the next node
            let mut num = 0;
            while i < chars.len() && chars[i].is_digit(10) {
                num = num * 10 + chars[i].to_digit(10).unwrap() as i32;
                i += 1;
            }

            let new_node = Rc::new(RefCell::new(TreeNode::new(num)));

            // Backtracking: adjust the stack when it does not match the depth we are currently in
            while stack.len() > depth {
                stack.pop();
            }

            // Add the new node to its rightfull parent node
            if let Some(parent) = stack.last() {
                let mut parent_ref = parent.borrow_mut();

                if parent_ref.left.is_none() {
                    parent_ref.left = Some(Rc::clone(&new_node));
                } else if parent_ref.right.is_none() {
                    parent_ref.right = Some(Rc::clone(&new_node));
                }
            }

            stack.push(new_node);
        }

        stack.first().cloned()
    }
}


#[test]
fn test_recover_from_preorder() {
    let traversal1 = String::from("1-2--3--4-5--6--7");
    let traversal2 = String::from("1-2--3---4-5--6---7");
    let traversal3 = String::from("1-401--349---90--88");

    assert!(Solution::recover_from_preorder(traversal1).unwrap().borrow().val == 1);
    assert!(Solution::recover_from_preorder(traversal2).unwrap().borrow().val == 1);
    assert!(Solution::recover_from_preorder(traversal3).unwrap().borrow().val == 1);
}