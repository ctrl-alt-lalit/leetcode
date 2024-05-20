use rand::Rng;
use std::collections::HashMap;

struct RandomizedSet {
    data: Vec<i32>,
    index: HashMap<i32, usize>,
    rng: rand::rngs::ThreadRng,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {
    fn new() -> Self {
        return Self {
            data: Vec::with_capacity(200_000),
            index: HashMap::with_capacity(200_000),
            rng: rand::thread_rng(),
        };
    }

    fn insert(&mut self, val: i32) -> bool {
        if self.index.contains_key(&val) {
            return false;
        }

        self.index.insert(val, self.data.len());
        self.data.push(val);
        return true;
    }

    fn remove(&mut self, val: i32) -> bool {
        if let Some(i) = self.index.remove(&val) {
            self.data.swap_remove(i);
            if i < self.data.len() {
                self.index.insert(self.data[i], i);
            }
            return true;
        }

        return false;
    }

    fn get_random(&mut self) -> i32 {
        let i = self.rng.gen_range(0..self.data.len());
        return self.data[i];
    }
}

/*
 * Your RandomizedSet object will be instantiated and called as such:
 * let obj = RandomizedSet::new();
 * let ret_1: bool = obj.insert(val);
 * let ret_2: bool = obj.remove(val);
 * let ret_3: i32 = obj.get_random();
 */
