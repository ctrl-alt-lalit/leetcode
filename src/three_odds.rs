pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
    for i in 2..arr.len() {
        if is_odd(arr[i]) && is_odd(arr[i - 1]) && is_odd(arr[i - 2]) {
            return true;
        }
    }

    return false;
}

fn is_odd(x: i32) -> bool {
    return (x % 2 == 1);
}
