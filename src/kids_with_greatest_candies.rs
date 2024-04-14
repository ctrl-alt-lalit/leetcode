pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    let old_highest = candies.iter().max().unwrap().clone();
    return candies
        .into_iter()
        .map(|candy_count| (candy_count + extra_candies) >= old_highest)
        .collect();
}
