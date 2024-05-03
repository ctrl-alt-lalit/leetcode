#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}
 
pub fn delete_middle(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let len = ll_len(head.as_ref());
    if len <= 1 {
        return None;
    }
    
    let mut curr = head.as_mut().unwrap().as_mut();
    for _ in 1..len/2 {
        curr = curr.next.as_mut().unwrap().as_mut();
    }

    curr.next = curr.next.as_mut().unwrap().next.take();
    return head;    
}

fn ll_len(head: Option<&Box<ListNode>>) -> i32 {
    if head.is_none() { return 0; }
    let mut curr = head.unwrap().as_ref();
    let mut len = 1;
    while let Some(next) = curr.next.as_ref() {
        len += 1;
        curr = next.as_ref();
    }
    return len;
}