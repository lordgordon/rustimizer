//! This module implements the low level computations for vectorized variables.
use ndarray::{Array1, ArrayView1};
use ndarray_stats::QuantileExt;

pub fn rescale_vector(v: ArrayView1<f64>, shift: f64, scaling_factor: f64) -> Array1<f64> {
    // TODO: we don't check for empty vectors because the caller guarantees for it
    let shift_vector = Array1::from_elem(v.len(), shift);
    (&v - shift_vector) * scaling_factor
}

pub fn rescale_and_invert_vector(
    v: ArrayView1<f64>,
    shift: f64,
    scaling_factor: f64,
) -> Array1<f64> {
    // This is scaling and then a 180° rotation to ensure the min is now the max, and vice-versa.
    // TODO: we don't check for empty vectors because the caller guarantees for it
    let ones = Array1::ones(v.len());
    (&rescale_vector(v, shift, scaling_factor) - ones) * -1.0
}

pub fn autorescale_vector(v: ArrayView1<f64>, inverted: bool) -> Array1<f64> {
    // TODO: handle the  case where min == max (single valued array, or multi valued with the same value)
    // TODO: we don't check for empty vectors because the caller guarantees for it
    let shift = v.min().unwrap();
    let scaling_factor = 1.0 / (v.max().unwrap() - shift);
    if inverted {
        rescale_and_invert_vector(v.view(), *shift, scaling_factor)
    } else {
        rescale_vector(v.view(), *shift, scaling_factor)
    }
}

#[cfg(test)]
mod tests {
    use ndarray::array;

    use super::*;

    #[test]
    fn rescale_vector_pos_already_scaled() {
        let v = array![0., 0.5, 1.];
        assert_eq!(rescale_vector(v.view(), 0., 1.), v);
        assert_eq!(autorescale_vector(v.view(), false), v);
    }

    #[test]
    fn rescale_vector_pos_scaling() {
        let v = array![1., 6., 11.];
        let expected_scaled = array![0., 0.5, 1.];
        assert_eq!(rescale_vector(v.view(), 1.0, 0.1), expected_scaled);
        assert_eq!(autorescale_vector(v.view(), false), expected_scaled);
    }

    #[test]
    fn rescale_vector_neg_scaling() {
        let v = array![-3., -2., -1.];
        let expected_scaled = array![0., 0.5, 1.];
        assert_eq!(rescale_vector(v.view(), -3.0, 0.5), expected_scaled);
        assert_eq!(autorescale_vector(v.view(), false), expected_scaled);
    }

    #[test]
    fn rescale_vector_scaling() {
        let v = array![0., 1., 6., 11., 12.];
        assert_eq!(
            rescale_vector(v.view(), 1., 0.1),
            array![-0.1, 0., 0.5, 1., 1.1]
        );
        assert_eq!(
            autorescale_vector(v.view(), false),
            array![0., 0.08333333333333333, 0.5, 0.9166666666666666, 1.]
        );
    }

    #[test]
    fn rescale_and_invert_vector_full() {
        let v = array![0., 1., 6., 11., 12.];
        assert_eq!(
            rescale_and_invert_vector(v.view(), 1., 0.1),
            array![1.1, 1., 0.5, 0., -0.10000000000000009]
        );
        assert_eq!(
            autorescale_vector(v.view(), true),
            array![1., 0.9166666666666666, 0.5, 0.08333333333333337, 0.]
        );
    }
}
