pub fn equal_substring(s: String, t: String, max_cost: i32) -> i32 {
    let v: Vec<i32> = s
        .bytes()
        .zip(t.bytes())
        .map(|(x, y)| u8::abs_diff(x, y) as i32)
        .collect();

    let mut max_len = 0;
    let mut l = 0; //inclusive
    let mut r = 0; //exclusive
    let mut sum = 0;

    while r < v.len() {
        if sum + v[r] <= max_cost {
            sum += v[r];
            r += 1;
            max_len += 1;
        } else {
            sum = sum - v[l] + v[r];
            l += 1;
            r += 1;
        }
    }

    return max_len;
}
