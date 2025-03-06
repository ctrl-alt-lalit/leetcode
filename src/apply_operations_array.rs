fn apply_operations(mut nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut res = Vec::with_capacity(n);

    for i in 0..(n - 1) {
        if nums[i] == nums[i + 1] {
            nums[i] *= 2;
            nums[i + 1] = 0;
        }

        if nums[i] > 0 {
            res.push(nums[i]);
        }
    }

    if nums[n - 1] > 0 {
        res.push(nums[n - 1]);
    }

    res.resize(n, 0);
    return res;
}
