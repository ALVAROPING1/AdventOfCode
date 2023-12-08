#[must_use]
pub fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

#[must_use]
pub fn gcd(first: usize, second: usize) -> usize {
    let mut max = first.max(second);
    let mut min = first.min(second);

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}
