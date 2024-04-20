pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    return (0..grid.len()).fold(0, |acc, i| {
        acc + (0..grid[0].len()).fold(0, |acc2, j| acc2 + dfs(&grid, i, j, &mut visited))
    });
}

fn dfs(grid: &Vec<Vec<char>>, i_init: usize, j_init: usize, visited: &mut Vec<Vec<bool>>) -> i32 {
    if (i_init >= grid.len())
        || (j_init >= grid[0].len())
        || (grid[i_init][j_init] == '0')
        || visited[i_init][j_init]
    {
        return 0;
    }

    let mut s = vec![(i_init, j_init)];
    visited[i_init][j_init] = true;

    while let Some((i, j)) = s.pop() {
        try_push(&mut s, i - 1, j, grid, visited);
        try_push(&mut s, i + 1, j, grid, visited);
        try_push(&mut s, i, j - 1, grid, visited);
        try_push(&mut s, i, j + 1, grid, visited);
    }

    return 1;
}

fn try_push(
    s: &mut Vec<(usize, usize)>,
    i: usize,
    j: usize,
    grid: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
) {
    if (i >= grid.len()) || (j >= grid[0].len()) || (grid[i][j] == '0') || visited[i][j] {
        return;
    }
    visited[i][j] = true;
    s.push((i, j));
    return;
}
