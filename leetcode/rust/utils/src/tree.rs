use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None
        }
    }
}

pub type TreeLink = Option<Rc<RefCell<TreeNode>>>;

#[macro_export]
macro_rules! tree {
    (&e:expr) => {
        TreeLink::leaf($e);
    };
    ($e:expr, $l:expr, $r:expr) => {
        TreeLink::branch($e, $l, $r);
    }
}

pub trait TreeMaker {
    fn branch(val: i32, left: TreeLink, right: TreeLink) -> TreeLink {
        Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
    }
    fn leaf(val: i32) -> TreeLink {
        Some(Rc::new(RefCell::new(TreeNode {
            val,
            left: None,
            right: None,
        })))
    }
}

impl TreeMaker for TreeLink {}