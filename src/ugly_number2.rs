use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn nth_ugly_number(n: i32) -> i32 {
    let mut pq: BinaryHeap<Reverse<i128>> = BinaryHeap::new();
    pq.push(Reverse(1));
    for _ in 1..n {
        let x = pq.pop().unwrap().0;
        pq.push(Reverse(x * 2));
        pq.push(Reverse(x * 3));
        pq.push(Reverse(x * 5));

        while pq.peek().unwrap().0 == x {
            _ = pq.pop();
        }
    }

    return pq.pop().unwrap().0 as i32;
}
