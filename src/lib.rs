use ::xensieve::Sieve as SieveRS;
use pyo3::prelude::*;

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

    fn __contains__(&self, v: i64) -> bool {
        self.s.contains(v as i128)
    }

    //--------------------------------------------------------------------------
    fn __invert__(&self) -> Self {
        let new: SieveRS = !self.s.clone();
        Self { s: new }
    }

    fn __xor__(&self, other: &Self) -> Self {
        let new: SieveRS = self.s.clone() ^ other.s.clone();
        Self { s: new }
    }

    fn __or__(&self, other: &Self) -> Self {
        let new: SieveRS = self.s.clone() | other.s.clone();
        Self { s: new }
    }

    fn __and__(&self, other: &Self) -> Self {
        let new: SieveRS = self.s.clone() & other.s.clone();
        Self { s: new }
    }

    //--------------------------------------------------------------------------
}

/// A Python module implemented in Rust.
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
}
