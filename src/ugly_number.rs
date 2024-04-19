pub fn is_ugly(n: i32) -> bool {
    let mut x = n;
    if x == 0 {
        return false;
    }

    while x % 2 == 0 {
        x /= 2;
    }

    while x % 3 == 0 {
        x /= 3;
    }

    while x % 5 == 0 {
        x /= 5;
    }

    return x == 1;
}
