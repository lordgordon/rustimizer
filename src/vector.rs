//! This module implements the low level computations for vectorized variables.
use ndarray::{Array1, ArrayView1, ArrayView2, Zip};
use ndarray_stats::QuantileExt;

// TODO: this should be modularized as a trait

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
    // compute l2 norm for each vector (row)
    let norms = l2_norm_vectors(m);

    // find the best (min) vector
    norms.argmin().unwrap()
}

fn rescale_value(x: f64, min_val: f64, max_val: f64) -> f64 {
    // TODO: address division by zero, guarantee max > min
    (x - min_val) / (max_val - min_val)
}

// fn rescale_vector(v: ArrayView1<f64>) -> ArrayView1 {
//     let min_val = v.max().unwrap();
//     let max_val = v.min().unwrap();

//     v.for_each(f);
// }

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
    fn rescale_value_min_pos_already_scaled() {
        assert_eq!(rescale_value(0., 0., 1.0), 0.);
    }

    #[test]
    fn rescale_value_max_pos_already_scaled() {
        assert_eq!(rescale_value(1., 0., 1.0), 1.);
    }

    #[test]
    fn rescale_value_half_pos_already_scaled() {
        assert_eq!(rescale_value(0.5, 0., 1.0), 0.5);
    }

    #[test]
    fn rescale_value_min_pos_with_scaling() {
        assert_eq!(rescale_value(1., 1., 10.0), 0.);
    }

    #[test]
    fn rescale_value_max_pos_with_scaling() {
        assert_eq!(rescale_value(10., 1., 10.0), 1.);
    }

    #[test]
    fn rescale_value_half_pos_with_scaling() {
        assert_eq!(rescale_value(5.5, 1., 10.0), 0.5);
    }

    #[test]
    fn rescale_value_min_neg_left() {
        assert_eq!(rescale_value(-1., -1., 1.0), 0.);
    }

    #[test]
    fn rescale_value_max_neg_left() {
        assert_eq!(rescale_value(1., -1., 1.0), 1.);
    }

    #[test]
    fn rescale_value_half_neg_left() {
        assert_eq!(rescale_value(0., -1., 1.0), 0.5);
    }

    #[test]
    fn rescale_value_min_neg_full() {
        assert_eq!(rescale_value(-3., -3., -1.0), 0.);
    }

    #[test]
    fn rescale_value_max_neg_full() {
        assert_eq!(rescale_value(-1., -3., -1.0), 1.);
    }

    #[test]
    fn rescale_value_half_neg_full() {
        assert_eq!(rescale_value(-2., -3., -1.0), 0.5);
    }

    #[test]
    fn rescale_value_out_of_range_is_valid_left() {
        assert_eq!(rescale_value(-1.0, 0., 1.0), -1.0);
    }

    #[test]
    fn rescale_value_out_of_range_is_valid_right() {
        assert_eq!(rescale_value(2.0, 0., 1.0), 2.0);
    }
}
