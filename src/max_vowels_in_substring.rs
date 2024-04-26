pub fn max_vowels(s: String, k: i32) -> i32 {
    let ku = k as usize;
    let mut best = s.chars().take(ku).fold(0, |acc, c| acc + is_vowel(c));

    let mut add_iter = s.chars().skip(ku);
    let mut rem_iter = s.chars();
    let mut curr = best;
    
    while let Some(add) = add_iter.next() {
        curr = curr + is_vowel(add) - is_vowel(rem_iter.next().unwrap());
        best = std::cmp::max(curr, best);
    }

    return best;
}

fn is_vowel(c: char) -> i32 {
    return match c {
        'a' | 'e' | 'i' | 'o' | 'u' => 1,
        _ => 0
    };
}