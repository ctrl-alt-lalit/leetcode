use std::cmp::{max, min};

type Vv<T> = Vec<Vec<T>>;

pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
    if !nk_valid(n, k) {
        return vec![];
    }

    let mut memo: Vv<Option<Vv<i32>>> = (0..k).map(|_| (0..n).map(|_| None).collect()).collect();

    return combo_internal(k, n, &mut memo);
}

fn combo_internal(k: i32, n: i32, memo: &mut Vv<Option<Vv<i32>>>) -> Vv<i32> {
    if let Some(cached) = &memo[k as usize - 1][n as usize - 1] {
        return cached.clone();
    }

    if !nk_valid(n, k) {
        return vec![];
    }

    if k == 1 {
        // n is guaranteed to be [1,9] from prior check
        return vec![vec![n]];
    }

    let mut res: Vv<i32> = Vv::new();
    let min_x = max(n - 9, 1);

    for x in min_x..=(n - 1) {
        let y = n - x;
        let mut tmp = combo_internal(k - 1, x, memo);

        for mut v in tmp.into_iter() {
            match v.binary_search(&y) {
                Ok(_) => {
                    continue;
                }
                Err(i) => v.insert(i, y),
            }

            match res.binary_search(&v) {
                Ok(_) => {}
                Err(i) => res.insert(i, v),
            }
        }
    }

    memo[k as usize - 1][n as usize - 1] = Some(res.clone());
    return res;
}

fn nk_valid(n: i32, k: i32) -> bool {
    let min = ((k + 1) * k) / 2;
    let max = match k {
        1 => 09,
        2 => 17,
        3 => 24,
        4 => 30,
        5 => 35,
        6 => 39,
        7 => 42,
        8 => 44,
        9 => 45,
        _ => panic!("k is out of range"),
    };
    return (n >= min) && (n <= max);
}
