//! This module define a problem as a matrix of variables
use crate::variables::traits::{GenericVariableProperties, Variable};
use std::collections::BTreeMap;
// TODO: reorganize modules for better visibility

pub struct Problem {
    variables: BTreeMap<String, Box<Variable>>,
}

impl Default for Problem {
    fn default() -> Self {
        Self::new()
    }
}

impl Problem {
    pub fn new() -> Self {
        Self {
            variables: BTreeMap::new(),
        }
    }

    pub fn add_variable<T: GenericVariableProperties + 'static>(&mut self, variable: T) -> usize {
        // TODO: all variables must have the same number of values
        // TODO: fail if the variable already exists
        self.variables
            .insert(variable.name().to_string(), Box::new(variable));
        self.variables.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::variables::variableautoscale::VariableAutoscale;
    use crate::variables::variableinvertedautoscale::VariableInvertedAutoscale;
    use ndarray::array;

    #[test]
    fn create_empty_problem() {
        Problem::default();
    }

    #[test]
    fn create_problem_with_variables() {
        let mut p = Problem::default();
        assert_eq!(
            p.add_variable(VariableAutoscale::new("x".to_string(), array![1., 2., 3.])),
            1
        );
        assert_eq!(
            p.add_variable(VariableInvertedAutoscale::new(
                "y".to_string(),
                array![3., 4., 5.]
            )),
            2
        );
    }
}
