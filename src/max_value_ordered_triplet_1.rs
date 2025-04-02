fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
    let n = nums.len();

    let mut best_k = vec![nums.last().copied().unwrap(); n];
    for (i, x) in nums.iter().copied().enumerate().rev().skip(1) {
        best_k[i] = i32::max(best_k[i + 1], x);
    }

    let mut max = 0;
    for i in 0..(n - 2) {
        for j in i + 1..(n - 1) {
            let x = (nums[i] - nums[j]) as i64;
            max = i64::max(max, x * best_k[j + 1] as i64);
        }
    }

    return max;
}
