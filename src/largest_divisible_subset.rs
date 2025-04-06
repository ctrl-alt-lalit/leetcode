fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    nums.sort_unstable();
    let mut memo = vec![1; n];

    let mut prev_best_i = 0;
    let mut prev_best_count = 1;

    for i in 1..n {
        let xi = nums[i];
        let best_add = nums
            .iter()
            .take(i)
            .enumerate()
            .filter_map(|(j, xj)| if (xi % xj == 0) { Some(memo[j]) } else { None })
            .max()
            .unwrap_or(0);
        memo[i] += best_add;

        if memo[i] > prev_best_count {
            prev_best_count = memo[i];
            prev_best_i = i;
        }
    }

    let mut target_count = prev_best_count;
    let mut dividee = nums[prev_best_i];
    let mut res = vec![];
    for j in (0..=prev_best_i).rev() {
        if (memo[j] == target_count) && (dividee % nums[j] == 0) {
            res.push(nums[j]);
            target_count -= 1;
            dividee = nums[j];
        }
    }

    res
}
