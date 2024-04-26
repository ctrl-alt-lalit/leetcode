pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
    // don't even need an average since k is constant per run
    let ku = k as usize;
    let mut best = nums.iter().take(ku).sum::<i32>();
    
    let mut curr = best;
    for i in ku..nums.len() {
        curr = curr - nums[i -ku] + nums[i];
        best = std::cmp::max(curr, best);
    }

    return (best as f64) / (k as f64);
}