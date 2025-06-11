//! This module implements the basic autoscaled vectorized variables.
use crate::variables::scaling::autorescale_vector;
use crate::variables::traits::{HasLength, HasValues, Rescalable};
use ndarray::{Array1, ArrayView1};

pub struct VariableInvertedAutoscale {
    values: Array1<f64>,
}

impl VariableInvertedAutoscale {
    pub fn new(values: Array1<f64>) -> Self {
        // TODO: guarantee that the array is not empty
        Self { values }
    }
}

impl HasValues for VariableInvertedAutoscale {
    fn values(&self) -> ArrayView1<f64> {
        self.values.view()
    }
}

impl HasLength for VariableInvertedAutoscale {}

impl Rescalable for VariableInvertedAutoscale {
    fn rescale(&self) -> Array1<f64> {
        autorescale_vector(self.values.view(), true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;

    #[test]
    fn create_variable_with_single_value_and_rescale() {
        let var = VariableInvertedAutoscale::new(array![0.]);
        assert_eq!(var.length(), 1);
        // assert_eq!(var.rescale(), array![0.]);
    }

    #[test]
    fn create_variable_with_values_and_rescale() {
        let var = VariableInvertedAutoscale::new(array![0., 0.5, 1., 1.5]);
        assert_eq!(var.length(), 4);
        assert_eq!(
            var.rescale(),
            array![1., 0.6666666666666667, 0.33333333333333337, 0.]
        );
    }
}
