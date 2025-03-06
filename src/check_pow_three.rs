fn check_powers_of_three(mut n: i32) -> bool {
    let mut divisor = i32::pow(3, 15);

    while (divisor > 0) {
        if n >= (2 * divisor) {
            return false;
        } else if n >= divisor {
            n -= divisor
        }
        divisor /= 3;
    }

    return true;
}
