//! This module define a problem as a matrix of variables
use super::vector::index_of_best_vector;
use crate::variables::{Name, VariableProperties};
use ndarray::{Array1, Array2, ArrayView1, Axis, stack};
use std::collections::BTreeMap;

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum SolvableProblemError {
    #[error("The problem cannot be empty")]
    Empty,
    #[error("All variables must have the same number of values")]
    VariableSizeMismatch,
    #[error("Each variable must have an unique name. You cannot redefine existing variables")]
    RedefinitionVariable,
}

#[derive(Debug)]
pub struct SolvableProblem {
    variables: BTreeMap<Name, Box<dyn VariableProperties>>,
}

impl SolvableProblem {
    fn new() -> Self {
        Self {
            variables: BTreeMap::new(),
        }
    }

    pub fn define(
        variables: Vec<Box<dyn VariableProperties>>,
    ) -> Result<Self, SolvableProblemError> {
        // an empty problem is not allowed
        if variables.is_empty() {
            return Err(SolvableProblemError::Empty);
        }

        let mut problem = SolvableProblem::new();
        let mut known_size: Option<usize> = Option::None;

        for variable in variables {
            // variable names must be unique
            if problem.variables.contains_key(variable.name()) {
                return Err(SolvableProblemError::RedefinitionVariable);
            }

            // all variables must have the same number of values
            let current_size = variable.values().values().len();
            if known_size.is_none() {
                known_size = Some(current_size)
            } else if known_size.unwrap() != current_size {
                return Err(SolvableProblemError::VariableSizeMismatch);
            }
            problem.add_variable(variable);
        }
        Ok(problem)
    }

    fn add_variable(&mut self, variable: Box<dyn VariableProperties>) -> usize {
        self.variables.insert(variable.name().clone(), variable);
        self.variables.len()
    }

    fn get_problem_matrix(&self) -> Array2<f64> {
        let rows: Vec<Array1<f64>> = self
            .variables
            .values()
            .map(|v| v.rescale().values().to_owned())
            .collect();
        let views: Vec<ArrayView1<f64>> = rows.iter().map(|a| a.view()).collect();
        let matrix = stack(Axis(0), &views).expect("Stack failed");
        matrix.reversed_axes()
    }

    pub fn solve(&self) -> usize {
        let matrix = self.get_problem_matrix();
        index_of_best_vector(matrix.view())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::variables::{Name, Values, VariableAutoscale, VariableInvertedAutoscale};
    use ndarray::array;
    use std::convert::TryFrom;

    fn create_test_problem() -> SolvableProblem {
        SolvableProblem::define(vec![
            Box::new(VariableAutoscale::new(
                Name::try_from("x").unwrap(),
                Values::try_from(array![1., 2., 3.]).unwrap(),
            )),
            Box::new(VariableInvertedAutoscale::new(
                Name::try_from("y").unwrap(),
                Values::try_from(array![3., 4., 5.]).unwrap(),
            )),
        ])
        .unwrap()
    }

    #[test]
    fn test_problem_matrix_is_rescaled() {
        let p = create_test_problem();
        assert_eq!(
            p.get_problem_matrix(),
            array![
                //x  y   (rescaled)
                [0., 1.],
                [0.5, 0.5],
                [1., 0.],
            ]
        );
    }

    #[test]
    fn test_problem_is_solved() {
        let p = create_test_problem();
        assert_eq!(p.solve(), 1,)
    }

    #[test]
    fn solve_problem_with_single_value() {
        let p = SolvableProblem::define(vec![Box::new(VariableAutoscale::new(
            Name::try_from("x").unwrap(),
            Values::try_from(array![1.,]).unwrap(),
        ))])
        .unwrap();
        assert_eq!(p.solve(), 0,)
    }

    #[test]
    fn test_add_variable() {
        let mut p = SolvableProblem::new();
        assert_eq!(
            p.add_variable(Box::new(VariableAutoscale::new(
                Name::try_from("x").unwrap(),
                Values::try_from(array![1., 2., 3.]).unwrap()
            ))),
            1
        );
        assert_eq!(
            p.add_variable(Box::new(VariableInvertedAutoscale::new(
                Name::try_from("y").unwrap(),
                Values::try_from(array![3., 4., 5.]).unwrap(),
            ))),
            2
        );
    }

    #[test]
    fn define_problem_empty_failure() {
        let err = SolvableProblem::define(vec![]).unwrap_err();
        assert_eq!(err, SolvableProblemError::Empty)
    }

    #[test]
    fn define_problem_redefine_variable_failure() {
        let err = SolvableProblem::define(vec![
            Box::new(VariableAutoscale::new(
                Name::try_from("x").unwrap(),
                Values::try_from(array![1., 2., 3.]).unwrap(),
            )),
            Box::new(VariableInvertedAutoscale::new(
                Name::try_from("x").unwrap(),
                Values::try_from(array![3., 4., 5.]).unwrap(),
            )),
        ])
        .unwrap_err();
        assert_eq!(err, SolvableProblemError::RedefinitionVariable)
    }

    #[test]
    fn define_problem_variable_with_different_size_failure() {
        let err = SolvableProblem::define(vec![
            Box::new(VariableAutoscale::new(
                Name::try_from("x").unwrap(),
                Values::try_from(array![1., 2., 3.]).unwrap(),
            )),
            Box::new(VariableInvertedAutoscale::new(
                Name::try_from("y").unwrap(),
                Values::try_from(array![3., 4.]).unwrap(),
            )),
        ])
        .unwrap_err();
        assert_eq!(err, SolvableProblemError::VariableSizeMismatch)
    }
}
