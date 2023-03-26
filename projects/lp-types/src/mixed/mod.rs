use crate::{LinearConstraint, LinearEquation, LinearVariable};
use num::BigInt;
use std::fmt::{Debug, Display, Formatter};
mod convert;
mod display;

pub struct MixedConstraint {
    wrapper: LinearConstraint<MixedValue>,
}

pub struct MixedVariable {
    wrapper: LinearVariable<MixedValue>,
}

pub struct MixedEquation {
    wrapper: LinearEquation<MixedValue>,
}

pub enum MixedValue {
    Boolean(bool),
    Decimal(f64),
    Integer(BigInt),
}

impl MixedVariable {
    pub fn free<S>(symbol: S) -> Self
    where
        S: Into<String>,
    {
        Self { wrapper: LinearVariable::new(symbol) }
    }
    pub fn ge<S, V>(symbol: S, lower: V) -> Self
    where
        S: Into<String>,
        V: Into<MixedValue>,
    {
        Self { wrapper: LinearVariable::ge(symbol, lower.into()) }
    }
    pub fn geq<S, V>(symbol: S, lower: V) -> Self
    where
        S: Into<String>,
        V: Into<MixedValue>,
    {
        Self { wrapper: LinearVariable::geq(symbol, lower.into()) }
    }
    pub fn le<S, V>(symbol: S, upper: V) -> Self
    where
        S: Into<String>,
        V: Into<MixedValue>,
    {
        Self { wrapper: LinearVariable::le(symbol, upper.into()) }
    }
    pub fn leq<S, V>(symbol: S, upper: V) -> Self
    where
        S: Into<String>,
        V: Into<MixedValue>,
    {
        Self { wrapper: LinearVariable::leq(symbol, upper.into()) }
    }
    pub fn bounds<S, V>(symbol: S, bound: MixedConstraint) -> Self
    where
        S: Into<String>,
        V: Into<MixedValue>,
    {
        Self { wrapper: LinearVariable::bounds(symbol, bound.wrapper) }
    }
}
