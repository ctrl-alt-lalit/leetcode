pub fn min_falling_path_sum(grid: Vec<Vec<i32>>) -> i32 {
    let mut memo: Vec<Vec<Option<i32>>> = vec![vec![None; grid.len()]; grid.len()];
    
    let mut best = i32::MAX;
    for j in 0..grid.len() {
        best = std::cmp::min(best, calc(&grid, 0, j, &mut memo));
    }

    return best;
}

fn calc(grid: &Vec<Vec<i32>>, i: usize, j: usize, mut memo: &mut Vec<Vec<Option<i32>>>) -> i32 {
    if i >= grid.len() || j >= grid.len() {
        return i32::MAX;
    }

    if i == grid.len() - 1 {
        return grid[i][j];
    }

    if let Some(cached) = memo[i][j] {
        return cached;
    }

    let mut best_below = i32::MAX;

    for j2 in 0..grid.len() {
        if j == j2 {
            continue;
        }

        best_below = std::cmp::min(best_below, calc(&grid, i+1, j2, &mut memo));
    }

    let best = grid[i][j] + best_below;
    memo[i][j] = Some(best);
    return best;
}