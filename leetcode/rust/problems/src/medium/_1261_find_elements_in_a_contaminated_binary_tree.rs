// Leetcode Problem 1261

use std::{cell::RefCell, collections::{HashSet, VecDeque}, rc::Rc};

use utils::TreeNode;

struct FindElements {
    values: HashSet<i32>
}

#[allow(dead_code)]
impl FindElements {

    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        // Store found nodes that still have to be decontaminated
        let mut q = VecDeque::new();

        let root = root.unwrap();

        // Store all generated values so that find() has O(1)
        let mut values = HashSet::new();

        q.push_back(root);

        // Decontaminate all found nodes in the queue
        while !q.is_empty() {
            if let Some(node) = q.pop_front() {
                let val;
                {
                    let mut node_ref = node.borrow_mut();

                    // Only the root node is inserted with value -1 therefor we set it to val 0
                    if node_ref.val == -1 {
                        node_ref.val = 0;
                    }
                    val = node_ref.val;
                }

                values.insert(val);

                // Correct the value of the left child and insert it into the queue
                if let Some(left_child) = node.borrow().left.clone() {
                    left_child.borrow_mut().val = 2 * val + 1;
                    q.push_back(left_child);
                }

                // Correct the value of the right child and insert it into the queue
                if let Some(right_child) = node.borrow().right.clone() {
                    right_child.borrow_mut().val = 2 * val + 2;
                    q.push_back(right_child);
                }
            }
        }

        FindElements {
            values
        }
    }

    fn find(&self, target: i32) -> bool {
        self.values.contains(&target)
    }

}