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
use std::cell::RefCell;
use std::rc::Rc;

type Node = Rc<RefCell<TreeNode>>;
fn lca_deepest_leaves(root: Option<Node>) -> Option<Node> {
    lca_inner(root.unwrap(), 0).0
}

fn lca_inner(curr: Node, depth: i32) -> (Option<Node>, i32) {
    let c = curr.borrow();

    let (l, ld) = if let Some(left) = c.left.clone() {
        lca_inner(left, depth + 1)
    } else {
        (None, -1)
    };

    let (r, rd) = if let Some(right) = c.right.clone() {
        lca_inner(right, depth + 1)
    } else {
        (None, -1)
    };

    let max_depth = [depth, ld, rd].into_iter().max().unwrap();

    return if max_depth == depth {
        (Some(curr.clone()), depth)
    } else if ld == rd {
        (Some(curr.clone()), ld)
    } else if ld == max_depth {
        (l, ld)
    } else {
        // rd == max_depth
        (r, rd)
    };
}
