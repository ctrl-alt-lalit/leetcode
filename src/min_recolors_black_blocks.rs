fn minimum_recolors(blocks: String, k: i32) -> i32 {
    let v = blocks.as_bytes();
    let j = k as usize;
    let mut x = v
        .iter()
        .take(j)
        .fold(0, |acc, b| acc + (*b == 'B' as u8) as i32);

    let mut max = x;
    for i in j..v.len() {
        x = x + val(v, i) - val(v, i - j);
        max = i32::max(x, max);
    }

    return k - max;
}

fn val(v: &[u8], i: usize) -> i32 {
    return (v[i] == 'B' as u8) as i32;
}
