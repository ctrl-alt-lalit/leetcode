pub fn min_moves_to_seat(mut seats: Vec<i32>, mut students: Vec<i32>) -> i32 {
    seats.sort_unstable();
    students.sort_unstable();

    return seats
        .into_iter()
        .zip(students.into_iter())
        .fold(0, |acc, (x, y)| acc + i32::abs(x - y));
}
