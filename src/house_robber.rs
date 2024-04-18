pub fn rob(nums: Vec<i32>) -> i32 {
    let mut memo: Vec<Option<i32>> = vec![None; nums.len()];
    return rob_internal(&nums, 0, &mut memo);
}

fn rob_internal(houses: &Vec<i32>, i: usize, memo: &mut Vec<Option<i32>>) -> i32 {
    if i >= houses.len() {
        return 0;
    }

    if let Some(cached) = memo[i] {
        return cached;
    }

    let rob_this = houses[i] + rob_internal(houses, i + 2, memo);
    let rob_next = rob_internal(houses, i + 1, memo);
    let best = std::cmp::max(rob_this, rob_next);

    memo[i] = Some(best);
    return best;
}
