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

pub fn distribute_coins(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    return eval(root).0;
}

// (moves_needed, excess)
fn eval(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
    let Some(rt) = root else { return (0, 0) };
    let n = rt.borrow();

    let (l_moves, l_extra) = eval(n.left.clone());
    let (r_moves, r_extra) = eval(n.right.clone());

    let total_extra = l_extra + r_extra + n.val - 1;
    let total_moves = l_moves + r_moves + i32::abs(total_extra);
    return (total_moves, total_extra);
}
