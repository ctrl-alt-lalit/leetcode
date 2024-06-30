use std::collections::HashSet;

// Find two prefix arrays with the same sum mod k
pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
    let mut p_set: HashSet<i32> = HashSet::with_capacity(2048);
    let mut prefix = 0;
    p_set.insert(0); // extra insertion at the beginning for the empty array
    let mut last_insert: (i32, i32) = (0, -1);

    for (x, i) in nums.into_iter().zip(0..) {
        prefix = (prefix + x) % k;
        let has_prefix = p_set.contains(&prefix);
        let valid_prefix = has_prefix && ((prefix != last_insert.0) || (i >= last_insert.1 + 2)); // make sure len is at least 2

        if (i >= 1) && valid_prefix {
            return true;
        }

        if !has_prefix {
            p_set.insert(prefix);
            last_insert = (prefix, i);
        }
    }

    return false;
}
