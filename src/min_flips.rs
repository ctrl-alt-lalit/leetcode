pub fn min_flips(a: i32, b: i32, c: i32) -> i32 {
    return (0_u8..32_u8).into_iter().fold(0, |acc, i| acc + check_bit(a, b, c, i));
}

fn check_bit(a: i32, b: i32, c: i32, i: u8) -> i32 {
    let mask = (1 << i);
    let a_set = ((a & mask) > 0);
    let b_set = ((b & mask) > 0);
    let c_set = ((c & mask) > 0);

    return match(a_set, b_set,c_set) {
        (true, true, true) => 0,
        (true, true, false) => 2,
        (true, false, true) => 0,
        (true, false, false) => 1,
        (false, true, true) => 0,
        (false, true, false) => 1,
        (false, false, true) => 1,
        (false, false, false) => 0,
    };
}