pub fn judge_square_sum(c: i32) -> bool {
    let cf = c as f64;
    let d = f64::sqrt(cf);
    for a2 in (0..=(d.trunc() as i32)).map(|n| (n * n) as f64) {
        let b = f64::sqrt(cf - a2);
        if (b == b.trunc()) {
            return true;
        }
    }

    return false;
}
