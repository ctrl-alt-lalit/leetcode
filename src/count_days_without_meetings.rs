fn count_days(days: i32, meetings: Vec<Vec<i32>>) -> i32 {
    let mut v: Vec<(i32, i32)> = meetings.into_iter().map(|x| (x[0], x[1])).collect();
    v.sort_unstable();
    v.push((days + 1, days + 1));

    let mut total_saved = 0;
    let mut left = 1;
    for (start, end) in v.into_iter() {
        //       4 5 6 ...
        // 1 2 3
        // -------------
        //     3 4 5 6 ...
        // 1 2 3 4
        if start < left {
            left = i32::max(end + 1, left);
        }
        // 1 2 3 4 ...
        // 1 2 3 4
        // -------------
        // 1 2 3 4 ...
        //     3 4 5 6
        else {
            total_saved += (start - left);
            left = end + 1;
        }
    }

    return total_saved;
}
