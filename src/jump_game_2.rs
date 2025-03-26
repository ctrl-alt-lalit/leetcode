fn jump(nums: Vec<i32>) -> i32 {
    let mut memo = vec![None; nums.len()];
    return f(nums.len() as i32 - 1, &nums, &mut memo);
}

// f(i) = minimum num jumps to reach ith position
fn f(i: i32, nums: &Vec<i32>, memo: &mut Vec<Option<i32>>) -> i32 {
    let u = i as usize;
    if i == 0 {
        return 0;
    } else if let Some(cached) = memo[u] {
        return cached;
    }

    let mut res = i32::MAX;
    for j in (0..i) {
        if j + nums[j as usize] >= i {
            res = i32::min(res, 1 + f(j, nums, memo))
        }
    }

    memo[u] = Some(res);
    return res;
}
