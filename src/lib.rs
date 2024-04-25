use std::ops::{Deref, DerefMut, Index, IndexMut};

/// A macro for creating a `PeriodicArray` from a list of elements.
///
/// # Examples
///
/// ```
/// use periodic_array::p_arr;
///
/// let pa = p_arr![1, 2, 3];
/// ```
#[macro_export]
macro_rules! p_arr {
    ($($x:expr),* $(,)?) => {{
        $crate::PeriodicArray::new([$($x),*])
    }};
}

/// A struct representing a fixed-size array that provides periodic access to its elements.
///
/// Elements in the array are accessed such that indexing beyond the array's bounds
/// will wrap around to the beginning, effectively treating the array as infinite/periodic.
/// Internally, bounds checks are skipped via the use of `get_unchecked` and `get_unchecked_mut`.
///
/// Copy is optionally derived when the `"copy"` feature is enabled. This separation is done for
/// those of us that want full control on when copies are performed.
///
/// # Type Parameters
///
/// * `T` - The type of elements held in the array.
/// * `N` - The compile-time fixed size of the array.
///
/// # Examples
///
/// ```
/// use periodic_array::p_arr;
///
/// let pa = p_arr![1, 2, 3];
/// assert_eq!(pa[1], 2);
/// assert_eq!(pa[4], 2); // Access beyond the length wraps around
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "copy", derive(Copy))]
#[repr(C)]
pub struct PeriodicArray<T: Clone + Copy, const N: usize> {
    /// The inner array.
    ///
    /// Note: This is public so that the `p_arr!` macro can work by explicitly
    /// declaring the array
    pub(crate) inner: [T; N],
}

impl<T: Clone + Copy, const N: usize> PeriodicArray<T, N> {
    #[inline(always)]
    pub fn new(inner: [T; N]) -> Self {
        PeriodicArray { inner }
    }
}

impl<T: Clone + Copy, const N: usize> Index<usize> for PeriodicArray<T, N> {
    type Output = T;
    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { self.inner.get_unchecked(index % N) }
    }
}

impl<T: Clone + Copy, const N: usize> IndexMut<usize> for PeriodicArray<T, N> {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { self.inner.get_unchecked_mut(index % N) }
    }
}

impl<T: Clone + Copy, const N: usize> Deref for PeriodicArray<T, N> {
    type Target = [T; N];
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T: Clone + Copy, const N: usize> DerefMut for PeriodicArray<T, N> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T: Clone + Copy, const N: usize> From<[T; N]> for PeriodicArray<T, N> {
    #[inline(always)]
    fn from(inner: [T; N]) -> Self {
        PeriodicArray { inner }
    }
}

#[cfg(test)]
mod tests {
    use crate::{p_arr, PeriodicArray};

    #[test]
    pub fn declare_with_macro() {
        let pa_macro = p_arr![1, 2, 3];

        let pa = PeriodicArray { inner: [1, 2, 3] };

        assert_eq!(pa, pa_macro);
    }

    #[test]
    pub fn index_into() {
        let pa = p_arr![1, 2, 3];

        // in domain
        assert_eq!(pa[0], 1);
        assert_eq!(pa[1], 2);
        assert_eq!(pa[2], 3);

        // periodic
        assert_eq!(pa[3], 1);
        assert_eq!(pa[4], 2);
        assert_eq!(pa[5], 3);
    }

    #[test]
    pub fn use_array_methods() {
        let mut pa = p_arr![1, 2, 3];

        // .map() method of array
        let _x2 = pa.map(|x| x * x);

        // .iter() and .iter_mut() method of array
        for x in pa.iter() {
            let _ = x * x;
        }
        for p in pa.iter_mut() {
            *p = *p * *p;
        }
    }
}
