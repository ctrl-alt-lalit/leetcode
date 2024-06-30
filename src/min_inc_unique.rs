pub fn min_increment_for_unique(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable();
    let mut prev = i32::MIN;
    let mut res = 0;

    for x in nums.into_iter() {
        if x <= prev {
            let y = prev + 1;
            res += (y - x);
            prev = y;
        } else {
            prev = x;
        }
    }

    return res;
}
