use std::cmp::max;

pub fn get_maximum_gold(grid: Vec<Vec<i32>>) -> i32 {
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut res = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            res = max(res, f(&grid, i, j, &mut visited));
        }
    }

    return res;
}

fn f(grid: &Vec<Vec<i32>>, i: usize, j: usize, visited: &mut Vec<Vec<bool>>) -> i32 {
    if (i >= grid.len()) || (j >= grid[0].len()) {
        return -1;
    }

    if visited[i][j] || (grid[i][j] == 0) {
        return -1;
    }
    visited[i][j] = true;

    let stay = grid[i][j];
    let left = stay + f(grid, i, j - 1, visited);
    let right = stay + f(grid, i, j + 1, visited);
    let up = stay + f(grid, i - 1, j, visited);
    let down = stay + f(grid, i + 1, j, visited);

    let res = max(max(max(left, right), max(up, down)), stay);

    visited[i][j] = false;
    return res;
}
