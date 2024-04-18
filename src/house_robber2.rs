pub fn rob(nums: Vec<i32>) -> i32 {
    if nums.len() == 1 {
        return nums[0];
    }

    let mut memo: Vec<Option<i32>> = vec![None; nums.len()];
    let rob_0 = rob_internal(&nums, 0, true, &mut memo);
    memo = vec![None; nums.len()];
    let rob_1 = rob_internal(&nums, 1, false, &mut memo);
    return std::cmp::max(rob_0, rob_1);
}

fn rob_internal(houses: &Vec<i32>, i: usize, did_rob_0: bool, memo: &mut Vec<Option<i32>>) -> i32 {
    if i >= houses.len() {
        return 0;
    }

    if did_rob_0 && (i == houses.len() - 1) {
        return 0;
    }

    if let Some(cached) = memo[i] {
        return cached;
    }

    let rob_this = houses[i] + rob_internal(houses, i + 2, did_rob_0, memo);

    if (i == 0) && did_rob_0 {
        memo[i] = Some(rob_this);
        return rob_this;
    }

    let rob_next = rob_internal(houses, i + 1, did_rob_0, memo);
    let best = std::cmp::max(rob_this, rob_next);

    memo[i] = Some(best);
    return best;
}
