pub fn min_operations(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable();
    nums.push(-1);

    let mut run_len = 1;
    let mut sum = 0;
    for i in 1..nums.len() {
        if nums[i] != nums[i - 1] {
            if let Ok(ops) = num_ops(run_len) {
                sum += ops;
            } else {
                return -1;
            }

            run_len = 1;
        } else {
            run_len += 1;
        }
    }

    return sum;
}

// 2 => 1
// 3 => 1
// 4 => 2 (2 + 2)
// 5 => 2 (3 + 2)
// 6 => 2 (3 + 3)
// 7 => 3 (2 + 2 + 3)
// 8 => 3 (2 + 3 + 3)
// 9 => 3 (3 + 3 + 3)
// 10 => 4 (2 + 2 + 3 + 3)
// 12 => 4 (3 + 3 + 3  + 3)
fn num_ops(count: i32) -> Result<i32, ()> {
    if count < 2 {
        return Err(());
    }

    return Ok((count + 2) / 3);
}
