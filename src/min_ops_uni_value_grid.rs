fn min_operations(grid: Vec<Vec<i32>>, x: i32) -> i32 {
    let mut v: Vec<i32> = grid.into_iter().flatten().collect();
    v.sort_unstable();
    let median = v[v.len() / 2]; // always optimal to converge on median
    let mut ops = 0;

    for num in v.iter() {
        if (num % x) != (median % x) {
            return -1;
        }

        ops += i32::abs(num - median) / x
    }

    return ops;
}
