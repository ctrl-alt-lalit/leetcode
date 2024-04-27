use std::collections::HashMap;

pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let mut col_grid = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            col_grid[j][i] = grid[i][j];
        }
    }

    let mut row_map = HashMap::with_capacity(n);
    for row in grid.into_iter() {
        row_map.entry(row).and_modify(|v| *v += 1).or_insert(1);
    }

    let mut col_map = HashMap::with_capacity(n);
    for col in col_grid.into_iter() {
        col_map.entry(col).and_modify(|v| *v += 1).or_insert(1);
    }

    let mut res = 0;
    for (row, row_count) in row_map.into_iter() {
        let col_count = col_map.get(&row).unwrap_or(&0).clone();
        res += row_count * col_count;
    }


    return res;
}