pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
    let mut stack: Vec<i32> = vec![];
    for a in asteroids.into_iter() {
        let mut new_alive = true;
        loop {
            let Some(top_ref) = stack.last() else { break };
            let top = top_ref.clone();

            if !(top.is_positive() && a.is_negative()) {
                break;
            } else if i32::abs(a) == i32::abs(top) {
                _ = stack.pop();
                new_alive = false;
                break;
            } else if i32::abs(a) > i32::abs(top) {
                _ = stack.pop();
            } else {
                new_alive = false;
                break;
            }
        }

        if new_alive {
            stack.push(a)
        }
    }
    return stack;
}
