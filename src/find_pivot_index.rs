pub fn pivot_index(nums: Vec<i32>) -> i32 {
    let total: i32 = nums.iter().sum();

    let mut left_sum = 0;
    let mut pivot = -1;

    for (i,x) in nums.iter().enumerate() {
        let right_sum = total - left_sum - x;
        if left_sum == right_sum {
            pivot = i as i32;
            break;
        }

        left_sum += x;
    }

    return pivot;
}