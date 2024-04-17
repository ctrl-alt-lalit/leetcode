pub fn letter_combinations(digits: String) -> Vec<String> {
    if digits.is_empty() {
        return vec![];
    }

    let mut res = get_letters(digits.chars().next().unwrap());
    for c in digits.chars().skip(1) {
        res = combine(res, get_letters(c));
    }
    return res;
}

fn combine(v1: Vec<String>, v2: Vec<String>) -> Vec<String> {
    return v1
        .iter()
        .flat_map(|s1| v2.iter().map(|s2| s1.clone() + s2.as_str()))
        .collect();
}

fn get_letters(n: char) -> Vec<String> {
    return match n {
        '1' => vec![String::from("")],
        '2' => vec![String::from("a"), String::from("b"), String::from("c")],
        '3' => vec![String::from("d"), String::from("e"), String::from("f")],
        '4' => vec![String::from("g"), String::from("h"), String::from("i")],
        '5' => vec![String::from("j"), String::from("k"), String::from("l")],
        '6' => vec![String::from("m"), String::from("n"), String::from("o")],
        '7' => vec![
            String::from("p"),
            String::from("q"),
            String::from("r"),
            String::from("s"),
        ],
        '8' => vec![String::from("t"), String::from("u"), String::from("v")],
        '9' => vec![
            String::from("w"),
            String::from("x"),
            String::from("y"),
            String::from("z"),
        ],
        _ => panic!("not a digit"),
    };
}
