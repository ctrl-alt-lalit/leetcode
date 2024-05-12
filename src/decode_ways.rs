pub fn num_decodings(s: String) -> i32 {
    let v = s.as_bytes();
    let mut memo = vec![None; v.len()];
    return f(v, 0, &mut memo);
}

fn f(s: &[u8], i: usize, memo: &mut Vec<Option<i32>>) -> i32 {
    if i == s.len() - 1 {
        let c1 = (s[i] as char).to_digit(10).unwrap();
        return (c1 != 0) as i32;
    }

    if i >= s.len() {
        return 1;
    }

    if let Some(cached) = memo[i] {
        return cached;
    }

    let c1 = (s[i] as char).to_digit(10).unwrap();
    let c2 = c1 * 10 + (s[i + 1] as char).to_digit(10).unwrap();

    let res = if (c1 == 0) {
        0
    } else if (c2 <= 26) {
        f(s, i + 1, memo) + f(s, i + 2, memo)
    } else {
        f(s, i + 1, memo)
    };

    memo[i] = Some(res);
    return res;
}
