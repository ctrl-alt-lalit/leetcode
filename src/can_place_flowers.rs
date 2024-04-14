pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
    // number of potential flower placements scales from number of 0-runs.
    // [0 or 1 or 2 => 0] 0 or 00 -> 0 flowers
    // [3 or 4 => 1] 000 or 0000 -> 1 flower (010 or 0100 or 0010)
    // [5 or 6 => 2] 00000 or 000000 -> 2 flowers (01010 or 010100 or 001010 or 010010)
    // k-run of 0s yields max(ceil([k-2]/2),0) or max(floor([k-1]/2), 0)

    let v = [0]
        .into_iter()
        .chain(flowerbed.into_iter())
        .chain([0, 1].into_iter()); // pad beginning & end with 0s, add a 1 at the very end to force counting last run

    let mut run_len: i32 = 0;
    let mut possible_flowers: i32 = 0;

    for x in v.into_iter() {
        if x == 0 {
            run_len += 1;
        } else {
            possible_flowers += core::cmp::max(0, (run_len - 1) / 2);
            run_len = 0;

            if possible_flowers >= n {
                break;
            };
        }
    }
    return possible_flowers >= n;
}
