pub fn min_steps(s: String, t: String) -> i32 {
    let sv = process_str(s);
    let tv = process_str(t);

    let diff = sv
        .into_iter()
        .zip(tv.into_iter())
        .fold(0, |acc, (x, y)| acc + i32::abs(x - y));

    return (diff + 1) / 2;
}

fn process_str(s: String) -> [i32; 26] {
    let mut res = [0; 26];

    for c in s.as_bytes() {
        res[(*c - ('a' as u8)) as usize] += 1;
    }

    return res;
}
