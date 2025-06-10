//! This module implements the basic autoscaled vectorized variables.
use crate::variables::scaling::autorescale_vector;
use crate::variables::traits::{HasLength, HasValues, Rescalable};
use ndarray::{Array1, ArrayView1};

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
        autorescale_vector(self.values.view())
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
