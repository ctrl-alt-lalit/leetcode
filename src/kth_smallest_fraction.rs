use std::collections::BinaryHeap;

#[derive(PartialOrd, Debug, Clone, PartialEq)]
struct Elem {
    frac: f64,
    num_i: usize,
    den_i: usize,
}

impl Eq for Elem {}

impl Ord for Elem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self.frac.total_cmp(&other.frac);
    }
}

impl Elem {
    fn new(arr: &Vec<i32>, num_i: usize, den_i: usize) -> Self {
        return Self {
            frac: -(arr[num_i] as f64) / (arr[den_i] as f64),
            num_i,
            den_i,
        };
    }
}

pub fn kth_smallest_prime_fraction(arr: Vec<i32>, k: i32) -> Vec<i32> {
    let n = arr.len();
    let mut pq: BinaryHeap<Elem> = BinaryHeap::with_capacity(n);

    // initialize with fractions that have the highest denominator
    for i in 0..(n - 1) {
        pq.push(Elem::new(&arr, i, n - 1));
    }

    // for the next k-1 times, replace fractions with slightly smaller denominators
    for _ in 0..(k - 1) {
        let Elem { num_i, den_i, .. } = pq.pop().unwrap();
        pq.push(Elem::new(&arr, num_i, den_i - 1));
    }

    let x = pq.pop().unwrap();
    return vec![arr[x.num_i], arr[x.den_i]];
}
