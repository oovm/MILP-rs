use crate::{utils::DisplayWrapper, LinearConstraint};
use num::{BigInt, BigUint};
use std::fmt::{Debug, Display, Formatter};

mod convert;
mod display;

pub struct LinearVariable<T> {
    kind: LinearVariableKind,
    symbol: String,
    bound: LinearConstraint<T>,
}

#[derive(Debug)]
pub enum LinearVariableKind {
    Boolean,
    Decimal,
    Integer,
}

/// Create a new variable with constraints
impl<T> LinearVariable<T> {
    /// Creates a new variable with no constraints.
    pub fn new<S>(symbol: S) -> Self
    where
        S: Into<String>,
    {
        Self { kind: LinearVariableKind::Decimal, symbol: symbol.into(), bound: LinearConstraint::None }
    }
    /// Creates a new variable with the given lower bound.
    pub fn ge<S>(symbol: S, lower: T) -> Self
    where
        S: Into<String>,
    {
        Self { kind: LinearVariableKind::Decimal, symbol: symbol.into(), bound: LinearConstraint::ge(lower) }
    }
    /// Creates a new variable with the given lower bound.
    pub fn geq<S>(symbol: S, lower: T) -> Self
    where
        S: Into<String>,
    {
        Self { kind: LinearVariableKind::Decimal, symbol: symbol.into(), bound: LinearConstraint::geq(lower) }
    }
    /// Creates a new variable with the given upper bound.
    pub fn le<S>(symbol: S, upper: T) -> Self
    where
        S: Into<String>,
    {
        Self { kind: LinearVariableKind::Decimal, symbol: symbol.into(), bound: LinearConstraint::le(upper) }
    }
    /// Creates a new variable with the given upper bound.
    pub fn leq<S>(symbol: S, upper: T) -> Self
    where
        S: Into<String>,
    {
        Self { kind: LinearVariableKind::Decimal, symbol: symbol.into(), bound: LinearConstraint::leq(upper) }
    }
    /// Creates a new variable with the given lower and upper bounds.
    pub fn bounds<S>(symbol: S, bound: LinearConstraint<T>) -> Self
    where
        S: Into<String>,
    {
        Self { kind: LinearVariableKind::Decimal, symbol: symbol.into(), bound }
    }
}

impl<T> LinearVariable<T> {
    pub fn get_symbol(&self) -> &str {
        &self.symbol
    }
    pub fn get_kind(&self) -> &LinearVariableKind {
        &self.kind
    }

    pub fn set_kind(&mut self, kind: LinearVariableKind) {
        self.kind = kind;
    }

    pub fn with_kind(mut self, kind: LinearVariableKind) -> Self {
        self.kind = kind;
        self
    }
}

impl<T: PartialOrd> LinearVariable<T> {
    pub fn contains(&self, other: &T) -> bool {
        self.bound.contains(other)
    }
}
