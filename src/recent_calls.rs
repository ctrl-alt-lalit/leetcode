struct RecentCounter {
    q: std::collections::VecDeque<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RecentCounter {

    fn new() -> Self {
        return Self { q: std::collections::VecDeque::new() };
    }
    
    fn ping(&mut self, t: i32) -> i32 {
        
        while let Some(x) = self.q.front() {
            if *x < t-3000 {
                _ = self.q.pop_front()
            } else {
                break;
            }
        }

        self.q.push_back(t);
        return self.q.len() as i32;
    }
}

/**
 * Your RecentCounter object will be instantiated and called as such:
 * let obj = RecentCounter::new();
 * let ret_1: i32 = obj.ping(t);
 */