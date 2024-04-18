// Definition for a binary tree node.
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
use std::cell::Ref;
use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;

impl Solution {
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let res = Self::rob_internal(root.as_deref());
        return max(res.0, res.1);
    }

    fn rob_internal(root_opt: Option<&RefCell<TreeNode>>) -> (i32, i32) {
        let Some(root_wrapper) = root_opt else { return (0,0) };
        let root: Ref<TreeNode> = root_wrapper.borrow();

        let rob_left = Self::rob_internal(root.left.as_deref());
        let rob_right = Self::rob_internal(root.right.as_deref());

        let with_self = root.val + rob_left.1 + rob_right.1; // self & grandchildren
        let with_children = max(rob_left.0, rob_left.1) + max(rob_right.0, rob_right.1); // best of children
        return (with_self, with_children);
    }
}
