pub fn partition(s: String) -> Vec<Vec<String>> {
    let bytes = s.as_bytes();
    let n = bytes.len();
    let mut res = vec![];
    g(bytes, 0, n, &mut vec![], &mut res);
    return res;
}

fn g(s: &[u8], l: usize, r: usize, prepend: &mut Vec<String>, res: &mut Vec<Vec<String>>) {
    if r <= l {
        return;
    }

    if l + 1 == r {
        res.push(prepend.clone());
        res.last_mut().unwrap().push(String::from(s[l] as char));
        return;
    }

    for i in l + 1..=r {
        if !is_palindrome(s, l, i) {
            continue;
        }
        let new_pal = String::from_utf8(s[l..i].to_vec()).unwrap();
        if i == r {
            res.push(prepend.clone());
            res.last_mut().unwrap().push(new_pal);
            continue;
        }

        prepend.push(new_pal);
        g(s, i, r, prepend, res);
        prepend.pop();
    }

    return;
}

fn is_palindrome(s: &[u8], l: usize, r: usize) -> bool {
    let mut i = l;
    let mut j = r - 1;

    while j < r && i <= j {
        if (s[i] != s[j]) {
            return false;
        }

        i += 1;
        j -= 1;
    }

    return true;
}
