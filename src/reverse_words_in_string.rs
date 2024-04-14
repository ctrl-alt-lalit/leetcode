pub fn reverse_words(s: String) -> String {
    //Challenge: do this in-place

    // Would likely want to use C++ for this.
    // 1. delete extra spaces
    // 2. completely reverse string
    // 3. (re-)reverse individual words

    // Steps below could be done all at once for efficiency, (e.g. have small buffer and rotate words chunks individually)
    // but that's a lot of index-fiddling I don't want to do.
}

pub fn reverse_words_trivial(s: String) -> String {
    let mut res =
        s.split(' ')
            .filter(|word| word.len() > 0)
            .rev()
            .fold(String::new(), |mut acc, word| {
                acc.push_str(word);
                acc.push(' ');
                return acc;
            });

    _ = res.pop();
    return res;
}
