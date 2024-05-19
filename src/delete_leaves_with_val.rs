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

pub fn remove_leaf_nodes(
    root: Option<Rc<RefCell<TreeNode>>>,
    target: i32,
) -> Option<Rc<RefCell<TreeNode>>> {
    let Some(rt) = root else { return None };
    let mut n = rt.borrow_mut();

    n.left = remove_leaf_nodes(n.left.take(), target);
    n.right = remove_leaf_nodes(n.right.take(), target);

    if n.left.is_none() && n.right.is_none() && n.val == target {
        return None;
    } else {
        drop(n);
        return Some(rt);
    }
}
