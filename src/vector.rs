//! This module implements the low level computations for vectorized variables.
use ndarray::{Array1, ArrayView1, ArrayView2, Zip};
use ndarray_stats::QuantileExt;

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

pub fn index_of_best_vector(m: ArrayView2<f64>) -> usize {
    // compute l2 norm for each vector (row) and find the best (min) vector
    l2_norm_vectors(m).argmin().unwrap()
}

#[cfg(test)]
mod tests {
    use ndarray::array;

    use super::*;

    const SQRT2: f64 = std::f64::consts::SQRT_2;

    #[test]
    fn l2_norm_single_value_zero() {
        let v = array![0.];
        assert_eq!(l2_norm(v.view()), 0.);
    }

    #[test]
    fn l2_norm_single_value_exact() {
        let v = array![2.];
        assert_eq!(l2_norm(v.view()), 2.);
    }

    #[test]
    fn l2_norm_vector2_ones() {
        let v = array![1., 1.];
        assert_eq!(l2_norm(v.view()), SQRT2);
    }

    #[test]
    fn l2_norm_vector_zeros() {
        let v = Array1::<f64>::zeros(5);
        assert_eq!(l2_norm(v.view()), 0.);
    }

    #[test]
    fn l2_norm_vector_ones() {
        let v = Array1::<f64>::ones(5);
        assert_eq!(l2_norm(v.view()), (5.0_f64).sqrt());
    }

    #[test]
    fn best_vector_from_matrix_success_1() {
        let m = array![[1., 1.], [0., 0.], [1., 0.], [0., 1.],];
        assert_eq!(l2_norm_vectors(m.view()), array![SQRT2, 0., 1., 1.]);
        assert_eq!(index_of_best_vector(m.view()), 1);
    }

    #[test]
    fn best_vector_from_matrix_success_2() {
        let m = array![[1., 1.], [2., 0.], [1., 0.], [0., 2.],];
        assert_eq!(l2_norm_vectors(m.view()), array![SQRT2, 2., 1., 2.]);
        assert_eq!(index_of_best_vector(m.view()), 2);
    }

    #[test]
    fn best_vector_from_matrix_success_3() {
        let sqrt3: f64 = (3.0_f64).sqrt();
        let m = array![[1., 1., 1.], [2., 0., 0.], [1., 0., 1.], [1., 0., 1.],];
        assert_eq!(l2_norm_vectors(m.view()), array![sqrt3, 2., SQRT2, SQRT2,]);
        assert_eq!(index_of_best_vector(m.view()), 2);
    }
}
