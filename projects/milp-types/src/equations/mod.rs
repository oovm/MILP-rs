use crate::{LinearConstraint, LpResult};
use std::{collections::BTreeMap, ops::AddAssign};

#[derive(Debug)]
pub struct LinearEquation<T> {
    coefficients: BTreeMap<String, LinearCoefficient<T>>,
    constraint: LinearConstraint<T>,
}

#[derive(Debug)]
pub struct LinearCoefficient<T> {
    symbol: String,
    coefficients: T,
}

impl<T> LinearCoefficient<T> {}

impl<T> LinearEquation<T> {
    pub fn new(constraint: LinearConstraint<T>) -> LpResult<Self> {
        Ok(Self { coefficients: BTreeMap::new(), constraint })
    }
    pub fn add_coefficient(&mut self, coefficient: T, symbol: &str)
    where
        T: AddAssign,
    {
        match self.coefficients.get_mut(symbol) {
            Some(s) => {
                s.coefficients += coefficient;
            }
            None => {
                self.coefficients
                    .insert(symbol.to_string(), LinearCoefficient { symbol: symbol.to_string(), coefficients: coefficient });
            }
        }
    }
    pub fn variables(&self) -> impl Iterator<Item = &str> {
        self.coefficients.keys().map(|s| s.as_str())
    }
}
