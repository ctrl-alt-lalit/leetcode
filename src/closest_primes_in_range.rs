fn closest_primes(left: i32, right: i32) -> Vec<i32> {
    let sqrt = usize::max(f64::sqrt(right as f64) as usize + 1, 10);
    let mut prime_list = Vec::with_capacity(sqrt);

    for x in 0..sqrt as i32 {
        try_expand_prime_list(x, &mut prime_list);
    }

    let mut tweener_primes = Vec::with_capacity((right - left) as usize);
    for x in left..=right {
        if is_prime(x, &prime_list) {
            tweener_primes.push(x);
        }
    }

    if tweener_primes.len() < 2 {
        return vec![-1, -1];
    }

    let mut min_pair_val = i32::MAX;
    let mut min_pair: (i32, i32) = (0, 0);
    for i in 0..(tweener_primes.len() - 1) {
        let j = i + 1;
        let pair_val = tweener_primes[j] - tweener_primes[i];
        if pair_val < min_pair_val {
            min_pair_val = pair_val;
            min_pair = (tweener_primes[i], tweener_primes[j]);
            if pair_val == 2 {
                break;
            }
        }
    }

    return vec![min_pair.0, min_pair.1];
}

fn try_expand_prime_list(n: i32, prime_list: &mut Vec<i32>) {
    if n < 2 {
        return;
    }

    if n == 2 {
        prime_list.push(2);
        return;
    }

    if !is_prime(n, prime_list) {
        return;
    }

    prime_list.push(n);
}

fn is_prime(x: i32, prime_list: &Vec<i32>) -> bool {
    for p in prime_list.iter().copied() {
        if x == p {
            return true;
        } else if x % p == 0 || x < p {
            return false;
        }
    }
    return true;
}
