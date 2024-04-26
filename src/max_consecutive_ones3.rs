pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
    let mut zeroes_used = 0;
    let mut left: usize = 0;
    let mut best: usize = 0;

    for (right, rv) in nums.iter().cloned().enumerate() {
        zeroes_used += ((rv == 0) as i32);
        
        while left < right && zeroes_used > k {
            if nums[left] == 0 {
                zeroes_used -= 1;
            }
            left += 1;
        }

        let dist = if (left == right) && (nums[left] == 0) && (k == 0) {
            0
        } else {
            right + 1 - left
        };

        best = std::cmp::max(best, dist);
    }

    return best as i32;
}