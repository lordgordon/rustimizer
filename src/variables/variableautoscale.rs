//! This module implements the basic autoscaled vectorized variables.
use crate::variables::scaling::autorescale_vector;
use crate::variables::traits::VariableProperties;
use ndarray::{Array1, ArrayView1};

pub struct VariableAutoscale {
    name: String,
    values: Array1<f64>,
}

impl VariableAutoscale {
    pub fn new(name: String, values: Array1<f64>) -> Self {
        // TODO: guarantee that the array is not empty
        // TODO: validate name
        Self { name, values }
    }
}

impl VariableProperties for VariableAutoscale {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn values(&self) -> ArrayView1<f64> {
        self.values.view()
    }

    fn rescale(&self) -> Array1<f64> {
        autorescale_vector(self.values.view(), false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_ulps_eq;
    use ndarray::array;

    #[test]
    fn create_variable_with_single_value_and_rescale() {
        let var = VariableAutoscale::new("x".to_string(), array![0.]);
        assert_eq!(var.length(), 1);
        assert_eq!(var.name(), "x");
        // assert_eq!(var.rescale(), array![0.]);
        // TODO: handle the single value case
    }

    #[test]
    fn create_variable_with_values_and_rescale() {
        let var = VariableAutoscale::new("x".to_string(), array![0., 0.5, 1., 1.5]);
        assert_eq!(var.length(), 4);
        assert_eq!(var.name(), "x");
        assert_ulps_eq!(
            var.rescale(),
            array![0., 0.3333333333333333, 0.6666666666666666, 1.]
        );
    }
}
