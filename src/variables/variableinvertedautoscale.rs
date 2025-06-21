//! This module implements the basic autoscaled vectorized variables.
use super::scaling::autorescale_vector;
use super::traits::VariableProperties;
use super::values::Values;

pub struct VariableInvertedAutoscale {
    name: String,
    values: Values,
}

impl VariableInvertedAutoscale {
    pub fn new(name: String, values: Values) -> Self {
        Self { name, values }
    }
}

impl VariableProperties for VariableInvertedAutoscale {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn values(&self) -> &Values {
        &self.values
    }

    fn rescale(&self) -> Values {
        autorescale_vector(self.values(), true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_ulps_eq;
    use ndarray::array;
    use std::convert::TryFrom;

    #[test]
    fn create_variable_with_single_value_and_rescale() {
        let var =
            VariableInvertedAutoscale::new("x".to_string(), Values::try_from(array![0.]).unwrap());
        assert_eq!(var.name(), "x");
        // assert_eq!(var.rescale(), array![0.]);
    }

    #[test]
    fn create_variable_with_values_and_rescale() {
        let var = VariableInvertedAutoscale::new(
            "x".to_string(),
            Values::try_from(array![0., 0.5, 1., 1.5]).unwrap(),
        );
        assert_eq!(var.name(), "x");
        assert_ulps_eq!(
            var.rescale().values(),
            array![1., 0.6666666666666667, 0.33333333333333337, 0.]
        );
    }
}
