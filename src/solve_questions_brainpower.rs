fn most_points(questions: Vec<Vec<i32>>) -> i64 {
    let mut memo = vec![None; questions.len()];
    return most_points_inner(&questions, 0, &mut memo);
}

fn most_points_inner(q: &Vec<Vec<i32>>, i: usize, memo: &mut Vec<Option<i64>>) -> i64 {
    if i >= q.len() {
        return 0;
    }

    if let Some(cached) = memo[i] {
        return cached;
    }

    let points = q[i][0] as i64;
    let brainpower = q[i][1] as usize;

    let solve = points + most_points_inner(q, i + brainpower + 1, memo);
    let skip = most_points_inner(q, i + 1, memo);
    let best = i64::max(solve, skip);
    memo[i] = Some(best);
    return best;
}
