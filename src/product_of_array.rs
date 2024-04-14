pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    // Constraint: Do not divide

    let prefixes = [1].iter().chain(nums.iter()).fold(vec![], |mut acc, x| {
        let y = acc.last().unwrap_or(&1);
        acc.push(x * y);
        return acc;
    });

    let suffixes = nums.iter().chain([1].iter()).rfold(vec![], |mut acc, x| {
        let y = acc.last().unwrap_or(&1);
        acc.push(x * y);
        return acc;
    }); // note that this array is reversed

    return (0..nums.len())
        .map(|i| prefixes[i] * suffixes[nums.len() - i - 1])
        .collect();
}

pub fn product_except_self2(nums: Vec<i32>) -> Vec<i32> {
    // Constraint: Do not divide
    // Same strategy as before, but instead of allocating 3 arrays, reuse the prefix array as output
    // and roll suffix into single accumulator

    let mut v = [1].iter().chain(nums.iter()).fold(vec![], |mut acc, x| {
        let y = acc.last().unwrap_or(&1);
        acc.push(x * y);
        return acc;
    });
    _ = v.pop();
    println!("{:?}", v);

    let mut suffix_acc = 1;
    for (mut x, num) in v.iter_mut().zip(nums.iter()).rev() {
        *x *= suffix_acc;
        suffix_acc *= num
    }

    return v;
}

pub fn product_except_self3(nums: Vec<i32>) -> Vec<i32> {
    // Constraint: Do not divide
    // Create prefix array using similar accumulator loop, rather than iterator magic

    let mut v = [1]
        .iter()
        .chain(nums.iter().take(nums.len() - 1))
        .map(|x| x.clone())
        .collect::<Vec<i32>>();
    let mut prev_val = 1;
    for mut x in v.iter_mut() {
        *x *= prev_val;
        prev_val = x.clone()
    }

    let mut suffix_acc = 1;
    for (mut x, num) in v.iter_mut().zip(nums.iter()).rev() {
        *x *= suffix_acc;
        suffix_acc *= num
    }

    return v;
}
