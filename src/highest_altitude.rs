pub fn largest_altitude(gain: Vec<i32>) -> i32 {
    let mut best = 0;
    let mut curr = 0;

    for y in gain.into_iter() {
        curr += y;
        best = std::cmp::max(best, curr);
    }

    return best;
}