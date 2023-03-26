use lp_types::{LinearConstraint, LinearEquation, LinearVariable, LpResult};
use num::{BigInt, BigUint};
use std::fmt::{Debug, Display, Formatter};
mod arithmetic;
mod convert;
mod display;

pub enum MixedValue {
    Boolean(bool),
    Decimal(f64),
    Integer(BigInt),
}

pub struct MixedConstraint {
    wrapper: LinearConstraint<MixedValue>,
}

pub struct MixedVariable {
    wrapper: LinearVariable<MixedValue>,
}

pub struct MixedEquation {
    wrapper: LinearEquation<MixedValue>,
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
    pub fn bounds<S, V>(symbol: S, bound: MixedConstraint) -> LpResult<Self>
    where
        S: AsRef<str>,
        V: Into<MixedValue>,
    {
        Ok(Self { wrapper: LinearVariable::bounds(symbol, bound.wrapper)? })
    }
    pub fn get_symbol(&self) -> &str {
        self.wrapper.get_symbol()
    }
    // pub fn get_kind(&self) -> &MixedVariableKind {
    //     self.wrapper.get_kind()
    // }
    // pub fn get_bound(&self) -> &MixedConstraint {
    //     self.wrapper.get_bound()
    // }
}

impl MixedEquation {
    pub fn new(constraint: MixedConstraint) -> LpResult<Self> {
        Ok(Self { wrapper: LinearEquation::new(constraint.wrapper)? })
    }
    pub fn variables(&self) -> impl Iterator<Item = &str> {
        self.wrapper.variables()
    }
    pub fn add_coefficient<V>(&mut self, coefficient: V, symbol: &str)
    where
        V: Into<MixedValue>,
    {
        self.wrapper.add_coefficient(coefficient.into(), symbol)
    }
    pub fn get_coefficients(&self) -> impl Iterator<Item = (&str, &MixedValue)> {
        self.wrapper.get_coefficients()
    }
}

impl MixedConstraint {
    pub fn ge<V>(lower: V) -> Self
    where
        V: Into<MixedValue>,
    {
        Self { wrapper: LinearConstraint::ge(lower.into()) }
    }
    pub fn geq<V>(lower: V) -> Self
    where
        V: Into<MixedValue>,
    {
        Self { wrapper: LinearConstraint::geq(lower.into()) }
    }
    pub fn le<V>(upper: V) -> Self
    where
        V: Into<MixedValue>,
    {
        Self { wrapper: LinearConstraint::le(upper.into()) }
    }
    pub fn leq<V>(upper: V) -> Self
    where
        V: Into<MixedValue>,
    {
        Self { wrapper: LinearConstraint::leq(upper.into()) }
    }
    pub fn contains(&self, variable: &MixedValue) -> bool {
        self.wrapper.contains(variable)
    }
}
