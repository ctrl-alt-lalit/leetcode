pub fn is_n_straight_hand(mut hand: Vec<i32>, group_size: i32) -> bool {
    if (hand.len() as i32) % group_size != 0 {
        return false;
    }

    hand.sort_unstable();
    let mut v: Vec<(i32, i32)> = Vec::with_capacity(hand.len());

    for x in hand.into_iter() {
        let (y, _) = v.last().unwrap_or(&(-1, 0));

        if *y == x {
            v.last_mut().unwrap().1 += 1;
        } else {
            v.push((x, 1));
        }
    }

    let n = v.len() as i32;
    for i in 0..n {
        let (x, needed_groups) = v[i as usize];
        if needed_groups == 0 {
            continue;
        }

        // needs a group, but can't make any
        if i + group_size > n {
            return false;
        }

        // check that group can be constructed
        for j in i..(i + group_size) {
            let (y, count) = v[j as usize];

            // element is consecutive and has enough members
            if (y != x + j - i) || (count < needed_groups) {
                return false;
            }

            // update the count for subsequent iterations
            v[j as usize].1 -= needed_groups;
        }
    }

    return true;
}
