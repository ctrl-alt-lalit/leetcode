fn check_valid_cuts(_n: i32, rectangles: Vec<Vec<i32>>) -> bool {
    let mut widths: Vec<_> = rectangles.iter().map(|rect| (rect[0], rect[2])).collect();
    widths.sort_unstable();
    if num_cuts(widths) >= 2 {
        return true;
    }
    let mut heights: Vec<_> = rectangles.iter().map(|rect| (rect[1], rect[3])).collect();
    heights.sort_unstable();
    return num_cuts(heights) >= 2;
}

fn num_cuts(v: Vec<(i32, i32)>) -> i32 {
    let mut prev_end = v[0].1;
    let mut count = 0;

    for (xl, xr) in v.into_iter() {
        if xl < prev_end {
            prev_end = i32::max(prev_end, xr);
        } else {
            count += 1;
            prev_end = xr;
        }
    }

    return count;
}
