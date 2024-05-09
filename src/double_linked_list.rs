//Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn double_it(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_none() {
        return None;
    }

    let mut real_head = Box::from(ListNode { val: 0, next: head });
    let mut curr_opt = Some(&mut real_head);
    while let Some(curr) = curr_opt {
        if let Some(next) = curr.next.as_mut() {
            curr.val += ((next.val * 2) / 10);
            next.val = ((next.val * 2) % 10);
        }

        curr_opt = curr.next.as_mut();
    }

    if real_head.val == 0 {
        return real_head.next;
    } else {
        return Some(real_head);
    }
}
