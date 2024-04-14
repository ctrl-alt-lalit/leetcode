pub fn remove_stars(s: String) -> String {
    let mut v = String::from("");
    for c in s.chars() {
        if c == '*' {
            _ = v.pop()
        } else {
            v.push(c)
        }
    }
    return v;
}
