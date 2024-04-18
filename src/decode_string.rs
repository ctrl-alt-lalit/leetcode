pub fn decode_string(s: String) -> String {
    return decode(s.as_str()).0;
}

fn decode(s: &str) -> (String, usize) {
    let mut iter = s.chars().enumerate();
    let mut res = String::new();
    let mut depth = 0;
    let mut offset = 0;

    loop {
        let Some((curr_idx, curr_ch)) = iter.next() else { break };
        offset = curr_idx;

        if curr_ch.is_alphabetic() {
            res.push(curr_ch);
            continue;
        }

        if curr_ch == ']' {
            depth -= 1;
            if depth < 0 {
                break;
            } else {
                continue;
            }
        }

        let lb_idx = iter.find(|(_, c)| *c == '[').unwrap().0;
        let k = s.get(curr_idx..lb_idx).unwrap().parse::<i32>().unwrap();
        depth += 1;

        let (sub, skip_num) = decode(s.get(lb_idx + 1..).unwrap());

        (0..k).for_each(|_| res.push_str(sub.as_str()));
        (0..skip_num).for_each(|_| {
            iter.next();
        })
    }

    return (res, offset);
}
