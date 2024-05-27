pub fn check_record(n: i32) -> i32 {
    let mut prev = State::new(1, 1, 0, 1, 0, 0);

    for _ in 2..=(n as usize) {
        let mut curr = State::new(0, 0, 0, 0, 0, 0);

        // Option 1: Prepend an absence
        curr.no_late_one_abs = mod_add2(curr.no_late_one_abs, prev.no_abs());

        // Option 2: Prepend a late
        curr.one_late_no_abs = mod_add2(curr.one_late_no_abs, prev.no_late_no_abs);
        curr.two_late_no_abs = mod_add2(curr.two_late_no_abs, prev.one_late_no_abs);
        curr.one_late_one_abs = mod_add2(curr.one_late_one_abs, prev.no_late_one_abs);
        curr.two_late_one_abs = mod_add2(curr.two_late_one_abs, prev.one_late_one_abs);

        // Option 3: Prepend a present
        curr.no_late_no_abs = mod_add2(curr.no_late_no_abs, prev.no_abs());
        curr.no_late_one_abs = mod_add2(curr.no_late_one_abs, prev.one_abs());

        prev = curr;
    }

    return prev.sum();
}

#[derive(Clone, Debug)]
struct State {
    no_late_no_abs: i32,
    one_late_no_abs: i32,
    two_late_no_abs: i32,
    no_late_one_abs: i32,
    one_late_one_abs: i32,
    two_late_one_abs: i32,
}

impl State {
    const fn new(p0: i32, p1: i32, p2: i32, a0: i32, a1: i32, a2: i32) -> Self {
        return Self {
            no_late_no_abs: p0,
            one_late_no_abs: p1,
            two_late_no_abs: p2,
            no_late_one_abs: a0,
            one_late_one_abs: a1,
            two_late_one_abs: a2,
        };
    }

    fn no_abs(&self) -> i32 {
        return mod_add3(
            self.no_late_no_abs,
            self.one_late_no_abs,
            self.two_late_no_abs,
        );
    }

    fn one_abs(&self) -> i32 {
        return mod_add3(
            self.no_late_one_abs,
            self.one_late_one_abs,
            self.two_late_one_abs,
        );
    }

    fn sum(&self) -> i32 {
        return mod_add2(self.no_abs(), self.one_abs());
    }
}

const MOD: i32 = 1_000_000_007;

fn mod_add2(a: i32, b: i32) -> i32 {
    return (a + b) % MOD; // assume a & b are already in MOD ring
}

fn mod_add3(a: i32, b: i32, c: i32) -> i32 {
    return mod_add2(mod_add2(a, b), c);
}
