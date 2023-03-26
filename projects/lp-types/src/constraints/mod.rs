mod display;
use crate::{LpError, LpResult};
use std::fmt::{Display, Formatter, Write};

mod convert;
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OptimizeDirection {
    Maximize,
    Minimize,
}

#[derive(Debug)]
pub enum LinearConstraint<T> {
    None,
    Greater { lower: T, inclusive: bool },
    Less { upper: T, inclusive: bool },
    Equal { value: T },
    NotEqual { value: T },
    Or { left: Box<LinearConstraint<T>>, right: Box<LinearConstraint<T>> },
    And { left: Box<LinearConstraint<T>>, right: Box<LinearConstraint<T>> },
}

impl<T> Default for LinearConstraint<T> {
    fn default() -> Self {
        LinearConstraint::None
    }
}

impl<T> LinearConstraint<T> {
    pub fn ge(lower: T) -> Self {
        LinearConstraint::Greater { lower, inclusive: false }
    }
    pub fn geq(lower: T) -> Self {
        LinearConstraint::Greater { lower, inclusive: true }
    }
    pub fn le(upper: T) -> Self {
        LinearConstraint::Less { upper, inclusive: false }
    }
    pub fn leq(upper: T) -> Self {
        LinearConstraint::Less { upper, inclusive: true }
    }
    pub fn eq(value: T) -> Self {
        LinearConstraint::Equal { value }
    }
    pub fn ne(value: T) -> Self {
        LinearConstraint::NotEqual { value }
    }
}

impl<T> LinearConstraint<T> {
    pub fn check_variable(&self) -> LpResult<()> {
        match self {
            LinearConstraint::None => {}
            LinearConstraint::Greater { .. } => {}
            LinearConstraint::Less { .. } => {}
            LinearConstraint::Equal { .. } | LinearConstraint::NotEqual { .. } => {
                Err(LpError::invalid_constraint("Variables cannot constrained by equal or not equal"))?
            }
            LinearConstraint::Or { left, right } => {
                left.check_variable()?;
                right.check_variable()?;
            }
            LinearConstraint::And { left, right } => {
                left.check_variable()?;
                right.check_variable()?;
            }
        }
        Ok(())
    }
}

impl<T> LinearConstraint<T> {
    pub fn contains(&self, value: &T) -> bool
    where
        T: PartialOrd,
    {
        match self {
            LinearConstraint::None => true,
            LinearConstraint::Greater { lower, inclusive } => {
                if *inclusive {
                    value >= lower
                }
                else {
                    value > lower
                }
            }
            LinearConstraint::Less { upper, inclusive } => {
                if *inclusive {
                    value <= upper
                }
                else {
                    value < upper
                }
            }
            LinearConstraint::Equal { value: v } => value == v,
            LinearConstraint::NotEqual { value: v } => value != v,
            LinearConstraint::Or { left, right } => left.contains(value) || right.contains(value),
            LinearConstraint::And { left, right } => left.contains(value) && right.contains(value),
        }
    }
}
