fn partition_labels(s: String) -> Vec<i32> {
    let mut last_pos: Vec<usize> = vec![0; 26];
    let v = s.as_bytes();

    for (c, pos) in v.iter().copied().zip(0..) {
        let i = (c - 'a' as u8) as usize;
        last_pos[i] = pos;
    }

    let mut res = Vec::with_capacity(26);
    let mut left = 0;

    while left < v.len() {
        let mut right = left;
        let mut i = left;

        while i <= right {
            let k = v[i];
            let k_i = (k - 'a' as u8) as usize;
            right = usize::max(right, last_pos[k_i]);
            i += 1;
        }

        res.push((right - left + 1) as i32);
        left = right + 1
    }

    return res;
}
