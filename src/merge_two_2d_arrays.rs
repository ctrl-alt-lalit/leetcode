use std::collections::BTreeMap;

fn merge_arrays(nums1: Vec<Vec<i32>>, nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut m = BTreeMap::new();

    for (id, val) in nums1.into_iter().map(|x| (x[0], x[1])) {
        m.insert(id, val);
    }

    for (id, val) in nums2.into_iter().map(|x| (x[0], x[1])) {
        m.entry(id).and_modify(|v| *v += val).or_insert(val);
    }

    return m.into_iter().map(|(id, val)| vec![id, val]).collect();
}
