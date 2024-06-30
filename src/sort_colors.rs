pub fn sort_colors(nums: &mut Vec<i32>) {
    let mut count = [0_usize; 3];
    for x in nums.iter().map(|x| *x as usize) {
        count[x] += 1;
    }

    count[1] += count[0];
    count[2] += count[1];

    for (i, x) in nums.iter_mut().enumerate() {
        if i < count[0] {
            *x = 0;
        } else if i < count[1] {
            *x = 1;
        } else {
            *x = 2;
        }
    }
}
