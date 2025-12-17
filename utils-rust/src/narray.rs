#![allow(clippy::inline_always)]
use std::ops::{Deref, DerefMut, Index, IndexMut};

/// Flattened N-Dimensional array
#[derive(Debug, PartialEq, Eq)]
pub struct NArray<T, const N: usize> {
    data: Box<[T]>,
    dimensions: [usize; N],
}

impl<T, const N: usize> NArray<T, N> {
    /// Create a new array
    ///
    /// # Parameters
    ///
    /// - `element`: initial value of the array
    /// - `dimensions`: size of each dimension, from outer-most to inner-most
    #[must_use]
    pub fn new(element: T, dimensions: [usize; N]) -> Self
    where
        T: Clone,
    {
        let count = dimensions.iter().product();
        Self {
            data: std::iter::repeat_n(element, count).collect(),
            dimensions,
        }
    }

    /// Get the dimension sizes array, ordered from outer-most dimension to inner-most
    #[must_use]
    pub const fn dimensions(&self) -> &[usize; N] {
        &self.dimensions
    }

    /// Get the index in the flattened array corresponding to a given coordinate
    #[must_use]
    #[inline(always)]
    pub fn index(&self, index: [usize; N]) -> usize {
        for (i, (pos, d)) in std::iter::zip(index, self.dimensions).enumerate() {
            debug_assert!(
                pos < d,
                "index out of bounds: the len of the dimension {i} is {d} but the index is {pos}"
            );
        }
        std::iter::zip(&index[1..], &self.dimensions[1..]).fold(index[0], |acc, (i, d)| acc * d + i)
    }
}

impl<T, const N: usize> Index<[usize; N]> for NArray<T, N> {
    type Output = T;

    #[inline(always)]
    fn index(&self, index: [usize; N]) -> &Self::Output {
        &self.data[self.index(index)]
    }
}

impl<T, const N: usize> IndexMut<[usize; N]> for NArray<T, N> {
    #[inline(always)]
    fn index_mut(&mut self, index: [usize; N]) -> &mut Self::Output {
        &mut self.data[self.index(index)]
    }
}

impl<T, const N: usize> Deref for NArray<T, N> {
    type Target = [T];
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T, const N: usize> DerefMut for NArray<T, N> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
