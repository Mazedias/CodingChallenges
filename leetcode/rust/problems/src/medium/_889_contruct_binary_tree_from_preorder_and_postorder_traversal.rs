// Leetcode Problem 889

use std::{cell::RefCell, collections::HashMap, rc::Rc};

use utils::TreeNode;

struct Solution;

fn build_tree(preoder: &Vec<i32>, post_map: &HashMap<i32, usize>, pre_idx: &mut usize, left: usize, right: usize) -> Option<Rc<RefCell<TreeNode>>> {
    if *pre_idx >= preoder.len() || left > right {
        return None;
    }

    let root_val = preoder[*pre_idx];
    let root = Rc::new(RefCell::new(TreeNode::new(root_val)));
    *pre_idx += 1;

    if left == right {
        return Some(root);
    }

    let left_child_val = preoder[*pre_idx];
    let left_child_idx = *post_map.get(&left_child_val).unwrap();

    root.borrow_mut().left = build_tree(preoder, post_map, pre_idx, left, left_child_idx);
    root.borrow_mut().right = build_tree(preoder, post_map, pre_idx, left_child_idx + 1, right - 1);

    Some(root)
}

#[allow(dead_code)]
impl Solution {
    pub fn construct_from_pre_post(preorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut pre_idx = 0;
        let mut post_map = HashMap::new();

        for (i, &val) in postorder.iter().enumerate() {
            post_map.insert(val, i);
        }

        build_tree(&preorder, &post_map, &mut pre_idx, 0, postorder.len() - 1)
    }
}
