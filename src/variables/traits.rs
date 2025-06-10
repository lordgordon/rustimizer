//! This module defines the traites for the vectorized variables.
use ndarray::{Array1, ArrayView1};

pub trait HasValues {
    fn values(&self) -> ArrayView1<f64>;
}

pub trait HasLength: HasValues {
    fn length(&self) -> usize {
        self.values().len()
    }
}

pub trait Rescalable: HasValues {
    fn rescale(&self) -> Array1<f64>;
}
