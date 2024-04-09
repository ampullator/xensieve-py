use ::xensieve::Sieve as SieveRS;
use pyo3::prelude::*;

#[pyclass(unsendable)] // decline making this sharable between threads
struct IterValue {
    // a boxed iterator of integers; it wraps a Sieve IterValueRS
    iter: Box<dyn Iterator<Item = i128>>,
}

#[pymethods]
impl IterValue {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<i128> {
        slf.iter.next()
    }
}

#[pyclass(unsendable)]
struct IterInterval {
    iter: Box<dyn Iterator<Item = i128>>,
}

#[pymethods]
impl IterInterval {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<i128> {
        slf.iter.next()
    }
}

#[pyclass(unsendable)]
struct IterState {
    iter: Box<dyn Iterator<Item = bool>>,
}

#[pymethods]
impl IterState {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<bool> {
        slf.iter.next()
    }
}

/// The representation of a Xenakis Sieve, constructed from a string notation of one or more Residual classes combined with logical operators. This implementation, backed by a Rust implementation, follows the Python implementation in Ariza (2005), with significant performance and interface enhancements: https://direct.mit.edu/comj/article/29/2/40/93957
#[pyclass(frozen)]
struct Sieve {
    pub(crate) s: SieveRS,
}

#[pymethods]
impl Sieve {
    #[new]
    fn new(expr: String) -> Self {
        Self {
            s: SieveRS::new(&expr),
        }
    }

    fn __repr__(&self) -> String {
        self.s.to_string()
    }

    /// Return true if the provided integer is included within the Sieve.
    fn __contains__(&self, v: i64) -> bool {
        self.s.contains(v as i128)
    }

    //--------------------------------------------------------------------------
    /// Return the inverse of this Sieve.
    fn __invert__(&self) -> Self {
        let new: SieveRS = !self.s.clone();
        Self { s: new }
    }

    /// Return the XOR combination (or symmetric difference) of the provided Sieve with this Sieve.
    fn __xor__(&self, other: &Self) -> Self {
        let new: SieveRS = self.s.clone() ^ other.s.clone();
        Self { s: new }
    }

    /// Return the OR combination (or union) of the provided Sieve with this Sieve.
    fn __or__(&self, other: &Self) -> Self {
        let new: SieveRS = self.s.clone() | other.s.clone();
        Self { s: new }
    }

    /// Return the AND combination (or intersection) of the provided Sieve with this Sieve.
    fn __and__(&self, other: &Self) -> Self {
        let new: SieveRS = self.s.clone() & other.s.clone();
        Self { s: new }
    }

    //--------------------------------------------------------------------------
    /// Return an iterator of the integers defined within this Sieve, given an inclusive start integer and in exclusive stop integer.
    fn iter_value(&self, start: i64, stop: i64) -> IterValue {
        let iter = self.s.iter_value(start as i128..stop as i128);
        IterValue {
            iter: Box::new(iter),
        }
    }

    /// Return an iterator of the interval width between defined integers within this Sieve, given an inclusive start integer and in exclusive stop integer.
    fn iter_interval(&self, start: i64, stop: i64) -> IterInterval {
        let iter = self.s.iter_interval(start as i128..stop as i128);
        IterInterval {
            iter: Box::new(iter),
        }
    }

    /// Return an iterator of the Boolean states of this Sieve, given an inclusive start integer and in exclusive stop integer.
    fn iter_state(&self, start: i64, stop: i64) -> IterState {
        let iter = self.s.iter_state(start as i128..stop as i128);
        IterState {
            iter: Box::new(iter),
        }
    }
}

#[pymodule]
fn xensieve(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Sieve>()?;
    Ok(())
}

//------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sieve_new_a() {
        let s = Sieve::new("3@2|7@4".to_string());
        assert_eq!(s.__repr__(), "Sieve{3@2|7@4}");
    }

    #[test]
    fn test_sieve_contains_a() {
        let s = Sieve::new("3@2|7@4".to_string());
        assert_eq!(s.__contains__(2), true);
        assert_eq!(s.__contains__(3), false);
    }

    #[test]
    fn test_sieve_invert_a() {
        let s1 = Sieve::new("3@2".to_string());
        let s2 = s1.__invert__();
        assert_eq!(s2.__repr__(), "Sieve{!(3@2)}");
    }

    #[test]
    fn test_sieve_xor_a() {
        let s1 = Sieve::new("3@2".to_string());
        let s2 = Sieve::new("5@1".to_string());

        let s3 = s1.__xor__(&s2);
        assert_eq!(s3.__repr__(), "Sieve{3@2^5@1}");
    }

    #[test]
    fn test_sieve_or_a() {
        let s1 = Sieve::new("3@2".to_string());
        let s2 = Sieve::new("5@1".to_string());

        let s3 = s1.__or__(&s2);
        assert_eq!(s3.__repr__(), "Sieve{3@2|5@1}");
    }

    #[test]
    fn test_sieve_and_a() {
        let s1 = Sieve::new("3@2".to_string());
        let s2 = Sieve::new("5@1".to_string());

        let s3 = s1.__and__(&s2);
        assert_eq!(s3.__repr__(), "Sieve{3@2&5@1}");
    }

    //--------------------------------------------------------------------------

    #[test]
    fn test_iter_value_a() {
        let s1 = Sieve::new("7@2|9@1".to_string());
        let mut it1 = s1.iter_value(0, 40);
        assert_eq!(it1.iter.next(), Some(1));
        assert_eq!(it1.iter.next(), Some(2));
    }

    #[test]
    fn test_iter_interval_a() {
        let s1 = Sieve::new("7@2|9@1".to_string());
        let mut it1 = s1.iter_interval(0, 40);
        assert_eq!(it1.iter.next(), Some(1));
        assert_eq!(it1.iter.next(), Some(7));
        assert_eq!(it1.iter.next(), Some(1));
        assert_eq!(it1.iter.next(), Some(6));
    }

    #[test]
    fn test_iter_state_a() {
        let s1 = Sieve::new("7@2|9@1".to_string());
        let mut it1 = s1.iter_state(1, 40);
        assert_eq!(it1.iter.next(), Some(true));
        assert_eq!(it1.iter.next(), Some(true));
        assert_eq!(it1.iter.next(), Some(false));
    }
}
