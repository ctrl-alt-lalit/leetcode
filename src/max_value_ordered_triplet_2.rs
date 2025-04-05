fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
    let n = nums.len();

    let mut highest_suffix = vec![nums.last().copied().unwrap(); n];
    for (i, x) in nums.iter().copied().enumerate().rev().skip(1) {
        highest_suffix[i] = i32::max(highest_suffix[i + 1], x);
    }

    let mut ans = 0;
    let mut i = 0;
    while i < (n - 2) {
        let x = nums[i];
        let mut j = i + 1;
        while j < (n - 1) {
            let prefix = (x - nums[j]) as i64;
            if prefix < 0 {
                break; // found a higher nums[i] to start from
            }

            ans = i64::max(ans, prefix * highest_suffix[j + 1] as i64);
            j += 1;
        }
        i = j;
    }

    return ans;
}
