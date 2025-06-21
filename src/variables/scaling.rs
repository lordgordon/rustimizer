//! This module implements the low level computations for vectorized variables.
use super::values::Values;
use ndarray::{Array1, ArrayView1};
use ndarray_stats::QuantileExt;
use std::convert::TryFrom;

pub fn rescale_vector(v: ArrayView1<f64>, shift: f64, scaling_factor: f64) -> Array1<f64> {
    // We don't check for empty vectors because the caller guarantees for it
    let shift_vector = Array1::from_elem(v.len(), shift);
    (&v - shift_vector) * scaling_factor
}

pub fn rescale_and_invert_vector(
    v: ArrayView1<f64>,
    shift: f64,
    scaling_factor: f64,
) -> Array1<f64> {
    // This is scaling and then a 180° rotation to ensure the min is now the max, and vice-versa.
    // We don't check for empty vectors because the caller guarantees for it
    let ones = Array1::ones(v.len());
    (&rescale_vector(v, shift, scaling_factor) - ones) * -1.0
}

pub fn autorescale_vector(v: &Values, inverted: bool) -> Values {
    // TODO: handle the  case where min == max (single valued array, or multi valued with the same value)
    let values = v.values();
    let shift = values.min().unwrap();
    let scaling_factor = 1.0 / (values.max().unwrap() - shift);
    if inverted {
        Values::try_from(rescale_and_invert_vector(values, *shift, scaling_factor)).unwrap()
    } else {
        Values::try_from(rescale_vector(values, *shift, scaling_factor)).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_ulps_eq;
    use ndarray::array;
    use std::convert::TryFrom;

    #[test]
    fn rescale_vector_pos_already_scaled() {
        let v = Values::try_from(array![0., 0.5, 1.]).unwrap();
        assert_ulps_eq!(rescale_vector(v.values(), 0., 1.), v.values());
        assert_ulps_eq!(autorescale_vector(&v, false).values(), v.values());
    }

    #[test]
    fn rescale_vector_pos_scaling() {
        let v = Values::try_from(array![1., 6., 11.]).unwrap();
        let expected_scaled = array![0., 0.5, 1.];
        assert_ulps_eq!(rescale_vector(v.values(), 1.0, 0.1), expected_scaled);
        assert_ulps_eq!(autorescale_vector(&v, false).values(), expected_scaled);
    }

    #[test]
    fn rescale_vector_neg_scaling() {
        let v = Values::try_from(array![-3., -2., -1.]).unwrap();
        let expected_scaled = array![0., 0.5, 1.];
        assert_ulps_eq!(rescale_vector(v.values(), -3.0, 0.5), expected_scaled);
        assert_ulps_eq!(autorescale_vector(&v, false).values(), expected_scaled);
    }

    #[test]
    fn rescale_vector_scaling() {
        let v = Values::try_from(array![0., 1., 6., 11., 12.]).unwrap();
        assert_ulps_eq!(
            rescale_vector(v.values(), 1., 0.1),
            array![-0.1, 0., 0.5, 1., 1.1]
        );
        assert_ulps_eq!(
            autorescale_vector(&v, false).values(),
            array![0., 0.08333333333333333, 0.5, 0.9166666666666666, 1.]
        );
    }

    #[test]
    fn rescale_and_invert_vector_full() {
        let v = Values::try_from(array![0., 1., 6., 11., 12.]).unwrap();
        assert_ulps_eq!(
            rescale_and_invert_vector(v.values(), 1., 0.1),
            array![1.1, 1., 0.5, 0., -0.1]
        );
        assert_ulps_eq!(
            autorescale_vector(&v, true).values(),
            array![1., 0.9166666666666666, 0.5, 0.08333333333333337, 0.]
        );
    }
}
