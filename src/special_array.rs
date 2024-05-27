pub fn special_array(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable_by(|a, b| b.cmp(a)); // reverse sort

    let min = *nums.last().unwrap();
    let len = nums.len() as i32;

    if len <= min {
        return len;
    }

    for i in 0..(nums.len() - 1) {
        if nums[i] == nums[i + 1] {
            continue;
        }

        let x = i as i32 + 1;
        if (nums[i] >= x) && (nums[i + 1] < x) {
            return x;
        }
    }

    return -1;
}
