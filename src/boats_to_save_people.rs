pub fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {
    people.sort_unstable();
    let mut l = 0;
    let mut r = people.len() - 1;
    let mut boats = 0;

    while l <= r && r < people.len() {
        l += ((people[l] + people[r]) <= limit) as usize;
        r -= 1;
        boats += 1;
    }

    return boats;
}
