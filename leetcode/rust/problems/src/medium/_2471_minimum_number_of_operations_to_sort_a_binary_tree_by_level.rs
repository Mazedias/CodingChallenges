// Leetcode Problem 2471

use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use utils::TreeNode;

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn minimum_operations(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // Queue to queue nodes using BFS
        let mut queue = VecDeque::from([root.clone().unwrap()]);
        let mut order = Vec::new();
        let mut count = 0;

        while !queue.is_empty() {
            // Vector for the nodes of one level
            order.extend(0..queue.len());
            order.sort_unstable_by_key(|&i| queue[i].borrow().val);

            // Count how many swaps are needed
            for i in 0..order.len() {
                while order[i] != i {
                    let j = order[i];
                    order.swap(i, j);
                    count += 1;
                }
            }

            // Dequeue all nodes from one level and add all their children to the queue (the next level)
            for _ in 0..queue.len() {
                let node = queue.pop_front().unwrap();
                let node = node.borrow();

                if let Some(left) = node.left.as_ref() {
                    queue.push_back(left.clone());
                }
                if let Some(right) = node.right.as_ref() {
                    queue.push_back(right.clone());
                }
            }

            // Clear the order vector so that it is ready for the next iteration
            order.clear();
        }
        
        count
    }
}