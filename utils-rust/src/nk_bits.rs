/// Compute the lexicographically next bit permutation with `K` set to 1
///
/// NOTE: adapted from <https://graphics.stanford.edu/~seander/bithacks.html#NextBitPermutation>
#[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
const fn bit_twiddle_permute(v: usize) -> usize {
    let t: isize = (v | (v - 1)) as isize; // t gets v's least significant 0 bits set to 1
    // Next set to 1 the most significant bit to change,
    // set to 0 the least significant ones, and add the necessary 1 bits.
    ((t + 1) | (((!t & -!t) - 1) >> (v.trailing_zeros() + 1))) as usize
}

/// Iterator over all `N` bit sequences with exactly `K` bits set to 1
#[derive(Debug, PartialEq, Eq)]
pub struct NKBits {
    state: usize,
    max: usize,
}

impl Iterator for NKBits {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        let res = self.state;
        self.state = bit_twiddle_permute(self.state);
        (res < self.max).then_some(res)
    }
}

impl NKBits {
    /// Creates a new iterator
    ///
    /// # Parameters
    ///
    /// * `n`: Size of the number
    /// * `k`: Amount of set bits
    #[must_use]
    pub const fn new(n: usize, k: usize) -> Self {
        Self {
            state: (1 << k) - 1,
            max: (1 << n),
        }
    }
}
