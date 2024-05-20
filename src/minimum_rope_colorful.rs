pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
    let s = colors.as_bytes();
    let mut sum = 0;
    let mut run_max = needed_time[0];

    for i in 1..s.len() {
        if s[i] != s[i - 1] {
            run_max = needed_time[i]
        } else {
            sum += i32::min(needed_time[i], run_max);
            run_max = i32::max(needed_time[i], run_max);
        }
    }

    return sum;
}
