pub fn longest_subarray(nums: Vec<i32>) -> i32 {
    let k = 1;
    let mut zeroes_used = 0;
    let mut left: usize = 0;
    let mut best: usize = 0;

    for (right, rv) in nums.iter().cloned().enumerate() {
        zeroes_used += (rv == 0) as i32;
        
        while left < right && zeroes_used > k {
            if nums[left] == 0 {
                zeroes_used -= 1;
            }
            left += 1;
        }
        
        best = std::cmp::max(best, right + 1 - left);
    }

    let res = best as i32 - 1;
    if res == nums.len() as i32 {
        return res - 1;
    } else if res < 0 {
        return 0;
    } else {
        return res;
    }
}