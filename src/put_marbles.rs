fn put_marbles(weights: Vec<i32>, k: i32) -> i64 {
    let mut v = Vec::with_capacity(weights.len() - 1);

    for i in 0..(weights.len() - 1) {
        v.push((weights[i] + weights[i + 1]) as i64); // combine pair into sum since we'll only use the total anyway
    }

    // max_sum = weights.first + weights.last + max(k-1 pairs)
    // min_sum = weights.first + weights.last + min(k-1 pairs)
    // so calculation for difference can omit array ends

    v.sort_unstable();

    let max: i64 = v.iter().copied().rev().take((k - 1) as usize).sum();
    let min: i64 = v.into_iter().take((k - 1) as usize).sum();
    return max - min;
}
