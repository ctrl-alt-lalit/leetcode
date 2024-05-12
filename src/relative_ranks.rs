pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
    let mut v: Vec<(usize, i32)> = score.into_iter().enumerate().collect();
    v.sort_unstable_by_key(|(_, x)| -*x);

    let mut res = vec![String::new(); v.len()];

    for ((i, _), rank) in v.into_iter().zip(1..) {
        res[i] = match rank {
            1 => String::from("Gold Medal"),
            2 => String::from("Silver Medal"),
            3 => String::from("Bronze Medal"),
            _ => rank.to_string(),
        };
    }

    return res;
}
