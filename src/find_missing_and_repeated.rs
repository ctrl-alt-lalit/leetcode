fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
    let n = grid.len();
    let mut counter: Vec<i8> = vec![0; n * n + 1];

    for i in 0..n {
        for j in 0..n {
            let k = grid[i][j] as usize;
            counter[k] += 1;
        }
    }

    let mut dupe: i32 = -1;
    let mut miss: i32 = -1;
    for (i, val) in counter.into_iter().enumerate() {
        match val {
            0 => miss = i as i32,
            2 => dupe = i as i32,
            _ => (),
        }
    }

    return vec![dupe, miss];
}
