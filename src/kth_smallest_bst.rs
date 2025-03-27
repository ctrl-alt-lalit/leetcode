use std::{
    cell::RefCell,
    collections::{BinaryHeap, VecDeque},
    rc::Rc,
};

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

fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
    let mut q = VecDeque::new();
    let mut pq = BinaryHeap::new();
    q.push_front(root.unwrap());

    while let Some(curr) = q.pop_front() {
        let c = curr.borrow();
        if let Some(l) = c.left.clone() {
            q.push_front(l);
        }

        if let Some(r) = c.right.clone() {
            q.push_back(r);
        }

        pq.push(c.val);
    }

    return pq.into_sorted_vec()[k as usize - 1];
}
