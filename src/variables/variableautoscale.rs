//! This module implements the basic autoscaled vectorized variables.
use super::Name;
use super::Values;
use super::VariableProperties;
use super::scaling::autorescale_vector;

#[derive(Debug)]
pub struct VariableAutoscale {
    name: Name,
    values: Values,
}

impl VariableAutoscale {
    pub fn new(name: Name, values: Values) -> Self {
        Self { name, values }
    }
}

impl VariableProperties for VariableAutoscale {
    fn name(&self) -> &Name {
        &self.name
    }

    fn values(&self) -> &Values {
        &self.values
    }

    fn rescale(&self) -> Values {
        autorescale_vector(self.values(), false)
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
        let var = VariableAutoscale::new(
            Name::try_from("x").unwrap(),
            Values::try_from(array![0.]).unwrap(),
        );
        assert_eq!(var.name(), "x");
        assert_eq!(var.rescale().values(), array![0.]);
    }

    #[test]
    fn create_variable_with_values_and_rescale() {
        let var = VariableAutoscale::new(
            Name::try_from("x").unwrap(),
            Values::try_from(array![0., 0.5, 1., 1.5]).unwrap(),
        );
        assert_eq!(var.name(), "x");
        assert_ulps_eq!(
            var.rescale().values(),
            array![0., 0.3333333333333333, 0.6666666666666666, 1.]
        );
    }
}
