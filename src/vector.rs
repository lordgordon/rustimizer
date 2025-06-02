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

// fn rescale_vector(v: ArrayView1<f64>) -> ArrayView1 {
//     let min_val = v.max().unwrap();
//     let max_val = v.min().unwrap();
// }

#[cfg(test)]
mod tests {
    use ndarray::array;

    use super::*;

    const SQRT2: f64 = std::f64::consts::SQRT_2;

    #[test]
    fn experiment_with_ndarray_single_norm() {
        let x1 = array![0.];
        assert_eq!(l2_norm(x1.view()), 0.);

        let x2 = array![2.];
        assert_eq!(l2_norm(x2.view()), 2.);

        let x3 = array![1., 1.];
        assert_eq!(l2_norm(x3.view()), SQRT2);
    }

    #[test]
    fn experiment_with_ndarray_matrix() {
        let sqrt3: f64 = (3.0_f64).sqrt();

        let m1 = array![[1., 1.], [0., 0.], [1., 0.], [0., 1.],];
        assert_eq!(l2_norm_vectors(m1.view()), array![SQRT2, 0., 1., 1.]);
        assert_eq!(index_of_best_vector(m1.view()), 1);

        let m1 = array![[1., 1.], [2., 0.], [1., 0.], [0., 2.],];
        assert_eq!(l2_norm_vectors(m1.view()), array![SQRT2, 2., 1., 2.]);
        assert_eq!(index_of_best_vector(m1.view()), 2);

        let m1 = array![[1., 1., 1.], [2., 0., 0.], [1., 0., 1.], [1., 0., 1.],];
        assert_eq!(l2_norm_vectors(m1.view()), array![sqrt3, 2., SQRT2, SQRT2,]);
        assert_eq!(index_of_best_vector(m1.view()), 2);
    }
}
