use crate::{LinearConstraint, LpResult};
use std::{
    collections::{BTreeMap, BTreeSet},
    ops::AddAssign,
};

pub struct LinearEquation<T> {
    coefficients: BTreeMap<String, LinearCoefficient<T>>,
    constraint: LinearConstraint<T>,
}

pub struct LinearCoefficient<T> {
    symbol: String,
    coefficients: T,
}

impl<T> LinearCoefficient<T> {}

impl<T> LinearEquation<T> {
    pub fn new(constraint: LinearConstraint<T>) -> Self {
        Self { coefficients: BTreeMap::new(), constraint }
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
}
