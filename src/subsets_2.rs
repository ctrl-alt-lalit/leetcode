use std::collections::HashSet;

fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut m = HashSet::with_capacity(1 << nums.len());
    let mut more = m.clone();
    m.insert(vec![]);

    for x in nums.into_iter() {
        more.clear();

        for mut v in m.iter().cloned() {
            v.push(x);
            v.sort_unstable();
            more.insert(v);
        }

        m.extend(more.drain());
    }

    return m.into_iter().collect();
}
