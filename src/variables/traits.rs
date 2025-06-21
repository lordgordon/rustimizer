//! This module defines the traites for the vectorized variables.
use super::values::Values;

pub trait VariableProperties {
    fn name(&self) -> &str;

    fn values(&self) -> &Values;

    fn rescale(&self) -> Values;
}
