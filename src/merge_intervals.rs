fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut v: Vec<_> = intervals.into_iter().map(|x| (x[0], x[1])).collect();
    v.sort_unstable();

    let mut res = Vec::new();
    let mut left = 0 - 1;
    let mut right = -1;

    for (start, end) in v.into_iter() {
        if start > right {
            if right != -1 {
                res.push(vec![left, right]);
            }

            left = start;
            right = end;
        } else {
            right = i32::max(right, end);
        }
    }

    res.push(vec![left, right]);

    return res;
}
