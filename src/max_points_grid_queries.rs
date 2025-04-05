use std::{cmp::Reverse, collections::BinaryHeap};

fn max_points(grid: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
    let mut v: Vec<_> = queries.into_iter().zip(0usize..).collect();
    v.sort_unstable_by_key(|x| x.0);

    let mut border = BinaryHeap::new();
    border.push(Reverse((grid[0][0], 0, 0)));

    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];

    let mut curr = 0;
    let mut res = vec![0; v.len()];
    for (query, i) in v.into_iter() {
        curr += traverse(query, &grid, &mut visited, &mut border);
        res[i] = curr;
    }

    return res;
}

fn traverse(
    query: i32,
    grid: &Vec<Vec<i32>>,
    visited: &mut Vec<Vec<bool>>,
    border: &mut BinaryHeap<Reverse<(i32, usize, usize)>>,
) -> i32 {
    let mut total = 0;
    while let Some(Reverse((val, i, j))) = border.peek().cloned() {
        if val >= query {
            break;
        }

        border.pop();

        if visited[i][j] {
            continue;
        }

        total += 1;
        visited[i][j] = true;

        if i > 0 && !visited[i - 1][j] {
            border.push(Reverse((grid[i - 1][j], i - 1, j)));
        }
        if i < grid.len() - 1 && !visited[i + 1][j] {
            border.push(Reverse((grid[i + 1][j], i + 1, j)));
        }
        if j > 0 && !visited[i][j - 1] {
            border.push(Reverse((grid[i][j - 1], i, j - 1)));
        }
        if j < grid[0].len() - 1 && !visited[i][j + 1] {
            border.push(Reverse((grid[i][j + 1], i, j + 1)));
        }
    }

    return total;
}
