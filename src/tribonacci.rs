pub fn tribonacci(n: i32) -> i32 {
    let mut memo = [None; 38];
    memo[0] = Some(0);
    memo[1] = Some(1);
    memo[2] = Some(1);

    return f(n as usize, &mut memo);
}

fn f(i: usize, memo: &mut [Option<i32>; 38]) -> i32 {
    if let Some(cached) = memo[i] {
        return cached;
    }

    let res = f(i-1, memo) + f(i-2, memo) + f(i-3, memo);
    memo[i] = Some(res);
    return res;
}