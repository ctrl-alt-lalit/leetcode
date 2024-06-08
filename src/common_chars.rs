pub fn common_chars(words: Vec<String>) -> Vec<String> {
    let mut used_overall = [i8::MAX; 26];

    for word in words {
        let mut used_word = [0; 26];
        for c in word.bytes() {
            used_word[(c as usize) - ('a' as usize)] += 1;
        }

        for (x, y) in used_overall.iter_mut().zip(used_word.into_iter()) {
            *x = i8::min(*x, y);
        }
    }

    return used_overall
        .into_iter()
        .enumerate()
        .filter(|(_, x)| *x > 0)
        .map(|(i, x)| vec![((i as u8) + ('a' as u8)) as char; x as usize])
        .flatten()
        .map(|c| String::from(c as char))
        .collect();
}
