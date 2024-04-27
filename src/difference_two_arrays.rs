use std::collections::HashSet;

pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
    let mut s1: HashSet<i32> = HashSet::from_iter(nums1.into_iter());
    let mut s2: HashSet<i32> = HashSet::from_iter(nums2.into_iter());

    let common: Vec<i32> = s1.intersection(&s2).copied().collect();
    for x in common.iter() {
        s1.remove(x);
        s2.remove(x);
    }

    let v1: Vec<i32> = s1.into_iter().collect();
    let v2: Vec<i32> = s2.into_iter().collect();
    return vec![v1, v2];
}