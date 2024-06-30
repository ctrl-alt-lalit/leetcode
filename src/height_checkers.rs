pub fn height_checker(heights: Vec<i32>) -> i32 {
    let mut expected = heights.clone();
    expected.sort_unstable();

    return heights
        .into_iter()
        .zip(expected.into_iter())
        .fold(0, |acc, (x, y)| acc + (x != y) as i32);
}
