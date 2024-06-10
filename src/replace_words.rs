use std::collections::BTreeSet;

pub fn replace_words(dictionary: Vec<String>, sentence: String) -> String {
    let dict: BTreeSet<String> = dictionary.into_iter().collect();

    let mut res = String::with_capacity(sentence.len());
    for word in sentence.split(' ') {
        let root = get_root(word, &dict);
        res.push_str(root);
        res.push(' ');
    }

    res.pop();
    return res;
}

fn get_root<'a>(word: &'a str, dict: &'a BTreeSet<String>) -> &'a str {
    let first = String::from(&word[0..1]);
    let last = String::from(word);

    for root in dict.range(first..last) {
        if root.len() <= word.len() && root.bytes().zip(word.bytes()).all(|(x, y)| x == y) {
            return root.as_str();
        }
    }

    return word;
}
