//! This module implements the definition of a vectorized variables.
use crate::vector::rescale_vector;
use ndarray::{Array1, ArrayView1};
use ndarray_stats::QuantileExt;

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

pub struct VariableAutoscale {
    values: Array1<f64>,
}

impl VariableAutoscale {
    pub fn new(values: Array1<f64>) -> Self {
        Self { values }
    }
}

impl HasValues for VariableAutoscale {
    fn values(&self) -> ArrayView1<f64> {
        self.values.view()
    }
}

impl HasLength for VariableAutoscale {}

impl Rescalable for VariableAutoscale {
    fn rescale(&self) -> Array1<f64> {
        // TODO: reuse when implementing VariableInvertedAutoscale
        let shift = self.values.min().unwrap();
        let scaling_factor = 1.0 / (self.values.max().unwrap() - shift);
        rescale_vector(self.values.view(), *shift, scaling_factor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;

    #[test]
    fn create_variable_with_single_value_and_rescale() {
        let var = VariableAutoscale::new(array![0.]);
        assert_eq!(var.length(), 1);
        // assert_eq!(var.rescale(), array![0.]);
        // TODO: handle the single value case
    }

    #[test]
    fn create_variable_with_values_and_rescale() {
        let var = VariableAutoscale::new(array![0., 0.5, 1.0, 1.5]);
        assert_eq!(var.length(), 4);
        assert_eq!(
            var.rescale(),
            array![0., 0.3333333333333333, 0.6666666666666666, 1.0]
        );
        // TODO: assert almost equal
    }
}
