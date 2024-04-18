use std::collections::VecDeque;

pub fn predict_party_victory(senate: String) -> String {
    let n = senate.len() as i32;
    let mut dq: VecDeque<i32> = VecDeque::new();
    let mut rq: VecDeque<i32> = VecDeque::new();

    for (c, i) in senate.chars().zip(0..) {
        if c == 'D' {
            dq.push_back(i);
        } else {
            rq.push_back(i);
        }
    }

    while !dq.is_empty() && !rq.is_empty() {
        let d = dq.pop_front().unwrap();
        let r = rq.pop_front().unwrap();

        if d < r {
            dq.push_back(d + n);
        } else {
            rq.push_back(r + n);
        }
    }

    return if rq.is_empty() {
        String::from("Dire")
    } else {
        String::from("Radiant")
    };
}
