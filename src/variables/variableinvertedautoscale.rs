//! This module implements the basic autoscaled vectorized variables.
use super::Name;
use super::Values;
use super::VariableProperties;
use super::scaling::autorescale_vector;

pub struct VariableInvertedAutoscale {
    name: Name,
    values: Values,
}

impl VariableInvertedAutoscale {
    pub fn new(name: Name, values: Values) -> Self {
        Self { name, values }
    }
}

impl VariableProperties for VariableInvertedAutoscale {
    fn name(&self) -> &Name {
        &self.name
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
        let var = VariableInvertedAutoscale::new(
            Name::try_from("x").unwrap(),
            Values::try_from(array![0.]).unwrap(),
        );
        assert_eq!(var.name(), "x");
        assert_eq!(var.rescale().values(), array![0.]);
    }

    #[test]
    fn create_variable_with_values_and_rescale() {
        let var = VariableInvertedAutoscale::new(
            Name::try_from("x").unwrap(),
            Values::try_from(array![0., 0.5, 1., 1.5]).unwrap(),
        );
        assert_eq!(var.name(), "x");
        assert_ulps_eq!(
            var.rescale().values(),
            array![1., 0.6666666666666667, 0.33333333333333337, 0.]
        );
    }
}
