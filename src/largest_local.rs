use std::cmp::max;

pub fn largest_local(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = grid.len();
    let mut v = vec![vec![0; n - 2]; n - 2];

    for i in 1..(n - 1) {
        for j in 1..(n - 1) {
            v[i - 1][j - 1] = largest_grid(&grid, i, j);
        }
    }

    return v;
}

fn largest_grid(grid: &Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
    let mut res = grid[i - 1][j - 1];
    res = max(res, grid[i - 1][j]);
    res = max(res, grid[i - 1][j + 1]);
    res = max(res, grid[i][j - 1]);
    res = max(res, grid[i][j]);
    res = max(res, grid[i][j + 1]);
    res = max(res, grid[i + 1][j - 1]);
    res = max(res, grid[i + 1][j]);
    res = max(res, grid[i + 1][j + 1]);
    return res;
}
