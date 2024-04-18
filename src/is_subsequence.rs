pub fn is_subsequence(s: String, t: String) -> bool {
    let mut s_it = s.chars().peekable();
    for c in t.chars() {
        if let Some(s_c) = s_it.peek() {
            if s_c == &c {
                _ = s_it.next();
            }
        } else {
            break;
        }
    }

    return s_it.peek().is_none();
}
