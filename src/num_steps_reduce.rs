pub fn num_steps(s: String) -> i32 {
    let mut v: [u128; 4] = [0, 0, 0, 0];

    for (chunk, i) in s.as_bytes().rchunks(128).zip((0..4).rev()) {
        v[i] = chunk
            .iter()
            .rev()
            .map(|c| c - ('0' as u8))
            .enumerate()
            .fold(0, |acc, (j, b)| acc + ((b as u128) << j));
    }

    let mut steps = 0;
    while !is_one(&v) {
        steps += 1;
        if is_even(&v) {
            div_two(&mut v);
        } else {
            add_one(&mut v);
        }
    }

    return steps;
}

fn add_one(v: &mut [u128; 4]) {
    let mut carry = true;
    for x in v.iter_mut().rev() {
        if !carry {
            break;
        }

        (*x, carry) = u128::overflowing_add(*x, 1);
    }
}

fn div_two(v: &mut [u128; 4]) {
    let mut underflow = 0;
    for x in v.iter_mut() {
        let next_underflow = (*x & 1) << 127;
        *x = (*x >> 1) | underflow;
        underflow = next_underflow;
    }
}

fn is_one(v: &[u128; 4]) -> bool {
    return (v[0] == 0) && (v[1] == 0) && (v[2] == 0) && (v[3] == 1);
}

fn is_even(v: &[u128; 4]) -> bool {
    return v[3] % 2 == 0;
}
