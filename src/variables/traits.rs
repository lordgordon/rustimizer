//! This module defines the traites for the vectorized variables.
use super::Name;
use super::Values;

pub trait VariableProperties: std::fmt::Debug {
    fn name(&self) -> &Name;

    fn values(&self) -> &Values;

    fn rescale(&self) -> Values;
}
