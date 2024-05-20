pub fn find_matrix(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut last_row = vec![usize::MAX; nums.len() + 1];
    let mut target_row = vec![0_usize; nums.len() + 1];
    let mut highest_row = 0;

    for (i, x) in nums.iter().copied().enumerate() {
        let target = last_row[x as usize] + 1;
        target_row[i] = target;
        last_row[x as usize] = target;
        highest_row = usize::max(highest_row, target);
    }

    let mut res: Vec<Vec<i32>> = vec![Vec::with_capacity(nums.len()); highest_row + 1];

    for (target, x) in target_row.into_iter().zip(nums.into_iter()) {
        res[target].push(x);
    }

    return res;
}
