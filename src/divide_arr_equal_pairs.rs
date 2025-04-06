use crate::util::BitSet;

fn divide_array(nums: Vec<i32>) -> bool {
    let mut count = vec![0; 501];
    for x in nums.into_iter() {
        count[x as usize] += 1;
    }

    return count.into_iter().all(|c| c % 2 == 0);
}

fn divide_array2(mut nums: Vec<i32>) -> bool {
    nums.sort_unstable();

    let mut running_count = 1;
    for i in 1..nums.len() {
        if nums[i] != nums[i - 1] {
            if running_count % 2 == 1 {
                return false;
            }
            running_count = 1;
        } else {
            running_count += 1;
        }
    }

    return (running_count % 2 == 0);
}

fn divide_array3(nums: Vec<i32>) -> bool {
    let mut needs_pair = BitSet::with_len(501);
    for x in nums.into_iter() {
        needs_pair.flip(x as usize);
    }
    return needs_pair.count() == 0;
}
