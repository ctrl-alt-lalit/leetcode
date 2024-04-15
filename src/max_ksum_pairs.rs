pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
    // This problem annoyingly doesn't account for language-dependent implicit const vs. non-const in fn signatures
    // but we can use this as an opportunity to take out any k/2 values
    let val_to_remove: Option<i32> = if k % 2 == 0 { Some(k / 2) } else { None };

    let mut v = nums
        .iter()
        .filter(|x| Some(*x) != val_to_remove.as_ref())
        .map(|x| *x)
        .collect::<Vec<i32>>();
    v.sort_unstable();

    let num_half_k = (nums.len() - v.len()) as i32;
    let mut num_pairs = num_half_k / 2;

    if v.is_empty() {
        return num_pairs;
    }

    let mut l = 0;
    let mut r = v.len() - 1;
    while l < r {
        // count up everything except k/2 in search loop
        let x = v[l];
        let y = v[r];

        if x + y == k {
            // equal -> count up pairs
            while (l < r) && (v[l] == x) && (v[r] == y) {
                l += 1;
                r -= 1;
                num_pairs += 1;
            }
        } else if x + y < k {
            // too low -> make left larger
            while (l < r) && (v[l] == x) {
                l += 1
            }
        } else {
            // too high ->m make -> right smaller
            while (l < r) && (v[r] == y) {
                r -= 1;
            }
        }
    }

    return num_pairs;
}

pub fn max_operations2(nums: Vec<i32>, k: i32) -> i32 {
    // Difference: abuse the fact that nums[i] is at least 1
    let val_to_remove = if k % 2 == 0 { k / 2 } else { -1 };
    let mut num_half_k = 0;

    let mut v = nums
        .into_iter()
        .filter(|x| {
            if *x >= k {
                return false;
            } else if *x == val_to_remove {
                num_half_k += 1;
                return false;
            } else {
                return true;
            }
        })
        .map(|x| x)
        .collect::<Vec<i32>>();
    v.sort_unstable();

    let mut num_pairs = num_half_k / 2;

    if v.is_empty() {
        return num_pairs;
    }

    let mut l = 0;
    let mut r = v.len() - 1;
    while l < r {
        // count up everything except k/2 in search loop
        let x = v[l];
        let y = v[r];

        if x + y == k {
            // equal -> count up pairs
            while (l < r) && (v[l] == x) && (v[r] == y) {
                l += 1;
                r -= 1;
                num_pairs += 1;
            }
        } else if x + y < k {
            // too low -> make left larger
            while (l < r) && (v[l] == x) {
                l += 1
            }
        } else {
            // too high ->m make -> right smaller
            while (l < r) && (v[r] == y) {
                r -= 1;
            }
        }
    }

    return num_pairs;
}
