fn minimum_index(nums: Vec<i32>) -> i32 {
    let dom = find_dominant(nums.clone());

    let mut d_arr = vec![0; nums.len()];
    let mut d_count = 0;

    for (i, x) in nums.iter().copied().enumerate() {
        d_count += (x == dom) as i32;
        d_arr[i] = d_count;
    }

    for (l_ct, l_len) in d_arr.iter().copied().zip(1..) {
        let dom_left = (l_ct * 2 > l_len);
        let r_len = (d_arr.len() - (l_len as usize)) as i32;
        let r_ct = d_count - l_ct;
        let dom_right = (r_ct * 2 > r_len) && (r_len > 0);

        if dom_left && dom_right {
            return l_len - 1;
        }
    }

    return -1;
}

fn find_dominant(mut v: Vec<i32>) -> i32 {
    v.sort_unstable();
    let mid = v.len() / 2;
    return v[mid];
}
