pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = Vec::with_capacity(1 << nums.len());
    res.push(vec![]);

    for x in nums.into_iter().rev() {
        let old_len = res.len();
        for i in 0..old_len {
            res.push(res[i].clone());
            res.last_mut().unwrap().push(x);
        }
    }

    return res;
}
