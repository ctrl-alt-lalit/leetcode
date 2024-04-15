pub fn max_area(height: Vec<i32>) -> i32 {
    // we have a known single initial state of w=n that we can iterate from
    // given a span (h_tall,h_short,w), it's only possible for a w-k subspan to be taller if it does not contain the column at h_short
    // so we can greedily iterate w -> w-1 by leaving the shorter column
    let mut curr_max = 0;
    let (mut l, mut r): (usize, usize) = (0, height.len() - 1);

    while l < r {
        let (hl, hr) = (height[l], height[r]);
        let area = std::cmp::min(hl, hr) * (r - l) as i32;
        curr_max = std::cmp::max(area, curr_max);

        if hl < hr {
            l += 1
        } else {
            r -= 1;
        }
    }

    return curr_max;
}
