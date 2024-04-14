pub fn increasing_triplet(nums: Vec<i32>) -> bool {
    // 1. Find doublet subseq at end
    // 2. Update doublet as scanning back
    // 3. Find x smaller than minor of doublet

    if nums.len() < 3 {
        return false;
    }

    let mut third_val = nums.last().unwrap().clone();
    let mut biggest = (third_val, nums.len() - 1);
    let mut sec_val = i32::MIN;
    let mut sec_biggest = (i32::MIN, 0 as usize);

    for (i, x_ref) in nums.iter().enumerate().rev().skip(1) {
        let x = x_ref.clone();
        if x > third_val {
            third_val = x;
            biggest = (x, i);
        } else if x == third_val {
            continue; // a lower index is never better and triple relationships are exclusive
        } else {
            sec_val = x;
            sec_biggest = (x, i);
            break;
        }
    }
    let num_to_skip = nums.len() - sec_biggest.1;

    let mut res = false;
    for (i, x) in nums.into_iter().enumerate().rev().skip(num_to_skip) {
        if x < sec_val {
            res = true;
            break;
        } else if x == sec_val {
            continue;
        }

        // x > sec_val below

        if x > biggest.0 {
            // exclusive because we want biggest to have a high index
            sec_biggest = biggest;
            biggest = (x, i)
        } else if x < biggest.0 && x >= sec_biggest.0 {
            // inclusive so sec_biggest has a low index
            sec_biggest = (x, i);
        }

        if sec_biggest.1 < biggest.1 {
            //third_val = biggest.0; // value never read
            sec_val = sec_biggest.0;
        }
    }

    return res;
}
