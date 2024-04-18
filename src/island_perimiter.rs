pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
    let mut res = 0;
    for i in 0..(grid.len() as i32) {
        for j in 0..(grid[0].len() as i32) {
            res += perimiter_contribution(&grid, i, j)
        }
    }
    return res;
}

fn perimiter_contribution(grid: &Vec<Vec<i32>>, i: i32, j: i32) -> i32 {
    if grid_inv(grid, i, j) == 1 {
        return 0;
    }

    return grid_inv(grid, i - 1, j)
        + grid_inv(grid, i, j - 1)
        + grid_inv(grid, i, j + 1)
        + grid_inv(grid, i + 1, j);
}

fn grid_inv(grid: &Vec<Vec<i32>>, i: i32, j: i32) -> i32 {
    return if (i < 0) || (i >= (grid.len() as i32)) {
        1
    } else if (j < 0) || (j >= (grid[0].len() as i32)) {
        1
    } else {
        1 - grid[i as usize][j as usize]
    };
}
