//! This module implements the concept of vectorized variables.
use ndarray::{array, Array1, ArrayView1};

fn l2_norm(x: ArrayView1<f64>) -> f64 {
    x.dot(&x).sqrt()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn experiment_with_ndarray() {
        let x1 = array![0.];
        assert_eq!(l2_norm(x1.view()), 0.);

        let x2 = array![2.];
        assert_eq!(l2_norm(x2.view()), 2.);

        let x3 = array![1., 1.];
        assert_eq!(l2_norm(x3.view()), 1.4142135623730951);
    }
}
