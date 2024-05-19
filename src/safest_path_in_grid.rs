use std::{
    cmp::min,
    collections::{BinaryHeap, VecDeque},
};

pub fn maximum_safeness_factor(grid: Vec<Vec<i32>>) -> i32 {
    let mut v = process_grid(grid);
    let n = v.len();

    // djikstra
    let mut pq: BinaryHeap<(i32, usize, usize)> = BinaryHeap::with_capacity(n * n / 4);
    pq.push((v[0][0], 0, 0));

    while let Some((val, i, j)) = pq.pop() {
        if (i == n - 1) && (j == n - 1) {
            return val;
        }

        search_try_push(i - 1, j, val, &mut v, &mut pq);
        search_try_push(i + 1, j, val, &mut v, &mut pq);
        search_try_push(i, j - 1, val, &mut v, &mut pq);
        search_try_push(i, j + 1, val, &mut v, &mut pq);
    }

    panic!("Result should have been found in search")
}

fn search_try_push(
    i: usize,
    j: usize,
    val: i32,
    v: &mut Vec<Vec<i32>>,
    pq: &mut BinaryHeap<(i32, usize, usize)>,
) {
    if (i >= v.len()) || (j >= v.len()) || v[i][j].is_negative() {
        return;
    }

    pq.push((min(val, v[i][j]), i, j));
    v[i][j] *= -1; // negate values in v to track visited state
}

fn process_grid(mut grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = grid.len();
    let mut q: VecDeque<(usize, usize)> = VecDeque::with_capacity(n * n / 2);

    for i in 0..n {
        for j in 0..n {
            if grid[i][j] == 1 {
                q.push_back((i, j));
                grid[i][j] = 0;
            } else {
                grid[i][j] = i32::MAX;
            }
        }
    }

    while let Some((i, j)) = q.pop_front() {
        let val = grid[i][j] + 1;

        process_try_push(i - 1, j, val, &mut grid, &mut q);
        process_try_push(i + 1, j, val, &mut grid, &mut q);
        process_try_push(i, j - 1, val, &mut grid, &mut q);
        process_try_push(i, j + 1, val, &mut grid, &mut q);
    }
    return grid;
}

fn process_try_push(
    i: usize,
    j: usize,
    val: i32,
    grid: &mut Vec<Vec<i32>>,
    q: &mut VecDeque<(usize, usize)>,
) {
    if (i < grid.len()) && (j < grid.len()) && (grid[i][j] > val) {
        q.push_back((i, j));
        grid[i][j] = val
    }
}
