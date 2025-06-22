//! This module defines the values of a variable.
use ndarray::{Array1, ArrayView1};

#[derive(Debug, Clone)]
pub struct Values(Array1<f64>);

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum ValuesError {
    #[error("The values cannot be empty")]
    Empty,
    #[error("The values cannot be NaN or Infinity")]
    NoFiniteValues,
}

impl Values {
    pub fn values(&self) -> ArrayView1<f64> {
        self.0.view()
    }
}

impl TryFrom<Array1<f64>> for Values {
    type Error = ValuesError;

    fn try_from(values: Array1<f64>) -> Result<Self, Self::Error> {
        if values.is_empty() {
            return Err(ValuesError::Empty);
        }
        let any_not_finite = values.iter().any(|&x| !x.is_finite());
        if any_not_finite {
            return Err(ValuesError::NoFiniteValues);
        }
        Ok(Values(values))
    }
}

impl TryFrom<&Array1<f64>> for Values {
    type Error = ValuesError;

    fn try_from(values: &Array1<f64>) -> Result<Self, Self::Error> {
        Values::try_from(values.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_array_size_1() {
        let expected = array![0.];
        let v = Values::try_from(&expected).unwrap();
        assert_eq!(v.values(), expected);
    }

    #[test]
    fn test_try_from_array_size_2() {
        let expected = array![0., 1.];
        let v = Values::try_from(&expected).unwrap();
        assert_eq!(v.values(), expected);
    }

    #[test]
    fn test_try_from_array_size_0_failure() {
        let err = Values::try_from(array![]).unwrap_err();
        assert_eq!(err, ValuesError::Empty);
    }

    #[test]
    fn test_try_from_array_with_nan_failure() {
        let err = Values::try_from(array![0., f64::NAN, 1.]).unwrap_err();
        assert_eq!(err, ValuesError::NoFiniteValues);
    }

    #[test]
    fn test_try_from_array_with_infinity_pos_failure() {
        let err = Values::try_from(array![0., f64::INFINITY, 1.]).unwrap_err();
        assert_eq!(err, ValuesError::NoFiniteValues);
    }

    #[test]
    fn test_try_from_array_with_infinity_neg_failure() {
        let err = Values::try_from(array![0., f64::NEG_INFINITY, 1.]).unwrap_err();
        assert_eq!(err, ValuesError::NoFiniteValues);
    }
}
