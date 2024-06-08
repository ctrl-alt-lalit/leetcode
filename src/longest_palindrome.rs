pub fn longest_palindrome(s: String) -> i32 {
    const UPPER_A: usize = ('A' as u8) as usize;
    const LOWER_Z: usize = ('z' as u8) as usize;
    let mut arr = [0i16; LOWER_Z - UPPER_A + 1];

    for c in s.bytes() {
        arr[(c as usize) - UPPER_A] += 1;
    }

    let mut total = 0;
    for x in arr.into_iter().map(|x| x as i32) {
        if x % 2 == 0 {
            total += x
        } else {
            total += (x - 1)
        }
    }

    if total < (s.len() as i32) {
        total += 1;
    }

    return total;
}
