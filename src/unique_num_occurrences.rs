use std::collections::{HashMap, HashSet};

pub fn unique_occurrences(arr: Vec<i32>) -> bool {
    let mut m = HashMap::with_capacity(1000);
    for x in arr.into_iter() {
        if let Some(count) = m.get_mut(&x) {
            *count += 1;
        } else {
            _ = m.insert(x, 1);
        }
    }

    let mut seen = HashSet::with_capacity(m.len());
    for (_, count) in m.drain() {
        if seen.contains(&count) {
            return false;
        } else {
            _ = seen.insert(count);
        }
    }

    return true;
}