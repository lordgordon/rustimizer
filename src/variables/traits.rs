//! This module defines the traites for the vectorized variables.
use ndarray::{Array1, ArrayView1};

pub trait VariableProperties {
    fn name(&self) -> &str;

    fn values(&self) -> ArrayView1<f64>;

    fn length(&self) -> usize {
        self.values().len()
    }

    fn rescale(&self) -> Array1<f64>;
}
