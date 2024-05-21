pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut loss_counter: Vec<i32> = vec![-1; 10_001];

    for (winner, loser) in matches.into_iter().map(|v| (v[0] as usize, v[1] as usize)) {
        loss_counter[loser] = i32::max(1, loss_counter[loser] + 1);
        loss_counter[winner] = i32::max(0, loss_counter[winner])
    }

    let mut res = vec![Vec::with_capacity(10_001), Vec::with_capacity(10_001)];

    for (loss_count, player) in loss_counter
        .into_iter()
        .zip(0..)
        .filter(|(l, _)| *l == 0 || *l == 1)
    {
        res[loss_count as usize].push(player);
    }

    return res;
}
