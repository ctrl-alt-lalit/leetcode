pub fn subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32 {
    let mut prefix = 0;
    let mut k_set = vec![0; k as usize + 1];
    k_set[0] = 1; // starting from very begining of array is valid

    for x in nums.into_iter() {
        prefix = (prefix + x) % k;
        if prefix < 0 {
            prefix += k
        }
        k_set[prefix as usize] += 1;
    }

    return k_set.into_iter().fold(0, |acc, x| acc + choose_2(x));
}

fn choose_2(n: i32) -> i32 {
    return n * (n - 1) / 2;
}
