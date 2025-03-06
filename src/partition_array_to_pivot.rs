fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
    let n = nums.len();
    let mut less: Vec<i32> = Vec::with_capacity(n);
    let mut greater: Vec<i32> = Vec::with_capacity(n);

    for x in nums.into_iter() {
        if x < pivot {
            less.push(x);
        } else if x > pivot {
            greater.push(x);
        }
    }

    let pivot_len = n - less.len() - greater.len();
    less.append(&mut vec![pivot; pivot_len]);
    less.append(&mut greater);
    return less;
}
