//! This module define a problem as a matrix of variables
use super::vector::index_of_best_vector;
use crate::variables::VariableProperties;
use ndarray::{Array1, Array2, ArrayView1, Axis, stack};
use std::collections::BTreeMap;

pub struct Problem {
    variables: BTreeMap<String, Box<dyn VariableProperties>>,
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

    pub fn add_variable<T: VariableProperties + 'static>(&mut self, variable: T) -> usize {
        // TODO: all variables must have the same number of values
        // TODO: fail if the variable already exists
        self.variables
            .insert(variable.name().to_string(), Box::new(variable));
        self.variables.len()
    }

    fn get_problem_matrix(&self) -> Array2<f64> {
        let rows: Vec<Array1<f64>> = self.variables.values().map(|v| v.rescale()).collect();
        let views: Vec<ArrayView1<f64>> = rows.iter().map(|a| a.view()).collect();
        let matrix = stack(Axis(0), &views).expect("Stack failed");
        matrix.reversed_axes()
    }

    pub fn solve(&self) -> usize {
        // TODO: should return the whole vector
        let matrix = self.get_problem_matrix();
        index_of_best_vector(matrix.view())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::variables::VariableAutoscale;
    use crate::variables::VariableInvertedAutoscale;
    use ndarray::array;

    fn create_test_problem() -> Problem {
        let mut p = Problem::default();
        p.add_variable(VariableAutoscale::new("x".to_string(), array![1., 2., 3.]));
        p.add_variable(VariableInvertedAutoscale::new(
            "y".to_string(),
            array![3., 4., 5.],
        ));
        p
    }

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

    #[test]
    fn problem_matrix_is_rescaled() {
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
    fn problem_is_solved() {
        let p = create_test_problem();
        assert_eq!(p.solve(), 1,)
    }
}
