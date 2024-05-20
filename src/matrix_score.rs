pub fn matrix_score(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len() as u32;
    let n = grid[0].len() as u32;

    let max_mask = u32::MAX >> (32 - n);
    let mut v: Vec<u32> = grid
        .into_iter()
        .map(|row| {
            let x = row
                .into_iter()
                .rev()
                .map(|x| x as u32)
                .zip(0u32..)
                .fold(0, |acc, (x, i)| acc | (x << i));
            return u32::max(x, max_mask - x); // maximize row flips while creating friendly array
        })
        .collect();

    // maximize by col
    for i in 0..n {
        let mask: u32 = (1 << i);
        let num_ones = v.iter().fold(0, |acc, x| acc + ((x & mask) > 0) as u32);
        let num_zeroes = m - num_ones;

        if (num_ones < num_zeroes) {
            for x in v.iter_mut() {
                *x ^= mask
            }
        }
    }

    return v.into_iter().sum::<u32>() as i32;
}
