use std::collections::BTreeSet;

pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
    let mut memo: Vec<Vec<Option<Vec<String>>>> = vec![vec![None; s.len() + 1]; s.len() + 1];
    let dict = BTreeSet::from_iter(word_dict.into_iter());
    f(&s, 0, s.len(), &dict, &mut memo);
    return memo[0][s.len()].take().unwrap_or_default();
}

fn f(
    s: &String,
    l: usize,
    r: usize,
    dict: &BTreeSet<String>,
    memo: &mut Vec<Vec<Option<Vec<String>>>>,
) -> i32 {
    if let Some(cached) = memo[l][r].as_ref() {
        return cached.len() as i32;
    }

    let mut res: Vec<String> = Vec::new();
    let slice = s.get(l..r).unwrap();
    if dict.contains(slice) {
        res.push(String::from(slice));
    }

    for m in (l + 1)..r {
        let x = f(s, l, m, dict, memo);
        let y = if (x > 0) { f(s, m, r, dict, memo) } else { 0 };
        if (x * y > 0) {
            res.reserve((x * y) as usize);
            for s1 in memo[l][m].as_ref().unwrap() {
                for s2 in memo[m][r].as_ref().unwrap() {
                    let mut s3 = s1.clone();
                    s3.reserve_exact(s2.len() + 1);
                    s3.push(' ');
                    s3.push_str(s2.as_str());
                    res.push(s3);
                }
            }
        }
    }

    res.sort();
    res.dedup();

    let len = res.len() as i32;
    memo[l][r] = Some(res);

    return len;
}
