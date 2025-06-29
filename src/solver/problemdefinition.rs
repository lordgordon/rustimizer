use super::solvableproblem::SolvableProblem;
use crate::variables::{Name, Values, VariableAutoscale, VariableProperties}; // VariableInvertedAutoscale
use ndarray::Array1;
use polars::frame::row::Row;
use polars::prelude::*;
use std::convert::TryFrom;

pub enum ProblemDefinitionError {
    // #[error("")]
    // ErrorName,
}

#[derive(Debug)]
pub struct ProblemDefinition {
    raw_data: DataFrame,
    problem: SolvableProblem,
}

fn build_solvable_problem(df: &DataFrame) -> SolvableProblem {
    // TODO: we are hardcoding the assumption that the first column is the "id". Fix this.
    let selected = df.clone().lazy().select([all().exclude(["id"])]);
    let mut variables: Vec<Box<dyn VariableProperties>> = Vec::new();
    for column in selected.collect().unwrap().iter() {
        let name = column.name().to_string();
        let values = Array1::from(
            column
                .f64()
                .unwrap()
                .into_no_null_iter()
                .collect::<Vec<f64>>(),
        );
        // TODO let values: Array1<f64> = series.f64()?.to_ndarray()?;

        // TODO: for each variable type, build the proper struct
        variables.push(Box::new(VariableAutoscale::new(
            Name::try_from(name).unwrap(),
            Values::try_from(Array1::from(values)).unwrap(),
        )));
    }
    SolvableProblem::define(variables).unwrap()
}

impl ProblemDefinition {
    pub fn new(df: DataFrame) -> Self {
        // TODO: validations
        //  - list of variables must match list of columns
        //  - must have id column (any type)
        //  - drop nans (customize behavior: soft remove, hard fail if nans)
        //  - save to solvableproblem to validate
        let p = build_solvable_problem(&df);
        Self {
            raw_data: df,
            problem: p,
        }
    }

    pub fn solve(&self) -> Row {
        // TODO: return a better structure with the variable name for each value
        self.raw_data.get_row(self.problem.solve()).unwrap()
        // TODO: slice instead of get_row
    }
}

#[cfg(test)]
mod tests {
    use polars::prelude::{AnyValue, df};

    use super::*;

    fn create_test_problem() -> ProblemDefinition {
        ProblemDefinition::new(
            df!(
                "id" => ["choice1", "choice2", "choice3"],
                "var1" => [2., 1., 3.],
                "var2" => [5., 4., 6.],
            )
            .unwrap(),
        )
    }

    #[test]
    fn test_problem_solution() {
        let p = create_test_problem();
        assert_eq!(
            p.solve().0,
            vec![
                AnyValue::String("choice2"),
                AnyValue::Float64(1.),
                AnyValue::Float64(4.)
            ]
        );
    }
}
