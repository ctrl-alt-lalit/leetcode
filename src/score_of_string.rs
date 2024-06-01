pub fn score_of_string(s: String) -> i32 {
    let v = s.as_bytes();
    let mut sum = 0;
    for i in 1..v.len() {
        sum += u8::abs_diff(v[i], v[i - 1]) as i32;
    }
    return sum;
}
