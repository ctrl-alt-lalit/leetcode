// 1, 4, 8, 12
// -, 2*2 + 0, 3*2 + 1*2, 4*2 + 2*2,  5*2 + 3*2
// for t >= 2: f(t) = 2 * (2t -2) = 4 * (t-1)
// for x >= 3: g(x) = 4 * sum(2..[x-1]) + 5 = 4 * [(x-1 * [x])/2 - 1] + 5 = (2 * [x-1] * x) + 1
// actually works for 1 and 2 too

fn colored_cells(n: i32) -> i64 {
    let x = n as i64;
    return (2 * (x - 1) * x) + 1;
}
