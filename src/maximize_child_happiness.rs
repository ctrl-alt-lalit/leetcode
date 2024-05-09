pub fn maximum_happiness_sum(happiness: Vec<i32>, k: i32) -> i64 {
    let mut v = happiness;
    v.sort_unstable();
    return v.into_iter().rev().zip(0..k).fold(0, |acc, (h,i)| acc + std::cmp::max(h-i, 0) as i64);
}