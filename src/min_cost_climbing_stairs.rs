pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
    let mut memo = vec![None; cost.len()];
    let _ = f(0, &cost, &mut memo);
    return std::cmp::min(memo[0].unwrap(), memo[1].unwrap());
}

fn f(i: usize, cost: &Vec<i32>, memo: &mut Vec<Option<i32>>) -> i32 {
    if i >= cost.len() {
        return 0;
    }

    if let Some(cached) = memo[i] {
        return cached;
    }

    let up_1 = cost[i] + f(i+1,cost, memo);
    let up_2 = cost[i] + f(i+2, cost, memo);
    let res = std::cmp::min(up_1, up_2);
    memo[i]=Some(res);
    return res;
}