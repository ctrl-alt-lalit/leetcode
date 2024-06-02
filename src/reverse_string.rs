// Constraints: Must modify in-place with O(1) extra space
pub fn reverse_string(s: &mut Vec<char>) {
    let mut i = 0;
    let mut j = s.len() - 1;
    while i < j {
        let x = s[i];
        s[i] = s[j];
        s[j] = x;
        i += 1;
        j -= 1;
    }

    // Or just s.reverse()
}
