use std::{cell::RefCell, rc::Rc};

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub fn evaluate_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let binding = root.unwrap();
    let r = binding.borrow();

    return match r.val {
        0 => false,
        1 => true,
        2 => evaluate_tree(r.left.clone()) || evaluate_tree(r.right.clone()),
        3 => evaluate_tree(r.left.clone()) && evaluate_tree(r.right.clone()),
        _ => unreachable!("invalid opcode"),
    };
}
