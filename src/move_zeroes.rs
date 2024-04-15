pub fn move_zeroes(nums: &mut Vec<i32>) {
    let og_len = nums.len();
    nums.retain(|x| x != 0);
    let zeroes_needed = og_len - nums.len();
    (0..zeroes_needed).for_each(|_| nums.push(0));
}
