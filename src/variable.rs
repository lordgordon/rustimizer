//! This module implements the definition of a vectorized variables.
use ndarray::Array1;

pub struct Variable {
    values: Array1<f64>,
}

impl Variable {
    pub fn new(values: Array1<f64>) -> Self {
        Self { values }
    }

    pub fn length(self) -> usize {
        self.values.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;

    #[test]
    fn create_variable_with_single_value() {
        let var = Variable::new(array![0.]);
        assert_eq!(var.length(), 1);
    }
}
