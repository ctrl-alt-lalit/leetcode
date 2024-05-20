pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
    let mut res = Vec::with_capacity(0);
    res.reserve_exact((1 << (nums.len() - 1)) + 1);
    res.push(0);

    for curr in nums.into_iter().rev() {
        let old_len = res.len();
        for i in 0..old_len {
            res.push(res[i] ^ curr);
        }
    }

    return res.into_iter().sum();
}
