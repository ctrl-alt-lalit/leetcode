pub fn number_of_beams(bank: Vec<String>) -> i32 {
    let v: Vec<i32> = bank
        .into_iter()
        .map(|s| {
            s.as_bytes()
                .into_iter()
                .fold(0, |acc, c| acc + (*c == ('1' as u8)) as i32)
        })
        .filter(|x| *x != 0)
        .collect();

    let mut res = 0;
    for i in 1..v.len() {
        res += v[i] * v[i - 1];
    }

    return res;
}
