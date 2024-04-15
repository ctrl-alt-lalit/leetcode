pub fn compress(chars: &mut Vec<char>) -> i32 {
    //Constraint: Modify in place
    let mut write: usize = 0;
    let mut prev_char: Option<char> = None;
    let mut run_len: usize = 0;

    for read in 0..chars.len() {
        let curr = chars[read];
        if let Some(prev) = prev_char {
            if prev != curr {
                write_to_chars(chars, &mut write, run_len, prev);
                run_len = 0;
            }
        }

        prev_char = Some(curr);
        run_len += 1;
    }

    if let Some(prev) = prev_char {
        write_to_chars(chars, &mut write, run_len, prev);
    }

    return write as i32;
}

fn write_to_chars(chars: &mut Vec<char>, write: &mut usize, run_len: usize, new_ch: char) {
    chars[*write] = new_ch;
    *write += 1;

    if run_len > 1 {
        for len_ch in run_len.to_string().chars().into_iter() {
            chars[*write] = len_ch;
            *write += 1;
        }
    }
}
