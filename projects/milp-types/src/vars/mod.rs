use crate::{utils::DisplayWrapper, LinearConstraint};
use std::fmt::{Debug, Display, Formatter};

mod display;

pub struct LinearVariable<T> {
    symbol: String,
    bound: LinearConstraint<T>,
}

/// Create a new variable with constraints
impl<T> LinearVariable<T> {
    /// Creates a new variable with no constraints.
    pub fn new<S>(symbol: S) -> Self
    where
        S: Into<String>,
    {
        Self { symbol: symbol.into(), bound: LinearConstraint::None }
    }
    /// Creates a new variable with the given lower bound.
    pub fn new_ge<S>(symbol: S, lower: T) -> Self
    where
        S: Into<String>,
    {
        Self { symbol: symbol.into(), bound: LinearConstraint::ge(lower) }
    }
    /// Creates a new variable with the given lower bound.
    pub fn new_geq<S>(symbol: S, lower: T) -> Self
    where
        S: Into<String>,
    {
        Self { symbol: symbol.into(), bound: LinearConstraint::geq(lower) }
    }
    /// Creates a new variable with the given upper bound.
    pub fn new_le<S>(symbol: S, upper: T) -> Self
    where
        S: Into<String>,
    {
        Self { symbol: symbol.into(), bound: LinearConstraint::le(upper) }
    }
    /// Creates a new variable with the given upper bound.
    pub fn new_leq<S>(symbol: S, upper: T) -> Self
    where
        S: Into<String>,
    {
        Self { symbol: symbol.into(), bound: LinearConstraint::leq(upper) }
    }
    /// Creates a new variable with the given lower and upper bounds.
    pub fn new_bounds<S>(symbol: S, bound: LinearConstraint<T>) -> Self
    where
        S: Into<String>,
    {
        Self { symbol: symbol.into(), bound }
    }
}

impl<T> LinearVariable<T> {
    pub fn symbol(&self) -> &str {
        &self.symbol
    }
}

impl<T: PartialOrd> LinearVariable<T> {
    pub fn contains(&self, other: &T) -> bool {
        self.bound.contains(other)
    }
}
