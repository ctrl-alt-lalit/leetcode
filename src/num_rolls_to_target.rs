pub fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 {
    let mut memo = vec![vec![None; (target + 1) as usize]; (n + 1) as usize];
    return f(n, k, target, &mut memo);
}

const MOD: i32 = 1_000_000_007;

fn f(num_dice: i32, num_faces: i32, target: i32, memo: &mut Vec<Vec<Option<i32>>>) -> i32 {
    if (target < num_dice) || (num_dice * num_faces < target) {
        return 0;
    }

    if num_dice == 1 {
        return 1;
    }

    if let Some(cached) = memo[num_dice as usize][target as usize] {
        return cached;
    }

    let mut sum = 0;
    for roll in 1..=i32::min(num_faces, target) {
        sum = (sum + f(num_dice - 1, num_faces, target - roll, memo)) % MOD;
    }

    memo[num_dice as usize][target as usize] = Some(sum);

    return sum;
}
