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

// Create an iterator through the positions of 1s in the number
pub fn iter_ones(mut value: usize) -> impl Iterator<Item = usize> {
    std::iter::from_fn(move || {
        if value == 0 {
            return None;
        }
        // Get the first 1, remove it from the current value, and yield it
        let i = value.trailing_zeros();
        value ^= 1 << i;
        Some(i as usize)
    })
}
