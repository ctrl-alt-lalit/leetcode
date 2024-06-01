pub fn count_triplets(arr: Vec<i32>) -> i32 {
    let mut count = 0;

    for l in 0..arr.len() {
        let mut xor = arr[l];
        for r in (l + 1)..arr.len() {
            xor ^= arr[r];
            if xor == 0 {
                count += (r - l) as i32
            }
        }
    }

    return count;
}
