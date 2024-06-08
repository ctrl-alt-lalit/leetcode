pub fn append_characters(s: String, t: String) -> i32 {
    let sv = s.as_bytes();
    let tv = t.as_bytes();
    let n = tv.len();

    let mut i = 0;
    for c in sv.iter().copied() {
        if i < n && c == tv[i] {
            i += 1;
        }
    }

    return (n - i) as i32;
}
