//! This module implements the low level computations for vectorized variables.
use ndarray::{Array1, ArrayView1, ArrayView2, Zip};
use ndarray_stats::QuantileExt;

// TODO: this should be modularized as a trait
// TODO: documentation

fn l2_norm(v: ArrayView1<f64>) -> f64 {
    v.dot(&v).sqrt()
}

fn l2_norm_vectors(m: ArrayView2<f64>) -> Array1<f64> {
    // compute l2 norm for each vector (row)
    let mut norms = Array1::zeros(m.nrows());

    Zip::from(&mut norms)
        .and(m.rows())
        .for_each(|norms, row| *norms = l2_norm(row.view()));
    norms
}

fn index_of_best_vector(m: ArrayView2<f64>) -> usize {
    // compute l2 norm for each vector (row) and find the best (min) vector
    l2_norm_vectors(m).argmin().unwrap()
}

fn rescale_vector(v: ArrayView1<f64>, shift: f64, scaling_factor: f64) -> Array1<f64> {
    let shift_vector = Array1::from_elem(v.len(), shift);
    (&v - shift_vector) * scaling_factor
}

fn rescale_and_invert_vector(v: ArrayView1<f64>, shift: f64, scaling_factor: f64) -> Array1<f64> {
    // This is scaling and then a 180° rotation to ensure the min is now the max, and vice-versa.
    let ones = Array1::ones(v.len());
    (&rescale_vector(v, shift, scaling_factor) - ones) * -1.0
}

// TODO: function to automatically compute the min and max
// TODO: address division by zero, guarantee max > min
//      let min_val = v.max().unwrap();
//      let max_val = v.min().unwrap();
//      shift = min_val
//      scaling_factor = max_value - min_value
// TODO: install openblas?

#[cfg(test)]
mod tests {
    // TODO: https://crates.io/crates/proptest

    use ndarray::array;

    use super::*;

    const SQRT2: f64 = std::f64::consts::SQRT_2;

    #[test]
    fn test_l2_norm_single_value_zero() {
        let v = array![0.];
        assert_eq!(l2_norm(v.view()), 0.);
    }

    #[test]
    fn test_l2_norm_single_value_exact() {
        let v = array![2.];
        assert_eq!(l2_norm(v.view()), 2.);
    }

    #[test]
    fn test_l2_norm_vector2_ones() {
        let v = array![1., 1.];
        assert_eq!(l2_norm(v.view()), SQRT2);
    }

    #[test]
    fn test_l2_norm_vector_zeros() {
        let v = Array1::<f64>::zeros(5);
        assert_eq!(l2_norm(v.view()), 0.);
    }

    #[test]
    fn test_l2_norm_vector_ones() {
        let v = Array1::<f64>::ones(5);
        assert_eq!(l2_norm(v.view()), (5.0_f64).sqrt());
    }

    #[test]
    fn test_best_vector_from_matrix_success_1() {
        let m = array![[1., 1.], [0., 0.], [1., 0.], [0., 1.],];
        assert_eq!(l2_norm_vectors(m.view()), array![SQRT2, 0., 1., 1.]);
        assert_eq!(index_of_best_vector(m.view()), 1);
    }

    #[test]
    fn test_best_vector_from_matrix_success_2() {
        let m = array![[1., 1.], [2., 0.], [1., 0.], [0., 2.],];
        assert_eq!(l2_norm_vectors(m.view()), array![SQRT2, 2., 1., 2.]);
        assert_eq!(index_of_best_vector(m.view()), 2);
    }

    #[test]
    fn test_best_vector_from_matrix_success_3() {
        let sqrt3: f64 = (3.0_f64).sqrt();
        let m = array![[1., 1., 1.], [2., 0., 0.], [1., 0., 1.], [1., 0., 1.],];
        assert_eq!(l2_norm_vectors(m.view()), array![sqrt3, 2., SQRT2, SQRT2,]);
        assert_eq!(index_of_best_vector(m.view()), 2);
    }

    #[test]
    fn test_rescale_vector_pos_already_scaled() {
        let v = array![0., 0.5, 1.];
        assert_eq!(rescale_vector(v.view(), 0., 1.), array![0., 0.5, 1.]);
    }

    #[test]
    fn test_rescale_vector_pos_scaling() {
        let v = array![1., 6., 11.];
        let shift = v.min().unwrap();
        let scaling_factor = 1.0 / (v.max().unwrap() - shift);
        assert_eq!(
            rescale_vector(v.view(), *shift, scaling_factor),
            array![0., 0.5, 1.]
        );
    }

    #[test]
    fn test_rescale_vector_neg_scaling() {
        let v = array![-3., -2., -1.];
        let shift = v.min().unwrap();
        let scaling_factor = 1.0 / (v.max().unwrap() - shift);
        assert_eq!(
            rescale_vector(v.view(), *shift, scaling_factor),
            array![0., 0.5, 1.]
        );
    }

    #[test]
    fn test_rescale_vector_scaling() {
        let v = array![0., 1., 6., 11., 12.];
        assert_eq!(
            rescale_vector(v.view(), 1., 0.1),
            array![-0.1, 0., 0.5, 1., 1.1]
        );
    }

    #[test]
    fn test_rescale_and_invert_vector() {
        let v = array![0., 1., 6., 11., 12.];
        assert_eq!(
            rescale_and_invert_vector(v.view(), 1., 0.1),
            array![1.1, 1., 0.5, 0., -0.10000000000000009]
        );
    }
}
