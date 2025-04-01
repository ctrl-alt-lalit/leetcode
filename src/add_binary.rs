fn add_binary(a: String, b: String) -> String {
    let (shorter, longer) = if a.len() < b.len() {
        (a.as_bytes(), b.as_bytes())
    } else {
        (b.as_bytes(), a.as_bytes())
    };
    let zero_padding = std::iter::repeat_n(b'0', longer.len() - shorter.len() + 1);
    let op1 = shorter.iter().copied().rev().chain(zero_padding);
    let op2 = longer.iter().copied().rev().chain(std::iter::once(b'0'));

    let mut res = String::from_utf8(vec![b'0'; longer.len() + 1]).unwrap();
    let mut i = longer.len();
    let mut carry = false;
    for (a, b) in op1.zip(op2) {
        let a_is_1 = (a == b'1');
        let b_is_1 = (b == b'1');

        let val = if (a_is_1 ^ b_is_1 ^ carry) {
            b'1'
        } else {
            b'0'
        };
        carry = (carry & (a_is_1 ^ b_is_1)) | (a_is_1 & b_is_1);

        // SAFETY: val is always '1' or '0' which are both valid utf-8
        unsafe {
            res.as_bytes_mut()[i] = val;
        }

        i -= 1;
    }

    if res.as_bytes()[0] != b'1' {
        return res.split_off(1);
    } else {
        return res;
    }
}
