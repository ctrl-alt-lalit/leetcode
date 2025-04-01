pub fn maximum_score(nums: Vec<i32>, mut k: i32) -> i32 {
    let mut high_tracker: Vec<_> = nums.iter().copied().zip(0usize..).collect();
    high_tracker.sort_unstable_by(|a, b| b.cmp(a));

    let sieve = make_sieve(high_tracker[0].0 as usize);
    let scores: Vec<_> = nums.iter().map(|x| prime_score(*x, &sieve)).collect();
    let left = make_left_arr(&scores);
    let right = make_right_arr(&scores);

    let mut res = 1i64;

    for (val, hti) in high_tracker.into_iter() {
        let num_subarrays = ((hti as i64 - left[hti] as i64) * (right[hti] as i64 - hti as i64));
        let num_ops = i64::min(num_subarrays, k as i64);
        let mult = mod_exp(val as i64, num_ops, 1_000_000_007);
        res = (res * mult) % 1_000_000_007;

        k -= num_ops as i32;
        if k <= 0 {
            break;
        }
    }

    return res as i32;
}

fn make_sieve(n: usize) -> Vec<i32> {
    let m = (f64::sqrt(n as f64).ceil() + 20.0) as usize;
    let mut v = vec![true; m];
    v[0] = false;
    v[1] = false;

    for i in 2..m {
        if (!v[i]) {
            continue;
        }

        for j in (i..m).step_by(i).skip(1) {
            v[j] = false;
        }
    }

    return v
        .into_iter()
        .zip(0..)
        .filter_map(|(b, i)| if b { Some(i) } else { None })
        .collect();
}

fn prime_score(mut x: i32, sieve: &Vec<i32>) -> i32 {
    let mut score = 0;
    let mut last_factor = 0;

    for y in sieve.iter().copied() {
        while x % y == 0 {
            score += (y != last_factor) as i32;
            last_factor = y;
            x /= y;
        }

        if y * y > x {
            break;
        }
    }

    score += (x > 1) as i32;

    return score;
}

fn mod_exp(mut base: i64, mut exp: i64, modulus: i64) -> i64 {
    let mut res = 1;
    base %= modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            res = (res * base) % modulus;
        }
        base = (base * base) % modulus;
        exp >>= 1;
    }
    return res;
}

/// cmp_fn(last, pushed) should return true if push can be made
struct MonotonicStack<T, F>
where
    F: Fn(&T, &T) -> bool,
{
    v: Vec<T>,
    cmp_fn: F,
}

impl<T, F> MonotonicStack<T, F>
where
    F: Fn(&T, &T) -> bool,
{
    pub fn with_capacity(n: usize, cmp_fn: F) -> Self {
        Self {
            v: Vec::with_capacity(n),
            cmp_fn,
        }
    }

    pub fn push(&mut self, elem: T) -> i32 {
        let mut num_popped = 0;

        while let Some(x) = self.v.last() {
            if !(self.cmp_fn)(x, &elem) {
                self.v.pop();
                num_popped += 1;
            } else {
                break;
            }
        }

        self.v.push(elem);
        return num_popped;
    }
}

// right[i] = index "j" of first element found, scanning right from score[i] where score[j] < score[i]
fn make_right_arr(scores: &Vec<i32>) -> Vec<i32> {
    let mut s: MonotonicStack<i32, _> = MonotonicStack::with_capacity(scores.len(), |l, r| l >= r);
    let mut right = vec![scores.len() as i32; scores.len()];
    let mut index_stack = Vec::with_capacity(scores.len());

    for (score, i) in scores.iter().copied().zip(0..) {
        let num_popped = s.push(score);
        for _ in 1..=num_popped {
            right[index_stack.pop().unwrap() as usize] = i;
        }
        index_stack.push(i);
    }

    return right;
}

// left[i] = index "j" of first element found, scanning left from score[i] where score[j] >= score[i]
fn make_left_arr(scores: &Vec<i32>) -> Vec<i32> {
    let mut s: MonotonicStack<i32, _> = MonotonicStack::with_capacity(scores.len(), |r, l| r > l);
    let mut left = vec![-1; scores.len()];
    let mut index_stack = Vec::with_capacity(scores.len());

    for (score, i) in scores.iter().copied().rev().zip((0..scores.len()).rev()) {
        let num_popped = s.push(score);
        for _ in 1..=num_popped {
            left[index_stack.pop().unwrap()] = i as i32;
        }

        index_stack.push(i);
    }

    return left;
}
